use std::net::ToSocketAddrs;

pub fn lookup_host(host: &str) {
    match (host, 0).to_socket_addrs() {
        Ok(iter) => {
            println!("IP адреса для {}:", host);
            for addr in iter {
                println!("  {}", addr.ip());
            }
        }
        Err(e) => {
            println!("Ошибка разрешения имени: {}", e);
        }
    }
}
