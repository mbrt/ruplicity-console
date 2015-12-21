#![deny(missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#![cfg_attr(feature = "nightly", allow(unstable_features))]
#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate ruplicity;
#[cfg(feature = "color")]
extern crate term;

use std::io::{self, Write};
use std::path::Path;
use std::process;

use ruplicity::backend::local::LocalBackend;
use ruplicity::Backup;


fn main() {
    let matches = clap_app!(app =>
        (version: &crate_version!()[..])
        (author: "Michele Bertasi <@brt_device>")
        (about: "Command line client for inspecting duplicity backups")
        (@subcommand info =>
            (about: "informations about snapshots present in a backup")
            (@arg INPUT: +required "the path to the backup")
        )
        (@subcommand ls =>
            (about: "list files inside a backup snapshot")
            (@arg index: -i --index +takes_value "index of the snapshot, defaults the last when omitted")
            (@arg INPUT: +required "the path to the backup")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("info") {
        // calling unwrap is safe here, because INPUT is required
        let path = matches.value_of("INPUT").unwrap();
        let backup = ordie(backup_from_path(path));
        let collection = ordie(backup.snapshots());
        println!("{}", collection.as_ref());
    } else if let Some(matches) = matches.subcommand_matches("ls") {
        let path = matches.value_of("INPUT").unwrap();
        let backup = ordie(backup_from_path(path));
        let snapshot = {
            if let Ok(index) = value_t!(matches.value_of("index"), usize) {
                ordie(backup.snapshots()).nth(index)
            } else {
                ordie(backup.snapshots()).last()
            }
        };
        match snapshot {
            Some(snapshot) => {
                let files = ordie(snapshot.files());
                println!("{}", files.as_signature_info().into_display());
            }
            None => {
                let _ = write!(&mut io::stderr(), "Cannot find the desired snapshot in the backup\n");
                process::exit(1);
            }
        }
    }
}


fn backup_from_path<P: AsRef<Path>>(path: P) -> io::Result<Backup<LocalBackend>> {
    let backend = LocalBackend::new(path);
    Backup::new(backend)
}

// taken from BurntSushi/tabwriter
fn ordie<T, E: ToString>(r: Result<T, E>) -> T {
    match r {
        Ok(r) => r,
        Err(e) => {
            let _ = writeln!(&mut io::stderr(), "{}", e.to_string());
            process::exit(1);
        }
    }
}
