use std::collections::HashMap;

//given an array of ints and a target
//return the indicies of the two numbers that add up to the target

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }

        map.insert(num, i);
    }

    //no solution
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum(nums, target));
}
// should return [0, 1] (indexes of two numbers that
// sum up to target
