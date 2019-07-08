use std::fmt::Display;

pub fn output_comments<T: Display>(comments: Vec<T>) {
    for comment in comments {
        println!("PARSED COMMENT --- {}", comment);
    }
}