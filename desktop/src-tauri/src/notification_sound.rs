//! Native notification-sound playback.
//!
//! WebKitGTK media playback can fail even when the host has working audio
//! codecs. Keep Linux notification sounds on the same native rodio output path
//! used by huddles, while the frontend retains HTML audio on other platforms.

use std::io::Cursor;

use tauri::State;

use crate::{app_state::AppState, huddle::audio_output::open_output_sink_by_name};

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
) -> Result<(), String> {
    let bytes = sound_bytes(&name)
        .ok_or_else(|| format!("unknown notification sound: {name}"))?;
    let output_device = state
        .audio_output_device
        .lock()
        .map_err(|e| e.to_string())?
        .clone();

    tokio::task::spawn_blocking(move || {
        let mut sink = open_output_sink_by_name(output_device.as_deref())?;
        sink.log_on_drop(false);
        let player = rodio::Player::connect_new(sink.mixer());
        let source = rodio::Decoder::new(Cursor::new(bytes))
            .map_err(|e| format!("decode notification sound: {e}"))?;
        player.append(source);
        player.sleep_until_end();
        Ok(())
    })
    .await
    .map_err(|e| format!("notification sound task failed: {e}"))?
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::sound_bytes;

    #[test]
    fn every_frontend_sound_name_has_embedded_audio() {
        for name in [
            "bong", "boo", "dng", "doo", "doodone", "doong", "doop", "flirl", "flutter",
            "oh-no", "ping", "unison",
        ] {
            let bytes = sound_bytes(name).unwrap();
            assert!(!bytes.is_empty(), "{name}");
            assert!(rodio::Decoder::new(Cursor::new(bytes)).is_ok(), "{name}");
        }
        assert!(sound_bytes("../not-a-sound").is_none());
    }
}
