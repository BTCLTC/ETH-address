use std::{fs::{File, self}, io::Write};

pub fn file_operation(address: &str, private: &str, directory: &str) {
    fs::create_dir_all(&directory).unwrap();
    let path = format!("{}/{}.txt", &directory, address);
    println!("directory: {}, path: {}", directory, path);
    let file = File::create(path);
    println!("{:?}", file);
    if let Ok(mut file_txt) = file {
        let _result = write!(
            &mut file_txt,
            "==========================================\np . u . b . k . e . y: {:?}\np . r . i . v . a . t . e: {}\n==========================================",
            address, private
        );
    }
}
