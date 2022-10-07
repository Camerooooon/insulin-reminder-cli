#[derive(serde::Deserialize, Debug)]
pub struct Dose {
    pub time: u64,
    pub units: u8,
}

#[derive(serde::Deserialize, Debug)]
pub struct DoseResponse {
    dose: Option<Dose>,
    timestamp: Option<String>,
    success: bool,
    insulin_time: bool,
    time_until: Option<i64>,
}
