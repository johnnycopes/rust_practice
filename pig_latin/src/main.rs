//Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that 
// start with a vowel have “hay” added to the end instead (“apple” becomes 
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    assert_eq!(pig_latinize("hello"), "ello-hay");
    assert_eq!(pig_latinize("Sup"), "up-Say");
    assert_eq!(pig_latinize("apple"), "apple-hay");
    assert_eq!(pig_latinize("Excelsior"), "Excelsior-hay");
}

fn pig_latinize(s: &str) -> String {
    let mut string = s.to_string();

    let first_char = string.remove(0);

    match first_char {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => format!("{}{}-hay", first_char, string),
        _ => format!("{}-{}ay", string, first_char)
    }   
}
