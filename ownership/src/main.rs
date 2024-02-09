fn main() {
    // ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // s is not valid here, it’s not yet declared
        let s:String = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid
         

    
    let x = 5;
    let y = x; // x is copied to y

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2 (s1 is no longer valid), s2 is the new owner, shallow copy

    println!("{}", s1); // this will not work, s1 is no longer valid

    
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is cloned to s2, s1 is still valid

    
    
    // ownership and functions ///////////////////////////////////////////////////////////////////////////////////////

    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...// ... and so is no longer valid here
    println!("{}", s); // this will not work, s is no longer valid                             
    
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
    println!("{}", x); // this will work, x is still valid

   
   
   
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.                               

    // intiger is a copy type, so it is not moved, it is copied
    // string is a move type, so it is moved, not copied

    let s2 = takes_and_gives_back(s2); // s2 is moved to the function 
    println!("{}", s2); // this will work, s2 is still valid its because s2 is returned from the function


    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
        let a_string = String::from("hello");           // a_string is returned and moves out to the calling function
        
        a_string  
    } 
}



