fn main() {
    println!("The value of sum is: {}\n", my_function_1(5, 6));
    
    let sum_2 = my_function_2(6, 7);
    println!("The value of sum is: {}\n", sum_2);
}

fn my_function_1(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let sum_1: i32 = x + y;
    sum_1
}

fn my_function_2(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
