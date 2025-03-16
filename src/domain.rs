pub enum DomainError {
    Network(String),
    Display(String),
    Parsing(String),
}

#[derive(Clone)]
pub struct Link {
    pub format: Format,
    pub host: String,
    pub port: u16,
    pub url: String,
}

#[derive(Clone)]
pub enum Format {
    GopherMap,
    TextFile,
}

impl Link {
    pub fn new_gopher(host: String) -> Self {
        Link {
            format: Format::GopherMap,
            host,
            port: 70,
            url: String::from("/"),
        }
    }
}

pub struct Page {
    pub link: Link,
    pub content: Content,
    pub errors: Vec<DomainError>,
}

pub enum Content {
    Map(Vec<Element>),
    File(String),
    None,
}

pub enum Element {
    Link(String, Link),
    Text(String),
}

pub enum Action {
    Quit,
    Load(Link),
}

pub trait Render {
    fn render(&self, page: &Page);
    fn get_action(&self) -> Action;
}