use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::stdin;
use std::process;
use rand::Rng;

fn main() {
    let words: Vec<String> = readfile("src/words.txt");
    
    let rand_num = rand::thread_rng().gen_range(0..words.len());
    let selectedword: &String = &words.get(rand_num).unwrap();


    println!("Welcome to my cli wordle clone made in rust");
    println!("Go ahead, take your first guess");

    let mut i = 1;

    while i < 6 {
        if round(&words, &selectedword) {
            println!("Yay you did it, you guessed right. The word was {}!", selectedword);
            println!("You did it {} guess!", &i);
            process::exit(1);
        }
        i = i+1;
    }
    println!("You didn't make it this time. Better luck next time!")

}

fn readfile(path: &str) -> Vec<String>{
    let file = File::open(path)
        .expect("Cound't find the file!");
    let buf_reader = BufReader::new(file);

    let mut output: Vec<String> = Vec::new();

    for line in buf_reader.lines() {
        output.push(line.unwrap());
    }
    
    return output;
}

fn round(words: &Vec<String>, selectedword: &String) -> bool{
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");

    input_string.remove(input_string.len()- 1);
    input_string.remove(input_string.len()- 1);
    


    if !input_string.len() == 5{
        println!("{} is the wrong length, try again", input_string);
        return round(words, &selectedword)
    }

    // input_string.remove(idx)
    // input_string.remove_matches("\n");

    
    if !words.contains(&input_string){
        println!("{} is not a valid word, try again", input_string);
        return round(words, &selectedword)
    }
    
    let mut i = 0;

    for char in input_string.chars() {
        if char == selectedword.chars().nth(i).unwrap(){
            print!("🟩");
        } else if selectedword.contains(char){
            print!("🟨");
        } else {
            print!("⬛");
        }
        i = i+1;
    }
    println!(" ");

    if input_string == selectedword.to_string() {
        return true;
    } else {
        return false;
    }
}