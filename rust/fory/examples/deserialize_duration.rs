use fory::Fory;
use std::time::Duration;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/negative_duration.txt".to_string());

    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("Failed to read {path}: {e}");
        eprintln!("Run NegativeDurationDemo.java first to generate the file.");
        std::process::exit(1);
    });
    println!("Rust  | bytes (hex)       : {}", to_hex(&bytes));
    let fory = Fory::default().xlang(true);
    match fory.deserialize::<Duration>(&bytes) {
        Ok(duration) => {
            println!("Rust  | duration.as_secs(): {}", duration.as_secs());
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}
