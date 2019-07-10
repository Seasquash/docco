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

/**
 * ## Parsing
 *
 * The comments are extracted from a block of code and separated by single "lines", while the "delimiter" passed in the
 * config, and leading empty spaces will be removed from the comment itself.
 */
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

// TODO: pass in config file name and move reader out, passed as a trait implementation, so this can be Unit tested.
pub fn extract_config() -> Result<Config, Error> {
    let index_file = File::open("docco.json")?;
    let reader = BufReader::new(index_file);
    let config: Config = serde_json::from_reader(reader)?;
    for v in &config.formats {
        println!("Format: {:?}", v);
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_remove_the_code_leading_to_a_doc_block() {
        let src = "this is some random code [\n* this is a random piece\n* of doc block\n]\nwith extra\ncode after that";
        assert_eq!(discard(src, "["), Ok(("[\n* this is a random piece\n* of doc block\n]\nwith extra\ncode after that", src)));
    }
}
