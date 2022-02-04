use clap::Parser;
use std::sync::mpsc::channel;
extern crate timer;
extern crate chrono;

mod utils;
mod study;

/// Simple study timer that also blocks applications
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Quickly add a flag for verbosity
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    /// Path to the app list
    #[clap(short = 'l',
    long = "app-list", 
    default_value = "~/.config/studytime/apps.txt")]
    app_list: String,
    /// Directory that contains the app .desktop files
    #[clap(short = 'd',
    long = "app-dir",
    default_value = "/usr/share/applications")]
    app_dir: String,
    /// Length of time to study for in the 00h00m00s format
    #[clap(short = 't',
    long = "time")]
    time: String
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse(); // use clap's derive api to parse cli flags
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init(); // use env_logger to parse the verbosity flag
    utils::vprintln("Verbose Mode Enabled".to_string());

    let app_list = utils::expand_paths(args.app_list);
    let app_dir = utils::expand_paths(args.app_dir);

    study::begin_study(&app_list, &app_dir)?;
    let time = utils::parse_time(args.time);

    let timer = timer::Timer::new(); //create a new timer
    let (tx, rx) = channel(); // setup transmitting and receiving channel
    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(time), move || {
        // start the timer for the duration specified in args.time
        let _ignored = tx.send(()); // if it errors or finishes, send the result to the tx channel
    });
    rx.recv()?; // proceed once finished

    study::end_study(&app_list, &app_dir)?;

    Ok(()) //return ok
}