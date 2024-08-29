use std::fs;
use std::path::PathBuf;
fn main() {
 let uri = "https://www.apipost.cn";
    let output = "apipost.md";
    println!("获取到的地址:{}",uri);
    let body = reqwest::blocking::get(uri).unwrap().text().unwrap();

    println!("正在转换中...");
    let md = html2md::parse_html(&body);
    fs::write(output,md.as_bytes()).unwrap();
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push(output);
    println!("md放到的位置;{:?}",config_path)
}
