use rand::seq::IteratorRandom;

pub struct RandomPicker {
    words: Vec<String>,
}

impl RandomPicker {
    pub fn new(words: Vec<String>) -> RandomPicker {
        return RandomPicker { words };
    }
}

impl Iterator for RandomPicker {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        return self.words.iter().choose(&mut rand::thread_rng()).cloned();
    }
}
