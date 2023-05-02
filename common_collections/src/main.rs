//  ----------------------------------------------------------------
// CHAPER 8 COMMON COLLECTIONS

// ----------------------------------------------------------------
// VECTORS
// vectors can be dynamic in size and feature a host of functions
// and methods that can be called to manipulate them.
pub mod vectors {
    pub mod create_new_vector {
        use std::{fmt::Debug, ops::Range, slice::Iter, vec};

        pub fn vectors_a() {
            // declaring an empty vector requires us to annotate type
            let mut vector: Vec<i32> = Vec::new();

            // adding items to a vector
            // adds 22
            vector.push(22);
            // adds 77
            vector.push(77);

            // declaring a vector with the vec! macro allows for type inference
            let mut inferred_vector = vec![1, 2, 3, 4, 5];

            // pushing items to infer vector
            // push the number 6
            inferred_vector.push(6);
            // pushes the number 7
            inferred_vector.push(7);

            // reading elements of vectors
            // mmethod one
            let third: &i32 = &inferred_vector[2];
            println!("The third element is {}", third);

            // uses Option to check if the 2 index exists and handles the error incase it doesnt.
            let third_option: Option<&i32> = inferred_vector.get(2);
            match third_option {
                // if index 2 exists print the line
                Some(option) => println!("The third option is {}", option),
                // otherwise print there is no third option
                None => println!("There is no third option"),
            }

            // packicks
            let mut vector_access = vec![1, 2, 3, 4, 5];
            // directly accessing the vector outside the range will cause
            // a panic with bracket notation
            let mut _does_not_match = &vector_access[200];
            println!("{:#?}", vector_access);

            // get returns either some or none but doesnt panic
            let does_not_match = vector_access.get(200);
            match does_not_match {
                Some(num) => println!("The value is {}", num),
                None => (),
            }
            vector_access[2] = 22;

            let mut v = vec![1, 2, 3, 4, 5];
            let first = &v[0];

            // gives an error as its editing while first reference is still in scope
            // can go after the println
            // v.push(6);

            println!("The first element is: {}", first);
            v.push(6);
        }
        pub fn iterating_vectors() {
            // creates a vector
            let mut vector = vec![1, 2, 3, 3, 4, 5];
            // creates a temporary vector
            let mut temp: Vec<i32> = Vec::new();
            // loop through a mutable reference to the vector using enumerate
            // for (_index, n_ref) in &mut vector.iter().enumerate().rev() {

            // option one, iterate through and increase each value in the original vector by 50
            for (n_ref) in &mut vector {
                // assign the variable to the value of dereferencing(accessing) the space in memory and update
                // it by adding 1
                let n_plus_one: i32 = *n_ref + 1;
                // push the original but plus one to the temp vector
                temp.push(n_plus_one);
                // add the 50 to the current number
                *n_ref += 50;
            }

            vector[0] = 22;
            println!("Temp is {:?}", temp);
            println!("Vector is {:?}", vector);
        }
        pub fn safety_using_iterators() {
            // what happpens behind the scenes with iterators in rust...
            let vector = vec![1, 2];
            // passes ownership of vector to iter..
            let mut iter: Iter<i32> = vector.iter();
            // passes moving to the next iteration to n1 by reference
            let n1: &i32 = iter.next().unwrap();
            // same as above but n2
            let n2: &i32 = iter.next().unwrap();
            // end of the loop called
            let end: Option<&i32> = iter.next();
            // prints each stage
            println!(
                "vector: {:?}, iter {:?}, n1: {n1} n2: {n2}, {:#?}",
                vector, iter, end
            );

            let mut vector: Vec<i32> = vec![1, 2];
            let mut iter: Range<usize> = 0..vector.len();
            let il: usize = iter.next().unwrap();
            let n1: &i32 = &vector[il];
        }
        // this wont allow us to duplicate
        // pub fn duplicate_in_place(v: &mut Vec<i32>) {
        //     for n_ref in &mut v.iter() {
        //         v.push(*n_ref);
        //     }
        // }
        pub fn using_enum_to_store_multiple_types() {
            // creating an enum can enable us to hold multiple types in
            // a vector
            #[derive(Debug)]
            pub enum SpreadsheetCell {
                Int(i32),
                Float(f64),
                String(String),
            }

            // creating a row vector that holds the spreadsheet cells
            let row = vec![
                SpreadsheetCell::Int(3),
                SpreadsheetCell::Float(10.12),
                SpreadsheetCell::String(String::from("Hello World")),
            ];

            // prints [int(3), float(10.12), String("Hello World")]
            println!("{:?}", row);
        }
    }
}

// use vectors::create_new_vector::{
//     iterating_vectors, safety_using_iterators, using_enum_to_store_multiple_types, vectors_a,
// };

// fn main() {
//     vectors_a();
//     iterating_vectors();
//     safety_using_iterators();
//     using_enum_to_store_multiple_types();
// }
// ----------------------------------------------------------------

// STORING UTF-8 ENCODED TEXT WITH STRINGS
// vectors and strings are essentially the same. A string is a variant
// of a vector with added limitations and restrictions

