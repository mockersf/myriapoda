use chrono;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Conference {
    name: String,
    cfp_start: Option<chrono::DateTime<chrono::Utc>>,
    cfp_end: Option<chrono::DateTime<chrono::Utc>>,
    begin: Option<chrono::DateTime<chrono::Utc>>,
    end: Option<chrono::DateTime<chrono::Utc>>,
    recurrence: Option<Recurrence>,
    location: Location,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Recurrence {
    location_change: bool,
    period_days: u16,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Location {
    name: String,
}
