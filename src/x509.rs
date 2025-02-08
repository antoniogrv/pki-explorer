use std::path::PathBuf;

use ratatui::{style::palette::material::GREEN, text::Line, widgets::ListItem};
use x509_certificate::CapturedX509Certificate;

/// Represents a PEM- or DER-encoded X.509-compliant TLS certificate.
#[derive(Debug)]
pub struct X509 {
    path: PathBuf,

    subject: String,
    issuer: String,
}

impl X509 {
    pub fn from(x509_certificate: &CapturedX509Certificate, path: PathBuf) -> Result<Self, String> {
        let subject: String = x509_certificate
            .subject_common_name()
            .ok_or("Couldn't parse the X509 subject.")?;

        let issuer: String = x509_certificate
            .issuer_common_name()
            .ok_or("Couldn't parse the X509 issuer.")?;

        Ok(X509 {
            path,
            subject,
            issuer,
        })
    }

    pub fn get_subject(&self) -> &String {
        &self.subject
    }

    pub fn get_issuer(&self) -> &String {
        &self.issuer
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

impl<'a> From<&'a X509> for ListItem<'a> {
    fn from(x509: &'a X509) -> Self {
        let line = Line::styled(x509.get_subject(), GREEN.a200);

        ListItem::new(line)
    }
}
