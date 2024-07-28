fn main() {

    let s = "Hello Mochen" .to_owned();
    fancy_print(&s);


    let s1 = "hello";
    let s2 = "world";
    println!("{}", concat_strings(&s1, &s2));
    println!("{}", s1);
    println!("{}", s2);

}


fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}-{}", s1, s2)
}


fn fancy_print(s: &String) {
    println!("{s}");
}





