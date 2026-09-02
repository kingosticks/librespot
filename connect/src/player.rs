use crate::{
    core::SpotifyUri,
    playback::player::{Player, PlayerEventChannel},
};

/// Playback interface required by Spotify Connect.
///
/// Implementations receive playback commands from [`crate::Spirc`] and provide
/// a stream of player events used to keep Connect state in sync. The default
/// implementation is [`librespot_playback::player::Player`], but consumers may
/// provide an alternative playback implementation.
pub trait PlayerController: Send + Sync {
    /// Loads a track and optionally starts playback at the requested position.
    fn load(&self, track_id: SpotifyUri, start_playing: bool, position_ms: u32);

    /// Preloads a track for subsequent playback.
    fn preload(&self, track_id: SpotifyUri);

    /// Starts or resumes playback.
    fn play(&self);

    /// Pauses playback.
    fn pause(&self);

    /// Stops playback.
    fn stop(&self);

    /// Seeks to the requested position in milliseconds.
    fn seek(&self, position_ms: u32);

    /// Returns a receiver for player events.
    fn get_player_event_channel(&self) -> PlayerEventChannel;

    /// Emits a volume-changed event.
    fn emit_volume_changed_event(&self, _volume: u16) {}

    /// Emits a session-connected event.
    fn emit_session_connected_event(&self, _connection_id: String, _user_name: String) {}

    /// Emits a session-disconnected event.
    fn emit_session_disconnected_event(&self, _connection_id: String, _user_name: String) {}

    /// Emits a session-client-changed event.
    fn emit_session_client_changed_event(
        &self,
        _client_id: String,
        _client_name: String,
        _client_brand_name: String,
        _client_model_name: String,
    ) {
    }

    /// Emits a filter-explicit-content-changed event.
    fn emit_filter_explicit_content_changed_event(&self, _filter: bool) {}

    /// Emits a shuffle-changed event.
    fn emit_shuffle_changed_event(&self, _shuffle: bool) {}

    /// Emits a repeat-changed event.
    fn emit_repeat_changed_event(&self, _context: bool, _track: bool) {}

    /// Emits an autoplay-changed event.
    fn emit_auto_play_changed_event(&self, _auto_play: bool) {}
}

impl PlayerController for Player {
    fn load(&self, track_id: SpotifyUri, start_playing: bool, position_ms: u32) {
        Player::load(self, track_id, start_playing, position_ms)
    }

    fn preload(&self, track_id: SpotifyUri) {
        Player::preload(self, track_id)
    }

    fn play(&self) {
        Player::play(self)
    }

    fn pause(&self) {
        Player::pause(self)
    }

    fn stop(&self) {
        Player::stop(self)
    }

    fn seek(&self, position_ms: u32) {
        Player::seek(self, position_ms)
    }

    fn get_player_event_channel(&self) -> PlayerEventChannel {
        Player::get_player_event_channel(self)
    }

    fn emit_volume_changed_event(&self, volume: u16) {
        Player::emit_volume_changed_event(self, volume)
    }

    fn emit_session_connected_event(&self, connection_id: String, user_name: String) {
        Player::emit_session_connected_event(self, connection_id, user_name)
    }

    fn emit_session_disconnected_event(&self, connection_id: String, user_name: String) {
        Player::emit_session_disconnected_event(self, connection_id, user_name)
    }

    fn emit_session_client_changed_event(
        &self,
        client_id: String,
        client_name: String,
        client_brand_name: String,
        client_model_name: String,
    ) {
        Player::emit_session_client_changed_event(
            self,
            client_id,
            client_name,
            client_brand_name,
            client_model_name,
        )
    }

    fn emit_filter_explicit_content_changed_event(&self, filter: bool) {
        Player::emit_filter_explicit_content_changed_event(self, filter)
    }

    fn emit_shuffle_changed_event(&self, shuffle: bool) {
        Player::emit_shuffle_changed_event(self, shuffle)
    }

    fn emit_repeat_changed_event(&self, context: bool, track: bool) {
        Player::emit_repeat_changed_event(self, context, track)
    }

    fn emit_auto_play_changed_event(&self, auto_play: bool) {
        Player::emit_auto_play_changed_event(self, auto_play)
    }
}
