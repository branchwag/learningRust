// decipher runes
// base 10
// never start number with leading zero
// runes are digits
// operators - + *
// range from -1000000 to 1000000, and will consist of only the digits 0-9, possibly a leading -, and maybe a few ?s
// ? - digit, need to figure out value
// takes in string representing expression, will return int value or -1 if no such rune exists.
use std::io;

pub fn solve_runes(runes: &str) -> Option<u8> {
    for d in b'0'..=b'9' {
        if runes.as_bytes().contains(&d) {
            continue;
        }

        let expr = runes.replace('?', &(d as char).to_string());

        if valid_expression(&expr) && evaluates_true(&expr) {
            return Some(d - b'0');
        }
    }

    None
}

fn valid_num(s: &str) -> bool {
    let n = s.strip_prefix('-').unwrap_or(s);

    if n.len() > 1 && n.starts_with('0') {
        return false;
    }

    !n.is_empty()
}

fn valid_expression(expr: &str) -> bool {
    let (left, right) = match expr.split_once('=') {
        Some(parts) => parts,
        None => return false,
    };

    let mut op_pos = None;

    for (i, c) in left.char_indices().skip(1) {
        if c == '+' || c == '-' || c == '*' {
            op_pos = Some(i);
            break;
        }
    }

    let op_pos = match op_pos {
        Some(pos) => pos,
        None => return false,
    };

    let a = &left[..op_pos];
    let b = &left[op_pos + 1..];
    let c = right;

    valid_num(a) && valid_num(b) && valid_num(c)
}

fn evaluates_true(expr: &str) -> bool {
    let (left, right) = match expr.split_once('=') {
        Some(parts) => parts,
        None => return false,
    };

    let mut op = None;
    let mut op_pos = 0;

    for (i, c) in left.char_indices().skip(1) {
        if c == '+' || c == '-' || c == '*' {
            op = Some(c);
            op_pos = i;
            break;
        }
    }

    let op = match op {
        Some(o) => o,
        None => return false,
    };

    let a = match left[..op_pos].parse::<i64>() {
        Ok(v) => v,
        Err(_) => return false,
    };

    let b = match left[op_pos + 1..].parse::<i64>() {
        Ok(v) => v,
        Err(_) => return false,
    };

    let c = match right.parse::<i64>() {
        Ok(v) => v,
        Err(_) => return false,
    };

    match op {
        '+' => a + b == c,
        '-' => a - b == c,
        '*' => a * b == c,
        _ => false,
    }
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim();
            println!("{:?}", solve_runes(input));
        }
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
        }
    }

    let input = input.trim();

    match solve_runes(input) {
        Some(d) => println!("{}", d),
        None => println!("-1"),
    }
}
