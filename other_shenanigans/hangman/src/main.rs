use std::collections::HashSet;
use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const MAX_WRONG_GUESSES: usize = 7;

/// Randomly pick a word.
///
/// Tries a free random-word API first; if the network call fails (offline,
/// timeout, etc.) we fall back to a built-in list so the game always runs.
fn pick_word() -> String {
    match fetch_word_from_api() {
        Ok(word) => word,
        Err(e) => {
            eprintln!("(Couldn't reach the word API: {e}. Using a built-in word.)");
            pick_word_offline()
        }
    }
}

/// Fetch a single random word from the free random-word API.
///
/// The endpoint returns a JSON array with one element, e.g. `["compiler"]`,
/// which we parse by stripping the surrounding brackets and quotes.
fn fetch_word_from_api() -> Result<String, ureq::Error> {
    let body = ureq::get("https://random-word-api.herokuapp.com/word")
        .timeout(Duration::from_secs(5))
        .call()?
        .into_string()?;

    let word: String = body
        .trim()
        .trim_matches(|c| c == '[' || c == ']' || c == '"')
        .to_lowercase();

    // Guard against an empty or non-alphabetic response.
    if word.is_empty() || !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return Ok(pick_word_offline());
    }

    Ok(word)
}

/// Pick a word from a built-in list, seeding off the system clock.
fn pick_word_offline() -> String {
    let words = [
        "rust",
        "cargo",
        "borrow",
        "lifetime",
        "ownership",
        "trait",
        "closure",
        "macro",
        "vector",
        "string",
        "compiler",
        "iterator",
        "pattern",
        "module",
    ];

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the epoch")
        .subsec_nanos() as usize;

    words[nanos % words.len()].to_string()
}

/// Draw the hangman based on how many wrong guesses have been made.
///
/// Each wrong guess adds a new part: noose, head, body, left arm, right arm,
/// left leg, right leg.
fn wrong_guess(wrong: usize) {
    let head = if wrong >= 2 { "O" } else { " " };
    let left_arm = if wrong >= 4 { "/" } else { " " };
    let body = if wrong >= 3 { "|" } else { " " };
    let right_arm = if wrong >= 5 { "\\" } else { " " };
    let left_leg = if wrong >= 6 { "/" } else { " " };
    let right_leg = if wrong >= 7 { "\\" } else { " " };

    // The top bar and noose appear on the first wrong guess.
    let (top, rope) = if wrong >= 1 {
        ("  +---+", "  |   |")
    } else {
        ("  +---+", "  |    ")
    };

    println!("{top}");
    println!("{rope}");
    println!("  |   {head}");
    println!("  |  {left_arm}{body}{right_arm}");
    println!("  |  {left_leg} {right_leg}");
    println!("  |");
    println!("=========");
}

/// Build the masked display of the word, e.g. "r u _ t" for "rust".
///
/// Letters the player has guessed are revealed; the rest stay as underscores.
fn mask_word(word: &str, guessed: &HashSet<char>) -> String {
    word.chars()
        .map(|c| if guessed.contains(&c) { c } else { '_' })
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Read a single guessed letter from the player.
///
/// Re-prompts until they enter exactly one alphabetic character.
fn read_guess() -> char {
    loop {
        print!("Guess a letter: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let trimmed = input.trim().to_lowercase();
        let mut chars = trimmed.chars();

        match (chars.next(), chars.next()) {
            (Some(c), None) if c.is_alphabetic() => return c,
            _ => println!("Please enter a single letter."),
        }
    }
}

/// Play one round of hangman. Returns true if the player won.
fn play_round() -> bool {
    let word = pick_word();
    let mut guessed: HashSet<char> = HashSet::new();
    let mut wrong = 0;

    loop {
        wrong_guess(wrong);
        println!("\nWord: {}", mask_word(&word, &guessed));
        println!("Wrong guesses left: {}", MAX_WRONG_GUESSES - wrong);

        let guess = read_guess();

        if guessed.contains(&guess) {
            println!("You already guessed '{guess}'.");
            continue;
        }
        guessed.insert(guess);

        if word.contains(guess) {
            // Right guess: reveal the letter (already inserted above).
            println!("Good guess!");
        } else {
            // Wrong guess: draw another part of the hangman.
            wrong += 1;
            println!("Nope, '{guess}' is not in the word.");
        }

        // Win: every letter in the word has been guessed.
        if word.chars().all(|c| guessed.contains(&c)) {
            println!("\nThe word was: {word}");
            println!("You WIN!");
            return true;
        }

        // Lose: the hangman is fully drawn.
        if wrong >= MAX_WRONG_GUESSES {
            wrong_guess(wrong);
            println!("\nThe word was: {word}");
            println!("You LOSE!");
            return false;
        }
    }
}

fn main() {
    println!("Welcome to Hangman!");

    loop {
        play_round();

        print!("\nPlay again? (y/n): ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Thanks for playing!");
            break;
        }
    }
}
