use serde::Serialize;

#[derive(Serialize)]
pub struct HealthDTO {
    status: &'static str,
    message: &'static str,
}

pub struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self
    }

    pub fn healthcheck(&self) -> HealthDTO {
        HealthDTO {
            status: "Ok",
            message: "API is running",
        }
    }
}
