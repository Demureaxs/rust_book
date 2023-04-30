// a vector is a type that can hold a list of the same type
// all collections are stored on the heap.
// places all values next to each other in the heap(memory)
pub mod vectors {
    pub mod create_new_vector {
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

            let third_option: Option<&i32> = inferred_vector.get(2);
            match third_option {
                Some(option) => println!("The third option is {}", option),
                None => println!("There is no third option"),
            }
        }
    }
}

fn main() {}
