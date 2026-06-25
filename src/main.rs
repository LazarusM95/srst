// step 0 create database
// step 1 confirm connections
// step 2 language selection
// step 3 deck selection aka pull decks
// step 4 get request to anki connect cards one time
// step 5 get word
// step 6 translate the word
// step 7 create example sentence
// step 8 translate example sentence
// step 9 input to database
// step 10 send to anki connect via json
use std::io;

fn language_select() -> u8 {
    println!("Please select your target language language:");
    println!("1)Serbian; 2)German; 3)Chinese; 4)Japanese");


    let language = loop {
        
    let mut language = String::new();

    io::stdin()
        .read_line(&mut language)
        .expect("Wrong input! Failed to read the line.");

     let language: u8 = language.trim().parse().expect("Please type corresponding number");
     
        if language >=1 && language <=4 {
            break language
        } else{
            println!("Please enter a valid selection of 1-4");
        }
    };
    
    language
}


#[tokio::main]
async fn main() {
    language_select();
}
