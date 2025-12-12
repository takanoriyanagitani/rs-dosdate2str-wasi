use rs_dosdate2str_wasi::{format_output, parse_dos_date};
use serde::Deserialize;
use std::io::{self, Read};

#[derive(Deserialize)]
struct Input {
    dostime: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_INPUT_BYTES: u64 = 128;
    let mut buffer = String::new();
    io::stdin()
        .take(MAX_INPUT_BYTES)
        .read_to_string(&mut buffer)?;

    let input: Input = serde_json::from_str(&buffer)?;
    let dostime_val = input.dostime;

    let dos_date = (dostime_val >> 16) as u16;

    let dos_date_components = parse_dos_date(dos_date)?;
    let output = format_output(dos_date_components);

    let json_output = serde_json::to_string_pretty(&output)?;
    println!("{}", json_output);

    Ok(())
}
