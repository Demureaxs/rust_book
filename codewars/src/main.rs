// use codewars1::bartender_drinks::run_bartender_drinks;
// use codewars1::check_the_exam::run_check_exam;
// use codewars1::difference_of_volumes_cuboids::run_find_difference;
// use codewars1::find_unique_value::run_stray;
// use codewars1::will_there_be_enough_space::run_enough;

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

pub mod codewars2 {
    pub mod expressions_matter {
        pub fn run_expressions_matter() {
            expressions_matter(2, 1, 2);
            expressions_matter(1, 1, 1);
            expressions_matter(2, 1, 1);
            expressions_matter(1, 2, 3);
            expressions_matter(1, 3, 1);
        }

        fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
            let combination1 = (a + b) * c;
            let combination2 = a + b + c;
            let combination3 = a * (b + c);
            let combination4 = a * b * c;
            let combination5 = a + b * c;

            let highest = combination1
                .max(combination2.max(combination3.max(combination4.max(combination5))));
            println!("{}", highest);
            return highest;
        }
    }

    pub mod how_many_lightsabers {

        pub fn run_lightsabers() {
            how_many_lightsabers_do_you_own("");
            how_many_lightsabers_do_you_own("Adam");
            how_many_lightsabers_do_you_own("Zach");
            how_many_lightsabers_do_you_own("zach");
        }

        fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
            match name {
                "Zach" => 18,
                _ => 0,
            }
        }
    }

    pub mod hello_name_or_world {
        pub fn run_hello() {
            hello("johN");
            hello("alice");
            hello("");
        }

        fn hello(name: &str) -> String {
            let mut greeting = String::from("Hello, ");
            if name.is_empty() {
                greeting.push_str("World!")
            } else {
                let (first, rest) = name.split_at(1);
                let cased = first.to_uppercase() + &rest.to_lowercase() + "!";
                greeting.push_str(&cased);
            }

            return greeting;
        }

        fn codewars_hello(name: &str) -> String {
            if name.is_empty() {
                return "Hello, World!".to_string();
            };
            format!(
                "Hello, {}{}!",
                name[..1].to_uppercase(),
                name[1..].to_lowercase()
            )
        }
    }

    pub mod holiday_8_duty_free {

        pub fn run_duty_free() {
            duty_free(12, 50, 1000);
            duty_free(17, 10, 500);
        }

        fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
            let discount_percent = discount as f32 / 100.0;
            let discount_per_bottle = price as f32 * discount_percent;
            let bottles_afforded = (holiday_cost as f32 / discount_per_bottle).floor() as i32;
            return bottles_afforded;
        }
    }

    pub mod sum_differences_in_array {

        pub fn run_sum_difference() {
            sum_of_differences(&[1, 2, 10]);
            sum_of_differences(&[-17, 17]);
            sum_of_differences(&[1, 1, 1, 1, 1]);
        }

        fn sum_of_differences(arr: &[i8]) -> Option<i8> {
            if arr.len() < 2 {
                return None;
            } else {
                let mut sorted_arr = arr.to_vec();
                sorted_arr.sort_by(|a, b| b.cmp(a));

                let mut sum = 0;
                for i in 0..sorted_arr.len() - 1 {
                    sum += sorted_arr[i] - sorted_arr[i + 1];
                }
                return Some(sum);
            }
        }
    }

    pub mod dna_to_rna {
        pub fn run_dna_to_rna() {
            cw_dna_to_rna("TTTT");
            dna_to_rna("GCAT");
        }

        fn dna_to_rna(dna: &str) -> String {
            let mut string = String::new();

            dna.chars().for_each(|c| {
                if c == 'T' {
                    string.push('U');
                } else {
                    string.push(c);
                }
            });
            println!("{}", string);
            return string;
        }

        fn cw_dna_to_rna(dna: &str) -> String {
            // more efficient and faster...
            return dna.replace("T", "U");
        }
    }

    pub mod flatten_and_sort {
        pub fn run_flatten_and_sort() {
            flatten_and_sort(&[vec![3, 2, 1], vec![7, 9, 8], vec![6, 4, 5]]);
            flatten_and_sort(&[vec![1, 3, 5], vec![100], vec![2, 4, 6]]);
        }

        fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
            let mut flat_arr: Vec<i32> = arr.concat();
            flat_arr.sort();
            return flat_arr;
        }
    }

    pub mod multiples_of_3_and_5 {
        pub fn run_solution() {
            solution(20);
            solution(40);
            solution(4);
        }

        fn solution(num: i32) -> i32 {
            // create a temporary array
            let mut temp: Vec<i32> = Vec::new();

            // loop for the range of the num and if its divisible by 3 or 5
            for index in 0..num {
                if index % 3 == 0 || index % 5 == 0 {
                    temp.push(index)
                }
            }

            let result = temp.iter().fold(0, |acc, cv| acc + cv);

            return result;
        }

        fn solution_cw2(num: i32) -> i32 {
            (3..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
        }
    }

    pub mod make_the_deadfish_swim {
        pub fn run_parse() {
            parse("iiisdoso");
            parse("iiisdosodddddiso");
        }

        fn parse(code: &str) -> Vec<i32> {
            let mut count = 0;
            let mut temp = Vec::new();

            code.chars()
                .into_iter()
                .for_each(|character| match character {
                    'i' => count += 1,
                    'd' => count -= 1,
                    's' => count = count * count,
                    'o' => temp.push(count),
                    _ => println!("Something went wrong"),
                });

            temp
        }
    }

    pub mod rectangles_into_squares {
        pub fn run_sq_in_rect() {
            sq_in_rect(5, 3); // should be vec![3,2,1,1]
            sq_in_rect(3, 5); // should be vec![3,2,1,1]
            sq_in_rect(5, 5); // should be none
        }

        fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
            if lng == wdth {
                return None;
            }
            let smallest_side = lng.min(wdth);
            let mut largest_side = lng.max(wdth);
            let mut remaining_area = smallest_side * largest_side;
            let mut squares: Vec<i32> = Vec::new();

            for num in (0..=smallest_side).rev() {
                let square_size = num;
                squares.push(square_size);
                let square_size_area = square_size * square_size;
                remaining_area -= square_size_area;
                largest_side -= smallest_side;

                if num * num == remaining_area {
                    squares.push(num);
                    remaining_area -= num;
                    break;
                }
            }

            if remaining_area > 0 {
                return None;
            }

            println!("{:?}", squares);
            Some(squares)
        }
    }
}

