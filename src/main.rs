mod domain;
mod gopher;
mod network;
mod terminal;

use crate::domain::*;
use gopher::load_page;
use terminal::Terminal;

fn main() {
    let mut page = match std::env::args().nth(1) {
        Some(link) => {
            if let Some(link) = Link::from_string(&link) {
                load_page(&link)
            } else {
                Page::default_page()
            }
        },
        None => Page::default_page(),
    };

    let render = Terminal;
    loop {
        render.render(&page);
        match render.get_action() {
            Action::Quit => break,
            Action::Load(link) => page = load_page(&link),
            Action::Follow(number) => match page.get_link(number) {
                Some(link) => page = load_page(link),
                None => {
                    render.show_error(&DomainError::Logic("Wrong number for link.".to_string()))
                }
            },
        }
    }
}
