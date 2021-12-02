use std::{fs::OpenOptions, io::Write};

pub fn file_operation(address: String, private: String) {
    let path = String::from("./ETH.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path);
    if let Ok(mut file_txt) = file {
        let _result = write!(
            &mut file_txt,
            "==========================================\np . u . b . k . e . y: {:?}\np . r . i . v . a . t . e: {}\n==========================================\n\n\n",
            address, private
        );
    }
}
