mod guess_the_number;

// use guess_the_number::guess_the_number;

mod data_type;
mod function;



fn main() {
    // guess_the_number();

    let y = {
        let x = 5;
        x + 1
    };
    println!("{:?}", y);

    function::print_hello();
    function::greeting("Zidane".to_owned());
    let result_add = function::add_numbers(5,4);
    let result_multiply = function::multiple_numbers(5, 4);
    println!("result of addition is:- {result_add}");
    println!("result of multiplication is:- {result_multiply}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        };
    };
    println!("{counter},{result}");


}
