use std::net::IpAddr;
use std::io;

pub fn show_interfaces() {
    println!("Сетевые интерфейсы:");

    // Пример: вывести все IP адреса интерфейсов
    // В реальной ОС нужно использовать netlink или syscalls, здесь демонстрация через std::net

    match get_interfaces() {
        Ok(interfaces) => {
            for (name, ips) in interfaces {
                println!("Интерфейс: {}", name);
                for ip in ips {
                    println!("  IP: {}", ip);
                }
            }
        }
        Err(e) => {
            println!("Ошибка получения интерфейсов: {}", e);
        }
    }
}

fn get_interfaces() -> io::Result<Vec<(String, Vec<IpAddr>)>> {
    // Для простоты здесь пример заглушки.
    // На настоящей системе нужно читать /sys/class/net или использовать netlink.
    // Возвращаем фиктивный интерфейс

    Ok(vec![
        ("eth0".to_string(), vec!["192.168.1.2".parse().unwrap()]),
        ("lo".to_string(), vec!["127.0.0.1".parse().unwrap()]),
    ])
}
