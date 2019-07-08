use std::io::Error;
use std::io::BufReader;
use std::fs::File;

use nom::{
  IResult,
  sequence::delimited,
  bytes::complete::tag,
  bytes::complete::take_until,
  do_parse,
  take_until
};

use super::models::Config;

fn discard(input: &str) -> IResult<&str, &str> {
    do_parse!(input,
        take_until!("/**") >>
        (input)
    )
}

pub fn extract_comments_from_block(block: &str) -> Vec<String> {
    let x: &[char] = &['*', ' '];
    block
        .lines()
        .map(|l| l.trim_start_matches(x).to_owned())
        .collect()
}

pub fn extract_comment_block(input: &str) -> IResult<&str, &str> {
    let res = discard(input)?;
    delimited(
        tag("/**"),
        take_until("*/"),
        tag("*/")
    )(res.0)
}

pub fn extract_config() -> Result<Config, Error> {
    let index_file = File::open("docco.json")?;
    let reader = BufReader::new(index_file);
    let config: Config = serde_json::from_reader(reader)?;
    for v in &config.formats {
        println!("Format: {:?}", v);
    }
    Ok(config)
}
