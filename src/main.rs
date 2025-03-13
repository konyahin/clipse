use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("sdf.org:70")?;
    stream.write(b"/\r\n")?;

    loop {
        let mut answer: [u8; 128] = [0; 128];
        let size = stream.read(&mut answer)?;
        let answer = str::from_utf8(&answer).unwrap();
        println!("{}", answer);

        if size < 128 {
            break;
        };
    }
    Ok(())
}
