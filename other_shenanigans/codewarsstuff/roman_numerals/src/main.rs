fn roman_as_num(roman: &str) -> u64 {
    fn value(c: char) -> i64 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    let chars: Vec<char> = roman.chars().collect();
    let mut total: i64 = 0;

    for i in 0..chars.len() {
        let current = value(chars[i]);
        let next = if i + 1 < chars.len() {
            value(chars[i + 1])
        } else {
            0
        };

        if current < next {
            total -= current;
        } else {
            total += current;
        }
    }

    total as u64
}

fn main() {
    println!("{}", roman_as_num("MM")); // 2000
    println!("{}", roman_as_num("MDCLXVI")); // 1666
    println!("{}", roman_as_num("M")); // 1000
    println!("{}", roman_as_num("CD")); // 400
    println!("{}", roman_as_num("XC")); // 90
    println!("{}", roman_as_num("XL")); // 40
    println!("{}", roman_as_num("I")); // 1
}