pub mod strings {
    pub fn create_a_string() {
        // creates a new string using the new function (also in vectors)
        let mut string = String::new();

        // the to string method
        let data = "initial comments";
        // uses data with the .to_string() method to create a new string
        let string_from_data = data.to_string();
        // uses a literal string with the .to_string() method to create a new string
        let string_direct_from_literal = "initial Contents".to_string();

        // String::from function.
        let string_from = String::from("initial comments");
    }
    pub fn update_a_string() {
        // pushes bar in to the string variable containing foo
        let mut string = String::from("foo");
        string.push_str("bar");

        // can also be rewritten as
        let mut string_b = String::from("foo");
        let string_to_append = "bar";
        // string.push_str doesn't take ownership of string_to_append
        // so we can still access it in the following line
        string_b.push_str(string_to_append);
        println!("The item to be appeneded is: {}", string_to_append);

        // pushing a character to a string with .push
        let mut string_c = String::from("lo");
        // noted that characters are signified with single quotes
        string_c.push('l');
    }
    pub fn concatenate_a_string() {
        // We have a few options for string contatenation
        let string_1 = String::from("Hello, ");
        let string_2 = String::from("world!");
        // note the referencing to string 2, if its not referenced we get a compile error
        // the reason for this is in line 184
        let string_concatenated = string_1 + &string_2;
        // string_concatenated uses this expression under the hood
        // fn add(self, string: &str) -> String {} // this is the call when we use the '+' operator
        println!("{}", string_concatenated);
        // important to noted that although  string_concatenated = string_1 + &string_2 looks
        // like it will copy both strings and create a new one, this statement instead does the following:

        // add takes ownership of s1,
        // it appends a copy of the contents of s2 to s1,
        // and then it returns back ownership of s1.

        // if string_1 has enough capacity for string_2 then no memory allocations occur,
        // However, if string_1 does not have enough capacity for s2, then s1 will internally make a
        // larger memory allocation to fit both strings

        // ----------------------------------------------
        let string_1b = String::from("tic");
        let string_2b = String::from("tac");
        let string_3b = String::from("toe");

        // this method can be quite verbose and difficult to read
        let full_string = string_1b + "-" + &string_2b + "-" + &string_3b;
        println!("{}", full_string);

        // note if we didnt add the redeclaration via shaddowing, we
        // would recieve an error that string_1 has been moved to full_string in line 204
        let string_1b = String::from("tic");
        let string_2b = String::from("tac");
        let string_3b = String::from("toe");
        // using to format macro significantly simplifies the process
        let format_string = format!("{}-{}-{}", string_1b, string_2b, string_3b);
        println!("{}", format_string);
    }
    pub fn indexing_into_strings() {
        // the following is not valid in rust, as is in other languages
        // this is because strings do not support indexing

        // let string_1 = String::from("hello");
        // let letter_h = string_1[0];

        // a string is simply a wrapper over Vec<u8>
        let hello = String::from("hola");

        let hello_cyrillic = String::from("Здравствуйте");
        // rust provides 3 ways to look at strings, as bytes, as scalar values, and grapheme clusters.
    }
    pub fn slicing_strings() {
        // it is really important to note that strings cannot be indexed into
        // in the same way that a vector can, this is because they are either bytes(one letter can be more than 1)
        // scalar values(which is the equivalent of of rusts Char type) but often have characters from outside the word
        // grapheme clusters which actually give us the 4 words.

        // ---------------------------------------
        // example of slicing into a string.
        let hello = "Здравствуйте";
        let letter = &hello[0..4]; // we have to use a range of bytes
                                   // if we dont get an exact amount of bytes we will get a panic.
        println!("{letter}"); // prints Зд

        // ---------------------------------------
    }
    pub fn iterating_strings() {
        //The best way to be operate on strings is to be specific about whether you want
        // characters or bytes.

        let mut string = "Hello World".to_string();

        // using the chars method.
        for char in string.chars() {
            println!("{}", char); // prints hello world virticallyy
        }

        // using the bytes method.
        for char in string.bytes() {
            println!("{}", char); // prints the bytes virtically
        }

        // remember that valid unicode characters are often made up of more than one byte and therefore
        // we can case errors if we try to index into them.
    }
}

// use strings::{
//     concatenate_a_string, create_a_string, iterating_strings, slicing_strings, update_a_string,
// };
// fn main() {
//     // create_a_string();
//     // update_a_string();
//     // slicing_strings();
//     iterating_strings();
// }

// ----------------------------------------------------------------
// STORING KEYS WITH ASSOCIATED VALUES IN HASH MAPS

// hashmaps are useful for storing key value pairs and are similar to objects and maps in JS
// just like in hashmaps data stored in them is allocated to the heap.
// they are not as commonly used so the support for them is quite limited and they are
// not included in the default scope.

