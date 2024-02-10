struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
} 
// like a toople this lets us store multiple types of data in one struct
// lets see data as name instead of index number like in toople

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"), 
        active: true,
        sign_in_count: 1, 
    }; 

    let name = user1.username;
    println!("The username is: {}", name);
    
    user1.username = String::from("anotherusername");
    println!("The username is: {}", user1.username);
    // we can change the value of the struct by using the dot notation


    let user2 = build_user(
        String::from("random@mail.com"),
        String::from("nickname123")
    );

    let user3 = User {
        email: String::from("randommail2@.com"),
        username: String::from("randomthethird"),
        ..user2
    };



}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,           // we can simiplify the code by using the field init shorthand
        active: true,
        sign_in_count: 1,
    }
}