pub mod codewars3 {
    pub mod consecutive_strings {
        pub fn run_longest_consec() {
            longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2); // "abigailtheta"
            longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3);
            // "ixoyx3452zzzzzzzzzzzz"
            longest_consec(
                vec![
                    "ejjjjmmtthh",
                    "zxxuueeg",
                    "aanlljrrrxx",
                    "dqqqaaabbb",
                    "oocccffuucccjjjkkkjyyyeehh",
                ],
                1,
            );
            longest_consec(vec![], 3);
        }

        fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
            if strarr.len() == 0 || strarr.len() < k {
                return "".to_string();
            }

            // temporary longest concatenation
            let mut length = 0;

            let mut longest_concat = String::new();

            // loop through the n length -1
            for index in 0..=strarr.len() - k {
                let mut temp_longest = String::new();
                for j in index..index + k {
                    temp_longest.push_str(strarr[j]);
                }

                if temp_longest.len() > length {
                    longest_concat = temp_longest;
                    length = longest_concat.len();
                } else {
                    continue;
                }
            }
            println!("{}", longest_concat);
            return longest_concat;
        }
    }

    pub mod shifting_zeros {
        pub fn run_move_zeros() {
            move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]);
            move_zeros(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9]);
            move_zeros(&[0, 0]);
            move_zeros(&[0]);
            move_zeros(&[]);
        }

        fn move_zeros(arr: &[u8]) -> Vec<u8> {
            if arr.len() == 0 {
                return vec![];
            }

            let mut zero_vector: Vec<u8> = Vec::new();
            let mut non_zero_vector: Vec<u8> = Vec::new();

            for index in 0..arr.len() {
                if arr[index] == 0 {
                    zero_vector.push(arr[index]);
                } else {
                    non_zero_vector.push(arr[index]);
                }
            }

            non_zero_vector.append(&mut zero_vector);
            println!("{:?}", non_zero_vector);
            return non_zero_vector;
        }
    }

    pub mod multiplication_table {
        pub fn run_multiplication_table() {
            multiplication_table(3);
        }

        fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
            let mut output: Vec<Vec<usize>> = vec![];

            for num in 1..=len {
                let mut temp_vec: Vec<usize> = Vec::new();

                for num2 in 1..=len {
                    temp_vec.push(num * num2);
                }

                output.push(temp_vec);
            }
            // println!("{:?}", output);
            return output;
        }
    }

    pub mod give_me_a_diamond {
        pub fn run_print() {
            print(3);
            // print(5);
            // print(-3);
            // print(1);
        }

        fn print(n: i32) -> Option<String> {
            if n < 0 || n % 2 == 0 {
                return None;
            }

            let mut temp_string = String::new();

            for index in 1..=n {
                if index % 2 == 0 {
                    continue;
                }
                temp_string.push_str(&" ".repeat(((n - index) / 2) as usize));
                temp_string.push_str(&"*".repeat(index as usize));
                temp_string.push_str("\n");
            }

            for index in (1..=n - 2).rev() {
                if index % 2 == 0 {
                    continue;
                }
                temp_string.push_str(&" ".repeat(((n - index) / 2) as usize));
                temp_string.push_str(&"*".repeat(index as usize));
                temp_string.push_str("\n");
            }
            println!("{}", temp_string);
            Some(temp_string)
        }

        fn print_cw(n: i32) -> Option<String> {
            if n < 0 || n % 2 == 0 {
                return None;
            }

            let n = n as usize;
            let diamond = (1..=n)
                .chain((1..n).rev())
                .step_by(2)
                .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
                .collect();

            Some(diamond)
        }
    }

    pub mod write_number_in_expanded_form {
        pub fn run_expanded_form() {
            expanded_form(12);
            expanded_form(42);
            expanded_form(70304);
        }

        fn expanded_form(n: u64) -> String {
            let temp_string = n.to_string();
            let length = n.to_string().len();
            let mut output = String::new();

            for (index, num) in temp_string.chars().enumerate() {
                if num == '0' {
                    continue;
                } else {
                    output.push(num);
                    output.push_str(&"0".repeat(length - (index + 1)));
                    output.push_str(" + ");
                }
            }

            let (output2, _) = output.split_at(output.len() - 3);

            println!("{:?}", output2);
            return output2.to_string();
        }
    }

    pub mod rgb_to_hex_conversion {
        pub fn run_rgb() {
            rgb(0, 0, 0); // 000000
            rgb(1, 2, 3); // 010203
            rgb(255, 255, 255); // ffffff
            rgb(254, 253, 252); // fefdfc
            rgb(-20, 275, 125); // 00ff7d
            rgb(88, 27, 64);
        }

        fn rgb(r: i32, g: i32, b: i32) -> String {
            let hex_value = format!(
                "{:02X}{:02X}{:02X}",
                r.max(00).min(255),
                g.max(00).min(255),
                b.max(00).min(255)
            );
            println!("{}", hex_value);
            hex_value
        }
    }

    pub mod max_subarr_sum {
        pub fn run_max_subarr_sum() {
            max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]); // 6

            max_sequence(&[11]); // 11

            max_sequence(&[-32]); // 0
        }

        fn max_sequence(seq: &[i32]) -> i32 {
            let mut max_so_far = 0;
            let mut max_ending_here = 0;

            for num in seq {
                max_ending_here += num;
                if max_so_far < max_ending_here {
                    max_so_far = max_ending_here;
                }
                if max_ending_here < 0 {
                    max_ending_here = 0;
                }
            }
            return max_so_far;
        }

        fn max_sequence_cw_solution(seq: &[i32]) -> i32 {
            let mut max_sum = 0;
            let mut tmp_sum = 0;
            for x in seq {
                tmp_sum = (tmp_sum + x).max(0);
                max_sum = max_sum.max(tmp_sum);
            }
            max_sum
        }
    }

    pub mod sum_of_pairs {
        pub fn run_sum_of_pairs() {
            sum_pairs(&[1, 4, 8, 7, 3, 15], 8); // some(1 / 7)
        }
        use std::collections::HashSet;

        fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
            let mut seen = HashSet::new();

            for num in ints {
                let difference = s - *num;
                if seen.contains(&difference) {
                    return Some((difference, *num));
                }
                seen.insert(num);
            }

            None
        }
    }

    pub mod binary_multiple_of_3 {

        // fn binary_multiple_of_3() -> Regex {
        //     return "^(0*(1(01*0)1)0)$";
        // }
    }

    pub mod pyramid_slide {
        pub fn run_longest_slide() {
            longest_slide_down(&vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]]);

            // medium
            longest_slide_down(&vec![
                vec![75],
                vec![95, 64],
                vec![17, 47, 82],
                vec![18, 35, 87, 10],
                vec![20, 4, 82, 47, 65],
                vec![19, 1, 23, 75, 3, 34],
                vec![88, 2, 77, 73, 7, 63, 67],
                vec![99, 65, 4, 28, 6, 16, 70, 92],
                vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
                vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
                vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
                vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
                vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
                vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
            ]);
        }

        fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
            // get the total length of the pyramid (i.e. its height)
            let length = pyramid.len();
            // clone the sencond last pyramid 
            let mut max_sum = pyramid[length - 1].clone();


            // this line allows us to work up the pyramid
            for i in (0..length - 1).rev() {
                
                for j in 0..pyramid[i].len() {
                    let left_sum = max_sum[j];
                    let right_sum = max_sum[j + 1];
                    let current_sum = pyramid[i][j] + left_sum.max(right_sum);

                    max_sum[j] = current_sum;
                }
            }
            println!("{}", max_sum[0]);
            return max_sum[0];
        }
    }
}

