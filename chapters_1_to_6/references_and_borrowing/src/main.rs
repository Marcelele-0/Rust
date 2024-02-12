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
    
    { // lets fix the above code
       
        let s1 = String::from("hello");
        let len = calculate_length(&s1);            // we borrow s1 here, not take ownership
        println!("The length of '{}' is {}.", s1, len);    // so now s1 is still valid

        fn calculate_length(s: &String) -> usize {
            let length = s.len();
            length
        } 
    }
   
   
    ////////////////////////////////////////////////////////////////////////////////////////////
   
    {
        let s1 = String::from("hello");
        change(&s1);

        fn change (some_string: &String) {
            some_string.push_str(", world");    // this will not work
        }
    }

    {  // lets fix the above code
        let mut s1 = String::from("hello");
        change(&mut s1);

        fn change (some_string: &mut String) {
            some_string.push_str(", world");    // this will work, without taking ownership
    }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////

    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        //let r2 = &mut s;    // this will not work, because we can't have two mutable references to the same variable in the same scope
        println!("{}, {}", r1, r2);
    }

    { // lets fix the above code
        let s = String::from("hello");

        let r1 = &s;
        let r2 = &s;    // this will work, because we can have multiple immutable references to the same variable in the same scope

        println!("{}, {}", r1, r2);

    }

    { // mix mutable and inmutable references
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        //let r3 = &mut s;    // this will not work, because we can't have a mutable reference while we have inmutable references

        println!("{}, {}", r1, r2);      // r1 and r2 are out of scope here

        let r3 = &mut s;    // this will work, because we don't have inmutable references anymore

    }



    // dangling references /////////////////////////////////////////////////////////////////////////////////////
    {
        let reference_to_nothing = dangle();
        println!("{}", reference_to_nothing);
    
        fn dangle() -> &String {
            let s = String::from("hello");   // we drop s at the end of the function
            &s                                       // so this will return a reference to nothing
        }
    }


    { /// fix 
        let reference_to_nothing = dangle();
        println!("{}", reference_to_nothing);
    
        fn dangle() -> &String {
            let s = String::from("hello");
            &s
        }
    }


    /*
    rules of references:
    - at any given time, you can have either one mutable reference or any number of immutable references
    - references must always be valid
     */
}
