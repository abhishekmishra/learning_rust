fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    let len2 = calculate_length2(&mut s1);
    println!("The length of '{}' is {}.", s1, len2);
}

fn calculate_length(s: &String) -> usize {
    //can't modify ref
    //s.push_str("what");
    s.len()
}

fn calculate_length2(s: &mut String) -> usize {
    //can't modify ref
    s.push_str("what");
    s.len()
}