fn main() {
    let mut s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    let f_word = first_word(&s);

    s.clear();

    println!("first word is {}", f_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
