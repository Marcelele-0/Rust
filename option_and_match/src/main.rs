// fn main() {
//     enum option<T> {
//         Some(T),
//         None,
//     }// they are so useful that they are included in the standard library


//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: Option<i32> = None;


//     let x: i8 = 5;                  // this is i8
//     let y: Option<i8> = Some(5);    // this is Optional i8
//     let z: Option<i8> = None;      // this is Optional i8


//     let sum_0 = x + y.unwrap_or(0);
//     let sum_1 = x +z.unwrap_or(0);

//     println! ("sum is {}", sum_0);
//     println! ("sum is {}", sum_1);

// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}", six);  
//     println!("{:?}", none);

// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }


// fn plus_one(x: Option<i32>) -> Option<i32> { 
//     match x {
//         Some(i) => Some(i + 1),
//         _ => None, // placeholder for all other cases
//     }
// }

fn main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (), // otherwise do nothing
    } // here we specify all the possible values that some_value could have and what we want to do in each case

    if let Some(3) = some_value { // if let, some value = some 3 => print three
        println!("three"); 
    } // here we specify a pattern and a value, and the code runs if the value matches the pattern

    
}




