fn main() {
    //control flow
    let if_num: i32 = 5;

    if if_num < 10 {        // have to be a boolean 
        println!("first con was true")
    } else if if_num < 20 {
        println!("second con was true")
    } else {
        println!("none of cons were true")
    }

    let condition: bool = true;
    let con_num = if condition { 1 } else { 0 };
    println!("con num is {}", con_num);


    // loops //////////////////////////////////////////

    // loop
    let mut counter: i32 =1;
    let result = loop {
        println!("counting: {}", counter);
        counter += 1;
        
        if counter == 10 {
            break counter;
        }
    };
    println!("result is {}", result);


    // while 
    let mut countdown = 10;

    while countdown != 0 {
        println!("{}!", countdown);
        countdown -= 1;
    }

    println!("{},take off!", countdown);


    // for 
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for digit in 1..10 {                // 10 excluded
        println!("number is:{}", digit);
    }


    // line coments 
    
    /*
    block
    comments
     */
}
