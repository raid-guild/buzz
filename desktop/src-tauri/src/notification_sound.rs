//! Native notification-sound playback.
//!
//! WebKitGTK media playback can fail even when the host has working audio
//! codecs. Keep Linux notification sounds on the same native rodio output path
//! used by huddles, while the frontend retains HTML audio on other platforms.

use std::{
    io::Cursor,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use tauri::State;

use crate::{app_state::AppState, huddle::audio_output::open_output_sink_by_name};

#[derive(Default)]
struct NotificationSoundInner {
    generation: AtomicU64,
    active: Mutex<Option<Arc<rodio::Player>>>,
}

#[derive(Default)]
pub(crate) struct NotificationSoundState {
    inner: Arc<NotificationSoundInner>,
}

impl NotificationSoundState {
    fn begin(&self) -> (u64, Arc<NotificationSoundInner>) {
        let generation = self.inner.generation.fetch_add(1, Ordering::SeqCst) + 1;
        if let Ok(mut active) = self.inner.active.lock() {
            if let Some(player) = active.take() {
                player.stop();
            }
        }
        (generation, Arc::clone(&self.inner))
    }

    fn stop(&self) -> Result<(), String> {
        self.inner.generation.fetch_add(1, Ordering::SeqCst);
        let mut active = self.inner.active.lock().map_err(|e| e.to_string())?;
        if let Some(player) = active.take() {
            player.stop();
        }
        Ok(())
    }
}

fn sound_bytes(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "bong" => include_bytes!("../../public/sounds/bong.mp3"),
        "boo" => include_bytes!("../../public/sounds/boo.mp3"),
        "dng" => include_bytes!("../../public/sounds/dng.mp3"),
        "doo" => include_bytes!("../../public/sounds/doo.mp3"),
        "doodone" => include_bytes!("../../public/sounds/doodone.mp3"),
        "doong" => include_bytes!("../../public/sounds/doong.mp3"),
        "doop" => include_bytes!("../../public/sounds/doop.mp3"),
        "flirl" => include_bytes!("../../public/sounds/flirl.mp3"),
        "flutter" => include_bytes!("../../public/sounds/flutter.mp3"),
        "oh-no" => include_bytes!("../../public/sounds/oh-no.mp3"),
        "ping" => include_bytes!("../../public/sounds/ping.mp3"),
        "unison" => include_bytes!("../../public/sounds/unison.mp3"),
        _ => return None,
    })
}

#[tauri::command]
/// Play one of the bundled notification sounds on the configured output device.
pub async fn play_notification_sound(
    name: String,
    state: State<'_, AppState>,
    playback: State<'_, NotificationSoundState>,
) -> Result<(), String> {
    let bytes = sound_bytes(&name).ok_or_else(|| format!("unknown notification sound: {name}"))?;
    let output_device = state
        .audio_output_device
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    let (generation, playback) = playback.begin();

    tokio::task::spawn_blocking(move || {
        let mut sink = open_output_sink_by_name(output_device.as_deref())?;
        sink.log_on_drop(false);
        let player = Arc::new(rodio::Player::connect_new(sink.mixer()));
        let source = rodio::Decoder::new(Cursor::new(bytes))
            .map_err(|e| format!("decode notification sound: {e}"))?;
        player.append(source);

        {
            let mut active = playback.active.lock().map_err(|e| e.to_string())?;
            if playback.generation.load(Ordering::SeqCst) != generation {
                player.stop();
                return Ok(());
            }
            if let Some(previous) = active.replace(Arc::clone(&player)) {
                previous.stop();
            }
        }

        player.sleep_until_end();

        let mut active = playback.active.lock().map_err(|e| e.to_string())?;
        if active
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, &player))
        {
            active.take();
        }
        Ok(())
    })
    .await
    .map_err(|e| format!("notification sound task failed: {e}"))?
}

#[tauri::command]
/// Stop the active bundled notification sound, if any.
pub fn stop_notification_sound(playback: State<'_, NotificationSoundState>) -> Result<(), String> {
    playback.stop()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use std::sync::atomic::Ordering;

    use super::{sound_bytes, NotificationSoundState};

    #[test]
    fn every_frontend_sound_name_has_embedded_audio() {
        for name in [
            "bong", "boo", "dng", "doo", "doodone", "doong", "doop", "flirl", "flutter", "oh-no",
            "ping", "unison",
        ] {
            let bytes = sound_bytes(name).unwrap();
            assert!(!bytes.is_empty(), "{name}");
            assert!(rodio::Decoder::new(Cursor::new(bytes)).is_ok(), "{name}");
        }
        assert!(sound_bytes("../not-a-sound").is_none());
    }

    #[test]
    fn stop_invalidates_pending_playback() {
        let state = NotificationSoundState::default();
        let (generation, inner) = state.begin();

        state.stop().unwrap();

        assert_ne!(inner.generation.load(Ordering::SeqCst), generation);
    }
}
