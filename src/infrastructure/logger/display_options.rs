#[derive(Debug, Clone, Copy)]
pub struct LogDisplayOptions {
    pub level: bool,
    pub target: bool,
    pub thread_ids: bool,
    pub thread_names: bool,
    pub source_location: bool,
}

impl Default for LogDisplayOptions {
    fn default() -> Self {
        Self {
            level: true,
            target: true,
            thread_ids: false,
            thread_names: false,
            source_location: false,
        }
    }
}