fn main() {
    {

        let s1 = String::from("hello");
        let (s2 , len) = calculate_length(s1);     // s1 is moved here
        println!("The length of '{}' is {}.", s2, len);

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }
    }
    
    {
        // lets fix the above code
        let s1 = String::from("hello");
        let len = calculate_length(&s1);            // s1 is not moved here, reference is passed      
        println!("The length of '{}' is {}.", s1, len);    // so now s1 is still valid

        fn calculate_length(s: &String) -> usize {
            let length = s.len();
            length
        } 
    }


}
