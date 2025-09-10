#[derive(Debug, Clone)]
pub struct Config {
    pub top_k: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { top_k: 5 }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }
}
