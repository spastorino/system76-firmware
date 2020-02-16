extern crate libc;
extern crate system76_firmware;

use std::{env, io, process};
use system76_firmware::*;

fn tool() -> Result<(), String> {
    if unsafe { libc::geteuid() } != 0 {
        return Err(format!("must be run as root"));
    }

    // Get I/O Permission
    if unsafe { libc::iopl(3) } < 0 {
        return Err(format!(
            "failed to get I/O permission: {}",
            io::Error::last_os_error()
        ));
    }

    //TODO: allow override with parameter
    let efi_dir = "/boot";

    let usage = "subcommands:\n  schedule\n  unschedule\n  thelio-io";
    match env::args().nth(1) {
        Some(arg) => match arg.as_str() {
            "schedule" => {
                let (digest, _changelog) = match download() {
                    Ok(ok) => ok,
                    Err(err) => return Err(format!("failed to download: {}", err))
                };

                match schedule(&digest, efi_dir) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(format!("failed to schedule: {}", err))
                }
            },
            "unschedule" => {
                match unschedule(efi_dir) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(format!("failed to unschedule: {}", err))
                }
            },
            "thelio-io" => {
                let (digest, _revision) = match thelio_io_download() {
                    Ok(ok) => ok,
                    Err(err) => return Err(format!("failed to download: {}", err))
                };

                match thelio_io_update(&digest) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(format!("failed to update: {}", err))
                }
            },
            other => Err(format!("invalid subcommand {} provided\n{}", other, usage)),
        },
        None => Err(format!("no subcommand provided\n{}", usage))
    }
}

fn main() {
    match tool() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("system76-firmware: {}", err);
            process::exit(1);
        }
    }
}
