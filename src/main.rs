use std::{error::Error, path::PathBuf};

use std::fs::{read, read_dir, DirEntry};

use x509::X509;
use x509_certificate::*;

mod pki_explorer;
mod x509;
mod x509_tui;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = color_eyre::install();

    let x509s: Vec<X509> = lookup_x509s("test_certs")?;

    let terminal = ratatui::init();
    let app_result = pki_explorer::PKIExplorerApp::new(x509s).run(terminal);

    ratatui::restore();

    Ok(app_result.unwrap_or(()))
}

fn lookup_x509s(dir: &str) -> Result<Vec<X509>, Box<dyn Error>> {
    let mut entries: Vec<X509> = Vec::new();
    let dir = read_dir(dir)?;

    for entry in dir.into_iter() {
        let entry: DirEntry = entry?;

        if entry.file_type()?.is_file() {
            let entry_path: PathBuf = entry.path();
            let entry_file: Vec<u8> = read(&entry_path)?;
            let x509: X509 =
                X509::from(&CapturedX509Certificate::from_pem(entry_file)?, entry_path)?;
            entries.push(x509);
        } else {
            let mut nested_elements = lookup_x509s(
                entry
                    .path()
                    .as_os_str()
                    .to_str()
                    .ok_or("Couldn't not parse the nested directory.")?,
            )?;
            entries.append(&mut nested_elements);
        }
    }

    Ok(entries)
}
