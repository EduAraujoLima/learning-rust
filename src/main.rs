fn main() {
    println!("Hello, world!");
}

fn length_of_last_word(s: String) -> i32 {
    s.chars()
        .rev()
        .skip_while(|c| *c == ' ')
        .take_while(|c| *c != ' ')
        .count() as i32
}
