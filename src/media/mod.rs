use futures::executor::block_on;

use serde::Serialize;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession as MediaSession,
    GlobalSystemMediaTransportControlsSessionManager as MediaManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
};

#[derive(Serialize)]
struct MusicInfo {
    title: String,
    artist: String,
    album_title: String,
    finished_percentage: String,
    status: String,
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
        .unwrap();

    let status_string: String = match status {
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Closed => "CLOSED".to_string(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Opened => "OPENED".to_string(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Changing => "CHANGING".to_string(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped => "STOPPED".to_string(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => "PLAYING".to_string(),
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => "PAUSED".to_string(),
        _ => unreachable!(), // Default case should be unreachable
    };

    MusicInfo {
        title: title.to_string(),
        artist: artist.to_string(),
        album_title: album_title.to_string(),
        finished_percentage: finished_percentage.to_string(),
        status: status_string.to_string(),
    }
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
    format!("{}", serde_json::to_string(&music_info).unwrap())
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
