use nom::number::complete::recognize_float;
use nom::ErrorKind;

named!(parser<&str, f64>, flat_map!(recognize_float, parse_to!(f64)));

#[test]
fn test() {
    assert_eq!(parser("123.45;"), Ok((";", 123.45)));
    assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Alt))));
}