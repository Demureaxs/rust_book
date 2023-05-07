pub mod generics_traits_and_lifetimes {
    pub mod generics {
        pub mod removing_duplication_by_extracting_a_function {
            // an example of refactoring is the following functions
            pub fn largest_number() {
                let number_list = vec![32, 50, 25, 100, 65];

                let mut largest = &number_list[0];

                for number in &number_list {
                    if number > largest {
                        largest = number;
                    }
                }

                println!("The largest number is {}", largest);
            }

            pub fn largest_number_2() {
                let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

                let mut largest = &number_list[0];

                for number in &number_list {
                    if number > largest {
                        largest = number;
                    }
                }

                println!("The largest number in 2 is {}", largest);
            }

            // this function allows us to pass any list of numbers and will return the largest
            pub fn largest_refactored(list: &[i32]) -> &i32 {
                let mut largest = &list[0];

                for item in list {
                    if item > largest {
                        largest = item;
                    }
                }
                return largest;
            }

            pub fn using_largest_number_refactored() {
                let number_list = vec![34, 50, 25, 100, 65];

                let result = largest_refactored(&number_list);
                println!("The largest number from vector a is {:?}", result);

                let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

                let result = largest_refactored(&number_list);
                println!("The largest number from vector b is {:?}", result);
            }
        }
        pub mod generic_data_types {
            pub fn largest_i32(list: &[i32]) -> &i32 {
                let mut largest = &list[0];

                for item in list {
                    if item > largest {
                        largest = item;
                    }
                }

                return largest;
            }

            pub fn largest_char(list: &[char]) -> &char {
                let mut largest = &list[0];

                for item in list {
                    if item > largest {
                        largest = item;
                    }
                }

                return largest;
            }

            pub fn use_largest_char_i32_example() {
                let number_list = vec![34, 50, 25, 100, 65];

                let result_num = largest_i32(&number_list);
                println!("The largest number is: {}", result_num);

                let char_list = vec!['y', 'm', 'a', 'q'];
                let result_char = largest_char(&char_list);
                let charb = &char_list[0];

                let charb = char_list[0];
                println!("The largest character is: {}", result_char);
            }

            pub mod generic_refactoring {
                // note that here we have to declare the generic before the parens and after the
                // function declaration to be able to use it
                // pub fn largest_anything<T>(list: &[T]) -> &[T] {
                //     let mut largest = &list[0];

                // for item in list {
                //     // cannot perform a binary operation on T because
                //     // no methods or functions are known to the compiler
                //     if item > largest {
                //         largest = item;
                //     }
                // }

                // // causes an error here too.
                // return largest;
                // }

                // structs can hold generics so that their types can be allocated at the time of creation
                pub mod struct_generics {
                    // both parameters here have the same type.
                    pub struct Point<T> {
                        x: T,
                        y: T,
                    }

                    pub fn struct_generic_ex() {
                        // this works as both are integers
                        let integer = Point { x: 5, y: 10 };
                        // again works as both are floats
                        let float = Point { x: 1.0, y: 4.0 };
                    }

                    pub fn incorrect_struct_generic_ex() {
                        // wont compile as x defines the generic T as an integer then invalidates
                        // y which is a float
                        // let wont_work = Point { x: 4, y: 5.0 };
                    }

                    // this will allow for 2 types to be dynamically allocated. They can be anything
                    pub struct PointMultiType<T, U> {
                        x: T,
                        y: U,
                    }

                    pub fn multi_type_struct_generics() {
                        // as both integers it works fine
                        let both_integer = PointMultiType { x: 5, y: 10 };
                        // as both floats it also works fine
                        let both_float = PointMultiType { x: 1.0, y: 2.1 };

                        // we can even mix floats and integers
                        let int_and_float = PointMultiType { x: 4, y: 8.3 };
                    }
                }

                pub mod enum_definitions {
                    // as with structs, enums can and do use generics
                    // here the basic option enum
                    enum Option<T> {
                        Some(T),
                        None,
                    }

                    // and the result enum
                    enum Result<T, E> {
                        Ok(T),
                        Err(E),
                    }
                }

                pub mod method_definitions {
                    struct Point<T> {
                        x: T,
                        y: T,
                    }
                    // we can call generics on implementations or associated functions for structs
                    // this implements a generic getter function
                    // we have to declare <T> after the impl to allow us to use it
                    impl<T> Point<T> {
                        fn example_function(&self) -> &T {
                            // returns x from Point
                            return &self.x;
                        }
                    }

                    // function to demonstrate use
                    pub fn methods_in_action() {
                        // creation of a new point assigned to the variable point
                        let point = Point { x: 5, y: 10 };

                        // printline call to the getter and return the value of x
                        println!("p.x = {}", point.example_function()); // returns 5
                    }

                    // we can also use different generic names for methods on structs
                    // they do not have to be the same as the generics used in the structs
                    // properties... Eg.

                    struct PointB<X1, Y1> {
                        x: X1,
                        y: Y1,
                    }

                    // we can mix generics so long as the relevent generic and their processes match
                    // must call the Generics before the struct to enable use of generics
                    impl<X1, Y1> PointB<X1, Y1> {
                        // function that takes the PointB struct and another PointB struct
                        fn mixup<X2, Y2>(self, other: PointB<X2, Y2>) -> PointB<X1, Y2> {
                            // returns a new PointB struct with its own x and others y values
                            PointB {
                                x: self.x,
                                y: other.y,
                            }
                        }
                    }
                }
            }
        }
        pub mod traits_defining_shared_behavior {
            pub mod defining_a_trait {
                // a declaration of a trait
                pub trait Summary {
                    // any methods defined in the trait end with a semicolon
                    // the caller is responsible for the body of the method
                    fn summarize(&self) -> String;
                }
            }

            pub mod implementing_a_trait_on_a_type {
                use std::fmt::format;

                // declaration of a trait called summerize
                pub trait Summary {
                    // referes to self which is whatever the trait is called on
                    fn summarize(&self) -> String;
                }

                // declaration of a struct called NewsArticle
                pub struct NewsArticle {
                    pub headline: String,
                    pub location: String,
                    pub author: String,
                    pub content: String,
                }

                // here we implement Summary for NewsArticle
                // note the use of the for keyword, this attaches the trait to the structu
                impl Summary for NewsArticle {
                    // we then add the summarize function to the trait
                    fn summarize(&self) -> String {
                        // and define the behaviour inside of the code body
                        format!("{}, by {} ({})", self.headline, self.author, self.location)
                    }
                }

                // this is the same as the implementation above just using the fields in the struct
                // which are different.
                pub struct Tweet {
                    pub username: String,
                    pub content: String,
                    pub reply: bool,
                    pub retweet: bool,
                }

                impl Summary for Tweet {
                    fn summarize(&self) -> String {
                        format!("{}: {}", self.username, self.content)
                    }
                }

                // example of calling the trait on a Tweet struct
                pub fn summarize_tweet() {
                    let tweet = Tweet {
                        username: "Max Cock".to_string(),
                        content: "I am some content".to_string(),
                        reply: false,
                        retweet: false,
                    };

                    println!("1 new tweet: {}", tweet.summarize())
                    // prints "1 new tweet: Max Cock: I am some content"
                }
            }

            pub mod default_implementations {
                // if we do not override the default summarize method then
                // it will print read more...
                pub trait Summary {
                    fn summarize(&self) -> String {
                        "(Read More...)".to_string()
                    }
                }
                // defines the struct for a news article
                pub struct NewsArticle {
                    headline: String,
                    location: String,
                    author: String,
                    content: String,
                }

                // as we have no custom implementation the Summary traits default implementation will occur
                impl Summary for NewsArticle {}

                pub fn implementing_default_trait() {
                    // creates a new article from the NewsArticle Struct
                    let article = NewsArticle {
                        headline: String::from("Penguins win the Stanley Cup Championship!"),
                        location: String::from("Pittsburgh, PA, USA"),
                        author: String::from("Iceburgh"),
                        content: String::from(
                            "The Pittsburgh Penguins once again are the best \
                            hockey team in the NHL",
                        ),
                    };
                    // Prints the line
                    println!("New article available: {}", article.summarize());
                    // output = New article available: (Read More...)
                }

                pub mod extended_calling_other_methods_in_traits {
                    pub trait Summary {
                        fn summerize_author(&self) -> String;
                        fn summarize(&self) -> String {
                            format!("(Read more from {}...)", self.summerize_author())
                        }
                    }

                    pub struct Tweet {
                        username: String,
                        content: String,
                        reply: bool,
                        retweet: bool,
                    }

                    impl Summary for Tweet {
                        fn summerize_author(&self) -> String {
                            format!("@{}", self.username)
                        }
                    }

                    pub fn extended_calling_traits() {
                        let tweet = Tweet {
                            username: String::from("Horse_ebooks"),
                            content: String::from("Of course, as you probably already know people"),
                            reply: false,
                            retweet: false,
                        };

                        println!("1 new tweet: {}", tweet.summarize());
                    }
                }
            }
        }
    }
}

