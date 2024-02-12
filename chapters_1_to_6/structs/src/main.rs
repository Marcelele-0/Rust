//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// } 
// // like a toople this lets us store multiple types of data in one struct
// // lets see data as name instead of index number like in toople

// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"), 
//         active: true,
//         sign_in_count: 1, 
//     }; 

//     let name = user1.username;
//     println!("The username is: {}", name);
    
//     user1.username = String::from("anotherusername");
//     println!("The username is: {}", user1.username);
//     // we can change the value of the struct by using the dot notation


//     let user2 = build_user(
//         String::from("random@mail.com"),
//         String::from("nickname123")
//     );

//     let user3 = User {
//         email: String::from("randommail2@.com"),
//         username: String::from("randomthethird"),
//         ..user2
//     };

//     // tuple structs =
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
//     // these are different types because they are different structs
//     // its helpful when= you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as a struct does


// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,           // we can simiplify the code by using the field init shorthand
//         active: true,
//         sign_in_count: 1,
//     }
// }

// example use case //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 { // its a method because it takes a reference to self as its only parameter
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // new implementation block for the sake of example
    fn square(size: u32) -> Rectangle { // associated function because it doesnt take a reference to self as a parameter
        Rectangle { 
            width: size, 
            height: size 
        }
    }
    
}

fn main() {
    // using individual variables
    let widht1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(widht1, height1) 
    );

    
    // using a toople
    let rect_toople = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.", 
        area_toople (rect_toople)
    );

    
    // using a struct
    let rect_struct = Rectangle { 
        width: 30, 
        height: 50 
    };

    let rect_struct2 = Rectangle { 
        width: 10, 
        height: 40 
    };

    let rect_struct3 = Rectangle { 
        width: 60, 
        height: 45 
    };

    
    let rect_struct4 = Rectangle::square(25);
    println!("rect4: {:#?}", rect_struct4);

    println!("Can rect1 hold rect2? {}", rect_struct.can_hold(&rect_struct2));
    println!("Can rect1 hold rect3? {}", rect_struct.can_hold(&rect_struct3));

    println!("rect: {:#?}", rect_struct);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_struct.area()
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_toople(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//fn area_struct(rectangle: &Rectangle) -> u32 { // we use the & to borrow the value of the struct because we dont want to take ownership of the value
//    rectangle.width * rectangle.height
//}