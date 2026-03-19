use tracing::info;

pub const EMPTY: &str = "";

pub struct Logger {
    context: &'static str,
}

impl Logger {
    pub fn new(context: &'static str) -> Self {
        Self { context }
    }

    pub fn info_start(&self, method: &str, data: &impl std::fmt::Debug) {
        info!(data = ?data, "{} - {} - Start", self.context, method)
    }

    pub fn info_end(&self, method: &str, data: &impl std::fmt::Debug) {
        info!(data = ?data, "{} - {} - End", self.context, method)
    }
}
