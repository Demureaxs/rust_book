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
                println!("The largest character is: {}", result_char);
            }

            pub mod generic_refactoring {
                // note that here we have to declare the generic before the parens and after the
                // function declaration to be able to use it
                pub fn largest_anything<T>(list: &[T]) -> &[T] {
                    let mut largest = &list[0];

                for item in list {
                    // cannot perform a binary operation on T because
                    // no methods or functions are known to the compiler
                    if item > largest {
                        largest = item;
                    }
                }

                // causes an error here too.
                return largest;
                }

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
    }
}

use generics_traits_and_lifetimes::generics::removing_duplication_by_extracting_a_function::{
    largest_number, largest_number_2, using_largest_number_refactored,
};

use generics_traits_and_lifetimes::generics::generic_data_types::*;

fn main() {
    // largest_number();
    // largest_number_2();
    // using_largest_number_refactored();
    use_largest_char_i32_example();
}