use std::vec;

use generics_traits_and_lifetimes::generics::generic_data_types::*;
use generics_traits_and_lifetimes::generics::removing_duplication_by_extracting_a_function::{
    largest_number, largest_number_2, using_largest_number_refactored,
};
use generics_traits_and_lifetimes::*;
// use generics_traits_and_lifetimes::generics::traits_defining_shared_behavior::implementing_a_trait_on_a_type::{summarize_tweet};
use generics_traits_and_lifetimes::generics::traits_defining_shared_behavior::default_implementations::{implementing_default_trait};
use generics_traits_and_lifetimes::generics::traits_defining_shared_behavior::default_implementations::extended_calling_other_methods_in_traits::{extended_calling_traits};

fn solution(word: &str, ending: &str) -> bool {
    let word_length = word.len();
    let start_point = &word.len() - &ending.len();
    let word_to_match = &word[start_point..word_length];
    println!("{}", word_to_match == ending);
    word_to_match == ending
}

fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let mut multiples = Vec::new();
    for num in 0..=limit {
        if num % n == 0 {
            multiples.push(num);
        }
    }
    return multiples;
}

// fn stray(arr: &[u32]) -> u32 {
//     let mut candidate1 = 0;
//     let mut candidate2 = 0;
//     let mut count1 = 0;
//     let mut count2 = 0;

