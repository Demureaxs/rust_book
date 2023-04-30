// a vector is a type that can hold a list of the same type
// all collections are stored on the heap.
// places all values next to each other in the heap(memory)
pub mod vectors {
    pub mod create_new_vector {
        use std::{fmt::Debug, ops::Range, slice::Iter, vec};

        pub fn vectors_a() {
            // declaring an empty vector requires us to annotate type
            let mut vector: Vec<i32> = Vec::new();

            // adding items to a vector
            vector.push(22);
            vector.push(77);

            // declaring a vector with the vec! macro allows for type inference
            let mut inferred_vector = vec![1, 2, 3, 4, 5];

            // pushing items to infer vector
            inferred_vector.push(6);
            inferred_vector.push(7);

            // reading elements of vectors
            // mmethod one
            let third: &i32 = &inferred_vector[2];
            println!("The third element is {}", third);

            // uses Option to check if the 2 index exists and handles the error incase it doesnt.
            let third_option: Option<&i32> = inferred_vector.get(2);
            match third_option {
                Some(option) => println!("The third option is {}", option),
                None => println!("There is no third option"),
            }

            let mut vector_access = vec![1, 2, 3, 4, 5];
            // directly accessing the vector outside the range will cause
            // a panic with bracket notation
            let mut _does_not_match = &vector_access[2];

            vector_access[2] = 22;

            println!("{:#?}", vector_access);
            // get returns either some or none but doesnt panic
            let does_not_match = vector_access.get(200);
            match does_not_match {
                Some(num) => println!("The value is {}", num),
                None => (),
            }

            let mut v = vec![1, 2, 3, 4, 5];
            let first = &v[0];

            // gives an error as its editing while first reference is still in scope
            // can go after the println
            // v.push(6);

            println!("The first element is: {}", first);
            v.push(6);
        }
        pub fn iterating_vectors() {
            let mut vector = vec![1, 2, 3, 3, 4, 5];
            let mut temp: Vec<i32> = Vec::new();
            // loop through a mutable reference to the vector using enumerate
            // for (_index, n_ref) in &mut vector.iter().enumerate().rev() {

            for (n_ref) in &mut vector {
                let n_plus_one: i32 = *n_ref + 1;
                // println!("{}", n_plus_one);
                temp.push(n_plus_one);
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
        pub fn using_enum_to_store_multiple_types() {}
    }
}

use vectors::create_new_vector::{
    iterating_vectors, safety_using_iterators, using_enum_to_store_multiple_types, vectors_a,
};

fn main() {
    // vectors_a();
    // iterating_vectors();
    // safety_using_iterators();
    using_enum_to_store_multiple_types();
}
