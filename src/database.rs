use std::fs::{OpenOptions, File, self};
use std::path::Path;
use std::io::{Read, Write};
use tracing::info;

// Constants can be moved here
pub const MAX_VALUE_SIZE: usize = 1024;

pub fn database_node_management(key: &str, value: &str) -> bool {
    let node_char:[char;36] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // Verify that all characters in the key are valid
    for c in key.chars() {
        if !(c.is_numeric() || node_char.contains(&c)) {
            println!("Invalid key character: {}", c);
            return false;
        }
    }

    // Truncate value if necessary (respecting UTF-8 boundaries)
    let truncated_value = if value.len() > MAX_VALUE_SIZE {
        info!("Value truncated from {} to {} bytes", value.len(), MAX_VALUE_SIZE);
        
        let mut size = 0;
        let mut char_indices = value.char_indices();
        let mut last_valid_index = 0;
        
        while let Some((i, c)) = char_indices.next() {
            let char_size = c.len_utf8();
            if size + char_size > MAX_VALUE_SIZE {
                break;
            }
            size += char_size;
            last_valid_index = i + char_size;
        }
        
        &value[0..last_valid_index]
    } else {
        value
    };

    let db_dir = Path::new("db");
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("Failed to create db directory");
    }

    let file_path = format!("db/{}", key);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&file_path)
        .expect("Unable to open or create file");

    file.write_all(truncated_value.as_bytes()).expect("Unable to write to file");
    true
}

pub fn get_data_from_key(key: &str) -> Result<String, String> {
    let file_path = format!("db/{}", key);
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err(format!("Key '{}' not found", key));
    }
    
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open file: {}", e)),
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}
