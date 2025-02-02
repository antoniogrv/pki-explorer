use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use x509::X509;

mod pki_explorer;
mod x509;
mod x509_tui;

fn main() -> Result<(), color_eyre::Report> {
    let _ = color_eyre::install();

    let x509s = lookup_x509s(Path::new("test_dir"))?;

    let terminal = ratatui::init();
    let app_result = pki_explorer::PKIExplorerApp::new(x509s).run(terminal);

    ratatui::restore();

    app_result
}

fn lookup_x509s(dir: &Path) -> Result<Vec<X509>, std::io::Error> {
    let mut entries: Vec<X509> = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry: fs::DirEntry = entry?;
            let path: PathBuf = entry.path();

            if path.is_dir() {
                let mut nested_elements: Vec<X509>;

                nested_elements = lookup_x509s(path.as_path())?;
                entries.append(&mut nested_elements);
            } else {
                entries.push(X509::new(Box::new(path)));
            }
        }

        return Ok(entries);
    }

    Err(Error::new(
        ErrorKind::AddrInUse,
        "Couldn't lookup any X509 files.",
    ))
}
