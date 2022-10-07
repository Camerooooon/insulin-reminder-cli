#[derive(serde::Deserialize, Debug)]
pub struct Dose {
    pub time: u64,
    pub units: u8,
}

#[derive(serde::Deserialize, Debug)]
pub struct DoseResponse {
    pub dose: Option<Dose>,
    pub timestamp: Option<String>,
    pub success: bool,
    pub insulin_time: bool,
    pub time_until: Option<i64>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DoseRequest {
    pub dose: u8,
}
