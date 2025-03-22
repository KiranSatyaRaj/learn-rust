use std::io;

fn main() {
    let mut s: String = String::new();

    println!("Enter a word");

    io::stdin()
        .read_line(&mut s)
        .expect("unable to read line");

    let word: &String = &s.trim().to_string();

    let letter = &word.chars().next().unwrap();

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let result = vowels.binary_search(letter);
    match result {
        Ok(_) => println!("{} in pig-latin is {}", word, word.clone() + "-hay"),
        Err(_) => println!("{} in pig-latin is {}", word, word.get(1..).unwrap().to_string() + "-" + &word.get(..1).unwrap() + "ay"),
    }
}