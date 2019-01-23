use std::collections::HashMap;
use std::str;

#[derive(PartialEq, Debug)]
pub struct Query<'a> {
    pub bang: Option<&'a str>,
    pub rest: &'a str,
    params: HashMap<String, String>,
}

impl<'a> Query<'a> {
    pub fn new(bang: Option<&'a str>, rest: &'a str) -> Self {
        Self{
            bang: bang,
            rest: rest,
            params: HashMap::new(),
        }
    }

    pub fn parse(s: &'a str) -> Self {
        match query(s.as_bytes()) {
            Ok((_, q)) => q,
            _ => Query::new(None, s),
        }
    }
}

use nom::{self, rest, multispace};

/* Grammar:

     query = bang + WHITESPACE? + rest
     bang = '!' + word
     word = ALNUM
*/

named!(bang <&[u8], Option<&str>>, do_parse!(
    b: alt!(
       preceded!(tag!("!"), nom::alphanumeric) |
       terminated!(nom::alphanumeric, tag!("!"))
    ) >> (str::from_utf8(b).ok())
));

named!(query <&[u8], Query>, do_parse!(
    b: bang >> multispace >> r: rest >>
    (Query{
        bang: b,
        rest:  str::from_utf8(r).unwrap_or_default(),
        params: HashMap::new()
    })
));

#[cfg(test)]
mod tests {
    use super::{bang, query, Query};

    #[test]
    fn parse_bang() {
        assert_eq!(
            bang(b"!foo"),
            Ok((&[][..], Some("foo")))
        );

        assert_eq!(
            bang(b"foo!"),
            Ok((&[][..], Some("foo")))
        );
    }

    #[test]
    fn parse_query() {
        assert_eq!(
            query(b"!foo bar"),
            Ok((&[][..], Query::new(Some("foo"), "bar")))
        );
        assert_eq!(
            query(b"!foo bar baz"),
            Ok((&[][..], Query::new(Some("foo"), "bar baz")))
        );

        assert_eq!(
            query(b"foo! bar"),
            Ok((&[][..], Query::new(Some("foo"), "bar")))
        );
        assert_eq!(
            query(b"foo! bar baz"),
            Ok((&[][..], Query::new(Some("foo"), "bar baz")))
        );
    }
}
