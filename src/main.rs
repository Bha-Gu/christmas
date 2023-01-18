fn main() {
    let day = [
        "partridge in a pear tree!",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings!",
        "Six geese a layin'",
        "Seven swans a swimmin'",
        "Eight maids milkin'",
        "Nine ladies dancin'",
        "Ten lords a leapin'",
        "Eleven pipers pipin'",
        "Twelve drummers drummin'",
    ];
    let count = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas\nMy true love sent to me",
            count[i]
        );
        for j in (0..i + 1).rev() {
            if j == 0 && i != 0 {
                print!("And a ");
            } else if j == 0 {
                print!("A ");
            }
            println!("{}", day[j]);
        }
    }
}
