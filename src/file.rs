use std::{fs::{File, self}, io::Write};

pub fn write_file(random_bytes: &[u8; 32], address: &str, private: &str, directory: &str) {
    fs::create_dir_all(&directory).unwrap();
    let path = format!("{}/{}.txt", &directory, address);
    let file = File::create(path);
    if let Ok(mut file_txt) = file {
        let _result = write!(
            &mut file_txt,
            "==========================================\nr . a . n . d . o . m: {:#?}\np . u . b . k . e . y: {}\np . r . i . v . a . t . e: {}\n==========================================",
            random_bytes, address, private
        );
    }
}
