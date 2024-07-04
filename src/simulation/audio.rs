use std::collections::VecDeque;

pub struct Audio {
    notes: VecDeque<f32>,
}

impl Audio {
    pub fn new() -> Self {
        Self {
            notes: VecDeque::new()
        }
    }

    pub fn add_note(&mut self, note: f32) {
        self.notes.push_back(note);
    }

    pub fn get_note(&mut self) -> Option<f32> {
        self.notes.pop_front()
    }
}