mod domain;
mod gopher;
mod network;
mod terminal;

use crate::domain::*;
use gopher::load_page;
use terminal::Terminal;

fn main() {
    let mut link = Link::new_gopher(String::from("sdf.org"));

    let render = Terminal;
    loop {
        let page = load_page(&link);
        render.render(&page);
        match render.get_action() {
            Action::Quit => break,
            Action::Load(new_link) => link = new_link,
            Action::Follow(number) => match page.get_link(number) {
                Some(new_link) => link = new_link.clone(),
                None => {
                    render.show_error(&DomainError::Logic("Wrong number for link.".to_string()))
                }
            },
        }
    }
}
