#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    missing_docs,
    clippy::all
)]

use clap::{Parser, Subcommand};
use media::{
    currently_playing, get_current_session, next_track, pause, play, previous_track, Manager,
};

pub mod media;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Options
    #[clap(subcommand)]
    command: Commands,
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
#[derive(Subcommand)]
enum Commands {
    /// Play current track
    Play,
    /// Pause current track
    Pause,
    /// Play next track
    Next,
    /// Play previous track
    Previous,
    /// See what's currently playing
    Current,
    /// Watch for media changes using media manager
    Watch,
}

#[doc(hidden)]
fn main() {
    let cli = Cli::parse();

    let current_session = get_current_session().unwrap();

    match &cli.command {
        Commands::Play => play(current_session),
        Commands::Pause => pause(current_session),
        Commands::Next => next_track(current_session),
        Commands::Previous => previous_track(current_session),
        Commands::Current => currently_playing(current_session),
        Commands::Watch => Manager::new().start_sync(),
    }
}
