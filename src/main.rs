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

fn main() {

    let args: Vec<String> = env::args().collect();

    let id: String = if args.len() >= 3 && args[1] == "--logo" {
        args[2].clone()
    } else {
        let content = fs::read_to_string("/etc/os-release").expect("Не вдалос відкрити файл, перевірте чи файл є чи він не пошкоджений");
        let mut fount = "unknown".to_string();
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("ID=") {
                fount = val.trim_matches('"').to_string();
                break;
            }
        }
        fount
    };

    if id == ("nixos") {
        println!("{CYAN}{BOLD}    _   ___       ____  _____{RESET}");
        println!("{CYAN}{BOLD}   / | / (_)  __ / __ \\/ ___/{RESET}");
        println!("{BLUE}{BOLD}  /  |/ / / |/_// / / /\\__ \\ {RESET}");
        println!("{BLUE}{BOLD} / /|  / />  < / /_/ /___/ / {RESET}");
        println!("{BLUE}{BOLD}/_/ |_/_/_/|_| \\____//____/  {RESET}");
        println!("{WHITE}┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄{RESET}");
        println!("  {CYAN}\u{f313} os{RESET}     {BOLD}NixOS{RESET}");
     } else if id == ("arch") {
        println!("{BLUE}{BOLD}    ___             __    {RESET}");
        println!("{BLUE}{BOLD}   /   |  __________/ /_   {RESET}");
        println!("{BLUE}{BOLD}  / /| | / ___/ ___/ __ \\  {RESET}");
        println!("{BLUE}{BOLD} / ___ |/ /  / /__/ / / /  {RESET}");
        println!("{BLUE}{BOLD}/_/  |_/_/   \\___/_/ /_/   {RESET}");
        println!("{WHITE}──────────────────────────────{RESET}");
        println!("  {BLUE}\u{f303} os{RESET}     {BOLD}Arch Linux{RESET}");
     } else if id == ("debian") {
        println!("{RED}{BOLD}    ____       __    _             {RESET}");
        println!("{RED}{BOLD}   / __ \\___  / /_  (_)___ _____   {RESET}");
        println!("{RED}{BOLD}  / / / / _ \\/ __ \\/ / __ `/ __ \\  {RESET}");
        println!("{RED}{BOLD} / /_/ /  __/ /_/ / / /_/ / / / /  {RESET}");
        println!("{RED}{BOLD}/_____/\\___/_.___/_/\\__,_/_/ /_/   {RESET}");
        println!("{WHITE}──────────────────────────────{RESET}");
        println!("  {RED}\u{f306} os{RESET}     {BOLD}Debian{RESET}");
    } else if id == ("fedora") {
        println!("{BLUE}{BOLD}    ______          __                 {RESET}");
        println!("{BLUE}{BOLD}   / ____/__  ____/ /___  _________ _  {RESET}");
        println!("{BLUE}{BOLD}  / /_  / _ \\/ __  / __ \\/ ___/ __ `/  {RESET}");
        println!("{BLUE}{BOLD} / __/ /  __/ /_/ / /_/ / /  / /_/ /   {RESET}");
        println!("{BLUE}{BOLD}/_/    \\___/\\__,_/\\____/_/   \\__,_/    {RESET}");
        println!("{WHITE}──────────────────────────────{RESET}");
        println!("  {BLUE}\u{f30a} os{RESET}    {BOLD}Fedora{RESET}");
    } else if id == ("void") {
        println!("{GREEN}{BOLD} _    __      _     __{RESET}");
        println!("{GREEN}{BOLD}| |  / /___  (_)___/ /{RESET}");
        println!("{GREEN}{BOLD}| | / / __ \\/ / __  / {RESET}");
        println!("{GREEN}{BOLD}| |/ / /_/ / / /_/ /  {RESET}");
        println!("{GREEN}{BOLD}|___/\\____/_/\\__,_/   {RESET}");
        println!("{WHITE}──────────────────────────────{RESET}");
        println!("  {GREEN}\u{f32e} os{RESET}    {BOLD}Void Linux{RESET}");
    } else if id == ("antigravity") {
        println!("{RED}{BOLD}    ___         __  _ ______                 _ __        {RESET}");
        println!("{YELLOW}{BOLD}   /   |  ____ / /_(_) ____/________ __   __(_) /___  __ {RESET}");
        println!("{GREEN}{BOLD}  / /| | / __ \\/ __/ / / __/ ___/ __ `/ | / / / __/ / / /{RESET}");
        println!("{CYAN}{BOLD} / ___ |/ / / / /_/ / /_/ / /  / /_/ /| |/ / / /_/ /_/ / {RESET}");
        println!("{BLUE}{BOLD}/_/  |_/_/ /_/\\__/_/\\____/_/   \\__,_/ |___/_/\\__/\\__, /  {RESET}");
        println!("{BLUE}{BOLD}                                                /____/   {RESET}");
        println!("{WHITE}──────────────────────────────────────────────────────────{RESET}");
        println!("  {CYAN}\u{e8da}  IDE{RESET}    {BOLD}AntiGravity{RESET}");
    } else {
        println!("{WHITE}{BOLD}    ____  _______________________ {RESET}");
        println!("{WHITE}{BOLD}   / __ \\/  _/_  __/ ____/ / / /  {RESET}");
        println!("{WHITE}{BOLD}  / /_/ // /  / / / /   / /_/ /   {RESET}");
        println!("{WHITE}{BOLD} / _, _// /  / / / /___/ __  /    {RESET}");
        println!("{WHITE}{BOLD}/_/ |_/___/ /_/  \\____/_/ /_/     {RESET}");
        println!("{WHITE}----------------------------------{RESET}");
        println!("  {WHITE}\u{f0aff} os{RESET}        {BOLD}Unknown{RESET}");
    }
}


