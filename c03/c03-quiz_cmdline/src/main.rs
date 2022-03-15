use std::fs;


//To test: cargo run -- https://www.rust-lang.org rust.md
fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let [_path, url, output] = args.as_slice() {
        println!(">> Processing: url={}, output={}", url, output);
        println!("Fetching url: {}", url);
        let body = reqwest::blocking::get(url).unwrap().text().unwrap();

        println!("Converting html to markdown ...");
        let md = html2md::parse_html(&body);

        fs::write(output, md.as_bytes()).unwrap();
        println!("Converted markdown has been saved in {}.", output);
    } else {
        eprintln!("!! Please input args in format of 'url output'");
    }

}
