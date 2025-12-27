use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Serialize, Deserialize, Debug)]
struct Moive {
    no: u32,
    year: String,
    name: String,
    tag: Option<String>,
}

pub fn read_txt_to_json(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let mut n: u32 = 0;
    let mut moives = Vec::new();

    let re1 = Regex::new(r"^(\d+)\.$")?;
    let re2: Regex = Regex::new(r"^(\d{4})(.*?)(（儿童）)?$")?;

    for line in content.lines() {
        if (!line.trim().is_empty()) {
            match re1.captures(line) {
                Some(caps) => {
                    n = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    println!("{:?}", n);
                }
                None => println!("没有匹配到东西"),
            };

            match re2.captures(line) {
                Some(caps) => moives.push(Moive {
                    no: n,
                    year: caps.get(1).unwrap().as_str().trim().to_string(),
                    name: caps.get(1).unwrap().as_str().trim().to_string(),
                    tag: Some(caps.get(1).unwrap().as_str().trim().to_string()),
                }),
                None => println!("没有匹配到东西"),
            }
        }
    }
    // 将moives转成json字符串
    let serialized = serde_json::to_string(&moives).unwrap();


    let path = FileDialog::new()
        .add_filter("json", &["json"])
        .set_directory("/Users/mac/Documents/GitHub/rust-invaders/moive_importer_02")
        .save_file().ok_or_else(|| "没有存进去")?;

    println!("{:?}", path);

    fs::write(&path, serialized)?;


    Ok(PathBuf::new())
}
