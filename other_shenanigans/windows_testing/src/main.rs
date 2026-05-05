fn main() {
    let s = b"hello";
    for w in s.windows(3) {
        println!("{:?}", w);
    }
}

/*
[104, 101, 108]  // "hel"
[101, 108, 108]  // "ell"
[108, 108, 111]  // "llo"
*/
