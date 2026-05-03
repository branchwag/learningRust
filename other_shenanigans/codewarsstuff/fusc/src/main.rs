//fusc
// 0 -> 0
// 1 -> 1
// 2 * n = fusc(n)
// 2 * n + 1 = fusc(n) + fusc(n+1)

//compute fusc of a number
// check if even, divide by 2 -> fusc n
//
// check if odd, divide by 2 and what's left over is the secnd function. (If it's not even, it's
// odd so fall through to here
//
// check if 0, check if 1
// use recursion to break down rest of number

fn fusc(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ if n % 2 == 0 => fusc(n / 2),
        _ => fusc(n / 2) + fusc(n / 2 + 1),
    }
}

fn main() {
    println!("{}", fusc(10)); //should print 3
}
