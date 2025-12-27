use rfd::FileDialog;
use std::error::Error;
use std::fs::File;
use std::process;
use moive_importer_02::read_txt_to_json;


fn main() -> Result<(), Box<dyn Error>> {
    // 1. 获取txt文件，读取里面的内容，存储成一个json文件
    match FileDialog::new()
        .add_filter("text", &["txt"])
        .set_directory("/Users/mac/Documents/GitHub/rust-invaders/moive_importer_02")
        .pick_file()
    {
        Some(p) => {
            println!("{:#?}", p);
            let file_path = read_txt_to_json(&p);
            Ok(())
        }
        None => {
            eprintln!("啥也没有选");
            process::exit(-1)
        }
    }
}
