fn main() { // slices do not take ownership of the data
    
    let s = String::from("hello world");
    let s2 = "hello world";

    let word = first_word(s2);
    println!("The first word is: {}", word);

    ///////// slicing arrays

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..2]; 

    println!("The slice is: {:?}", slice); // its not just {} because its an array, and it needs to be printed with debug formatting
}

    
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
    



}



