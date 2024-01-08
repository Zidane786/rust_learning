// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.
// Calling a function is an expression. Calling a macro is an expression,
// even single entity like 6 or 6+4 which gives 10 is also an example of expression


// normal function taking no parameters and no return value:-
pub fn print_hello(){
    println!("Hello, World!");
}

// function which takes single parameter and do something:-
pub fn greeting(name: String){
    println!("Hello, {name}");
}

// function that takes multiple arguments and return a evaluted result:-
// to return a value we need return type specify
pub fn add_numbers(x:i32,y:i32) -> i32{
    return x + y;
}

// will return result without using return keyword
// to return a value we need return type specify
pub fn multiple_numbers(x:i32,y:i32) -> i32{
    // last expression without semicolon will be considered returned value
    x * y // adding semicolon will return in error since we defined return type
}

// Expressions do not include ending semicolons. If you add a semicolon to the end of an expression,
// you turn it into a statement, and it will then not return a value like in above multiple_numbers() we saw
// here is another example of expression not having semicolon:-
// let y = {
//     let x = 5;
//     x + 1
// };
// the above code will have y assign as 6 since in bracket we define x=5 and then we added up with 1
// and not define a semicolon in end so it will return the result of x+1 to y.