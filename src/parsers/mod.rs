use nom::{
  IResult,
  sequence::delimited,
  bytes::complete::{ tag, take_till },
  do_parse,
  take_until,
};

use super::types::DocMap;
use super::extractors::{ extract_comments_from_block, extract_comment_block };

fn find_header(input: &str) -> IResult<&str, String> {
    let (parsed, _) = do_parse!(input,
        take_until!("#") >>
        (input)
    )?;
    let res = delimited(
        take_till(|c| c == '#'),
        take_till(|c| c == '\n'),
        tag("\n")
    )(parsed)?;
    Ok((res.0, res.1.to_owned()))
}

pub fn parse_src<'a>(src: &'a str, map: DocMap) -> DocMap {
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