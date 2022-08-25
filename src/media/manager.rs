use futures::executor::block_on;
use serde::Serialize;
use windows::{
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Media::{
        Control::{
            GlobalSystemMediaTransportControlsSession,
            GlobalSystemMediaTransportControlsSessionManager,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus,
        },
        MediaPlaybackType,
    },
};

use crate::controller::ThreadMessage;

/// Messages that the MediaManager can send
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum ManagerMessage {
    SessionChanged,
    TimelineChanged,
    PlaybackInfoChanged,
    MediaChanged,
}

/// Media Manager.
#[derive(Debug)]
pub struct Manager {
    current_session: GlobalSystemMediaTransportControlsSession,
    manager: GlobalSystemMediaTransportControlsSessionManager,
    media_changed: EventRegistrationToken,
    session_changed: EventRegistrationToken,
    timeline_changed: EventRegistrationToken,
    playbackinfo_changed: EventRegistrationToken,

    tx: crossbeam_channel::Sender<ThreadMessage>,
    rx: crossbeam_channel::Receiver<ThreadMessage>,
}

#[derive(Serialize)]
struct MediaProps {
    // TODO Maybe check out adding thumbnails
    album_artist: String,   // HSTRING -> String
    album_title: String,    // HSTRING -> String
    album_track_count: i32, // (UNCHANGED)
    artist: String,         // HSTRING -> String
    playback_type: i32,     // MediaPlaybackType -> i32
    subtitle: String,       // HSTRING -> String
    title: String,          // HSTRING -> String
    track_number: i32,      // (UNCHANGED)
}

#[derive(Serialize)]
struct TimelineProps {
    last_updated_time: i32, // DateTime -> i32
    pos: i32,               // TimeSpan -> i32
    max_seek_time: i32,     // TimeSpan -> i32
    min_seek_time: i32,     // TimeSpan -> i32
    endtime: i32,           // TimeSpan -> i32
    start_time: i32,        // TimeSpan -> i32
}

#[derive(Serialize)]
struct PlaybackInfoProps {
    auto_repeat_mode: i32,           // MediaPlaybackAutoRepeatMode -> i32
    active_controls: ActiveControls, // GlobalSystemMediaTransportControlsSessionPlaybackControls -> ActiveControls
    shuffle_active: bool,            // (UNCHANGED)
    playback_status: i32,            // MediaPlaybackStatus -> i32
    playback_type: i32,              // MediaPlaybackType -> i32
    playback_rate: f64,              // (UNCHANGED)
}

#[derive(Serialize)]
struct ActiveControls {
    is_play_enabled: bool,
    is_pause_enabled: bool,
    is_stop_enabled: bool,
    is_record_enabled: bool,
    is_fast_forward_enabled: bool,
    is_rewind_enabled: bool,
    is_next_enabled: bool,
    is_previous_enabled: bool,
    is_channel_up_enabled: bool,
    is_channel_down_enabled: bool,
    is_play_pause_toggle_enabled: bool,
    is_shuffle_enabled: bool,
    is_repeat_enabled: bool,
    is_playback_rate_enabled: bool,
    is_playback_position_enabled: bool,
}

impl Manager {
    /// Create a new media manager
    pub fn new(
        tx: crossbeam_channel::Sender<ThreadMessage>,
        rx: crossbeam_channel::Receiver<ThreadMessage>,
    ) -> Self {
        let manager =
            block_on(GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap())
                .unwrap();
        // TODO wait until a session is availible if there isn't one on creation
        let current_session = manager.GetCurrentSession().unwrap();

        // Add event listeners
        let new_tx = tx.clone();
        let media_changed = current_session
            .MediaPropertiesChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::MediaChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        let new_tx = tx.clone();
        let timeline_changed = current_session
            .TimelinePropertiesChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::TimelineChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        let new_tx = tx.clone();
        let playbackinfo_changed = current_session
            .PlaybackInfoChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::PlaybackInfoChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        let new_tx = tx.clone();
        let session_changed = manager
            .CurrentSessionChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::SessionChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        println!("[Media Manager] Spawned new media manager");

        Self {
            current_session,
            manager,
            media_changed,
            session_changed,
            timeline_changed,
            playbackinfo_changed,

            tx,
            rx,
        }
    }

    /// Start a thread blocking event loop
    pub fn start_sync(&mut self) {
        loop {
            let msg = self.rx.recv().unwrap();

            match msg {
                ThreadMessage::Stop => {
                    println!("[Media Manager] Stopping Manager...");
                    drop(self);
                    break;
                }
                ThreadMessage::Media(ManagerMessage::SessionChanged) => {
                    println!(
                        "[Media Manager] Session changed... Attempting to update session info."
                    );
                    self.session_changed();
                }
                ThreadMessage::Media(ManagerMessage::TimelineChanged) => {
                    Self::timeline_changed().unwrap();
                }
                ThreadMessage::Media(ManagerMessage::PlaybackInfoChanged) => {
                    Self::playback_info_changed().unwrap();
                }
                ThreadMessage::Media(ManagerMessage::MediaChanged) => {
                    Self::media_props_changed().unwrap();
                }
                _ => (),
            }
        }
    }

