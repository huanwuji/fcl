use nom::number::complete::recognize_float;

named!(parser<&str, f64>, flat_map!(recognize_float, parse_to!(f64)));
//named!(parser<&str, f64>, parse_to!(f64));

#[test]
fn test() {
    eprintln!("haha = {:?}", parser("123.33"));
//    assert_eq!(parser("123.45;"), Ok((";", 123.45)));

//    assert_eq!(parser("abc"), Err(error::Error(("abc", ErrorKind::Alt))));
}