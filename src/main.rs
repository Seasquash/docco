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
use outputs::output_comments;
use parsers::parse_src;
use types::DocMap;

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
        let f_name = entry.file_name().to_string_lossy();

        for format in &config.formats {

            if f_name.ends_with(&format.extension) {
                println!("{}", f_name);
                let src = read_to_string(f_name.to_string())?;
                results.push(parse_src(&src, HashMap::new()));
            }
        }
    }
    Ok(results)
}

fn main() -> Result<(), Error> {
    let config = extract_config()?;
    let files = read_files(&config)?;

    let result = merge_maps(&files);

    if config.index.len() > 0 {
        let ordered = order_comments(result, config.index);
        output_comments(ordered);
    } else {
        let output = result
            .iter()
            .flat_map(|(k, v)| {
                let mut new_vec = v.clone();
                new_vec.insert(0, k.to_owned());
                new_vec
            })
            .collect::<Vec<String>>();
        output_comments(output);
    }

    Ok(())
}

fn order_comments(comments: DocMap, index: Vec<String>) -> Vec<String> {
    let mut map = comments.clone();
    let mut output: Vec<String> = vec!();
    for i in index {
        if map.contains_key(&i[..]) {
            let values = map.remove_entry(&i[..]).unwrap();
            output.push(i.clone());
            for v in values.1 {
                output.push(v.to_owned());
            }
        }
    }
    for (k, v) in map {
        output.push(k.to_owned());
        for value in v {
            output.push(value.to_owned());
        }
    }
    output
}
