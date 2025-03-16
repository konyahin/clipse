mod domain;
mod gopher;
mod network;
mod terminal;

use crate::domain::*;
use gopher::load_page;
use terminal::show;

fn main() {
    let link = Link::new_gopher(String::from("sdf.org"));
    let page = load_page(&link);
    show(&page);
}
