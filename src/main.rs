
mod slice_type;
mod user;
use user::User;
#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // control_flow();
    // loop_test();
    // slice_type_test();
    let user = User {
        active: true,
        username: String::from("some user"),
        email: String::from("abc@example.com"),
        sign_in_count: 1,
    };
    let user1 = User{
        email: String::from("Abc@#gmail.com"),
        ..user
    };
    // user.show_name();
    user1.show_name();
    
    let back = Color(0, 2, 0);
    let mut origin = Point(0, 0, 0);
    let mut abc = &mut origin;
    abc.0 =1 ;
    
    
    println!("Size: {}, {}", back.0, back.1);
    
}

fn slice_type_test() {
    println!("This is size: {}",  slice_type::first_word(&"Hello"));
}

fn control_flow() {
    let number = 3;
    if number % 2 == 0 {
        println!("Even number ")
    } else {
        println!("Odd number")
    }

    let x = if number % 2 == 0 {true} else { false };

    println!("Value of x: {}", x);

}

fn loop_test () {
    let mut number = 5;
    loop {
        if(number >= 0) {
            println!("value of i: {}", number);
            number = number - 1;
        }
        else { break }

    }
}