    fn session_changed(&mut self) {
        // Drop old event listeners
        self.current_session
            .RemoveMediaPropertiesChanged(self.media_changed)
            .unwrap();
        self.current_session
            .RemoveTimelinePropertiesChanged(self.timeline_changed)
            .unwrap();
        self.current_session
            .RemoveTimelinePropertiesChanged(self.playbackinfo_changed)
            .unwrap();

        // Re-add event listeners to the new session
        // TODO PANIC: when closing all sessions
        //        Solution: wait for new sessions in a blocked loop
        //              --> this for when you start the manager without a session too
        //              --> i think making a function for this is the right way
        let current_session = self.manager.GetCurrentSession().unwrap();

        let new_tx = self.tx.clone();
        let media_changed = current_session
            .MediaPropertiesChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::MediaChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        let new_tx = self.tx.clone();
        let timeline_changed = current_session
            .TimelinePropertiesChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::TimelineChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        let new_tx = self.tx.clone();
        let playbackinfo_changed = current_session
            .PlaybackInfoChanged(TypedEventHandler::new(move |_, _| {
                new_tx
                    .send(ThreadMessage::Media(ManagerMessage::PlaybackInfoChanged))
                    .unwrap();
                Ok(())
            }))
            .unwrap();

        // Update manager
        self.current_session = current_session;
        self.media_changed = media_changed;
        self.timeline_changed = timeline_changed;
        self.playbackinfo_changed = playbackinfo_changed;

        println!(
            "[Media Manager] New Session ID: {}",
            self.current_session.SourceAppUserModelId().unwrap()
        );
    }

    fn timeline_changed() -> windows::core::Result<()> {
        let session = crate::media::get_current_session().unwrap();
        let status = session.GetPlaybackInfo().unwrap().PlaybackStatus().unwrap();
        if status != GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing {
            return Ok(());
        }

        let timeline_props = session.GetTimelineProperties()?;

        println!(
            "\
            -- START TIMELINE CHANGE --\n\
            \tendtime: {}\n\
            \tlast updated time: {}\n\
            \tmax seek time: {}\n\
            \tmin seek time: {}\n\
            \tpos: {}\n\
            \tstart time: {}\n\
            -- END TIMELINE CHANGE --\
            \n",
            timeline_props.EndTime().unwrap().Duration,
            timeline_props.LastUpdatedTime().unwrap().UniversalTime,
            timeline_props.MaxSeekTime().unwrap().Duration,
            timeline_props.MinSeekTime().unwrap().Duration,
            timeline_props.Position().unwrap().Duration,
            timeline_props.StartTime().unwrap().Duration,
        );
        Ok(())
    }

    fn media_props_changed() -> windows::core::Result<()> {
        let session = crate::media::get_current_session().unwrap();
        let media_props = block_on(session.TryGetMediaPropertiesAsync().unwrap()).unwrap();

        println!(
            "\
            -- START MEDIA_PROP CHANGE --\n\
            \talbum artist: {}\n\
            \talbum title: {}\n\
            \talbum track count: {}\n\
            \tartist: {}\n\
            \tpb type: {:?}\n\
            \tsubtitle: {}\n\
            \ttitle: {}\n\
            \ttrack #: {}\n\
            -- END MEDIA_PROP CHANGE --\
            \n",
            media_props.AlbumArtist().unwrap(),
            media_props.AlbumTitle().unwrap(),
            media_props.AlbumTrackCount().unwrap(),
            media_props.Artist().unwrap(),
            {
                if media_props.PlaybackType().is_ok() {
                    media_props.PlaybackType().unwrap().Value().unwrap()
                } else {
                    MediaPlaybackType::Unknown
                }
            },
            media_props.Subtitle().unwrap(),
            media_props.Title().unwrap(),
            media_props.TrackNumber().unwrap(),
        );
        Ok(())
    }

    fn playback_info_changed() -> windows::core::Result<()> {
        let session = crate::media::get_current_session().unwrap();
        let playback_info = session.GetPlaybackInfo()?;

        println!(
            "\
            -- START PLAYBACK_INFO CHANGE --\n\
            \tshuffle active?: {}\n\
            \tpb status: {:?}\n\
            \tpb type: {:?}\n\
            -- END PLAYBACK_INFO CHANGE --\
            \n",
            // playback_info.AutoRepeatMode().unwrap().Value().unwrap().0,
            // playback_info.Controls().unwrap(),
            {
                if playback_info.IsShuffleActive().is_ok() {
                    playback_info.IsShuffleActive().unwrap().Value().unwrap()
                } else {
                    false
                }
            },
            // playback_info.PlaybackRate().unwrap().Value().unwrap(),
            {
                if playback_info.PlaybackStatus().is_ok() {
                    crate::media::playback_status_string(playback_info.PlaybackStatus().unwrap())
                } else {
                    "UNDEFINED".to_string()
                }
            },
            {
                if playback_info.PlaybackType().is_ok() {
                    playback_info.PlaybackType().unwrap().Value().unwrap()
                } else {
                    MediaPlaybackType::Unknown
                }
            },
        );

        Ok(())
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        self.current_session
            .RemoveMediaPropertiesChanged(self.media_changed)
            .ok();
        self.current_session
            .RemoveTimelinePropertiesChanged(self.timeline_changed)
            .ok();
        self.current_session
            .RemovePlaybackInfoChanged(self.playbackinfo_changed)
            .ok();
        self.manager
            .RemoveCurrentSessionChanged(self.session_changed)
            .ok();
        println!("[Media Manager] Disposed of the media manager");
    }
}
