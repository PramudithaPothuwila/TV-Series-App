use serde::{Deserialize, Serialize};

pub fn search() {
    println!("--- Search TV Series ---");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvSeries {
    pub title: String,
    pub seasons: i32,
}
fn load_data() -> Vec<TvSeries> {
    let mut tv_series = Vec::new();

    let file = std::fs::File::open("data/tv_series.json");

    match file {
        Ok(data) => {
            match serde_json::from_reader(data) {
                Ok(data) => {
                    tv_series = data;
                },
                Err(_) => {
                    println!("Error : Failed to parse data");
                    return tv_series;
                }
            }
        },
        Err(_) => {
            println!("Error : Failed to load data");
        }
    }
    return tv_series;
}