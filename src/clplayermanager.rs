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
        match self.player_manager.lock().await.poll_next_event().await {
            Some(ManagerEvent::ActiveSessionChanged) => String::from("ActiveSessionChanged"),
            Some(ManagerEvent::SystemSessionChanged) => String::from("SystemSessionChanged"),
            Some(ManagerEvent::SessionsChanged) => String::from("SessionsChanged"),
            None => String::from("None"),
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
        // loop through the sessions and figure out the active session
        println!("{:?}", sessions);
        for session in sessions {
            println!("Checking active session for: {:?}", session);
            let player = self.get_session(session.clone()).await;
            if let Some(player) = player {
                println!("Player found for session: {:?}", session);
                let status = player.get_status().await;
                println!("{:?}", status.status);
                if status.status == GlobalSystemMediaTransportControlsSessionPlaybackStatus(4) {
                    println!("Active session found: {:?}", session);
                    return Some(session);
                }
                // println!("{:?}", status);
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
