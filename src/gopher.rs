use crate::domain::*;
use crate::network::download_content;

pub fn load_page(link: &Link) -> Page {
    let content = match download_content(link) {
        Ok(content) => content,
        Err(err) => {
            return Page {
                link: link.clone(),
                content: Content::None,
                errors: vec![err],
            };
        }
    };

    let mut errors: Vec<DomainError> = vec![];
    let content = match link.format {
        Format::TextFile => Content::File(content),
        Format::GopherMap => {
            let (content, mut parse_errors) = parse_map(content);
            errors.append(&mut parse_errors);
            content
        }
    };

    Page {
        link: link.clone(),
        content,
        errors,
    }
}

fn parse_map(content: String) -> (Content, Vec<DomainError>) {
    let (elements, erros) = content
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(parse_element)
        .fold((Vec::new(), Vec::new()), |mut acc, result| {
            match result {
                Ok(element) => acc.0.push(element),
                Err(error) => acc.1.push(error),
            }
            acc
        });

    (Content::Map(elements), erros)
}

fn parse_element(text: &str) -> Result<Element, DomainError> {
    match text.as_bytes() {
        [b'i', ..] => {
            let text = text[1..].to_string();
            Ok(Element::Text(
                text.split('\t').next().unwrap_or(&text).to_string(),
            ))
        }
        [b'.', ..] => Ok(Element::Text(".".to_string())),
        [b'1', ..] => parse_link(&text[1..], Format::GopherMap),
        [b'0', ..] => parse_link(&text[1..], Format::TextFile),
        [first, ..] => Err(DomainError::Parsing(format!(
            "Unknown gopher type {}!",
            *first as char
        ))),
        [] => Ok(Element::Text("".to_string())),
    }
}

fn parse_link(text: &str, format: Format) -> Result<Element, DomainError> {
    let mut parts = text.split('\t');
    let title = parts.next().unwrap_or("").to_string();

    let url = match parts.next() {
        Some(url) => url.to_string(),
        None => return parse_error(format!("Missing url in line {}", text)),
    };

    let host = match parts.next() {
        Some(host) => host.to_string(),
        None => return parse_error(format!("Missing host in line {}", text)),
    };

    let port = match parts.next() {
        Some(port) => port,
        None => return parse_error(format!("Missing port in line {}", text)),
    };

    let port = match port.trim_end().parse::<u16>() {
        Ok(port) => port,
        Err(_) => return parse_error(format!("Port is not a number in line {}", text)),
    };

    Ok(Element::Link(
        title,
        Link {
            format,
            host,
            port,
            url,
        },
    ))
}

fn parse_error<T>(text: String) -> Result<T, DomainError> {
    Result::Err(DomainError::Parsing(text))
}
