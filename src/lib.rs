use std::fs;
use std::env;
use xmltojson;
use serde_json;

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let args = env::args();
    let mut arr = Vec::new();
    for xmlpath in args.skip(1) {
        let xml = fs::read_to_string(xmlpath)?;
        let json = xmltojson::to_json(&xml).unwrap();
        arr.push(json);
    }
    let j = serde_json::to_string(&arr)?;
    println!("{}", j);
    Ok(())
}
