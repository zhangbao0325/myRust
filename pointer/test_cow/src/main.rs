use std::borrow::Cow;

#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    fn new<S: Into<Cow<'a, str>>>(raw: S) -> Token<'a> {
        Token { raw: raw.into() }
    }
}

fn main() {
    let s1 = "this is a test cow";
    let token1 = Token::new(Cow::Borrowed(s1));
    println!("token1: {:?}", token1);

    let s2 = "this is a test cow2".to_string();
    let token2 = Token::new(Cow::Owned(s2)); // 这里s2的所有权被转移到了Cow里，下面无法继续Println
    println!("token2: {:?}", token2);
    // println!("s2: {:?}", s2);
}
