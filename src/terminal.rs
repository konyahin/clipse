use colored::Colorize;
use crate::domain::{DomainError, Element, Page, Action, Render};

pub struct Terminal;

impl Render for Terminal {
    fn render(&self, page: &Page) {
        show(page);
    }

    fn get_action(&self) -> Action {
        Action::Quit
    }
}

fn show(page: &Page) {
    println!(
        "{}",
        format!("=== | Show {}:{}{}", page.link.host, page.link.port, page.link.url).green()
    );
    match &page.content {
        crate::domain::Content::Map(lines) => lines.iter().for_each(draw_line),
        crate::domain::Content::File(text) => println!("{text}"),
        crate::domain::Content::None => println!(),
    };
    if !page.errors.is_empty() {
        println!("{}", "=== | Errors:".red());
        page.errors.iter().for_each(show_error);
    }
}

pub fn show_error(error: &DomainError) {
    match error {
        DomainError::Network(message) => println!("{}: {message}", "Network Error".red()),
        DomainError::Display(message) => println!("{}: {message}", "Render Error".red()),
        DomainError::Parsing(message) => println!("{}: {message}", "Parsing Error".red()),
    }
}

fn draw_line(line: &Element) {
    match line {
        Element::Link(text, _link) => println!("{}| {text}", "link".blue()),
        Element::Text(text) => println!("    | {text}"),
    }
}
