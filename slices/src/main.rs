fn main() {
    let hw = String::from("Hello world!");
    let hello = &hw[0..5];
    let world = &hw[6..11];
    println!("{} and {}", hello, world);
    println!("The end of first word is {}", first_word(&hw));
    println!("The first word is {}", first_word_slice(&hw));
    println!("The first word is {}", first_word_slice(&hw[6..]));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}