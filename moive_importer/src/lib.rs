use regex::Regex;
use rfd::FileDialog;
use std::{error::Error, fs, path::PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Moive {
    no: u32,
    year: String,
    title: String,
    tag: Option<String>,
}

fn disc_number(line: &str, re: &Regex) -> Option<u32> {
    re.captures(line)
        .map(|caps| caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
}

fn disc_moive(no: u32, line: &str, re: &Regex) -> Option<Moive> {
    re.captures(line).map(|caps| Moive {
        no,
        year: caps.get(1).unwrap().as_str().trim().to_string(),
        title: caps.get(2).unwrap().as_str().trim().to_string(),
        tag: caps.get(3).map(|c| c.as_str().trim().to_string()),
    })
}

fn save_moive_to_json(moives: Vec<Moive>) -> Result<PathBuf, Box<dyn Error>> {
    let serialized = serde_json::to_string_pretty(&moives)?;
    let path = FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_title("Save data to JSON file")
        .set_directory("/Users/mac/Documents/GitHub/rust-invaders/moive_importer")
        .save_file()
        .ok_or_else(|| "没有存储到文件")?;

    fs::write(&path, serialized)?;

    Ok(path)
}

pub fn read_txt_to_json(path: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let txt = fs::read_to_string(path)?;

    let mut disc_no = 0;
    let mut moives: Vec<Moive> = Vec::new();

    let re1 = Regex::new(r"^(\d+)\.$")?;
    let re2: Regex = Regex::new(r"^(\d{4})(.*?)(（儿童）)?$")?;

    for line in txt.lines().map(str::trim).filter(|l| !l.is_empty()) {
        // 获取no
        if let Some(no) = disc_number(line, &re1) {
            disc_no = no;
        } else if let Some(moive) = disc_moive(disc_no, line, &re2) {
            moives.push(moive);
        }
    }

    // 将moives存入json文件中
    save_moive_to_json(moives)
}
