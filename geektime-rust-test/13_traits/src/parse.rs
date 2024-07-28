use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        // Regex::new(reg!(r"^[0-9]+$")).unwrap().replace_all(s, "").to_string().parse().unwrap()
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(0, |f| f.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

#[test]
pub fn parse_shold_work() {
    assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("1abcd"), 1);
    assert_eq!(u8::parse("abc"), 0);
}

fn main() {
    println!("result: {}", u8::parse("255 hello world"));
}
