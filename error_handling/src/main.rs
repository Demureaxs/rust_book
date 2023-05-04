pub mod error_handling {

    pub mod unrecoverable_errors_with_panic {
        pub fn errors_with_panic() {
            panic!("crash and burn");
        }

        // run with RUST_BACKTRACE=1 cargo run
        pub fn panic_backtrace() {
            let v = vec![1, 2, 3];

            v[99];
        }
    }

    pub mod recoverable_errors_with_result {
        pub fn recoverable_errors_with_result() {
            // this is the result enum.
            enum Result<T, E> {
                OK(T),
                Err(E),
            }

            // brings the file system into scope
            use std::fs::File;

            use std::io::ErrorKind;
            // creates a variable that is the result of opening a file called hello.txt
            // this is an Option that returns a result or an error so needs handling
            // type is :Result<File, Error>
            let greeting_file_result = File::open("hello.txt");

            // this way enables us to just primarily handle success and failure...
            // let greeting_file = match greeting_file_result {
            //     // if the file exists return it
            //     Ok(file) => file,
            //     // if not return the error
            //     Err(error) => panic!("Problem opening the file: {}", error),
            // };

            // this mothod lets us choose specific actions based on the error that occured
            // checks if there is a file
            let greeting_file = match greeting_file_result {
                // if so returns the file
                Ok(file) => file,
                // if not run the error with a nested match to check the error kind
                Err(error) => match error.kind() {
                    // if the error kind is not found create a new file with a match expression
                    ErrorKind::NotFound => match File::create("hello.txt") {
                        // if the file creation is successfull return the created file
                        Ok(file_created) => file_created,
                        // if there was an error then list the error related to creating the file
                        Err(error) => panic!("Problem creating the file: {:?}", error),
                    },
                    // for any other error panic and return the error
                    other_error => {
                        panic!("Problem opening the file: {:?}", other_error);
                    }
                },
            };
        }
        pub fn recovering_with_closures() {
            use std::fs::File;
            use std::io::ErrorKind;

            // this is a more concise way to handle the errors and uses closures (similar to nested functions in js)
            // tries to open the file and unwrap(return the result) or error (closure)
            let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
                // if the error kind is not found
                if error.kind() == ErrorKind::NotFound {
                    // create the file and unwrap(return the result if exists) or error (with a closure)
                    File::create("hello.txt").unwrap_or_else(|error| {
                        // Panic if there is an error
                        panic!("Problem creating the file: {:?}", error);
                    })
                    // if the error is anything else
                } else {
                    // panic anyway
                    panic!("Problem opening the file {:?}", error)
                }
            });
        }

        pub mod shortcuts_for_panic {
            use std::fs::File;
            // there are two shortcuts for panic on error. .unwrap() and .expect()
            pub fn shortcuts_for_panic_on_Error_with_unwrap() {
                let greeting_file = File::open("hello.txt").unwrap();
                // this returns:
                // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
                // code: 2, kind: NotFound, message: "No such file or directory" }',
                // src/main.rs:4:49
            }

            pub fn shortcuts_for_panic_on_Error_with_expect() {
                // the main difference here is that the error message will contain the contents of expect()
                let greeting_file =
                    File::open("Hello.txt").expect("hello.txt should be included in this project ");
            }
        }

        pub mod propogating_errors {

            use std::fs::File;
            use std::io::{self, Read};

            // this function reads a username from a file
            pub fn read_username_from_file() -> Result<String, io::Error> {
                // sets the file result to the opening of a file hello.txt
                let username_file_result = File::open("hello.txt");

                // creates a mutable variable that is the result of the match of fileresult
                let mut username_file = match username_file_result {
                    // if ok return the username
                    Ok(username) => username,
                    // otherwise return error
                    Err(e) => return Err(e),
                };

                // create a new string variable called username
                let mut username = String::new();

                // match expression on username_file (read to string) with a reference to &mut username
                // effectively reading the contents and writing them to the username variable if ok
                return match username_file.read_to_string(&mut username) {
                    // if ok assign the username variable to the result
                    Ok(_) => Ok(username),
                    // otherwise throw an error
                    Err(e) => Err(e),
                };
            }
        }

        pub mod shortcut_for_propogation {
            // a shortcut for propogating errors is the usage of the ? operator,
            // this is placed after a result value is defined to work in almost the same way as match expressions
            // that we defined to handle the Result values.

            // if the value is ok the code continues as expected...
            // however if there is an error of any kind it will be propogated to its caller.
            use std::fs::File;
            use std::io;
            use std::io::Read;

            pub fn read_username_from_file() -> Result<String, io::Error> {
                // assigns opening the file to username_file(if it fails propogates an error with "?")
                let mut username_file = File::open("hello.txt")?;
                // creates a new string called username
                let mut username = String::new();
                // attempts to write the result of reading to string the usernamefile(if it fails propogates an error with ?)
                username_file.read_to_string(&mut username)?;
                // if all goes well return the username and ok value
                return Ok(username);
            }

            // this function is the same as above but its more concise and demonstrates the power
            // of boilerplate reduction by using the ? operator on result types instead of
            // match statements or expect/unwrap
            pub fn chaining_using_qmark_operator() -> Result<String, io::Error> {
                // creates a new string variable called username
                let mut username = String::new();

                // opens a file hello.txt (if error return and propogate)... if ok then read to string and write to username
                // variable (if error return and propogate)
                File::open("hello.txt")?.read_to_string(&mut username)?;

                // return the username if all was ok.
                return Ok(username);
            }

            // yet a quicker and more consice way to achieve the above is as follows
            use std::fs;

            pub fn directly_read_uname_from_file() -> Result<String, io::Error> {
                // this is direct call to commit both actions with one method
                return fs::read_to_string("hello.txt");
                // this method already incorporates all of the error handling in the previous 3 versions
                // however its important to understand whats happening under the hood when we perform
                // common operations like this
            }
        }

        pub mod where_the_qmark_operator_can_be_used {
            // the ? operator can only be used if the return type of the function is a Result or Option value.
            // this is because it is only compatible with operations that allow an explicity return type

            use std::fs::File;

            pub fn naughty_naughty() {
                // the following will not work because the return type is not a Result or Option

                let greeting_file = File::open("hello.txt")?;

                //----------------Compiler output --------------------------------
                //                 $ cargo run
                //    Compiling error-handling v0.1.0 (file:///projects/error-handling)
                // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
                //  --> src/main.rs:4:48
                //   |
                // 3 | / fn main() {
                // 4 | |     let greeting_file = File::open("hello.txt")?;
                //   | |                                                ^ cannot use the `?` operator in a function that returns `()`
                // 5 | | }
                //   | |_- this function should return `Result` or `Option` to accept `?`
                //   |
                //   = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

                // For more information about this error, try `rustc --explain E0277`.
                // error: could not compile `error-handling` due to previous error

                // you can fix this by changing the return type of the function to Result or Option
                // Option will return none if an error occurs, or some if everything runs ok
            }

            // this is an example of using the ? operator after next in this function
            pub fn last_char_of_first_line(text: &str) -> Option<char> {
                // it will return the last character of the first line of a given text
                return text.lines().next()?.chars().last();

                // This function returns Option<char> because it’s possible that there is a character there, but it’s also 
                // possible that there isn’t. This code takes the text string slice argument and calls the lines method on it, 
                // which returns an iterator over the lines in the string. Because this function wants to examine the first line,
                // it calls next on the iterator to get the first value from the iterator. If text is the empty string, this call 
                // to next will return None, in which case we use ? to stop and return None from last_char_of_first_line. If text
                // is not the empty string, next will return a Some value containing a string slice of the first line in text.
            }
         }
    }
}

use error_handling::recoverable_errors_with_result::{
    shortcuts_for_panic::{
        shortcuts_for_panic_on_Error_with_expect, shortcuts_for_panic_on_Error_with_unwrap,
    },
    *,
};
use error_handling::unrecoverable_errors_with_panic::*;

fn main() {
    // errors_with_panic();
    // panic_backtrace();
    recoverable_errors_with_result();
}