pub mod hashmaps {
    // we have to use in the hashmap from the std lib
    use std::collections::HashMap;
    pub fn create_a_new_hashmap() {
        // then create a new hashmap(note: it takes 2 arguments one for key and one for value)
        // types must be uniform after declaration so once declared it can only store data of
        // the same types i.e string, i32 in this case
        let mut scores = HashMap::new();
        // insert is a method called on a hashmap to insert a new value
        // inserts john, 10
        scores.insert("john".to_string(), 10);
        // inserts mike, 22
        scores.insert("mike".to_string(), 22);

        println!("The hashmap contains {:#?}", scores); // prints:
                                                        //  The hashmap contains {
                                                        // "john": 10,
                                                        // "mike": 22
                                                        //  }
    }
    pub fn accessing_values_in_hashmap() {
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);

        // declares the name of the team we want as a string assigned to team_name
        let team_name = String::from("Blue");
        // assigns the output of scores.get(referencing team_name).copied().unwrap_or()
        // in this instance .copied.unwrap_or() are the same as using an Option<i32> but is less verbose
        // if something is returned it will be returned or if none is found it will return 0
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("{team_name}'s score is {:?}", score); // prints blues score is 10
    }
    pub fn iterating_hashmaps() {
        // creates a HashMap called scores
        let mut scores = HashMap::new();
        // inserts the values "Blue".to_string(), 10
        scores.insert("Blue".to_string(), 10);
        // inserts the values "Yellow".to_string(), 50
        scores.insert("Yellow".to_string(), 50);

        // loops through the reference to scores
        for (key, value) in &scores {
            // prints the key and value from the scores
            println!("{}:{}", key, value)
        }
    }
    pub fn hashmaps_and_ownership() {
        // note that these are similar to js objects with the restriction that they cannot have dynamic types
        // for types that implement the copy trait (like i32), the values are copied into the hashmap,
        // for owned types like String, the values will be moved and the hashmap will be the owner of those values
        // eg
        let field_name = "Favorite color".to_string();
        let field_value = "Blue".to_string();

        // passes ownership to hashmap of fieldname and value
        let mut hashmap = HashMap::new();
        hashmap.insert(field_name, field_value);

        // this wont work as fieldname and field value have been moved to the hashmap
        // println!("Key: {}, Value: {}", field_name, field_value);

        // this must be used instead
        println!("{:#?}", hashmap); // prints { "favorite color": "Blue" }

        // if we move references to value into the hashmap the values wont be moved into the hashmap
        // the values that the references point to must be valid for at least as long as the hash map is valid....
        // This is very important. Lifetimes of references must exceed the lifetime of the hashmap.
    }
    pub fn updating_a_hashmap() {
        // there are 4 approaches to updating a hashmap

        // overwriting a value
        let mut scores = HashMap::new();
        // inserts an original value into the hashmap
        scores.insert("Blue".to_string(), 10);
        // overwrites the value(10) to the value (25)
        scores.insert("Blue".to_string(), 25);

        println!("{:?}", scores); // prints { "Blue": 25 }
    }
    pub fn add_key_value_if_not_present() {
        // using the entry keyword is a litttle like checking if key exists do nothing else create a new entry

        // creates a new hashmap (empty)
        let mut scores = HashMap::new();
        // inserts Blue(to string), with a value of 10(i32)
        scores.insert("Blue".to_string(), 10);

        // checks if scores has an entry yellow... if so do nothing
        // if not create a value yellow with the value 50(i32)
        // or insert is if None
        scores.entry("Yellow".to_string()).or_insert(50);

        // same as above checks if the key exists, if it does do nothing.
        // otherwise create it with the value of 50
        // or insert is if None
        scores.entry("Blue".to_string()).or_insert(50);

        println!("Scores: {:?}", scores) // prints { "Blue": 10, "Yellow": 50 }
    }
    pub fn update_value_based_on_old_value() {
        // declares a string slice with the value to be used
        let text = "hello world wonderful world";

        // creates an empty hashmap
        let mut hashmap = HashMap::new();
        // loops through the text variable(splitting at whitespace into a vector)
        for word in text.split_whitespace() {
            // creates a variable called count and stores the entry to it.
            // if there is no count it will create an entry from the word, then give it 0 to start with
            // if there is a count it stores that entry in count
            let count = hashmap.entry(word).or_insert(0);
            // updates the count (and the hashmap entry by 1)
            *count += 1;

            // this means that it will add one to the current count of existing entries,
            // of create a new entry with a count of one.
        }

        println!("Hashmap: {:?}", hashmap); // prints Hashmap: {"world": 2, "wonderful": 1, "hello": 1}
    }
}

// use hashmaps::{
//     accessing_values_in_hashmap, add_key_value_if_not_present, create_a_new_hashmap,
//     hashmaps_and_ownership, iterating_hashmaps, update_value_based_on_old_value,
//     updating_a_hashmap,
// };
// fn main() {
//     // create_a_new_hashmap();
//     // accessing_values_in_hashmap();
//     // iterating_hashmaps();
//     // hashmaps_and_ownership();
//     // updating_a_hashmap();
//     // add_key_value_if_not_present();
//     // update_value_based_on_old_value();
// }