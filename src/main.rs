use std::io;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool
}

enum GameProgress {
    InProgress,
    Won,
    Lost
}

fn main() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = selected_word();
    let mut letters = create_letters(&selected_word);

    println!("Benvenuto nel gioco dell'impiccato!");

    loop {
        println!("\nHai {} turni rimanenti.", turns_left);
        display_progress(&letters);

        println!("\nInserisci una lettera:");
        let user_char = read_user_input_character();

        if user_char == '*' {
            break;
        }

        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("Congratulazioni, hai vinto! La parola era {}", selected_word);
                break;
            }
            GameProgress::Lost => {
                println!("\nMi dispiace, hai perso! La parola era {}", selected_word);
                break;
            }
        }

    }

    println!("Arrivederci!");

}

fn selected_word() -> String {
    let mut file = File::open("words.txt").expect("Errore durante l'apertura del file");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Errore durante la lettura del file!");

    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        })
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string: String = String::from("Progresso:");

    for letter in letters {
        display_string.push(' ');
        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }
        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character() -> char {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { return c; }
                None => { return '*' }
            }
        }
        Err(_) => { return '*' }
    }
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    let mut all_revealed = true; 

    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }
    
    return GameProgress::Lost;
}