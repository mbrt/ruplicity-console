#![deny(missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#![cfg_attr(feature = "nightly", allow(unstable_features))]
#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]

#[cfg(feature = "color")]
extern crate ansi_term;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate ruplicity;

mod console;
mod logger;

use std::io::{self, Write};
use std::path::Path;
use std::process;

use ruplicity::{Backend, Backup};
use ruplicity::backend::local::LocalBackend;
use ruplicity::time_utils::TimeDisplay;


fn main() {
    let matches = clap_app!(app =>
        (version: &crate_version!()[..])
        (author: "Michele Bertasi <@brt_device>")
        (about: "Command line client for inspecting duplicity backups")
        (@arg verbose: -v ... +global "Sets the level of verbosity")
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

    // init logging functionality
    let log_level = match matches.occurrences_of("verbose") {
        0 => log::LogLevelFilter::Error,
        1 => log::LogLevelFilter::Debug,
        _ => log::LogLevelFilter::Trace,
    };
    if let Err(e) = logger::init(log_level) {
        println!("Logger initialization error {}", e);
        process::exit(1);
    };

    if let Some(matches) = matches.subcommand_matches("info") {
        // calling unwrap is safe here, because INPUT is required
        let path = matches.value_of("INPUT").unwrap();
        let backup = ordie(backup_from_path(path));
        dump_info(&backup);
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
                let files = ordie(snapshot.entries());
                println!("{}", files);
            }
            None => {
                fatal!("Cannot find the desired snapshot in the backup");
            }
        }
    }
}


fn backup_from_path<P: AsRef<Path>>(path: P) -> io::Result<Backup<LocalBackend>> {
    info!("Loading backup from path {:?}", path.as_ref());
    let backend = LocalBackend::new(path);
    Backup::new(backend)
}

fn dump_info<B: Backend>(backup: &Backup<B>) {
    let snapshots = ordie(backup.snapshots());
    for chain in snapshots.as_collections().backup_chains() {
        let num_vol = chain.full_set().num_volumes() +
                      chain.inc_sets()
                           .map(|i| i.num_volumes())
                           .fold(0, |a, i| a + i);
        console_good!("Backup chain");
        println!("Start time:            {}",
                 chain.start_time().into_local_display());
        println!("End time:              {}",
                 chain.end_time().into_local_display());
        println!("Number of backup sets: {}", chain.inc_sets().count() + 1);
        println!("Number of volumes:     {}", num_vol);

        console_warn!("Backup sets (type, time, num volumes):");
        fn pset(set: &ruplicity::collections::BackupSet) {
            println!("    {:<12} {:<13} {:>5}",
                     if set.is_full() { "Full" } else { "Incremental" },
                     set.end_time().into_local_display(),
                     set.num_volumes());
        }
        pset(chain.full_set());
        for inc in chain.inc_sets() {
            pset(inc);
        }
        // empty line between chains
        println!("");
    }
}

fn ordie<T, E: ToString>(r: Result<T, E>) -> T {
    match r {
        Ok(r) => r,
        Err(e) => {
            fatal!("{}", e.to_string());
        }
    }
}
