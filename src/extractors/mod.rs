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

/**
 * ## TODO
 *
 * - Extract at least 2 levels of sub-comments that can be grouped under the main section.
 *      Example, if two blocks have a section # Title, a subsection # Title2 and two subsection under that, # Sub1 and # Sub2 , instead of #Title2 being repeated, # Sub1 and # Sub2 should be grouped under one # Title2
 */
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
    use nom::Needed::Size;
    use nom::Err::Incomplete;

    #[test]
    fn should_remove_all_the_code_leading_to_a_doc_block() {
        let src = "this is some random code [\n* this is a random piece\n* of doc block\n]\nwith extra\ncode after that";
        assert_eq!(discard(src, "["), Ok(("[\n* this is a random piece\n* of doc block\n]\nwith extra\ncode after that", src)));
    }

    #[test]
    fn should_return_an_error_if_no_doc_block_found() {
        let src = "this is some random code [\n* this is a random piece\n* of non-doc block\n]\nwith extra\ncode after that";
        assert_eq!(discard(src, "/**"), Err(Incomplete(Size(3))));
    }
}
