fn main() {
    enum option<T> {
        Some(T),
        None,
    }// they are so useful that they are included in the standard library


    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;


    let x: i8 = 5;                  // this is i8
    let y: Option<i8> = Some(5);    // this is Optional i8
    let z: Option<i8> = None;      // this is Optional i8


    let sum_0 = x + y.unwrap_or(0);
    let sum_1 = x +z.unwrap_or(0);

    println! ("sum is {}", sum_0);
    println! ("sum is {}", sum_1);

}


