use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::domain::{DomainError, Link};

pub fn download_content(link: &Link) -> Result<String, DomainError> {
    let mut stream = TcpStream::connect(&link.host_port).map_err(to_domain_error)?;

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
