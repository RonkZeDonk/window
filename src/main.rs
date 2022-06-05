//! Get's and changes the current media session's properties.

pub mod media;

#[doc(hidden)]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let current_session = match media::get_current_session() {
        Ok(session) => session,
        Err(err) => return println!("ERROR: {}", err),
    };

    if args.len() == 1 {
        // If no args are specified just run currently playing as default
        media::currently_playing(current_session);
        return;
    };
    match args[1].as_str() {
        "cp" => loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            media::currently_playing(media::get_current_session().unwrap());
            std::thread::sleep(std::time::Duration::from_secs(2));
        },
        "cp_raw" => println!("{}", media::currently_playing_raw(current_session)),
        "pl" => media::play(current_session),
        "pa" => media::pause(current_session),
        "nt" => media::next_track(current_session),
        "pt" => media::previous_track(current_session),
        "wc" => media::Manager::new().start_sync(),
        _ => println!(
            "\
            cp\tCurrently Playing\n\
            pl\tPlay\n\
            pa\tPause\n\
            nt\tNext Track\n\
            pt\tPrevious Track\n\
        "
        ),
    };
}
