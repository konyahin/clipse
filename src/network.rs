use std::{
    io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, time::Duration
};

use crate::domain::{DomainError, Link};

pub fn download_content(link: &Link) -> Result<String, DomainError> {
    let addrs = link.host_port.to_socket_addrs().map_err(to_domain_error)?;

    let addr = match addrs.last() {
        Some(addr) => addr,
        None => return Err(DomainError::Parsing(format!("Wrong link: {}", link.host_port)))
    };

    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5)).map_err(to_domain_error)?;

    stream
        .write_all(format!("{}\r\n", link.url).as_bytes())
        .map_err(to_domain_error)?;

    let mut answer = String::new();
    let _ = stream
        .read_to_string(&mut answer)
        .map_err(to_domain_error)?;

    Ok(answer)
}

fn to_domain_error(err: std::io::Error) -> DomainError {
    DomainError::Network(err.to_string())
}
