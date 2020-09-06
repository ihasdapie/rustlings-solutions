// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    if maybe_number.is_none(){
        println!("No Number!");
    }
    else{
        println!("printing: {}", maybe_number.unwrap_or(0));
    }
}

fn main() {
    print_number(Some(14));
    print_number(Some(99));

    let mut numbers = [Some(0); 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };
        numbers[iter as usize] = Some(number_to_add);
    }

    print_number(numbers[0]);
    print_number(numbers[1]);
    print_number(numbers[2]);
    print_number(numbers[3]);
    print_number(numbers[4]);
}
