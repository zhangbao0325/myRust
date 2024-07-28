use std::fs;

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Clone, Copy)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}
#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}


fn main() {
    scrape_url_args();
    
}


#[warn(dead_code)]
fn test_struct() {
    let alice = User{id: UserId(1), name: "Alice".into(), gender: Gender::Female};
    let bob = User{id: UserId(2), name: "Bob".into(), gender: Gender::Male};

    let topic = Topic{id: TopicId(1), name: "Rust".into(), owner: UserId(1)};

    let event1 = Event::Join((alice.id,topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "hello world!".into()));
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}


#[warn(dead_code)]
fn scrape_url_args() {
    let args = std::env::args().collect::<Vec<String>>();
    if let [_path, url, output, ..] = args.as_slice() {
        println!("_path: {},  url: {}, output: {}", _path, url, output);
    }else {
        eprintln!("invalid arguments");
    }
}


#[warn(dead_code)]
fn scrape_url() {
    let url = "http://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    // let body = reqwest::get(&url).await?.text().await?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Wrote markdown to {}", output);
}