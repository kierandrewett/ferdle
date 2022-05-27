use std::{env};

use colored::*;
use console::*;

mod list;
use list::{ POSSIBLE_GUESSES, ANSWERS };
use rand::prelude::SliceRandom;

static NUMBER_OF_GUESSES: usize = 6;
static WORD_LENGTH: usize = POSSIBLE_GUESSES[0].len();

const ALLOWED_CHARACTERS: [&str; 26] = [
    "a", "b", "c", "d",
    "e", "f", "g", "h",
    "i", "j", "k", "l",
    "m", "n", "o", "p",
    "q", "r", "s", "t",
    "u", "v", "w", "x",
    "y", "z"
];

const WIN_MESSAGES: [&str; 6] = [
    "Genius", 
    "Magnificent", 
    "Impressive", 
    "Splendid", 
    "Great", 
    "Phew"
];

fn cleanse_guess(guess: &String) -> String {
    return guess
        .to_string()
        .to_lowercase()
        .replace('[', "")
        .replace(']', "")
        .trim()
        .to_string();
}

fn is_debug() -> bool {
    env::var("FERDLE_GAME_DEBUG").is_ok()
}

fn main() {
    let stdout = Term::buffered_stdout();

    let mut board: Vec<Vec<String>> = vec![vec![]; NUMBER_OF_GUESSES];

    let mut current_row = 0;
    let mut current_col = 0;

    let mut rng = rand::thread_rng();
    let word = ANSWERS.choose(&mut rng).unwrap().trim().to_string();

    fn get_gap() -> String {
        let termsize::Size { cols, .. } = termsize::get().unwrap(); 

        (0..cols/12).map(|_| " ").collect::<String>()
    }

    fn redraw_board(board: &Vec<Vec<String>>) {
        let gap = get_gap();

        // clear screen
        if !is_debug() {
            print!("\x1B[2J\x1B[1;1H");
        }

        println!("{gap}{}{gap}{gap}", "FERDLE".underline().bold());
        println!();

        for row in board {
            let mut formatted_row = String::new();
    
            for i in 0..WORD_LENGTH {
                formatted_row += match row.get(i) {
                    Some(v) => v,
                    None => "[ ]"
                };
            }
    
            println!("{gap}{formatted_row}{gap}")
        }

        println!();
    }

    redraw_board(&board);

    'ferdle_loop: loop {
        if is_debug() {
            println!("rows={current_row} col={current_col} word={word}");
        }

        if let Ok(key) = stdout.read_key() {
            match key {
                Key::Char(c) => {
                    if ALLOWED_CHARACTERS.contains(&c.to_lowercase().to_string().trim()) && (0..5).contains(&current_col) {
                        board[current_row].push(format!("[{}]", c.to_uppercase()));
    
                        current_col += 1;
                    }

                    redraw_board(&board);
                }
                Key::Backspace => {
                    if (1..=5).contains(&current_col) {
                        board[current_row].pop();

                        current_col -= 1;
                    }

                    redraw_board(&board);
                }
                Key::Enter => {
                    let gap = get_gap();

                    if current_col >= WORD_LENGTH {
                        let joined_guess = cleanse_guess(&board[current_row]
                            .join("")
                        ).to_string();

                        if [POSSIBLE_GUESSES, ANSWERS].concat().contains(&joined_guess.as_str()) {
                            current_row += 1;

                            let mut cut_word = word.split("")
                                .filter(|x| x.is_ascii())
                                .collect::<Vec<_>>();

                            cut_word.remove(0);
                            cut_word.remove(cut_word.len()-1);

                            if is_debug() {
                                println!("{cut_word:?}");
                            }

                            for (i, x) in board[current_row-1].to_vec().iter().enumerate() {
                                if cut_word.contains(&cleanse_guess(x).as_str()) {
                                    let pos = cut_word.iter().position(|&r| r == cleanse_guess(x).as_str()).unwrap();

                                    if pos == i {
                                        board[current_row-1][i] = x.to_uppercase().green().to_string();
                                    } else {
                                        board[current_row-1][i] = x.to_uppercase().yellow().to_string();
                                    }
                                }
                            }

                            if joined_guess == word {
                                board[current_row-1] = cut_word
                                    .iter()
                                    .map(|x| format!("[{x}]").green().bold().to_uppercase()).collect();
                            }
                            
                            redraw_board(&board);

                            if joined_guess == word {
                                println!("{gap}{}{gap}", WIN_MESSAGES[current_row-1].green().bold());
                                println!("{gap}The word was {}.{gap}", word.underline().bold());
                                break 'ferdle_loop;
                            } else if current_row > NUMBER_OF_GUESSES-1 {
                                println!("{gap}{}{gap}", "Game over!".red().bold());
                                println!("{gap}The word was {}.{gap}", word.underline().bold());
                                break 'ferdle_loop;
                            } else {
                                // Go to next line
                                current_col = 0;
                            }
                        } else {
                            redraw_board(&board);

                            println!("{gap}{}{gap}", "Not in word list.".red().bold());
                        }
                    } else {
                        redraw_board(&board);

                        println!("{gap}{}{gap}", "Not enough letters.".white().bold()); 
                    }
                }
                _ => {}
            }
        }
    }
}