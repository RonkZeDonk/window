use clap::{Parser, Subcommand};
use window::{
    controller::{Thread, ThreadController, ThreadMessage},
    media::{
        currently_playing, currently_playing_raw, get_current_session, next_track, pause, play,
        previous_track, Manager,
    },
};

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
    /// Get the currently playing data in JSON format
    CurrentJSON,
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
        Commands::CurrentJSON => println!("{}", currently_playing_raw(current_session)),
        Commands::Watch => {
            let (tx, rx) = crossbeam_channel::unbounded();

            let txc = tx.clone();
            ctrlc::set_handler(move || {
                txc.send(ThreadMessage::Stop).unwrap();
            })
            .expect("Error setting ctrlc handler");

            let txc = tx.clone();
            ThreadController::new(rx)
                .add_thread(Thread::new(move |rx| {
                    Manager::new(txc, rx).start_sync();
                }))
                .begin();
        }
    }
}
