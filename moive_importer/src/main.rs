use moive_importer::read_txt_to_json;
use rfd::FileDialog;
use std::process;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 选择一个文件
    // 2. 读取文件内容
    // 3. 将文件内容存储到一个json中
    match FileDialog::new()
        .add_filter("选择txt文件", &["txt"])
        .set_directory("/Users/mac/Documents/GitHub/rust-invaders/moive_importer")
        .pick_file() {
            Some(path) => {
                // 将文件内容存储到json中
                let saved_path = read_txt_to_json(&path);
                Ok(())
            },
            None => {
                eprintln!("File not selected.");
                process::exit(-1)
            }
        }

}
