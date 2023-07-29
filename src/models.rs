use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Report {
    pub url: String,
    pub has_valid_format: bool,
    pub is_merged: bool,
    pub has_error: bool,
}

impl Report {
    pub fn new(url: String) -> Self {
        Report {
            url,
            has_valid_format: true,
            is_merged: false,
            has_error: false,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct Output {
    pub reports: Vec<Report>,
    pub failed_qty: u32,
}
