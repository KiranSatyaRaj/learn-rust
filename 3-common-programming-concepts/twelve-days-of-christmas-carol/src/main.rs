fn main() {
    let numbers = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let lyrics = [
      "and a partridge in a pear tree.",
        "Two turtle dove,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese-a-laying,",
        "Seven swans-a-swimming,",
        "Eight maids-a-milking,",
        "Nine ladies a dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for number in 0..12 {
        println!("On the {} day of Christmas,", numbers[number]);
        println!("my true love gave to me");
        if number == 0 {
            println!("A partridge in a pear tree.");
        } else {
            for lyric in (0..number + 1).rev() {
                println!("{}", lyrics[lyric]);
            }
        }
        println!();
    }

}