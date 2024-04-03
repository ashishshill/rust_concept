fn main() {
    // All Ways variable name are Uppercare when you set up constant variable
    // const SUBSCRIBE_NUMBER: u32 = 10000;
    println!("Hello, world!");
    let sum = my_function(11, 12);
    println!("the sum value is :{}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("the number is : {}", x);
    println!("the string is: {}", y);
    // this is a expression beacause we return that value
    x + y
    // return this value for
}
fn main() {
    // Control Flow
    let condition = false;
    let number = if condition { 5 } else { 6 };
    let number = 5;
    if number < 5 {
        println!("number is true");
    } else if number < 22 {
        println!("number is false");
    }
    println!("okay {}", number);
}
fn main() {
    // there is 3type of loop another 2 loop
    // one is whole loop and oters one for loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("Number is {}", result);
}
