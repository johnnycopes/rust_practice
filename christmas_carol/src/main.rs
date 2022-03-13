// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main() {
    let lyrics = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three French hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five golden rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven swans a-swimming"),
        ("eighth", "eight maids a-milking"),
        ("ninth", "nine pipers piping"),
        ("tenth", "ten ladies dancing"),
        ("eleventh", "eleven lords a-leaping"),
        ("twelfth", "twelve drummers drumming"),
    ];
    let mut current_day_index = 0;

    while current_day_index < lyrics.len() {
        let (day, gift) = lyrics[current_day_index];

        println!("On the {} day of Christmas, my true love gave to me: \n  {}", day, gift);
    
        if current_day_index > 0 {
            let mut previous_day_index = current_day_index - 1;
            loop {
                let (_day, gift) = lyrics[previous_day_index];

                println!("  {}", gift);

                if previous_day_index > 0 {
                    previous_day_index -= 1;
                } else {
                    break;
                }
            }
        }
        println!("-----");
        current_day_index += 1;
    }
}
