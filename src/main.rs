use clap::Parser;
use clap::*;
use std::fs::{read, read_dir, DirEntry};
use std::{error::Error, path::PathBuf};
use x509::X509;
use x509_certificate::*;

mod pki_explorer;
mod x509;
mod x509_tui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short, long, action, default_value = "true", action=ArgAction::SetFalse)]
    silent: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = color_eyre::install();

    let workdir: String = args.path;

    let x509s: Vec<X509> = lookup_x509s(workdir.as_str(), args.silent)?;

    let terminal = ratatui::init();
    let app_result = pki_explorer::PKIExplorerApp::new(x509s, workdir).run(terminal);

    ratatui::restore();

    Ok(app_result.unwrap_or(()))
}

fn lookup_x509s(dir: &str, silent: bool) -> Result<Vec<X509>, Box<dyn Error>> {
    let mut entries: Vec<X509> = Vec::new();

    if !silent {
        println!(".. Looking for X509s in {}", dir);
    };

    let dir = read_dir(dir)?;

    for entry in dir.into_iter() {
        let entry: DirEntry = entry?;

        if entry.path().is_file() {
            let entry_path: PathBuf = entry.path();
            let entry_file: Vec<u8> = read(&entry_path)?;
            let entry_raw = &X509Certificate::from_pem(entry_file);

            if let Ok(decoded_cert) = entry_raw {
                if !silent {
                    println!(
                        ".. Found valid X509: {}",
                        &entry_path
                            .as_os_str()
                            .to_str()
                            .ok_or("Can't parse the directory name.")?
                    );
                };

                let x509: X509 = X509::from(decoded_cert, entry_path)?;
                entries.push(x509);
            };
        } else {
            let mut nested_elements = lookup_x509s(
                entry
                    .path()
                    .as_os_str()
                    .to_str()
                    .ok_or("Couldn't not parse the nested directory.")?,
                silent,
            )?;
            entries.append(&mut nested_elements);
        }
    }

    Ok(entries)
}
