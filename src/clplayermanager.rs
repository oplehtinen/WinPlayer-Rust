use super::clplayer::ClPlayer;
use crate::playermanager::{ManagerEvent, PlayerManager};
use std::sync::Arc;
use tokio::sync::Mutex;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus;

pub struct ClPlayerManager {
    player_manager: Arc<Mutex<PlayerManager>>,
}

impl ClPlayerManager {
    pub fn new(player_manager: Arc<Mutex<PlayerManager>>) -> Self {
        ClPlayerManager { player_manager }
    }

    pub async fn poll_next_event(&mut self) -> String {
        let result = tokio::time::timeout(
            tokio::time::Duration::from_secs(2), // Set the timeout duration to 2 seconds
            async { self.player_manager.lock().await.poll_next_event().await },
        )
        .await;

        match result {
            Ok(Some(ManagerEvent::ActiveSessionChanged)) => String::from("ActiveSessionChanged"),
            Ok(Some(ManagerEvent::SystemSessionChanged)) => String::from("SystemSessionChanged"),
            Ok(Some(ManagerEvent::SessionsChanged)) => String::from("SessionsChanged"),
            Ok(None) => String::from("None"),
            Err(_) => String::from("Timeout"), // Handle the timeout case
        }
    }
    pub async fn get_active_session(&self) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.lock().await.get_active_session() {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub async fn get_session(&self, aumid: String) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.lock().await.get_session(&aumid) {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub async fn get_sessions_keys(&self) -> Vec<String> {
        self.player_manager.lock().await.get_sessions_keys()
    }

    pub async fn get_system_session(&self) -> Option<ClPlayer> {
        if let Some(player) = self.player_manager.lock().await.get_system_session() {
            return Some(ClPlayer::new(player));
        }
        None
    }

    pub async fn figure_out_active_session(&self) -> Option<String> {
        let sessions = self.get_sessions_keys().await;

        // First, check for any players that are playing
        for session in &sessions {
            let player = self.get_session(session.clone()).await;
            if let Some(player) = player {
                let status = player.get_status().await;
                if status.status == GlobalSystemMediaTransportControlsSessionPlaybackStatus(4) {
                    return Some(session.clone());
                }
            } else {
                println!("No player found for session: {:?}", session);
            }
        }

        // If no players are playing, check for paused players
        for session in sessions {
            let player = self.get_session(session.clone()).await;
            if let Some(player) = player {
                let status = player.get_status().await;
                if status.status == GlobalSystemMediaTransportControlsSessionPlaybackStatus(5) {
                    return Some(session);
                }
            } else {
                println!("No player found for session: {:?}", session);
            }
        }

        return None;
        /* `std::string::String` value */
    }

    pub async fn update_system_session(&mut self) {
        self.player_manager.lock().await.update_system_session()
    }

    pub async fn update_sessions(&mut self, denylist: Option<Vec<String>>) {
        self.player_manager
            .lock()
            .await
            .update_sessions(denylist.as_ref())
    }
}
