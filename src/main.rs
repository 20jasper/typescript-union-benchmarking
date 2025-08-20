use core::ops::RangeInclusive;
use std::fs;
use std::path::PathBuf;

// should be 53 at max
const POWER: i64 = 1;
const JS_MAX_SAFE_INTEGER: i64 = (1 << POWER) - 1;
const JS_MIN_SAFE_INTEGER: i64 = -JS_MAX_SAFE_INTEGER;
const JS_SAFE_INTEGER_RANGE: RangeInclusive<i64> = JS_MIN_SAFE_INTEGER..=JS_MAX_SAFE_INTEGER;

// Convert a byte length to a human-readable string (binary units: KiB, MiB, ...)
fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    if bytes < 1024 {
        return format!("{} B", bytes);
    }
    let mut size = bytes as f64;
    let mut unit_index = 0usize;
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    let mut s = format!("{:.2}", size);
    if s.contains('.') {
        while s.ends_with('0') {
            s.pop();
        }
        if s.ends_with('.') {
            s.pop();
        }
    }
    format!("{} {}", s, UNITS[unit_index])
}

fn main() {
    println!("Generating file for {:?}", JS_SAFE_INTEGER_RANGE);

    let out_file = PathBuf::from("ts-playground/safe-integer.ts");

    let res = {
        let ts_union = JS_SAFE_INTEGER_RANGE
            .map(|x| format!("|{}", x))
            .collect::<String>();
        format!(
            "type SafeInteger = {ts_union};
const y: SafeInteger = 0;
// @ts-expect-error
const X: SafeInteger = Infinity;",
        )
    };

    fs::write(&out_file, res).unwrap();

    println!("Generated {}", out_file.display());
    let size_bytes = fs::metadata(&out_file).map(|m| m.len()).unwrap();
    println!(
        "File size: {} ({} bytes)",
        human_size(size_bytes),
        size_bytes
    );
}
