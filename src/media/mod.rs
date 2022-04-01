use futures::executor::block_on;

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as MediaSession,
    GlobalSystemMediaTransportControlsSessionManager as MediaManager,
};

struct MusicInfo {
    title: windows::core::HSTRING,
    artist: windows::core::HSTRING,
    album_title: windows::core::HSTRING,
    finished_percentage: windows::core::HSTRING,
    status: windows::core::HSTRING,
}

impl MusicInfo {
    #[must_use]
    fn new(
        title: windows::core::HSTRING,
        artist: windows::core::HSTRING,
        album_title: windows::core::HSTRING,
        finished_percentage: windows::core::HSTRING,
        status: windows::core::HSTRING,
    ) -> Self {
        Self {
            title,
            artist,
            album_title,
            finished_percentage,
            status,
        }
    }
}

impl std::fmt::Display for MusicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ \"{}\": \"{}\", \"{}\": \"{}\", \"{}\": \"{}\", \"{}\": \"{}\", \"{}\": \"{}\" }}",
            "title", self.title,
            "artist", self.artist,
            "album_title", self.album_title,
            "finished_percentage", self.finished_percentage,
            "status", self.status
        )
    }
}

/// Gets the current media session. This value will be used in most other function
pub fn get_current_session() -> Result<MediaSession, &'static str> {
    let sessions = MediaManager::RequestAsync();
    let sessions_results = block_on(sessions.unwrap()).unwrap();

    if sessions_results.GetCurrentSession().is_err() {
        return Err("There is no current session");
    }
    let current_session = sessions_results.GetCurrentSession().unwrap();
    return Ok(current_session);
}

/// Gets a hashmap containing information of currently playing music/media
fn get_music_info(session: MediaSession) -> MusicInfo {
    let media_properties = block_on(session.TryGetMediaPropertiesAsync().unwrap()).unwrap();
    let title = media_properties.Title().unwrap();
    let artist = media_properties.Artist().unwrap();
    let album_title = media_properties.AlbumTitle().unwrap();

    let timeline_props = session.GetTimelineProperties().unwrap();
    let finished_percentage = ((timeline_props.Position().unwrap().Duration as f32
        / timeline_props.MaxSeekTime().unwrap().Duration as f32)
        * 100.0)
        .round() as u8;

    let status = session
        .GetPlaybackInfo()
        .unwrap()
        .PlaybackStatus()
        .unwrap()
        .0;

    // Possible retured status values
    // from windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus

    // Closed: 0
    // Opened: 1
    // Changing: 2
    // Stopped: 3
    // Playing: 4
    // Paused: 5

    let status_string: String = match status {
        0 => "CLOSED".to_string(),
        1 => "OPENED".to_string(),
        2 => "CHANGING".to_string(),
        3 => "STOPPED".to_string(),
        4 => "PLAYING".to_string(),
        5 => "PAUSED".to_string(),
        _ => "UNKNOWN".to_string(), // Default case should be unreachable
    };

    MusicInfo::new(
        title,
        artist,
        album_title,
        windows::core::HSTRING::from(finished_percentage.to_string()),
        windows::core::HSTRING::from(status_string.to_string())
    )
}

/// Needed make sure that the command is fully processed before exiting
#[doc(hidden)]
fn post_change_routine(
    res: Result<windows::Foundation::IAsyncOperation<bool>, windows::core::Error>,
) {
    if res.is_ok() {
        std::thread::sleep(std::time::Duration::from_millis(50))
    }
}

/// Goes to the previous track on the given session
pub fn previous_track(session: MediaSession) {
    let res = session.TrySkipPreviousAsync();
    post_change_routine(res);
}

/// Goes to the next track on the given session
pub fn next_track(session: MediaSession) {
    let res = session.TrySkipNextAsync();
    post_change_routine(res);
}

/// Resumes playback on the given session
pub fn play(session: MediaSession) {
    let res = session.TryPlayAsync();
    post_change_routine(res);
}

/// Pauses playback on the given session
pub fn pause(session: MediaSession) {
    let res = session.TryPauseAsync();
    post_change_routine(res);
}

/// Returns raw currently playing of the given session
pub fn currently_playing_raw(session: MediaSession) -> String {
    let music_info = get_music_info(session);
    format!("{}", music_info)
}

/// Get formated currently playing info (printed out in console)
pub fn currently_playing(session: MediaSession) {
    let music_info = get_music_info(session);
    println!(
        "=======================================\n\
        Currently Playing: {} - {}\n\
        {}% Finished -- {}\n\
         =======================================",
        music_info.artist,
        music_info.title,
        music_info.finished_percentage,
        music_info.status
    );
}
