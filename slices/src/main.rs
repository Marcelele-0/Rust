fn main() {
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s);

    s.clear(); // error!

    fn first_word(s: &str) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
        
    }

}

