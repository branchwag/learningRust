use crossterm::{
    event::{self, poll, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let menu_items = vec!["Add Employee", "List Department", "List All", "Quit"];
    let mut selected = 0;

    // Try raw mode first
    if let Ok(_) = enable_raw_mode() {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))?;

        loop {
            draw_menu_interactive(&menu_items, selected)?;

            // Use poll to avoid blocking indefinitely
            if poll(Duration::from_millis(100))? {
                match event::read() {
                    Ok(Event::Key(key_event)) => match key_event.code {
                        KeyCode::Up => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if selected + 1 < menu_items.len() {
                                selected += 1;
                            }
                        }
                        KeyCode::Enter => {
                            execute!(stdout, Clear(ClearType::All))?;
                            disable_raw_mode()?;
                            println!("You selected: {}", menu_items[selected]);
                            return Ok(());
                        }
                        KeyCode::Esc => {
                            disable_raw_mode()?;
                            return Ok(());
                        }
                        _ => {}
                    },
                    Ok(_) => {}
                    Err(_) => {
                        disable_raw_mode()?;
                        break;
                    }
                }
            }
        }
    }

    // Fallback to simple mode (only if raw mode fails)
    draw_menu_simple(&menu_items)?;
    Ok(())
}

fn draw_menu_interactive(items: &[&str], selected: usize) -> std::io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;

    for (i, item) in items.iter().enumerate() {
        if i == selected {
            println!("> {}", item);
        } else {
            println!("  {}", item);
        }
    }

    println!("\nUse arrow keys to navigate, Enter to select, Esc to quit");
    stdout.flush()?;
    Ok(())
}

fn draw_menu_simple(items: &[&str]) -> std::io::Result<()> {
    println!("\n=== Simple Menu Mode ===");
    println!("Raw terminal mode not available. Using simple keyboard input.");

    for (i, item) in items.iter().enumerate() {
        println!("{}: {}", i + 1, item);
    }
    println!("q: Quit");
    println!();

    loop {
        print!("Enter your choice (1-{} or q): ", items.len());
        std::io::stdout().flush()?;

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim().to_lowercase();

                if trimmed == "q" {
                    println!("You selected: {}", items[items.len() - 1]);
                    return Ok(());
                }

                match trimmed.parse::<usize>() {
                    Ok(choice) if 1 <= choice && choice <= items.len() => {
                        println!("You selected: {}", items[choice - 1]);
                        return Ok(());
                    }
                    Ok(_) | Err(_) => {
                        println!("Invalid choice. Please enter a number between 1 and {}, or 'q' to quit.", items.len());
                    }
                }
            }
            Err(_) => {
                println!("Error reading input. Exiting.");
                return Ok(());
            }
        }
    }
}
