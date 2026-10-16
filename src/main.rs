use std::fs;
use std::env;

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

fn main() {
    // 1. Значення за замовчуванням (якщо конфігу немає)
    let mut logo_cfg = "auto".to_string();
    let mut show_os = true;
    let mut show_kernel = true;
    let mut os_label = "os".to_string();
    let mut kernel_label = "kernel".to_string();

    // 2. Читаємо ~/.config/ritch/config.nix (або створюємо дефолтний)
    if let Ok(home) = env::var("HOME") {
        let config_dir = format!("{home}/.config/ritch");
        let config_path = format!("{config_dir}/config.nix");

        if !fs::exists(&config_path).unwrap_or(false) {
            let _ = fs::create_dir_all(&config_dir);
            let _ = fs::write(&config_path, DEFAULT_CONFIG);
        }

        if let Ok(cfg_content) = fs::read_to_string(&config_path) {
            show_os = false;
            show_kernel = false;

            for line in cfg_content.lines() {
                // Відрізаємо коментарі після '#'
                let clean = line.split('#').next().unwrap_or("").trim();
                // Розбираємо рядки виду: ключ = значення;
                if let Some((key, val)) = clean.split_once('=') {
                    let key = key.trim();
                    let val = val.trim().trim_end_matches(';').trim().trim_matches('"');

                    match key {
                        "logo" => logo_cfg = val.to_string(),
                        "showOs" => show_os = val == "true",
                        "showKernel" => show_kernel = val == "true",
                        "osLabel" => os_label = val.to_string(),
                        "kernelLabel" => kernel_label = val.to_string(),
                        _ => {}
                    }
                }
            }
        }
    }

    // 3. Визначаємо ID дистрибутива (пріоритет: --logo -> config.nix -> /etc/os-release)
    let args: Vec<String> = env::args().collect();

    let id: String = if args.len() >= 3 && args[1] == "--logo" {
        args[2].clone()
    } else if logo_cfg != "auto" && !logo_cfg.is_empty() {
        logo_cfg
    } else {
        let content = fs::read_to_string("/etc/os-release")
            .expect("Не вдалось відкрити файл, перевірте чи файл є чи він не пошкоджений");
        let mut fount = "unknown".to_string();
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("ID=") {
                fount = val.trim_matches('"').to_string();
                break;
            }
        }
        fount
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
