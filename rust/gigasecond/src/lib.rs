use time::{Duration, PlainDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: PlainDateTime) -> PlainDateTime {
    start + Duration::seconds(1000000000)
}
