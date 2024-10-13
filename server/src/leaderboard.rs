use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct Leaderboard {
    scores: Arc<Mutex<HashMap<usize, u32>>>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            scores: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn update_score(&self, client: usize, score: u32) {
        let mut scores = self.scores.lock().await;
        scores.insert(client, score);
    }

    pub async fn get_scores(&self) -> Vec<(usize, u32)> {
        let scores = self.scores.lock().await;
        let mut scores_vec: Vec<_> = scores.iter().map(|(k, v)| (k.clone(), *v)).collect();
        scores_vec.sort_by(|a, b| b.1.cmp(&a.1));
        scores_vec
    }
}
