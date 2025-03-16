use text_io::try_scan;

pub enum DomainError {
    Network(String),
    Parsing(String),
    Logic(String),
}

#[derive(Clone)]
pub struct Link {
    pub format: Format,
    pub host_port: String,
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
            host_port: format!("{host}:70"),
            url: String::from("/"),
        }
    }

    pub fn from_string(link: String) -> Self {
        // TODO too bad, should write my
        Link::scan_full_link(link).unwrap()
    }

    fn scan_full_link(link: String) -> Result<Link, Box<dyn std::error::Error>> {
        let host_port: String;
        let url: String;

        try_scan!(link.bytes() => "{}/{}", host_port, url);

        Ok(Link {
            format: Format::GopherMap,
            host_port,
            url,
        })
    }
}

pub struct Page {
    pub link: Link,
    pub content: Content,
    pub errors: Vec<DomainError>,
}

impl Page {
    pub fn get_link(&self, n: usize) -> Option<&Link> {
        let elements = match &self.content {
            Content::File(_) => return None,
            Content::None => return None,
            Content::Map(elements) => elements,
        };

        let links = elements
            .iter()
            .filter_map(|e| match e {
                Element::Link(_, link) => Some(link),
                Element::Text(_) => None,
            })
            .collect::<Vec<&Link>>();

        links.get(n).copied()
    }
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
    Follow(usize),
}

pub trait Render {
    fn render(&self, page: &Page);
    fn get_action(&self) -> Action;
    fn show_error(&self, error: &DomainError);
}
