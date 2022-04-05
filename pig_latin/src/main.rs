// Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that 
// start with a vowel have “hay” added to the end instead (“apple” becomes 
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn main() {
    assert_eq!(pig_latinize("hello"), "ello-hay");
    assert_eq!(pig_latinize("Sup"), "up-Say");
    assert_eq!(pig_latinize("apple"), "apple-hay");
    assert_eq!(pig_latinize("Excelsior"), "Excelsior-hay");

    assert_eq!(pig_latinize_slices("hello"), "ello-hay");
    assert_eq!(pig_latinize_slices("Sup"), "up-Say");
    assert_eq!(pig_latinize_slices("apple"), "apple-hay");
    assert_eq!(pig_latinize_slices("Excelsior"), "Excelsior-hay");

    assert_eq!(pig_latinize_mut(String::from("hello")), "ello-hay");
    assert_eq!(pig_latinize_mut(String::from("Sup")), "up-Say");
    assert_eq!(pig_latinize_mut(String::from("apple")), "apple-hay");
    assert_eq!(pig_latinize_mut(String::from("Excelsior")), "Excelsior-hay");
}

// Initial answer. Not the most efficient because it creates a new String allocation

fn pig_latinize(s: &str) -> String {
    let mut string = s.to_string();

    let first_char = string.remove(0);

    match first_char {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => format!("{}{}-hay", first_char, string),
        _ => format!("{}-{}ay", string, first_char)
    }   
}

// Alternative answer using only slices

fn pig_latinize_slices(s: &str) -> String {
    let (first, rest) = (&s[..1], &s[1..]);

    match first {
        "a" | "A" | "e" | "E" | "i" | "I" | "o" | "O" | "u" | "U" => {
            format!("{}-hay", s)
        }
        _ => format!("{}-{}ay", rest, first)
    }
}

// Alternative answer which takes ownership of and mutates the original String

fn pig_latinize_mut(mut s: String) -> String {
    let first_char = s.remove(0);

    match first_char {
        'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => {
            s.insert(0, first_char);
            s.push_str("-hay");
        }
        _ => {
            s.push('-');
            s.push(first_char);
            s.push_str("ay");
        }
    }

    s
}
