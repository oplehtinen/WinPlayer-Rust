use crate::{
    cltypes::{ClMetadata, ClPosition, ClStatus},
    player::{Player, PlayerEvent},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};
pub struct ClPlayer {
    player: Arc<Mutex<Player>>,
}

impl ClPlayer {
    pub fn new(player: Arc<Mutex<Player>>) -> Self {
        ClPlayer { player }
    }

    pub async fn poll_next_event(&mut self) -> String {
        let result = time::timeout(
            Duration::from_secs(2), // Set the timeout duration to 2 seconds
            async { self.player.lock().await.poll_next_event().await },
        )
        .await;

        match result {
            Ok(Some(PlayerEvent::PlaybackInfoChanged)) => String::from("PlaybackInfoChanged"),
            Ok(Some(PlayerEvent::MediaPropertiesChanged)) => String::from("MediaPropertiesChanged"),
            Ok(Some(PlayerEvent::TimelinePropertiesChanged)) => {
                String::from("TimelinePropertiesChanged")
            }
            Ok(None) => String::from("None"),
            Err(_) => String::from("Timeout"), // Handle the timeout case
        }
    }

    pub async fn get_status(&self) -> ClStatus {
        let player_guard = Arc::clone(&self.player);
        let status = {
            let player_guard = player_guard.lock().await;
            player_guard.get_status().await
        };

        ClStatus::from(status)
    }

    pub async fn get_aumid(&self) -> String {
        self.player.lock().await.get_aumid()
    }

    pub async fn play(&self) -> bool {
        self.player.lock().await.play().await
    }

    pub async fn pause(&self) -> bool {
        self.player.lock().await.pause().await
    }

    pub async fn play_pause(&self) -> bool {
        self.player.lock().await.play_pause().await
    }

    pub async fn stop(&self) -> bool {
        self.player.lock().await.stop().await
    }

    pub async fn get_playback_status(&self) -> String {
        self.player.lock().await.get_playback_status()
    }

    pub async fn next(&self) -> bool {
        println!("TRYING TO PLAY NEXT");
        self.player.lock().await.next().await
    }

    pub async fn previous(&self) -> bool {
        self.player.lock().await.previous().await
    }

    pub async fn set_shuffle(&self, value: bool) -> bool {
        self.player.lock().await.set_shuffle(value).await
    }

    pub async fn get_shuffle(&self) -> bool {
        self.player.lock().await.get_shuffle()
    }

    pub async fn set_repeat(&self, value: String) -> bool {
        self.player.lock().await.set_repeat(value).await
    }

    pub async fn get_repeat(&self) -> String {
        self.player.lock().await.get_repeat()
    }

    pub async fn seek(&self, offset_s: f64) -> bool {
        self.player.lock().await.seek(offset_s).await
    }

    pub async fn seek_percentage(&self, percentage: f64) -> bool {
        self.player.lock().await.seek_percentage(percentage).await
    }

    pub async fn set_position(&self, position_s: f64) -> bool {
        self.player.lock().await.set_position(position_s).await
    }

    pub async fn get_position(&self, wants_current_position: bool) -> Option<ClPosition> {
        if let Some(position) = self
            .player
            .lock()
            .await
            .get_position(wants_current_position)
            .await
        {
            return Some(ClPosition::from(position));
        }
        None
    }
}
