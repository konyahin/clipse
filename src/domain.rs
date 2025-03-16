pub enum DomainError {
    Network(String),
    Parsing(String),
    Logic(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub format: Format,
    pub host_port: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn from_string(link: &str) -> Option<Self> {
        let mut parts = link.split("/");

        let host_port = if let Some(host_port) = parts.next() {
            if host_port.is_empty() {
                return None;
            }
            if host_port.contains(":") {
                host_port.to_owned()
            } else {
                format!("{host_port}:70")
            }
        } else {
            return None;
        };
        

        let url = format!("/{}", parts.collect::<Vec<&str>>().join("/"));

        Some(Link {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_wrong_format() {
        assert_eq!(
            Link::from_string("/example.com"),
            None,
        );
    }

    #[test]
    fn test_link_format_host() {
        assert_eq!(
            Link::from_string("example.com").unwrap(),
            Link {
                format: Format::GopherMap,
                host_port: "example.com:70".to_owned(),
                url: "/".to_owned()
            },
        );
    }

    #[test]
    fn test_link_format_host_port() {
        assert_eq!(
            Link::from_string("example.com:89").unwrap(),
            Link {
                format: Format::GopherMap,
                host_port: "example.com:89".to_owned(),
                url: "/".to_owned()
            },
        );
    }

    #[test]
    fn test_link_format_host_port_url() {
        assert_eq!(
            Link::from_string("example.com:69/test").unwrap(),
            Link {
                format: Format::GopherMap,
                host_port: "example.com:69".to_owned(),
                url: "/test".to_owned()
            },
        );
    }

    #[test]
    fn test_link_format_host_url() {
        assert_eq!(
            Link::from_string("example.com/test").unwrap(),
            Link {
                format: Format::GopherMap,
                host_port: "example.com:70".to_owned(),
                url: "/test".to_owned()
            },
        );
    }
}
