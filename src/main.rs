
mod slice_type;

fn main() {
    // control_flow();
    // loop_test();
    slice_type_test();
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
