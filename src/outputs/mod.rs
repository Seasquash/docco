use std::fmt::Display;
use std::fs::File;
use std::io::{ Write, Error };

pub fn output_comments<T: Display>(comments: Vec<T>) {
    for comment in comments {
        println!("PARSED COMMENT --- {}", comment);
    }
}

pub fn write_to_file(comments: Vec<String>, file_name: &str) -> Result<(), Error> {
    let mut file = File::create(file_name)?;
    for line in &comments {
        write!(file, "{}\n", line)?;
    }
    Ok(())
}
