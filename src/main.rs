// use serde_json::Result;
use serde_json::Value;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let foo = fs::read_to_string("data/proxy.json")?;
    println!("My JSON {}", foo);
    let data: Value = serde_json::from_str(&foo)?;
    println!("{}", data["id"]);

    Ok(())
}
