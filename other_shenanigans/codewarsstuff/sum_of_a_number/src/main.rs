fn exp_sum(n: u64) -> u64 {
    let n = n as usize;
    let mut dp = vec![0u64; n + 1];

    //base case
    // there is exactly one way to make the sum 0
    //we are counting ways
    dp[0] = 1;

    for k in 1..=n {
        for i in k..=n {
            dp[i] += dp[i - k];
        }
    }
    dp[n]
}

fn main() {
    assert_eq!(exp_sum(1), 1);
}
