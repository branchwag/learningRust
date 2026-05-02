// n - numerator
// d - denominator

// reduced proper fraction if gcd(n,d) = 1
// if given number n - how many proper fractions can be built using d as a denominator?

fn proper_fractions(d: u64) -> u64 {
    //euler's totient function
    //compute how many numbers 1 to d-1 do not share any factor with d
    //count integers n where 1<=n<d and gcd(n,d) == 1
    if d == 1 {
        return 0;
    }

    let mut n = d;
    let mut result = d; //current count estimate
    let mut p = 2; //possible prime factor

    //try possible factors
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }

            result -= result / p;
        }

        p += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

fn main() {
    println!("{}", proper_fractions(1)); // 0
    println!("{}", proper_fractions(2)); // 1
    println!("{}", proper_fractions(5)); // 4
    println!("{}", proper_fractions(15)); // 8
    println!("{}", proper_fractions(25)); // 20   
}
