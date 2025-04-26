fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", 
        "sixth", "seventh", "eighth", "ninth", "tenth", 
        "eleventh", "twelfth"
    ];
    
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];
    
    for day in 0..12 {
        println!("\nOn the {} day of Christmas,", days[day]);
        println!("my true love sent to me:");
        
        for gift_index in (0..=day).rev() { //reverse order (from current day tofirst day)
            if day > 0 && gift_index == 0 {
                print!("And ");
            }
            
            println!("{}", gifts[gift_index]);
        }
    }
}
