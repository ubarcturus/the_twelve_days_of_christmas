fn main() {
    let ordinal_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mirth_without_mischief_gifts = [
        "a Partridge in a pear-tree.",
        "two Turtle doves, and",
        "three French hens,",
        "four Colly birds,",
        "five Gold rings,",
        "six Geese a laying,",
        "seven Swans a swimming,",
        "eight Maids a milking,",
        "nine Drummers drumming,",
        "ten Pipers piping,",
        "eleven Ladies dancing,",
        "twelve Lords a leaping,",
    ];
    for (i, ordinal_number) in ordinal_numbers.iter().enumerate() {
        println!("The {ordinal_number} day of Christmas my true love sent to me");
        for i in (0..=i).rev() {
            println!("{}", mirth_without_mischief_gifts[i]);
        }
        println!();
    }
}
