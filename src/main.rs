use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

use rnix::ast::{self, BinOpKind, HasEntry, InterpolPart, UnaryOpKind};

// ANSI кольори
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";
const WHITE: &str = "\x1b[37m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";

// Дефолтний вміст config.nix, який створюється автоматично
const DEFAULT_CONFIG: &str = r#"# Конфігурація ritch (Nix формат)
{
  # Вибір логотипу:
  # "auto", "nixos", "arch", "debian", "fedora", "void",
  # "ubuntu", "gentoo", "mint", "manjaro", "alpine", "antigravity"
  logo = "auto";

  # Вмикання / вимикання інфо-рядків (true або false, або просто видали рядок)
  showOs = true;
  showKernel = true;

  # Кастомні підписи біля іконок
  osLabel = "os";
  kernelLabel = "kernel";
}
"#;

#[derive(Clone, Debug, PartialEq, Eq)]
enum NixValue {
    Str(String),
    Bool(bool),
}

fn eval_expr(expr: &ast::Expr, vars: &HashMap<String, NixValue>) -> Option<NixValue> {
    match expr {
        ast::Expr::Str(s) => {
            let mut result = String::new();
            for part in s.normalized_parts() {
                match part {
                    InterpolPart::Literal(lit) => result.push_str(&lit),
                    InterpolPart::Interpolation(interp) => {
                        if let Some(inner) = interp.expr() {
                            match eval_expr(&inner, vars)? {
                                NixValue::Str(val) => result.push_str(&val),
                                NixValue::Bool(b) => result.push_str(if b { "true" } else { "false" }),
                            }
                        }
                    }
                }
            }
            Some(NixValue::Str(result))
        }
        ast::Expr::Ident(ident) => {
            let token = ident.ident_token()?;
            match token.text() {
                "true" => Some(NixValue::Bool(true)),
                "false" => Some(NixValue::Bool(false)),
                other => vars.get(other).cloned(),
            }
        }
        ast::Expr::Paren(paren) => paren.expr().and_then(|e| eval_expr(&e, vars)),
        ast::Expr::IfElse(ifelse) => {
            let cond = eval_expr(&ifelse.condition()?, vars)?;
            if matches!(cond, NixValue::Bool(true)) {
                eval_expr(&ifelse.body()?, vars)
            } else {
                eval_expr(&ifelse.else_body()?, vars)
            }
        }
        ast::Expr::UnaryOp(unary) => match (unary.operator()?, eval_expr(&unary.expr()?, vars)?) {
            (UnaryOpKind::Invert, NixValue::Bool(b)) => Some(NixValue::Bool(!b)),
            _ => None,
        },
        ast::Expr::BinOp(bin) => {
            let lhs = eval_expr(&bin.lhs()?, vars)?;
            let rhs = eval_expr(&bin.rhs()?, vars)?;
            match (bin.operator()?, lhs, rhs) {
                (BinOpKind::Equal, a, b) => Some(NixValue::Bool(a == b)),
                (BinOpKind::NotEqual, a, b) => Some(NixValue::Bool(a != b)),
                (BinOpKind::Add, NixValue::Str(a), NixValue::Str(b)) => Some(NixValue::Str(a + &b)),
                (BinOpKind::And, NixValue::Bool(a), NixValue::Bool(b)) => Some(NixValue::Bool(a && b)),
                (BinOpKind::Or, NixValue::Bool(a), NixValue::Bool(b)) => Some(NixValue::Bool(a || b)),
                (BinOpKind::Implication, NixValue::Bool(a), NixValue::Bool(b)) => {
                    Some(NixValue::Bool(!a || b))
                }
                _ => None,
            }
        }
        ast::Expr::Apply(apply) => {
            let func = apply.lambda()?.to_string();
            let arg = eval_expr(&apply.argument()?, vars)?;
            match (func.trim(), arg) {
                ("builtins.getEnv" | "getEnv", NixValue::Str(var)) => {
                    Some(NixValue::Str(env::var(&var).unwrap_or_default()))
                }
                ("toString" | "builtins.toString", NixValue::Str(s)) => Some(NixValue::Str(s)),
                ("toString" | "builtins.toString", NixValue::Bool(b)) => {
                    Some(NixValue::Str(if b { "1".to_string() } else { String::new() }))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn attr_name(attrpath: &ast::Attrpath, vars: &HashMap<String, NixValue>) -> Option<String> {
    let attr = attrpath.attrs().next()?;
    match attr {
        ast::Attr::Ident(ident) => Some(ident.ident_token()?.text().to_string()),
        ast::Attr::Str(s) => match eval_expr(&ast::Expr::Str(s), vars)? {
            NixValue::Str(val) => Some(val),
            NixValue::Bool(_) => None,
        },
        ast::Attr::Dynamic(_) => None,
    }
}

fn parse_nix_ast(nix_src: &str) -> Option<HashMap<String, NixValue>> {
    let root = rnix::Root::parse(nix_src);
    let mut expr = root.tree().expr()?;
    let mut vars = HashMap::new();

    // Розгортаємо дужки, `with ...;` та `let ... in { ... }` у rnix
    loop {
        match expr {
            ast::Expr::Paren(paren) => expr = paren.expr()?,
            ast::Expr::With(with_expr) => expr = with_expr.body()?,
            ast::Expr::LetIn(let_in) => {
                for entry in let_in.attrpath_values() {
                    if let (Some(path), Some(val_expr)) = (entry.attrpath(), entry.value())
                        && let (Some(key), Some(val)) = (attr_name(&path, &vars), eval_expr(&val_expr, &vars))
                    {
                        vars.insert(key, val);
                    }
                }
                expr = let_in.body()?;
            }
            _ => break,
        }
    }

    let ast::Expr::AttrSet(set) = expr else {
        return None;
    };

    let is_rec = set.rec_token().is_some();
    let mut map = HashMap::new();
    for entry in set.attrpath_values() {
        if let (Some(path), Some(val_expr)) = (entry.attrpath(), entry.value())
            && let (Some(key), Some(val)) = (attr_name(&path, &vars), eval_expr(&val_expr, &vars))
        {
            if is_rec {
                vars.insert(key.clone(), val.clone());
            }
            map.insert(key, val);
        }
    }
    Some(map)
}

fn detect_system_distro() -> String {
    let content = fs::read_to_string("/etc/os-release")
        .expect("Не вдалось відкрити файл, перевірте чи файл є чи він не пошкоджений");
    for line in content.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            return val.trim_matches('"').to_string();
        }
    }
    "unknown".to_string()
}

fn main() {
    // 1. Визначаємо реальний дистрибутив системи з /etc/os-release
    let system_id = detect_system_distro();

    // 2. Значення за замовчуванням (якщо конфігу немає)
    let mut logo_cfg = "auto".to_string();
    let mut show_os = true;
    let mut show_kernel = true;
    let mut os_label = "os".to_string();
    let mut kernel_label = "kernel".to_string();

    // 3. Читаємо ~/.config/ritch/config.nix (або створюємо дефолтний)
    if let Ok(home) = env::var("HOME") {
        let config_dir = format!("{home}/.config/ritch");
        let config_path = format!("{config_dir}/config.nix");

        if !fs::exists(&config_path).unwrap_or(false) {
            let _ = fs::create_dir_all(&config_dir);
            let _ = fs::write(&config_path, DEFAULT_CONFIG);
        }

        // На NixOS використовуємо справжній інтерпретатор Nix (nix-instantiate),
        // а на інших дистрибутивах (або як фолбек) — парсер rnix
        let evaluated_nix = if system_id == "nixos" {
            Command::new("nix-instantiate")
                .args(["--eval", "--strict", &config_path])
                .output()
                .ok()
                .filter(|out| out.status.success())
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .or_else(|| fs::read_to_string(&config_path).ok())
        } else {
            fs::read_to_string(&config_path).ok()
        };

        if let Some(nix_src) = evaluated_nix
            && let Some(cfg_map) = parse_nix_ast(&nix_src)
        {
            show_os = false;
            show_kernel = false;

            for (key, val) in cfg_map {
                match (key.as_str(), val) {
                    ("logo", NixValue::Str(s)) => logo_cfg = s,
                    ("showOs", NixValue::Bool(b)) => show_os = b,
                    ("showKernel", NixValue::Bool(b)) => show_kernel = b,
                    ("osLabel", NixValue::Str(s)) => os_label = s,
                    ("kernelLabel", NixValue::Str(s)) => kernel_label = s,
                    _ => {}
                }
            }
        }
    }

    // 4. Визначаємо ID для логотипу (пріоритет: --logo -> config.nix -> /etc/os-release)
    let args: Vec<String> = env::args().collect();

    let id: String = if args.len() >= 3 && args[1] == "--logo" {
        args[2].clone()
    } else if logo_cfg != "auto" && !logo_cfg.is_empty() {
        logo_cfg
    } else {
        system_id
    };

    // 4. Читаємо версію ядра
    let kernel_raw = fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "unknown".to_string());
    let kernel = kernel_raw.trim();

    // 5. Виводимо логотип і отримуємо (колір, іконку, назву) для нижніх рядків
    let (color, icon, name) = if id == "nixos" {
        println!(
            "{CYAN}{BOLD}    _   ___       ____  _____{RESET}\n\
             {CYAN}{BOLD}   / | / (_)  __ / __ \\/ ___/{RESET}\n\
             {BLUE}{BOLD}  /  |/ / / |/_// / / /\\__ \\ {RESET}\n\
             {BLUE}{BOLD} / /|  / />  < / /_/ /___/ / {RESET}\n\
             {BLUE}{BOLD}/_/ |_/_/_/|_| \\____//____/  {RESET}\n\
             {WHITE}┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄{RESET}"
        );
        (CYAN, "\u{f313}", "NixOS")
    } else if id == "arch" {
        println!(
            "{BLUE}{BOLD}    ___             __    {RESET}\n\
             {BLUE}{BOLD}   /   |  __________/ /_   {RESET}\n\
             {BLUE}{BOLD}  / /| | / ___/ ___/ __ \\  {RESET}\n\
             {BLUE}{BOLD} / ___ |/ /  / /__/ / / /  {RESET}\n\
             {BLUE}{BOLD}/_/  |_/_/   \\___/_/ /_/   {RESET}\n\
             {WHITE}──────────────────────────────{RESET}"
        );
        (BLUE, "\u{f303}", "Arch Linux")
    } else if id == "debian" {
        println!(
            "{RED}{BOLD}    ____       __    _             {RESET}\n\
             {RED}{BOLD}   / __ \\___  / /_  (_)___ _____   {RESET}\n\
             {RED}{BOLD}  / / / / _ \\/ __ \\/ / __ `/ __ \\  {RESET}\n\
             {RED}{BOLD} / /_/ /  __/ /_/ / / /_/ / / / /  {RESET}\n\
             {RED}{BOLD}/_____/\\___/_.___/_/\\__,_/_/ /_/   {RESET}\n\
             {WHITE}──────────────────────────────{RESET}"
        );
        (RED, "\u{f306}", "Debian")
    } else if id == "fedora" {
        println!(
            "{BLUE}{BOLD}    ______          __                 {RESET}\n\
             {BLUE}{BOLD}   / ____/__  ____/ /___  _________ _  {RESET}\n\
             {BLUE}{BOLD}  / /_  / _ \\/ __  / __ \\/ ___/ __ `/  {RESET}\n\
             {BLUE}{BOLD} / __/ /  __/ /_/ / /_/ / /  / /_/ /   {RESET}\n\
             {BLUE}{BOLD}/_/    \\___/\\__,_/\\____/_/   \\__,_/    {RESET}\n\
             {WHITE}──────────────────────────────{RESET}"
        );
        (BLUE, "\u{f30a}", "Fedora")
    } else if id == "void" {
        println!(
            "{GREEN}{BOLD} _    __      _     __{RESET}\n\
             {GREEN}{BOLD}| |  / /___  (_)___/ /{RESET}\n\
             {GREEN}{BOLD}| | / / __ \\/ / __  / {RESET}\n\
             {GREEN}{BOLD}| |/ / /_/ / / /_/ /  {RESET}\n\
             {GREEN}{BOLD}|___/\\____/_/\\__,_/   {RESET}\n\
             {WHITE}──────────────────────────────{RESET}"
        );
        (GREEN, "\u{f32e}", "Void Linux")
    } else if id == "ubuntu" {
        println!(
            "{YELLOW}{BOLD}   __  ____               __       {RESET}\n\
             {YELLOW}{BOLD}  / / / / /_  __  ______  / /___  __{RESET}\n\
             {RED}{BOLD} / / / / __ \\/ / / / __ \\/ __/ / / /{RESET}\n\
             {RED}{BOLD}/ /_/ / /_/ / /_/ / / / / /_/ /_/ / {RESET}\n\
             {YELLOW}{BOLD}\\____/_.___/\\__,_/_/ /_/\\__/\\__,_/  {RESET}\n\
             {WHITE}──────────────────────────────────{RESET}"
        );
        (YELLOW, "\u{f31b}", "Ubuntu")
    } else if id == "gentoo" {
        println!(
            "{MAGENTA}{BOLD}   ______           __            {RESET}\n\
             {MAGENTA}{BOLD}  / ____/__  ____  / /_____  ____ {RESET}\n\
             {MAGENTA}{BOLD} / / __/ _ \\/ __ \\/ __/ __ \\/ __ \\{RESET}\n\
             {MAGENTA}{BOLD}/ /_/ /  __/ / / / /_/ /_/ / /_/ /{RESET}\n\
             {MAGENTA}{BOLD}\\____/\\___/_/ /_/\\__/\\____/\\____/ {RESET}\n\
             {WHITE}──────────────────────────────────{RESET}"
        );
        (MAGENTA, "\u{f30d}", "Gentoo")
    } else if id == "linuxmint" || id == "mint" {
        println!(
            "{GREEN}{BOLD}    __  ____       __ {RESET}\n\
             {GREEN}{BOLD}   /  |/  (_)___  / /_{RESET}\n\
             {GREEN}{BOLD}  / /|_/ / / __ \\/ __/{RESET}\n\
             {GREEN}{BOLD} / /  / / / / / / /_  {RESET}\n\
             {GREEN}{BOLD}/_/  /_/_/_/ /_/\\__/  {RESET}\n\
             {WHITE}──────────────────────────{RESET}"
        );
        (GREEN, "\u{f30e}", "Linux Mint")
    } else if id == "manjaro" {
        println!(
            "{GREEN}{BOLD}    __  ___              _                 {RESET}\n\
             {GREEN}{BOLD}   /  |/  /___ _____    (_)___ ___________ {RESET}\n\
             {GREEN}{BOLD}  / /|_/ / __ `/ __ \\  / / __ `/ ___/ __ \\{RESET}\n\
             {GREEN}{BOLD} / /  / / /_/ / / / / / / /_/ / /  / /_/ /{RESET}\n\
             {GREEN}{BOLD}/_/  /_/\\__,_/_/ /_/_/ /\\__,_/_/   \\____/ {RESET}\n\
             {GREEN}{BOLD}                  /___/                   {RESET}\n\
             {WHITE}──────────────────────────────────────────{RESET}"
        );
        (GREEN, "\u{f312}", "Manjaro")
    } else if id == "alpine" {
        println!(
            "{BLUE}{BOLD}    ___    __      _          {RESET}\n\
             {BLUE}{BOLD}   /   |  / /___  (_)___  ___ {RESET}\n\
             {CYAN}{BOLD}  / /| | / / __ \\/ / __ \\/ _ \\{RESET}\n\
             {CYAN}{BOLD} / ___ |/ / /_/ / / / / /  __/{RESET}\n\
             {BLUE}{BOLD}/_/  |_/_/ .___/_/_/ /_/\\___/ {RESET}\n\
             {BLUE}{BOLD}        /_/                   {RESET}\n\
             {WHITE}──────────────────────────────{RESET}"
        );
        (BLUE, "\u{f300}", "Alpine Linux")
    } else if id == "antigravity" {
        println!(
            "{RED}{BOLD}    ___         __  _ ______                 _ __        {RESET}\n\
             {YELLOW}{BOLD}   /   |  ____ / /_(_) ____/________ __   __(_) /___  __ {RESET}\n\
             {GREEN}{BOLD}  / /| | / __ \\/ __/ / / __/ ___/ __ `/ | / / / __/ / / /{RESET}\n\
             {CYAN}{BOLD} / ___ |/ / / / /_/ / /_/ / /  / /_/ /| |/ / / /_/ /_/ / {RESET}\n\
             {BLUE}{BOLD}/_/  |_/_/ /_/\\__/_/\\____/_/   \\__,_/ |___/_/\\__/\\__, /  {RESET}\n\
             {BLUE}{BOLD}                                                /____/   {RESET}\n\
             {WHITE}──────────────────────────────────────────────────────────{RESET}"
        );
        (CYAN, "\u{e8da} ", "AntiGravity")
    } else {
        println!(
            "{WHITE}{BOLD}    ____  _______________________ {RESET}\n\
             {WHITE}{BOLD}   / __ \\/  _/_  __/ ____/ / / /  {RESET}\n\
             {WHITE}{BOLD}  / /_/ // /  / / / /   / /_/ /   {RESET}\n\
             {WHITE}{BOLD} / _, _// /  / / / /___/ __  /    {RESET}\n\
             {WHITE}{BOLD}/_/ |_/___/ /_/  \\____/_/ /_/     {RESET}\n\
             {WHITE}----------------------------------{RESET}"
        );
        (WHITE, "\u{f0aff}", "Unknown")
    };

    // 6. Виводимо інфо-рядки згідно з config.nix
    if show_os {
        println!("  {color}{icon} {os_label:<6}{RESET} {BOLD}{name}{RESET}");
    }
    if show_kernel {
        println!("  {color}\u{f31a} {kernel_label:<6}{RESET} {BOLD}{kernel}{RESET}");
    }
}