//     for num in arr {
//         if *num == candidate1 {
//             count1 += 1;
//         } else if *num == candidate2 {
//             count2 += 1;
//         } else {
//             if count1 == 0 {
//                 candidate1 = *num;
//                 count1 = 1;
//             } else if count2 == 0 {
//                 candidate2 = *num;
//                 count2 = 1;
//             } else {
//                 count1 -= 1;
//                 count2 -= 1;
//             }
//         }
//     }

//     if count1 == 1 {
//         println!("{}", candidate1);
//         candidate1
//     } else {
//         println!("{}", candidate2);
//         candidate2
//     }
// }

fn stray(arr: &[u32]) -> u32 {
    // the fold method on iter takes a starting value and a closure that will use the bitwise
    // XOR operator to return a value if it is unique.
    let unique_num = arr.iter().fold(0, |acc, el| acc ^ el);

    println!("{}", unique_num);
    return unique_num;
}

fn main() {
    // largest_number();
    // largest_number_2();
    // using_largest_number_refactored();
    // use_largest_char_i32_example();
    // summarize_tweet();
    // solution("abc", "c");
    // solution("strawberry", "bannana");
    // solution("Hi there", "ere");
    // implementing_default_trait();
    // extended_calling_traits();

    // find_multiples(2, 2000);
    let vector = vec![0, 0, 0, 0, 1, 0];

    let vecb = vec![3, 2, 2, 2, 2];

    stray(&vector);
    stray(&vecb);
}
