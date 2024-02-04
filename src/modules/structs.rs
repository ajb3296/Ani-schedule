use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub week: String,
    pub animeNo: i32,
    pub status: String,
    pub time: String,
    pub subject: String,
    pub genres: String,
    pub captionCount: i32,
    pub startDate: String,
    pub endDate: String,
    pub website: String,
}

#[derive(Debug, Deserialize)]
pub struct AniData {
    pub code: String,
    pub data: Vec<Item>,
}