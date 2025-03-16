use std::io::{self, Write};

use crate::domain::{Action, DomainError, Element, Link, Page, Render};
use colored::Colorize;

const USAGE: &str = "
Available commands:
quit                 Exit the application
follow n             Navigate to the n-th link on the current page
go sdf.org:70/phlogs Follow the specified link
help                 Display this help message";

pub struct Terminal;

impl Render for Terminal {
    fn render(&self, page: &Page) {
        println!(
            "{}",
            format!("=== | Show {}{}", page.link.host_port, page.link.url).green()
        );

        let mut link_number = 0;
        match &page.content {
            crate::domain::Content::Map(lines) => lines.iter().for_each(|l| match l {
                Element::Link(text, _link) => {
                    println!("{:4}| {text}", link_number.to_string().blue());
                    link_number += 1;
                }
                Element::Text(text) => println!("    | {text}"),
            }),
            crate::domain::Content::File(text) => println!("{text}"),
            crate::domain::Content::None => println!(),
        };
        if !page.errors.is_empty() {
            println!("{}", "=== | Errors:".red());
            page.errors.iter().for_each(|e| self.show_error(e));
        }
    }

    fn get_action(&self) -> Action {
        loop {
            print!("command: ");
            let _ = io::stdout().flush();

            let mut line = String::new();
            if std::io::stdin().read_line(&mut line).is_err() {
                return Action::Quit;
            }

            let line = line.trim();

            match line.split(' ').collect::<Vec<&str>>().as_slice() {
                ["quit"] => return Action::Quit,
                ["follow", number] => match number.parse::<usize>() {
                    Ok(number) => return Action::Follow(number),
                    Err(_) => println!("You should specify link number"),
                },
                ["go", link] => match Link::from_string(link) {
                    Some(link) => return Action::Load(link),
                    None => println!("Wrong format for link: {link}"),
                },
                ["help"] => println!("{USAGE}"),
                [""] => {}
                _ => println!("Unknown command: {}", line),
            }
        }
    }

    fn show_error(&self, error: &DomainError) {
        match error {
            DomainError::Network(message) => println!("{}: {message}", "Network Error".red()),
            DomainError::Parsing(message) => println!("{}: {message}", "Parsing Error".red()),
            DomainError::Logic(message) => println!("{}", message.yellow()),
        }
    }
}
