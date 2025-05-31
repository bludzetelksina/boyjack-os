use std::net::{UdpSocket, ToSocketAddrs};
use std::time::{Duration, Instant};

pub fn ping_host(host: &str) {
    // Примитивный UDP ping для демонстрации (в реальной системе — ICMP, но ICMP требует raw socket)
    let addr = match (host, 33434).to_socket_addrs() {
        Ok(mut iter) => match iter.next() {
            Some(addr) => addr,
            None => {
                println!("Не удалось разрешить адрес {}", host);
                return;
            }
        },
        Err(_) => {
            println!("Некорректный хост {}", host);
            return;
        }
    };

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            println!("Ошибка создания сокета: {}", e);
            return;
        }
    };

    socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();

    let msg = [0u8; 32];
    let start = Instant::now();

    match socket.send_to(&msg, addr) {
        Ok(_) => {}
        Err(e) => {
            println!("Ошибка отправки пакета: {}", e);
            return;
        }
    }

    let mut buf = [0u8; 512];
    match socket.recv_from(&mut buf) {
        Ok((_, _)) => {
            let elapsed = start.elapsed();
            println!("Хост {} доступен, время отклика: {:?}", host, elapsed);
        }
        Err(_) => {
            println!("Нет ответа от хоста {}", host);
        }
    }
}
