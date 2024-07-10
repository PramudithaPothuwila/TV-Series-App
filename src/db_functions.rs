use serde::{Deserialize, Serialize};
use std::io::Write;
#[derive(Debug, Serialize, Deserialize)]
pub struct TvSeries {
    pub title: String
}

const FILE_PATH: &str = "data\\tv_series.json";
pub fn add() {
    clearscreen::clear().expect("failed to clear screen");
    println!("--- Add TV Series ---");
    print!("Enter title : ");
    let mut tv_series = TvSeries {
        title: String::new()
    };

    match std::io::stdout().flush() {
        Ok(_) => {
            let mut title = String::new();
            match std::io::stdin().read_line(&mut title) {
                Ok(_) => {
                    tv_series.title = title.trim().to_string()
                },
                Err(error) => {
                    println!("Error : {}", error);
                    return add();
                }
            }
        },
        Err(error) => {
            println!("Error : {}", error);
            return add();
        }
    }
    save(tv_series);
}

pub fn list() {
    clearscreen::clear().expect("failed to clear screen");
    println!("--- List TV Series ---");
    let tv_series_list = load();
    for tv_series in tv_series_list {
        println!("{}", tv_series.title);
    }
}

fn save(tv_series: TvSeries) {
    let mut tv_series_list = load();
    if check_duplicates(&tv_series.title, &tv_series_list) {
        println!("Title already exists");
        return;
    }
    tv_series_list.push(tv_series);
    save_to_file(tv_series_list);
}

fn save_to_file(tv_series: Vec<TvSeries>) {
    let current_dir = match std::env::current_dir(){
        Ok(data) => {
            data
        },
        Err(error) => {
            println!("Path Error : {}", error);
            return;
        }
    };
    let path = current_dir.join(FILE_PATH);
    let parent_path = match path.parent() {
        Some(data) => data,
        None => {
            println!("Parent Path Error");
            return;
        }
    };
    if !path.exists() {
        match std::fs::create_dir_all(parent_path) {
            Ok(_) => {
                println!("Path created successfully");
            },
            Err(error) => {
                println!("Path Creation Error : {} \n Path: {}", error,path.display());
                return;
            }
        }
    }
    let mut file = match std::fs::File::create(path) {
        Ok(data) => data,
        Err(error) => {
            println!("File Read Error : {}", error);
            return;
        }
    };

    match serde_json::to_string(&tv_series) {
        Ok(data) => {
            let data = data.as_bytes();
            match file.write_all(data) {
                Ok(_) => {
                    println!("Data saved successfully");
                },
                Err(error) => {
                    println!("Error : {}", error);
                }
            }
        },
        Err(error) => {
            println!("Error : {}", error);
        }
    }
}

fn check_duplicates(title: &str, tv_series_list: &Vec<TvSeries>) -> bool {
    for tv_series in tv_series_list {
        if tv_series.title.to_lowercase() == title.to_lowercase() {
            return true;
        }
    }
    false
}

fn load() -> Vec<TvSeries> {
    let current_dir = std::env::current_dir().unwrap();
    let path = current_dir.join(FILE_PATH);
    match std::fs::read_to_string(path) {
        Ok(data) => {
            serde_json::from_str(&data).unwrap_or_else(|error| {
                println!("Error : {}", error);
                Vec::new()
            })
        },
        Err(_) => Vec::new()
    }
}