use std::collections::HashSet;

//get canonical form of snowflake
fn canonical(s: &[i32; 6]) -> [i32; 6] {
    let mut rotations: Vec<[i32; 6]> = Vec::with_capacity(12);

    //clockwise
    for i in 0..6 {
        let mut r = [0; 6];
        for j in 0..6 {
            r[j] = s[(i + j) % 6];
        }
        rotations.push(r);
    }

    //counter-clockwise
    let mut rev = [0; 6];
    for i in 0..6 {
        rev[i] = s[5 - i];
    }

    for i in 0..6 {
        let mut r = [0; 6];
        for j in 0..6 {
            r[j] = rev[(i + j) % 6];
        }
        rotations.push(r);
    }

    //get smallest rotation
    rotations.into_iter().min().unwrap()
}

fn has_duplicate_snowflakes(snowflakes: &Vec<[i32; 6]>) -> bool {
    let mut seen: HashSet<[i32; 6]> = HashSet::new();

    for s in snowflakes {
        let c = canonical(s);
        if seen.contains(&c) {
            return true;
        }
        seen.insert(c);
    }

    false
}

fn main() {
    let snowflakes = vec![
        [1, 2, 3, 4, 5, 6],
        [3, 4, 5, 6, 1, 2], // rotation of first
        [7, 8, 9, 10, 11, 12],
    ];

    println!(
        "{}",
        if has_duplicate_snowflakes(&snowflakes) {
            "Yes, there are identical snowflakes"
        } else {
            "All snowflakes are unique"
        }
    );
}