// use codewars2::dna_to_rna::*;
// use codewars2::expressions_matter::*;
// use codewars2::flatten_and_sort::*;
// use codewars2::hello_name_or_world::*;
// use codewars2::holiday_8_duty_free::*;
// use codewars2::how_many_lightsabers::*;
// use codewars2::make_the_deadfish_swim::*;
// use codewars2::multiples_of_3_and_5::*;
// use codewars2::rectangles_into_squares::*;
// use codewars2::sum_differences_in_array::*;
// use codewars3::consecutive_strings::*;
// use codewars3::give_me_a_diamond::*;
// use codewars3::multiplication_table::*;
// use codewars3::shifting_zeros::*;
// use codewars3::write_number_in_expanded_form::*;]
// use codewars3::rgb_to_hex_conversion::*;
// use codewars3::max_subarr_sum::*;
// use codewars3::sum_of_pairs::*;
// use codewars3::pyramid_slide::*;

pub mod codewars4 {
    
}

fn main() {
    // run_longest_slide();
    // run_sum_of_pairs();
    // run_max_subarr_sum();
    // run_rgb();
    // run_expanded_form();
    // run_print();
    // run_multiplication_table();
    // run_move_zeros();
    // run_longest_consec();
    // run_sq_in_rect();
    // run_parse();
    // run_solution();
    // run_dna_to_rna();
    // run_sum_difference();
    // run_duty_free();
    // run_hello();
    // run_lightsabers();
    // run_expressions_matter();
    // run_find_difference();
    // run_enough();
    // run_bartender_drinks();
    // run_check_exam();
    // run_stray();
}
