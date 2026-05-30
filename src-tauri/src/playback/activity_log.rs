use super::events::PlaybackEvent;

#[derive(Clone, Debug, Default)]
pub struct PlaybackActivityLog {
    items: Vec<PlaybackEvent>,
    limit: usize,
}

impl PlaybackActivityLog {
    pub fn new(limit: usize) -> Self {
        Self {
            items: Vec::new(),
            limit,
        }
    }

    pub fn append(&mut self, item: PlaybackEvent) {
        if self.limit == 0 {
            return;
        }

        if self.items.len() >= self.limit {
            self.items.drain(0..1);
        }

        self.items.push(item);
    }

    pub fn snapshot(&self) -> Vec<PlaybackEvent> {
        self.items.clone()
    }
}
