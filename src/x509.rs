use std::{fs::File, io::BufReader, path::PathBuf};

use ratatui::{style::palette::material::GREEN, text::Line, widgets::ListItem};
use x509_parser::pem::Pem;

/// Represents a PEM-encoded X.509 TLS certificate.
#[derive(Clone)]
pub struct X509 {
    pub file_path: Box<PathBuf>,
    pub file_path_raw: String,
    pub pem: Pem,
}

impl X509 {
    pub fn new(file_path: Box<PathBuf>) -> Self {
        let file_path_raw = file_path
            .as_os_str()
            .to_str()
            .unwrap_or("Couldn't convert the X509 into a string.")
            .to_string();

        let file_contents = File::open(&file_path_raw).unwrap();

        let pem = x509_parser::pem::Pem::read(BufReader::new(file_contents))
            .unwrap()
            .0;

        Self {
            file_path,
            file_path_raw,
            pem,
        }
    }
}

impl std::fmt::Display for X509 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_string: Option<&str> = self.file_path.as_os_str().to_str();

        match path_string {
            Some(path_string) => {
                writeln!(f, "{}", path_string)?;
                Ok(())
            }
            _ => Err(std::fmt::Error),
        }
    }
}

impl<'a> From<&'a X509> for ListItem<'a> {
    fn from(value: &'a X509) -> Self {
        let path_str = value
            .file_path
            .as_os_str()
            .to_str()
            .unwrap_or("Couldn't parse the X509.");
        let line = Line::styled(path_str, GREEN.a200);

        ListItem::new(line)
    }
}
