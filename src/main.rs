use std::fs;
// use std::io::prelude::*;
// use std::io::BufReader;
use std::io::stdin;
use std::process;
use rand::Rng;

fn main() {
    let lines = fs::read_to_string("src/words.txt").expect("Couldn't open file!");
    let words: Vec<&str> = lines.lines().collect();

    let rand_num = rand::thread_rng().gen_range(0..words.len());
    let selectedword = &words[rand_num];


    println!("Welcome to my cli wordle clone made in rust");
    println!("Go ahead, take your first guess");

    for i in 1..7 {
        if round(words.as_slice(), selectedword) {
            println!("Yay you did it, you guessed right. The word was {}!", selectedword);
            println!("You did it {} guess!", &i);
            process::exit(1);
        }
    }
    println!("You didn't make it this time. Better luck next time!")

}


fn round(words: &[&str], selectedword: &str) -> bool{
    let mut input_string = String::with_capacity(5);
    stdin().read_line(&mut input_string)
        .expect("Failed to read line");

    let input_string = input_string.trim();
    

    if !input_string.len() == 5{
        println!("{} is the wrong length, try again", input_string);
        return round(words, &selectedword)
    }

    
    if !words.contains(&input_string){
        println!("{} is not a valid word, try again", input_string);
        return round(words, &selectedword)
    }

    let correct_word = selectedword.chars().collect::<Vec<char>>();


    for (i, c) in input_string.chars().enumerate() {
        if c == correct_word[i] {
            print!("🟩");
        } else if selectedword.contains(c) {
            print!("🟨");
        } else {
            print!("⬛");
        }
    }

    println!(" ");

    input_string == selectedword
}