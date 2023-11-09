use std::fs::File;
use std::fs;
use std::io::Read;
use std::collections::BTreeMap;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

pub fn read_file(filename: &str) -> Vec<(i32, i32)> {
    let mut file = File::open(filename.to_string()).unwrap();

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let mut vector: Vec<(i32, i32)> = Vec::new();
    let mut index: i32 = 0;

    for chunk in buffer.chunks_exact(4) {
        let value = i32::from_be_bytes(chunk.try_into().expect("Failed to convert slice to i32: Invalid byte sequence"));
        vector.push((index, value));
        index += 1;
    }

    if !buffer.chunks_exact(4).remainder().is_empty() {
        eprintln!("Warning: File Size is not a multiple of 4 bytes")
    }

    return vector
}

pub fn write_to_file(file_name: &str, vector: &Vec<(i32, i32)>) {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for (key, value) in vector {
            map.insert(*key, *value);
        }

    let json_data = json!(map);
    fs::write(
        file_name.to_string(),
        json_data.to_string()).expect("Unable to write to file")
}

