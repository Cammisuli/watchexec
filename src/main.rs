#[macro_use]
extern crate clap;
extern crate globset;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate notify;

#[cfg(unix)]
extern crate nix;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

#[cfg(test)]
extern crate mktemp;

mod cli;
mod gitignore;
mod notification_filter;
mod process;
mod run;
mod signal;
mod watcher;
mod pathop;

fn main() {
    let args = cli::get_args();
    if let Err(e) = run::run(args) {
        eprintln!("Error: {}", e);

        #[cfg(unix)]
        std::process::exit(1)
    }
}
