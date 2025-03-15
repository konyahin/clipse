use std::io::prelude::*;
use std::net::TcpStream;

use ratatui::crossterm::event::{self, Event};
use ratatui::widgets::Paragraph;
use ratatui::Frame;


fn download_content(host: &str, url: &str) -> Result<String, std::io::Error> {
    let mut stream = TcpStream::connect(host)?;

    let mut url = url.to_owned();
    url.push_str("\r\n");

    stream.write_all(url.as_bytes())?;

    let mut answer = String::new();
    stream.read_to_string(&mut answer)?;

    Ok(answer)
}

fn render(frame: &mut Frame, content: &str) {
    frame.render_widget(Paragraph::new(content), frame.area());
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let answer = download_content("sdf.org:70", "/")?;
    
    loop {
        terminal.draw(|frame| render(frame, &answer))?;
        if matches!(event::read()?, Event::Key(_)) {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
