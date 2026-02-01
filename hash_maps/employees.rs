use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Employee Manager");
    println!("Commands:");
    println!("  Add <Name> to <Department>");
    println!("  List <Department>");
    println!("  List All");
    println!("  Quit");

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.starts_with("Add ") {
            handle_add(input, &mut company);
        } else if input.starts_with("List All") {
            handle_list_all(&company);
        } else if input.starts_with("List ") {
            handle_list_department(input, &company);
        } else {
            println!("Unknown command");
        }
    }
}

fn handle_add(input: &str, company: &mut HashMap<String, Vec<String>>) {
    // Expected format: Add Name to Department
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() < 4 || parts[2] != "to" {
        println!("Invalid format. Use: Add <Name> to <Department>");
        return;
    }

    let name = parts[1].to_string();
    let department = parts[3..].join(" ");

    let employees = company.entry(department).or_insert_with(Vec::new);
    employees.push(name);
    employees.sort();

    println!("Employee added");
}

fn handle_list_department(input: &str, company: &HashMap<String, Vec<String>>) {
    let department = input.strip_prefix("List ").unwrap();

    match company.get(department) {
        Some(employees) => {
            for name in employees {
                println!("{}", name);
            }
        }
        None => println!("No such department"),
    }
}

fn handle_list_all(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = company.keys().collect();
    departments.sort();

    for department in departments {
        println!("{}:", department);
        if let Some(employees) = company.get(department) {
            for name in employees {
                println!("  {}", name);
            }
        }
    }
}
