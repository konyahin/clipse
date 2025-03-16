mod domain;
mod gopher;
mod network;
mod terminal;

use crate::domain::*;
use gopher::load_page;
use terminal::Terminal;

fn main() {
    let link = Link::new_gopher(String::from("sdf.org"));
    let page = load_page(&link);

    let render = Terminal;
    loop {
        render.render(&page);
        match render.get_action() {
            Action::Quit => break,
            Action::Load(link) => todo!(),
        }
    }
}
