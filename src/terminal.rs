// use std::io::Stdout;

// use ratatui::crossterm::event::{self, Event};
// use ratatui::prelude::CrosstermBackend;
// use ratatui::widgets::Paragraph;
// use ratatui::{Frame, Terminal};

use crate::domain::{DomainError, Element, Page};

pub fn show(page: &Page) {
    println!(
        "=== | Show {}:{}{}",
        page.link.host, page.link.port, page.link.url
    );
    match &page.content {
        crate::domain::Content::Map(lines) => lines.iter().for_each(draw_line),
        crate::domain::Content::File(text) => println!("{text}"),
        crate::domain::Content::None => println!(),
    };
    if !page.errors.is_empty() {
        println!("=== | Errors:");
        page.errors.iter().for_each(show_error);
    }
}

pub fn show_error(error: &DomainError) {
    match error {
        DomainError::Network(message) => println!("Network Error: {message}"),
        DomainError::Display(message) => println!("Render Error: {message}"),
        DomainError::Parsing(message) => println!("Parsing Error: {message}"),
    }
}

fn draw_line(line: &Element) {
    match line {
        Element::Link(text, _link) => println!("link| {text}"),
        Element::Text(text) => println!("    | {text}"),
    }
}
// pub fn draw(terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<> {
//     terminal.draw(|frame| render(frame, &answer))?;
//     if matches!(event::read()?, Event::Key(_)) {
//         break;
//     }
// }

// fn render(frame: &mut Frame, content: &str) {
//     frame.render_widget(Paragraph::new(content), frame.area());
// }
