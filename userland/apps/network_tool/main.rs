mod ping;
mod dns;
mod interfaces;

use std::env;

fn print_help() {
    println!("Network Tool - команды:");
    println!("  ping <host>      - проверить доступность хоста");
    println!("  lookup <host>    - получить IP по имени хоста");
    println!("  ifconfig         - показать сетевые интерфейсы");
    println!("  help             - показать эту помощь");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "ping" => {
            if args.len() < 3 {
                println!("Использование: ping <host>");
                return;
            }
            ping::ping_host(&args[2]);
        }
        "lookup" => {
            if args.len() < 3 {
                println!("Использование: lookup <host>");
                return;
            }
            dns::lookup_host(&args[2]);
        }
        "ifconfig" => {
            interfaces::show_interfaces();
        }
        "help" | _ => {
            print_help();
        }
    }
}
