use std::io; /* To obtain user input and the print the result as output, we need to bring the 'io' input/output library into scope.
The io library comes from the standard library, known as 'std': */
// If a type you want to use isn't in the prelude, you need to bring it into scope via a 'use'
// statement

fn main() {
    println!("Guess the number!");

    println!("Please input your guess."); /* 'println!' is a macro that prints a string to the screen */

    let mut guess = String::new(); /* We use the 'let' statement to create the variable and 'mut' to make it mutable;
    On the right of the '=' sign is the value that guess is bound to, which is the result if calling 'String::new', a funtion that
    returns a new instance of 'String'. 'String' is a string type provided by the standard library that is a growable UTF-8
    encoded bit of text. The '::' syntax in the '::new' line indicates that 'new' is an associated function of the 'String' type. An
    associated function is a function thats implemented on a type, ie 'String'. This 'new' function creates a new, empty string.
    In full, the 'let mut guess = String::new();' line has created a mutable variable that is currently bound to a new, empty instance
    of a 'String'.
     */

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
// The 'main' function is the entry point into the program
// The 'fn' syntax declares a new function; the parentheses, '()', indicate there are no
// parammeters; and the curly bracket, '{', starts the body of a function
