use std::fs::read_to_string;
use std::io::Error;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::fmt::Display;
use nom::{
  IResult,
  sequence::delimited,
  bytes::complete::tag,
  bytes::complete::take_until,
  bytes::complete::take_till,
  take_until,
  take_till,
  do_parse,
  named,
  tag,
  take,
  delimited
};

use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct Config {
    index: Vec<String>
}

fn extract_config() -> Result<Config, Error> {
    let index_file = File::open("docco.json")?;
    let reader = BufReader::new(index_file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn main() -> Result<(), Error> {
    let src = read_to_string("file.ts")?;
    let config = extract_config();
    let hash_map = HashMap::new();
    let result = parse_src(&src, hash_map);

    match config {
        Ok(c) => {
            let ordered = order_comments(result, c.index);
            output_comments(ordered);
        },
        _ => {
            let output = result
                .iter()
                .flat_map(|(k, v)| {
                    let mut new_vec = v.clone();
                    new_vec.insert(0, k);
                    new_vec
                })
                .collect::<Vec<&str>>();
            output_comments(output);
        }
    }

    Ok(())
}

fn order_comments(comments: HashMap<&str, Vec<&str>>, index: Vec<String>) -> Vec<String> {
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

fn output_comments<T: Display>(comments: Vec<T>) {
    for comment in comments {
        println!("PARSED COMMENT --- {}", comment);
    }
}

fn parse_src<'a>(src: &'a str, map: HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>>{
    let parsed = extract_comment_block(&src);
    match parsed {
        Ok((rest, comment_block)) => {
            let mut cloned = map.clone();
            let header_comment = find_header(comment_block);
            match header_comment {
                Ok((comment_lines, header)) => {
                    println!("HEADER: {:?}", header);
                    let comments = extract_comments_from_block(comment_lines);
                    cloned.insert(header, comments);
                    parse_src(rest, cloned)
                },
                Err(e) => { println!("HEADER NOT FOUND: {:?}", e); map }
            }
        },
        Err(e) => { println!("PARSE ERROR: {:?}", e); map }
    }
}

fn extract_comments_from_block(block: &str) -> Vec<&str> {
    let x: &[char] = &['*', ' '];
    block
        .lines()
        .map(|l| l.trim_start_matches(x))
        .collect()
}

fn discard(input: &str) -> IResult<&str, &str> {
    do_parse!(input,
        take_until!("/**") >>
        (input)
    )
}

fn extract_comment_block(input: &str) -> IResult<&str, &str> {
    let res = discard(input)?;
    delimited(
        tag("/**"),
        take_until("*/"),
        tag("*/")
    )(res.0)
}

fn find_header(input: &str) -> IResult<&str, &str> {
    let (parsed, _) = do_parse!(input,
        take_until!("#") >>
        (input)
    )?;
    delimited(
        take_till(|c| c == '#'),
        take_till(|c| c == '\n'),
        tag("\n")
    )(parsed)
}
