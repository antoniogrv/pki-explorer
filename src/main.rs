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

    #[arg(short, long, default_value = "2")]
    depth: u8,

    #[arg(short, long, action, default_value = "false", action=ArgAction::SetTrue)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = color_eyre::install();

    let workdir: String = args.path;

    let x509s: Vec<X509> = lookup_x509s(workdir.as_str(), args.verbose, (0, args.depth))?;

    let terminal = ratatui::init();
    let app_result = pki_explorer::PKIExplorerApp::new(x509s, workdir).run(terminal);

    ratatui::restore();

    Ok(app_result.unwrap_or(()))
}

fn lookup_x509s(dir: &str, verbose: bool, depth: (u8, u8)) -> Result<Vec<X509>, Box<dyn Error>> {
    let mut entries: Vec<X509> = Vec::new();

    if depth.0 >= depth.1 {
        return Ok(entries);
    } else {
        if verbose {
            println!(".. Looking for X509s in {} at depth {}", dir, depth.0);
        };
    }

    let dir = read_dir(dir)?;

    for entry in dir.into_iter() {
        let entry: DirEntry = entry?;

        if entry.path().is_file() {
            let entry_path: PathBuf = entry.path();
            let entry_file: Vec<u8> = read(&entry_path)?;
            let entry_raw = &X509Certificate::from_pem(entry_file);

            if let Ok(decoded_cert) = entry_raw {
                if verbose {
                    println!(
                        ".. Found valid X509: {} at depth {}",
                        &entry_path
                            .as_os_str()
                            .to_str()
                            .ok_or("Can't parse the directory name.")?,
                        depth.0
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
                verbose,
                (depth.0 + 1, depth.1),
            )?;
            entries.append(&mut nested_elements);
        }
    }

    Ok(entries)
}
