pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_one_works() {
        assert_eq!(3, add_one(2));
    }
}
