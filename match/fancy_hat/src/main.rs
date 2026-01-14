use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crossterm::terminal;
use unicode_width::UnicodeWidthStr;

fn center_text(text: &str) -> String {
    if let Ok((width, _)) = terminal::size() {
        let text_width = UnicodeWidthStr::width(text);
        let padding = if width as usize > text_width {
            (width as usize - text_width) / 2
        } else {
            0
        };
        format!("{}{}", " ".repeat(padding), text)
    } else {
        text.to_string()
    }
}

fn main() {
    // Create perfectly aligned box accounting for emoji width
    let title = "DICE ADVENTURE GAME";
    let inner_content = format!("🎲  {}  🎲", title);
    
    // Calculate visual width of the middle line content
    let content_width = UnicodeWidthStr::width(inner_content.as_str());
    
    // Create border lines with same visual width
    let top_line = format!("╔{}╗", "═".repeat(content_width + 2)); // +2 for the spaces after ║
    let mid_line = format!("║ {} ║", inner_content);
    let bot_line = format!("╚{}╝", "═".repeat(content_width + 2));
    
    println!();
    println!("{}", center_text(&top_line));
    println!("{}", center_text(&mid_line));
    println!("{}", center_text(&bot_line));
    println!();
    
    loop {
        println!("{}", center_text("Press ENTER to roll the dice (or 'q' to quit)..."));
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if input.trim().eq_ignore_ascii_case("q") {
            println!();
            println!("{}", center_text("👋 Thanks for playing! Goodbye!"));
            println!();
            break;
        }
        
        // Animate dice roll
        animate_dice_roll();
        
        let dice_roll = rand::thread_rng().gen_range(1..=9);
        
        // Show the dice result
        display_dice(dice_roll);
        
        println!();
        println!("{}", center_text(&format!("🎲 You rolled a {}!", dice_roll)));
        println!();
        
        // Handle the result
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        
        println!();
        println!("{}", center_text(&"─".repeat(45)));
    }
}

fn animate_dice_roll() {
    let frames = vec!["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"];
    
    print!("\nRolling");
    io::stdout().flush().unwrap();
    
    for _ in 0..8 {
        for frame in &frames {
            print!("\r🎲 Rolling... {} ", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
    println!("\r                              ");
}

fn display_dice(num: u8) {
    let dice_art = match num {
        1 => vec![
            "┌─────────┐",
            "│         │",
            "│    ●    │",
            "│         │",
            "└─────────┘",
        ],
        2 => vec![
            "┌─────────┐",
            "│  ●      │",
            "│         │",
            "│      ●  │",
            "└─────────┘",
        ],
        3 => vec![
            "┌─────────┐",
            "│  ●      │",
            "│    ●    │",
            "│      ●  │",
            "└─────────┘",
        ],
        4 => vec![
            "┌─────────┐",
            "│  ●   ●  │",
            "│         │",
            "│  ●   ●  │",
            "└─────────┘",
        ],
        5 => vec![
            "┌─────────┐",
            "│  ●   ●  │",
            "│    ●    │",
            "│  ●   ●  │",
            "└─────────┘",
        ],
        6 => vec![
            "┌─────────┐",
            "│  ●   ●  │",
            "│  ●   ●  │",
            "│  ●   ●  │",
            "└─────────┘",
        ],
        7 => vec![
            "┌─────────┐",
            "│  ●   ●  │",
            "│  ● ● ●  │",
            "│  ●   ●  │",
            "└─────────┘",
        ],
        8 => vec![
            "┌─────────┐",
            "│  ● ● ●  │",
            "│  ●   ●  │",
            "│  ● ● ●  │",
            "└─────────┘",
        ],
        9 => vec![
            "┌─────────┐",
            "│  ● ● ●  │",
            "│  ● ● ●  │",
            "│  ● ● ●  │",
            "└─────────┘",
        ],
        _ => vec![
            "┌─────────┐",
            "│    ?    │",
            "│    ?    │",
            "│    ?    │",
            "└─────────┘",
        ],
    };
    
    for line in dice_art {
        println!("{}", center_text(line));
    }
}

fn add_fancy_hat() {
    println!("{}", center_text("✨ Special Event! ✨"));
    println!();
    println!("{}", center_text("      🎩"));
    println!("{}", center_text("     ╱ ╲"));
    println!("{}", center_text("    ╱   ╲"));
    println!("{}", center_text("   ╱─────╲"));
    println!("{}", center_text("      😊"));
    println!("{}", center_text("     ╱│ │╲"));
    println!("{}", center_text("    ╱ │ │ ╲"));
    println!("{}", center_text("      │ │"));
    println!("{}", center_text("     ╱   ╲"));
    println!();
    println!("{}", center_text("🎩 You found a fancy hat! Looking dapper!"));
    println!();
}

fn remove_fancy_hat() {
    println!("{}", center_text("💨 Special Event! 💨"));
    println!();
    println!("{}", center_text("                🎩"));
    println!("{}", center_text("               ╱ ╲  💨"));
    println!("{}", center_text("              ╱   ╲    💨"));
    println!("{}", center_text("             ╱─────╲      💨"));
    println!("{}", center_text("                😢"));
    println!("{}", center_text("               ╱│ │╲"));
    println!("{}", center_text("              ╱ │ │ ╲"));
    println!("{}", center_text("                │ │"));
    println!("{}", center_text("               ╱   ╲"));
    println!();
    println!("{}", center_text("💨 Oh no! The wind blew away your fancy hat!"));
    println!();
}

fn move_player(num_spaces: u8) {
    println!("{}", center_text(&format!("🚶 Moving forward {} spaces!", num_spaces)));
    println!();
    
    // Calculate centering for animation
    let (term_width, _) = terminal::size().unwrap_or((80, 24));
    let max_move = 9 * 3; // Maximum animation width
    let start_padding = if term_width as usize > max_move {
        (term_width as usize - max_move) / 2
    } else {
        0
    };
    
    // Animate movement
    for i in 0..num_spaces {
        let padding = " ".repeat(start_padding + (i * 3) as usize);
        print!("\r{}🚶 ", padding);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(150));
    }
    println!("\n");
    
    // Show different messages based on distance
    let message = match num_spaces {
        1..=2 => "🐢 A small step forward!",
        3..=5 => "🏃 Nice progress!",
        6..=9 => "🚀 Wow! That's a big leap!",
        _ => "🎉 Amazing!",
    };
    println!("{}", center_text(message));
    
    // Show progress bar
    let filled = "█".repeat(num_spaces as usize);
    let empty = "░".repeat((9 - num_spaces) as usize);
    let progress = format!("Progress: [{}{}] {}/9", filled, empty, num_spaces);
    println!();
    println!("{}", center_text(&progress));
}
