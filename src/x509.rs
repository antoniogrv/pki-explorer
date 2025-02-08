use chrono::Utc;
use ratatui::{
    style::{palette::material::GREEN, Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use std::path::PathBuf;
use x509_certificate::X509Certificate;

/// Represents a PEM- or DER-encoded X.509-compliant TLS certificate.
#[derive(Debug)]
pub struct X509 {
    path: PathBuf,

    subject: String,
    issuer: String,
    self_signed: bool,
    not_before: chrono::DateTime<Utc>,
    not_after: chrono::DateTime<Utc>,
}

impl X509 {
    pub fn from(x509_certificate: &X509Certificate, path: PathBuf) -> Result<Self, String> {
        let subject: String = x509_certificate
            .subject_common_name()
            .ok_or("Couldn't parse the X509 subject.")?;

        let issuer: String = x509_certificate
            .issuer_common_name()
            .ok_or("Couldn't parse the X509 issuer.")?;

        let self_signed: bool = x509_certificate.subject_is_issuer();

        let not_before: chrono::DateTime<Utc> = x509_certificate.validity_not_before();
        let not_after: chrono::DateTime<Utc> = x509_certificate.validity_not_after();

        Ok(X509 {
            path,
            subject,
            issuer,
            self_signed,
            not_before,
            not_after,
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

    pub fn is_self_signed(&self) -> &bool {
        &self.self_signed
    }

    pub fn get_not_before(&self) -> &chrono::DateTime<Utc> {
        &self.not_before
    }

    pub fn get_not_after(&self) -> &chrono::DateTime<Utc> {
        &self.not_after
    }

    pub fn get_is_currently_valid(&self) -> bool {
        let now = chrono::offset::Utc::now();

        now <= *self.get_not_after() && now >= *self.get_not_before()
    }

    pub fn get_default_lines(&self) -> Vec<Line> {
        let mut lines = vec![
            self.parse_subject(),
            self.parse_issuer(),
            self.parse_path(),
            self.parse_self_signed(),
        ];

        lines.append(&mut self.parse_is_currently_valid());

        lines
    }

    pub fn parse_subject(&self) -> Line {
        Line::from(vec![
            Span::styled("Subject: ", Style::default().fg(Color::LightYellow)),
            Span::styled(self.get_subject(), Style::default().fg(Color::default())),
        ])
    }

    pub fn parse_issuer(&self) -> Line {
        Line::from(vec![
            Span::styled("Issuer: ", Style::default().fg(Color::LightYellow)),
            Span::styled(self.get_issuer(), Style::default().fg(Color::default())),
        ])
    }

    pub fn parse_path(&self) -> Line {
        Line::from(vec![
            Span::styled("Path: ", Style::default().fg(Color::LightYellow)),
            Span::styled(
                self.get_path()
                    .to_str()
                    .unwrap_or("Couldn't fetch the X509 path."),
                Style::default().fg(Color::default()),
            ),
        ])
    }

    pub fn parse_self_signed(&self) -> Line {
        let is_self_signed: (String, Color) = if *self.is_self_signed() {
            ("Yes".to_string(), Color::Red)
        } else {
            ("No".to_string(), Color::Green)
        };

        Line::from(vec![
            Span::styled("Is Self-Signed? ", Style::default().fg(Color::LightYellow)),
            Span::styled(is_self_signed.0, Style::default().fg(is_self_signed.1)),
        ])
    }

    pub fn parse_is_currently_valid(&self) -> Vec<Line> {
        let is_currently_valid: (String, Color) = if self.get_is_currently_valid() {
            ("Yes".to_string(), Color::Green)
        } else {
            ("No".to_string(), Color::Red)
        };

        let not_before: String = self.get_not_before().to_string();
        let not_after: String = self.get_not_after().to_string();

        let is_currently_valid: Line = Line::from(vec![
            Span::styled(
                "Is Currently Valid? ",
                Style::default().fg(Color::LightYellow),
            ),
            Span::styled(
                is_currently_valid.0,
                Style::default().fg(is_currently_valid.1),
            ),
        ]);

        let valid_after: Line = Line::from(vec![
            Span::styled("\t .. valid after ", Style::default().fg(Color::White)),
            Span::styled(not_before, Style::default().fg(Color::White)),
        ]);

        let up_until: Line = Line::from(vec![
            Span::styled("\t .. up until ", Style::default().fg(Color::White)),
            Span::styled(not_after, Style::default().fg(Color::White)),
        ]);

        vec![is_currently_valid, valid_after, up_until]
    }
}

impl<'a> From<&'a X509> for ListItem<'a> {
    fn from(x509: &'a X509) -> Self {
        let line = Line::styled(x509.get_subject(), GREEN.a200);

        ListItem::new(line)
    }
}
