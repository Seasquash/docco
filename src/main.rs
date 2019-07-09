mod parsers;
mod types;
mod outputs;
mod extractors;
mod models;

use std::fs::read_to_string;
use std::io::Error;
use std::collections::HashMap;
use walkdir::WalkDir;

use extractors::extract_config;
use models::Config;
use outputs::write_to_file;
use parsers::parse_src;
use types::DocMap;

/**
 * # Docco
 *
 * A simple parser wrapper.
 * It scans different source files to find Doc Blocks, groups them all and
 * writes the result in a README.md file, generating the documentation
 * automatically.
 */
fn main() -> Result<(), Error> {
    let config = extract_config()?;
    let files = read_files(&config)?;

    let result = merge_maps(&files);

    if config.index.len() > 0 {
        let ordered = order_comments(result, config.index);
        write_to_file(ordered, "README.md")?;
    } else {
        let output = result
            .iter()
            .flat_map(|(k, v)| {
                let mut new_vec = v.clone();
                new_vec.insert(0, k.to_owned());
                new_vec
            })
            .collect::<Vec<String>>();
        write_to_file(output, "README.md")?;
    }

    Ok(())
}

fn merge_maps(maps: &Vec<DocMap>) -> DocMap {
    maps.iter().fold(HashMap::new(), |mut acc, map| {
        for (k, v) in map {
            if acc.contains_key(k) {
                let empty = Vec::new();
                let combined: Vec<String> = vec!(v, acc.get(k).unwrap_or(&empty))
                    .iter()
                    .flat_map(move |s| s.iter().map(|e| e.to_owned()).collect::<Vec<String>>())
                    .collect();
                acc.insert(k.to_owned(), combined);
            } else {
                acc.insert(k.to_owned(), v.to_vec());
            }
        }
        acc
    })
}

fn read_files(config: &Config) -> Result<Vec<DocMap>, Error> {
    let mut results = Vec::new();
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let full_name = entry.path().with_file_name(entry.file_name());
        for format in &config.formats {
            if entry.file_name().to_string_lossy().ends_with(&format.extension) {
                println!("{:?}", full_name);
                let src = read_to_string(&full_name)?;
                results.push(parse_src(&src, HashMap::new(), &format.start, &format.end, format.delimiter));
            }
        }
    }
    Ok(results)
}

/**
 * ## Ordering
 *
 * If there is an `index` entry within the configuration json, it will be used
 * to figure out the order in which the comments have to be written to the
 * documentation, otherwise spits them as they are found while processing.
 */
fn order_comments(comments: DocMap, index: Vec<String>) -> Vec<String> {
    let mut map = comments.clone();
    let mut output: Vec<String> = vec!();
    for i in index {
        if map.contains_key(&i[..]) {
            let values = map.remove_entry(&i[..]).unwrap();
            output.push(i.clone());
            for value in values.1 {
                output.push(value);
            }
        }
    }
    for (key, values) in map {
        output.push(key);
        for value in values {
            output.push(value);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_comments_with_index() {
        let mut comments = HashMap::new();
        comments.insert("1".to_owned(), vec!("one".to_owned()));
        comments.insert("2".to_owned(), vec!("two".to_owned()));
        comments.insert("3".to_owned(), vec!("three".to_owned()));
        let index = vec!("1".to_owned(), "3".to_owned());
        assert_eq!(order_comments(comments, index), vec!("1", "one", "3", "three", "2", "two"));
    }

    #[test]
    fn order_comments_with_no_index() {
        let mut comments = HashMap::new();
        comments.insert("1".to_owned(), vec!("one".to_owned()));
        comments.insert("2".to_owned(), vec!("two".to_owned()));
        let index: Vec<String> = Vec::new();
        let result = order_comments(comments, index);
        assert_eq!(result.len(), 4);
        assert_eq!(result.contains(&"1".to_owned()), true);
    }

    #[test]
    fn order_comments_with_no_matching_index() {
        let mut comments = HashMap::new();
        comments.insert("1".to_owned(), vec!("one".to_owned()));
        comments.insert("2".to_owned(), vec!("two".to_owned()));
        let index = vec!("3".to_owned(), "4".to_owned());
        let result = order_comments(comments, index);
        assert_eq!(result.len(), 4);
        assert_eq!(result.contains(&"1".to_owned()), true);
    }

    #[test]
    fn order_empty_comments() {
        let comments = HashMap::new();
        let result: Vec<String> = Vec::new();
        assert_eq!(order_comments(comments, Vec::new()), result);
    }
}