
// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// Lyrics
// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree
// On the second day of Christmas, my true love sent to me
// Two turtledoves
// And a partridge in a pear tree
// On the third day of Christmas, my true love sent to me
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the fourth day of Christmas, my true love sent to me
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the fifth day of Christmas, my true love sent to me
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the sixth day of Christmas, my true love sent to me
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the seventh day of Christmas, my true love sent to me
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the eighth day of Christmas, my true love sent to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the ninth day of Christmas, my true love sent to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the tenth day of Christmas, my true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the eleventh day of Christmas, my true love sent to me
// I sent eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// On the twelfth day of Christmas, my true love sent to me
// Twelve drummers drumming
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five gold rings (five golden rings)
// Four calling birds
// Three French hens
// Two turtledoves
// And a partridge in a pear tree
// And a partridge in a pear tree

fn main() {

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelth"];

    let lines = [
        "partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",];

    for i in 1..13 {
        println!("On the {} day of Christmas, my true love sent to me", days[i -1]);
        for j in 0..i {
            if i - j == 1 && j != 0 {
                print!("And a ")
            } else if i - j == 1 {
                print!("A ")
            }
            println!("{}",lines[i - j - 1])
        }
    }
    println!("And a {} ", lines[0])

}
