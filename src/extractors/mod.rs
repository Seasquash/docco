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

/**
 * ## Parsing
 *
 * The comments within the DocBlock will be analysed, the src code is left as
 * it is.
 */
fn discard<'a>(input: &'a str, start: &'a str) -> IResult<&'a str, &'a str> {
    do_parse!(input,
        take_until!(start) >>
        (input)
    )
}

pub fn extract_comments_from_block(block: &str, delimiter: char) -> Vec<String> {
    let x: &[char] = &[delimiter, ' '];
    block
        .lines()
        .map(|l| l.trim_start_matches(x).to_owned())
        .collect()
}

pub fn extract_comment_block<'a>(input: &'a str, start: &'a str, end: &'a str) -> IResult<&'a str, &'a str> {
    let res = discard(input, start)?;
    delimited(
        tag(start),
        take_until(end),
        tag(end)
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
