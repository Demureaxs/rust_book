use codewars1::bartender_drinks::run_bartender_drinks;
use codewars1::check_the_exam::run_check_exam;
use codewars1::difference_of_volumes_cuboids::run_find_difference;
use codewars1::find_unique_value::run_stray;
use codewars1::will_there_be_enough_space::run_enough;

pub mod codewars1 {

    pub mod find_unique_value {
        // function to return a unique value in an array.

        pub fn run_stray() {
            let vector = vec![0, 0, 0, 0, 1, 0];
            let vecb = vec![3, 2, 2, 2, 2];

            stray(&vector);
            stray(&vecb);
        }

        pub fn stray(arr: &[u32]) -> u32 {
            // the fold method on iter takes a starting value and a closure that will use the bitwise
            // XOR operator to return a value if it is unique.
            let unique_num = arr.iter().fold(0, |acc, el| acc ^ el);

            println!("{}", unique_num);
            return unique_num;
        }
    }

    pub mod check_the_exam {
        // function to compare answers to student answers, +4 if correct -1 if incorrect
        // and +0 if a blank answer

        pub fn run_check_exam() {
            check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]);
            check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]);
            check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]);
            check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]);
        }

        fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
            let mut score: i64 = 0;

            for (index, element) in arr_a.iter().enumerate() {
                if element == &arr_b[index] {
                    score += 4;
                } else if element != &arr_b[index] && arr_b[index] != "" {
                    score -= 1;
                }
            }
            if score >= 0 {
                println!("{}", score);
                return score;
            } else {
                println!("{}", 0);
                return 0;
            }
        }

        use std::cmp::max;
        // this version makes use of the max function in the std library to reduce if statements
        fn check_exam_from_answers(arr_a: &[&str], arr_b: &[&str]) -> i64 {
            let mut score: i64 = 0;

            for (index, element) in arr_a.iter().enumerate() {
                if element == &arr_b[index] {
                    score += 4;
                } else if element != &arr_b[index] && arr_b[index] != "" {
                    score -= 1;
                }
            }
            return max(score, 0);
        }
    }

    pub mod bartender_drinks {
        pub fn run_bartender_drinks() {
            get_drink_by_profession("jabrOni");
            get_drink_by_profession("scHOOl counselor");
            get_drink_by_profession("prOgramMer");
            get_drink_by_profession("bike ganG member");
            get_drink_by_profession("poLiTiCian");
            get_drink_by_profession("rapper");
            get_drink_by_profession("pundit");
            get_drink_by_profession("Pug");
        }

        fn get_drink_by_profession(param: &str) -> &'static str {
            // using match as an expression to return the static string
            // based on matching the lowercase input
            match param.to_lowercase().as_str() {
                "jabroni" => "Patron Tequila",
                "school counselor" => "Anything with Alcohol",
                "programmer" => "Hipster Craft Beer",
                "bike gang member" => "Moonshine",
                "politician" => "Your tax dollars",
                "rapper" => "Cristal",
                _ => "Beer",
            }
        }
    }

    pub mod will_there_be_enough_space {
        pub fn run_enough() {
            enough(10, 5, 5);
            enough(100, 60, 50);
            enough(20, 5, 5);
        }

        fn enough(cap: i32, on: i32, wait: i32) -> i32 {
            if cap - on >= wait {
                println!("{}", 0);
                return 0;
            } else {
                println!("{}", wait - (cap - on));
                return wait - (cap - on);
            }
        }
    }

    pub mod difference_of_volumes_cuboids {

        pub fn run_find_difference() {
            find_difference(&[3, 2, 5], &[1, 4, 4]);
            find_difference(&[9, 7, 2], &[5, 2, 2]);
            find_difference(&[11, 2, 5], &[1, 10, 8]);
            find_difference(&[4, 4, 7], &[3, 9, 3]);
            find_difference(&[15, 20, 25], &[10, 30, 25]);
            find_difference(&[12, 28, 10], &[25, 11, 16]);
        }

        fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
            let difference =
                a.iter().fold(1, |acc, cv| acc * cv) - b.iter().fold(1, |acc, cv| acc * cv);
            println!("{}", difference.abs());
            return difference.abs();
        }

        fn codewars_find_difference() {
            fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
                a.iter()
                    .fold(1, |acc, cv| acc * cv)
                    // abs_diff returns the absolute difference between what it is called on and what is 
                    // called inside
                    .abs_diff(b.iter().fold(1, |acc, cv| acc * cv)) as i32
            }
        }
    }
}

fn main() {
    // run_stray();
    // run_check_exam();
    // run_bartender_drinks();
    // run_enough();
    run_find_difference();
}
