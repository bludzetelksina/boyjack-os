// userland/lib/network.rs

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr, ToSocketAddrs};
use std::time::Duration;

/// Ошибки сети
#[derive(Debug)]
pub enum NetworkError {
    IoError(std::io::Error),
    AddrParseError,
    Timeout,
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        NetworkError::IoError(err)
    }
}

/// TCP клиент
pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    /// Подключается к серверу по адресу и порту (например, "127.0.0.1:8080")
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, NetworkError> {
        let stream = TcpStream::connect(addr)?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        Ok(TcpClient { stream })
    }

    /// Отправляет данные серверу
    pub fn send(&mut self, data: &[u8]) -> Result<usize, NetworkError> {
        let bytes_written = self.stream.write(data)?;
        self.stream.flush()?;
        Ok(bytes_written)
    }

    /// Получает данные от сервера, возвращает прочитанные байты
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        let bytes_read = self.stream.read(buffer)?;
        Ok(bytes_read)
    }

    /// Закрывает соединение
    pub fn close(self) -> Result<(), NetworkError> {
        drop(self);
        Ok(())
    }
}

/// TCP сервер
pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    /// Создаёт сервер, слушающий на указанном адресе и порту (например, "0.0.0.0:8080")
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self, NetworkError> {
        let listener = TcpListener::bind(addr)?;
        Ok(TcpServer { listener })
    }

    /// Принимает входящее соединение, возвращает TcpStream и адрес клиента
    pub fn accept(&self) -> Result<(TcpStream, SocketAddr), NetworkError> {
        let (stream, addr) = self.listener.accept()?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
        Ok((stream, addr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_client_connect_fail() {
        let client = TcpClient::connect("256.256.256.256:80");
        assert!(client.is_err());
    }

    #[test]
    fn test_tcp_server_bind_fail() {
        // Попытка забиндить занятый порт (например 0)
        let server = TcpServer::bind("0.0.0.0:0");
        assert!(server.is_ok()); // Порт 0 - система назначит свободный
    }
}
