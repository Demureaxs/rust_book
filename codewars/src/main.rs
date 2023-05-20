// use codewars1::bartender_drinks::run_bartender_drinks;
// use codewars1::check_the_exam::run_check_exam;
// use codewars1::difference_of_volumes_cuboids::run_find_difference;
// use codewars1::find_unique_value::run_stray;
// use codewars1::will_there_be_enough_space::run_enough;

// use codewars1::difference_of_volumes_cuboids;

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
    pub mod integers_create_one {
        pub fn run_list_squared() {
            list_squared(1, 250);
            // list_squared(42, 250);
            // list_squared(300, 600);
        }

        fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
            let mut results: Vec<(u64, u64)> = Vec::new();
            for num in m..n {
                let mut divisors: Vec<u64> = Vec::new();
                for i in 1..=(num as f64).sqrt() as u64 {
                    if num % i == 0 {
                        divisors.push(i.pow(2));
                        if i * i != num {
                            divisors.push((num / i) * (num / i));
                        }
                    }
                }
                let mut sum: u64 = divisors.iter().sum();
                let sqrt = (sum as f64).sqrt() as u64;

                if sqrt * sqrt == sum {
                    results.push((num, sum));
                }
            }
            return results;
        }
    }

    pub mod descending_order {
        pub fn run_desc_order() {
            descending_order(1254859723);
            descending_order(145263);
            descending_order(123456789);
            descending_order(0);
        }

        fn descending_order(x: u64) -> u64 {
            let mut digits: Vec<char> = x.to_string().chars().collect();
            digits.sort_by(|a, b| b.cmp(a));
            let output = digits.iter().collect::<String>().parse::<u64>().unwrap();

            println!("{}", output);
            return output;
        }
    }

    pub mod max_n_occurances {
        use std::collections::HashMap;
        // need an algorithm that will remove duplacates that appear over n times
        pub fn run_delete_this() {
            // delete_nth(&[20, 37, 20, 21], 1);
            delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3);
        }

        fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
            let mut counts: HashMap<u8, u32> = HashMap::new();
            let mut result: Vec<u8> = Vec::new();

            for num in lst {
                // count is a mutable reference to the entry if its not there it will be added with a value of 0
                let count = counts.entry(*num).or_insert(0);
                // if the count is less than n
                if *count < n as u32 {
                    // increment the count
                    *count += 1;
                    // push the number to the result array
                    result.push(*num)
                }
            }
            // print the result array
            println!("{:?}", result);
            // return result
            return result;
        }
    }

    pub mod parts_of_a_list {
        pub fn run_part_list() {
            part_list(vec!["I", "wish", "I", "hadn't", "come"]);
        }

        fn part_list(arr: Vec<&str>) -> String {
            let mut result = String::new();
            for index in 1..arr.len() {
                let (start, end) = arr.split_at(index);
                result.push_str("(");
                result.push_str(&start.join(" "));
                result.push_str(", ");
                result.push_str(&end.join(" "));
                result.push_str(")")
            }

            println!("{}", result);
            return result;
        }
    }

    pub mod english_beggars {
        use std::io::read_to_string;

        pub fn run_beggars() {
            // beggars(&[1, 2, 3, 4, 5], 1);
            beggars(&[1, 2, 3, 4, 5], 3);
        }

        fn beggars(values: &[u32], n: usize) -> Vec<u32> {
            if n == 0 {
                return vec![];
            }
            // creates a vector from the array
            let temp = values.to_vec();
            // uninitialized beggar index
            let mut beggar_index: usize;
            // initialize sum to zero
            let mut sum = 0;
            // the final results vector
            let mut results: Vec<u32> = Vec::new();

            // loop through each beggar starting at zero
            for beggar in 0..n {
                // set the beggar index to the initially to the index of the beggar
                beggar_index = beggar;
                // loop through each item of temp
                for (index, item) in temp.iter().enumerate() {
                    // if the beggar index matches the index
                    if beggar_index == index {
                        // add the item to the sum
                        sum += item;
                        // increase the beggar index by the number of beggars
                        beggar_index += n;
                    }
                }
                // push the sum to the results vector
                results.push(sum);
                // reset the sum
                sum = 0;
            }
            // return the results vector
            println!("{:?}", results);
            return results;
        }
    }

    pub mod does_my_number_look_big_in_this {
        pub fn run_narcissistic() {
            // narcissistic(7);
            narcissistic(371);
            // narcissistic(4887);
        }

        fn narcissistic(num: u64) -> bool {
            // get the length of the number by converting it to a string
            let length = num.to_string().len();
            // println!("{}", length);

            // create a sum variable
            let mut sum: u64 = 0;

            // loop over each number and multiply it by the power of the length
            for num in num.to_string().chars() {
                // sum += num.to_digit(10).map(|num| num.pow(length as u32)).unwrap();
                sum += (num.to_digit(10).unwrap() as u64).pow(length as u32);
            }
            println!("{}", sum);

            if sum == num {
                return true;
            } else {
                return false;
            }
        }
    }

    pub mod counting_duplicates {
        pub fn run_count_duplicates() {
            count_duplicates_b("indivisibility");
            count_duplicates("abcdea");
        }

        use std::collections::HashMap;
        use std::collections::HashSet;

        fn count_duplicates(text: &str) -> u32 {
            let mut word_map: HashMap<char, u32> = HashMap::new();

            for letter in text.to_lowercase().chars() {
                *word_map.entry(letter).or_insert(0) += 1;
            }

            let results: Vec<&char> = word_map
                .iter()
                .filter(|item| *item.1 > 1)
                .map(|item| item.0)
                .collect();
            println!("{}", results.len());
            return results.len() as u32;
        }

        fn count_duplicates_b(text: &str) -> u32 {
            let mut word_map: HashMap<char, u32> = HashMap::new();
            for letter in text.to_lowercase().chars() {
                *word_map.entry(letter).or_insert(0) += 1;
            }
            return word_map.values().filter(|&&count| count > 1 as u32).count() as u32;
        }
    }
}

pub mod codewars5 {
    pub mod human_time {
        pub fn run_make_readable() {
            // make_readable(60);
            make_readable(3600);
            // make_readable(86399);
        }

        fn make_readable(seconds: u32) -> String {
            let hours = seconds / 3600;
            let mins = (seconds % 3600) / 60;
            let secs = (seconds % 3600) % 60;

            let time = format!("{:02}:{:02}:{:02}", hours, mins, secs);

            println!("{}", time);
            return time;
        }
    }

    pub mod valid_braces {
        pub fn run_valid_braces() {}

        fn valid_braces(s: &str) -> bool {
            if s.len() % 2 != 0 {
                return false;
            }

            let mut stack: Vec<char> = Vec::new();

            for brace in s.chars() {
                match brace {
                    '(' | '[' | '{' => stack.push(brace),
                    ')' => {
                        if stack.pop() != Some('(') {
                            return false;
                        }
                    }
                    ']' => {
                        if stack.pop() != Some('[') {
                            return false;
                        }
                    }
                    '}' => {
                        if stack.pop() != Some('{') {
                            return false;
                        }
                    }
                    _ => {}
                }

                return stack.is_empty();
            }

            todo!()
        }
    }

    pub mod is_fibonnaci {
        pub fn run_product_fib() {
            // product_fib(4895);
            product_fib(5895);
        }

        fn product_fib(prod: u64) -> (u64, u64, bool) {
            let mut result: (u64, u64, bool) = (0, 0, false);
            // base
            let mut a = 0;
            // current number
            let mut b = 1;
            // while b is less than the input value
            while b < prod {
                // create a temp variable assigned to the value of b
                let temp = b;
                // increment b by the value of a
                b += a;
                // set a to the value of temp
                a = temp;

                // check the multiples of a and b if its less than the input value
                if a * b < prod {
                    continue;
                } else if a * b == prod {
                    return (a, b, true);
                } else if a * b > prod {
                    return (a, b, false);
                }
            }
            return result;
        }

        fn product_fib_codewars_solution(prod: u64) -> (u64, u64, bool) {
            let mut a = 0;
            let mut b = 1;
            while a * b < prod {
                let c = a + b;
                a = b;
                b = c;
            }
            (a, b, a * b == prod)
        }
    }

    pub mod spin_words {
        pub fn run_spin_words() {
            spin_words("Seriously this is the last one");
        }

        fn spin_words(words: &str) -> String {
            let binding = words.to_string();
            // split the string at spaces to a vector
            let word_vector: Vec<&str> = binding.split_whitespace().collect();
            // result array
            let mut result: Vec<String> = Vec::new();
            // loop through the vector
            for word in word_vector {
                // for each word, reverse it and push it to results if over 5 letters
                if word.len() > 4 {
                    result.push(word.chars().rev().collect());
                } else {
                    // otherwise just push the string
                    result.push(word.to_string());
                }
            }
            // return results.
            println!("{:?}", result.join(" "));
            return result.join(" ");
        }
    }

    pub mod duplacate_encoder {
        pub fn run_duplicate_encoder() {
            duplicate_encode("Success");
        }

        fn duplicate_encode(word: &str) -> String {
            // loop through the word
            let mut result = String::new();
            // create a hashmap
            for (index, letter) in word.to_lowercase().chars().enumerate() {
                let mut multiple_appearances = false;
                // use entry.or_insert to keep a counter for each letter
                for (i, ch) in word.to_lowercase().chars().enumerate() {
                    if i != index && ch == letter {
                        multiple_appearances = true;
                    }
                }
                if multiple_appearances {
                    result.push_str(")");
                } else {
                    result.push_str("(");
                }
            }
            println!("{}", result);
            todo!()
        }
    }

    pub mod order_please {
        use std::collections::HashMap;

        pub fn run_order() {
            order("is2 Thi1s T4est 3a");
            order_codewars("is2 Thi1s T4est 3a");
        }

        fn order(sentence: &str) -> String {
            if sentence.len() < 1 {
                return "".to_string();
            }
            let mut word_pos: HashMap<&str, u32> = HashMap::new();

            for word in sentence.split(' ') {
                for letter in word.chars() {
                    if letter.is_ascii_digit() {
                        word_pos.insert(word, letter.to_digit(10).unwrap());
                    }
                }
            }
            let mut entries: Vec<_> = word_pos.iter().collect();
            entries.sort_by_key(|entry| entry.1);
            let result: Vec<&str> = entries.iter().map(|item| *item.0).collect();

            println!("{:?}", result.join(" "));
            return result.join(" ");
        }

        fn order_codewars(sentence: &str) -> String {
            // split the word at whitespace and map to strings, collect into a vector
            let mut words: Vec<_> = sentence.split_whitespace().map(String::from).collect();
            // sort the words by key,
            // split into characters and find the number in the word,
            // sort by that number and unwrap
            words.sort_by_key(|word| word.chars().find(|letter| letter.is_digit(10)).unwrap());

            // return words joined
            println!("{}", words.join(" "));
            words.join(" ")
        }
    }

    pub mod mean_of_array {
        pub fn run_get_average() {
            get_average(&[1, 2, 15, 15, 17, 11, 12, 17, 17, 14, 13, 15, 6, 11, 8, 7]);
        }

        fn get_average(marks: &[i32]) -> i32 {
            let result = marks.iter().sum::<i32>() / marks.len() as i32;
            println!("{}", result);

            return result;
        }
    }

    pub mod find_max {
        pub fn run_find_max() {
            minimum(&[-52, 56, 30, 29, -54, 0, -110]);
            maximum(&[-52, 56, 30, 29, -54, 0, -110]);
        }

        // longer but easier to read
        fn minimum(arr: &[i32]) -> i32 {
            let min = *arr.iter().min().unwrap();
            println!("{}", min);
            return min;
        }

        // cleanest way of returning the correct value
        fn maximum(arr: &[i32]) -> i32 {
            *arr.iter().max().unwrap()
        }
    }

    pub mod find_middle_element {
        pub fn run_gimme() {
            gimme([2, 3, 1]);
            gimme([-2, -3, -1]);
        }

        fn gimme(input_array: [i32; 3]) -> usize {
            let mut temp = input_array.clone();
            temp.sort();
            println!("{:?}", temp);

            // find index of temp[1] and return it
            for (index, number) in input_array.iter().enumerate() {
                if *number == temp[1] {
                    return index;
                }
            }
            todo!()
        }
    }

    pub mod next_biggest_number {
        pub fn run_next_biggest_number() {
            // next_bigger_number(2017);
            // next_bigger_number(144);
            // next_bigger_number(15624168087763422193);
            // next_bigger_number(1234567890);
            // next_bigger_number(16012115342466112173);
            // next_bigger_number(2523968329512038086);
            next_bigger_number(17912558137371449165);
        }

        fn next_bigger_number(n: u64) -> Option<u64> {
            // return None is n is less than 10 (lowest possible number that cannot be rearranged)
            if n < 10 {
                return None;
            }
            // create a vector of digits from the number split to chars, and mapped to u8's
            let mut digits: Vec<u8> = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            // establish the length of the number using digits.len()
            let len = digits.len();
            // create a tracker for the index based on length of digits
            let mut i = len - 1;
            // when the index is greater than zero and digits[before the current index] is greater than digits at index
            while i > 0 && digits[i - 1] >= digits[i] {
                // reduce the index by 1
                i -= 1;
            }
            // if the index is equal to zero return non
            if i == 0 {
                return None;
            }
            // create an inner loop variable j equal to len -1
            let mut j = len - 1;
            // while j >= i and digits j is less or equal to digits i -1
            while j >= i && digits[j] <= digits[i - 1] {
                // reduce the value of j
                j -= 1;
            }
            // swap digits i-1 with j
            digits.swap(i - 1, j);
            // reverse digits from i onward
            digits[i..].reverse();
            // fold the digits array into results
            let result = digits
                .iter()
                .fold(0u64, |acc, &digit| acc * 10 + digit as u64);
            // if the result is less than the number
            if result > n {
                // return the result
                Some(result)
            } else {
                // otherwise return None
                None
            }
        }
    }

    pub mod sort_the_odd {

        pub fn run_sort_array() {
            sort_array(&[5, 3, 2, 8, 1, 4]);
            // sort_array(&[5, 3, 1, 8, 0]);
        }

        fn sort_array(arr: &[i32]) -> Vec<i32> {
            let mut odds_index: Vec<(usize, i32)> = Vec::new();
            let mut result: Vec<i32> = Vec::new();

            for (index, number) in arr.clone().iter().enumerate() {
                if number % 2 != 0 {
                    odds_index.push((index, *number));
                }
            }

            odds_index.sort_by(|a, b| a.0.cmp(&b.0));
            let mut odds_values = odds_index.clone();
            odds_values.sort_by(|a, b| a.1.cmp(&b.1));

            for (index, number) in arr.clone().iter().enumerate() {
                let mut number_pushed = false;
                for (ind, (pos, num)) in odds_index.iter().enumerate() {
                    if index == *pos {
                        result.push(odds_values[ind].1);
                        number_pushed = true;
                        continue;
                    }
                }
                if number_pushed {
                    continue;
                } else {
                    result.push(*number);
                }
            }

            println!("{:?}", result);
            return result;
        }
    }

    pub mod dont_give_me_five {
        use core::num;

        pub fn run_dont_give_me_five() {
            dont_give_me_five(4, 17);
        }

        fn dont_give_me_five(start: isize, end: isize) -> isize {
            let mut count: isize = 0;
            // loop through the range from start to the end.
            for number in start..=end {
                // if the current number contains a 5 anywhere, skip it
                if number.to_string().contains('5') {
                    continue;
                }
                // otherwise add 1 to the count and continue.
                count += 1;
            }
            return count;
        }

        fn dont_give_me_five_cw_ver(start: isize, end: isize) -> isize {
            (start..end + 1)
                .filter(|x| !x.to_string().contains('5'))
                .count() as isize
        }
    }
}

pub mod codewars6 {
    pub mod find_the_missing_letter {
        use std::char::from_u32;

        pub fn run_find_missing_letter() {
            find_missing_letter(&['a', 'b', 'c', 'd', 'f']);

            // find_missing_letter(&['O', 'Q', 'R', 'S']);
        }

        fn find_missing_letter(chars: &[char]) -> char {
            let mut next_ascii_char = chars[1] as u32;
            for (index, letter) in chars.iter().enumerate() {
                if *letter as u32 != next_ascii_char - 1 {
                    return std::char::from_u32(next_ascii_char - 1).unwrap();
                }
                next_ascii_char += 1;
            }
            todo!()
        }

        fn find_missing_letter_2(chars: &[char]) -> char {
            (chars
                .windows(2)
                .map(|w| (w[0] as u8, w[1] as u8))
                .find(|&w| w.0 + 1 != w.1)
                .unwrap()
                .0
                + 1) as char //Add 1 to previous character, to get the correct character.
        }
    }

    pub mod rot_13 {
        pub fn run_rot13() {
            // rot13("test");
            // rot13("Test");
            // rot13("Codewars");
            rot13("2nT3u2Ordj4HQvJvhyZRncrEZABAPU4cxk3C8VvtJfixERPOqZsf0n4UU9GdtkgODtXiNuaOPXwJw2RLNaEzyKxXtz4Mdx3CQShWaruBfpPKmymwX45a");
            // 2aG3h2Beqw4UDiWiulMEapeRMNONCH4pkx3P8IigWsvkRECBdMfs0a4HH9TqgxtBQgKvAhnBCKjWj2EYAnRmlXkKgm4Zqk3PDFuJnehOscCXzlzjK45n
            // 2aG3h2Beqw4UDiWiulMEapeRMNONCH4pkx3P8IigWsvkRECBdMfs0a4HH9TqgxtBQgKvAhnBCKjWj2EYAnRmlXkKgm4Zqk3PDFuJnehOscCXzlzjK45n
        }

        fn rot13(message: &str) -> String {
            println!("starting index {} ending index {}", 'a' as u32, 'z' as u32);
            println!(
                "starting index Caps {}, ending index caps {}",
                'A' as u32, 'Z' as u32
            );
            println!(
                "Starting index nums {}, ending index nums {}",
                '0' as u32, '9' as u32
            );
            // creates the output string
            let mut output = String::new();

            // loops through each letter in the message
            for letter in message.chars() {
                // if the letter is within the caps index
                if letter as u8 >= 65 && letter as u8 <= 90 {
                    // set temp to index of letter plus 13;
                    let temp = letter as u8 + 13;
                    // if that number is larger than 90 (largest capital index)
                    if temp > 90 {
                        // push the remainder added to the start as a char
                        output.push((64 + (temp % 90)) as char);
                    } else {
                        // otherwise push the temp
                        output.push(temp as char);
                    }
                // same again for lowercase
                } else if letter as u8 >= 97 && letter as u8 <= 122 {
                    let temp = letter as u8 + 13;
                    if temp > 122 {
                        output.push((96 + (temp % 122)) as char)
                    } else {
                        output.push(temp as char)
                    }
                // if the letter is anything else, push it as is.
                } else {
                    output.push(letter);
                }
            }
            // print the output string to the console.
            println!("{}", output);
            return output;
        }
    }

    pub mod find_the_divisors {
        pub fn run_divisors() {
            // divisors(15);
            // divisors(13);
            divisors(12);
        }

        fn divisors(integer: u32) -> Result<Vec<u32>, String> {
            let mut result: Vec<u32> = Vec::new();
            // loop from 0 to the number
            for i in 2..integer {
                if integer % i == 0 {
                    result.push(i);
                }
                println!("integer {}, index {}, int / i {}", integer, i, integer % i);
            }
            // if integer / number = 0
            if result.is_empty() {
                return Err(format!("{} is prime", integer));
            } else {
                return Ok(result);
            }
        }
    }

    pub mod printer_errors {
        pub fn run_printer_errors() {
            printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz");
            // 6/60

            // printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu");
            // 11/65
        }

        fn printer_error(s: &str) -> String {
            let result: Vec<_> = s.chars().filter(|letter| *letter as u8 > 109).collect();
            println!("{}/{}", result.len(), s.len());
            format!("{}/{}", result.len(), s.len())
        }
    }

    pub mod count_odds {
        pub fn run_odd_count() {
            odd_count(15023);
            // odd_count(15);
        }

        fn odd_count(n: u64) -> u64 {
            return n / 2;
        }
    }

    pub mod hit_the_target {
        pub fn run_solution() {
            // solution(&[vec!['>', ' '], vec![' ', 'x']]);

            solution(&[
                vec![' ', ' ', ' ', ' ', ' '],
                vec![' ', ' ', ' ', ' ', ' '],
                vec![' ', ' ', ' ', ' ', ' '],
                vec![' ', ' ', '>', ' ', 'x'],
                vec![' ', ' ', ' ', ' ', ' '],
            ]);
        }

        fn solution(mtrx: &[Vec<char>]) -> bool {
            let mut arr_i: (usize, usize) = (0, 0);
            let mut x_i: (usize, usize) = (0, 0);

            let mut direction: char = '^';

            for (x_index, symbol) in mtrx.iter().enumerate() {
                for (y_index, sym) in mtrx[x_index].iter().enumerate() {
                    if ['^', '>', 'v', '<'].contains(sym) {
                        arr_i = (x_index, y_index);
                        direction = *sym;
                    } else if *sym == 'x' {
                        x_i = (x_index, y_index);
                    }
                }
            }
            println!("arrow location {:?} cross location {:?}", arr_i, x_i);
            println!("{}", direction);
            match direction {
                '^' => {
                    if arr_i.1 == x_i.1 && arr_i.0 > x_i.0 {
                        return true;
                    }
                }
                '>' => {
                    if arr_i.0 == x_i.0 && arr_i.1 < x_i.1 {
                        return true;
                    }
                }
                'v' => {
                    if arr_i.1 == x_i.1 && arr_i.0 < x_i.0 {
                        return true;
                    }
                }
                '<' => {
                    if arr_i.0 == x_i.0 && arr_i.1 > x_i.1 {
                        return true;
                    }
                }
                _ => (),
            }
            return false;
        }
    }

    pub mod snail {
        pub fn run_snail() {
            // snail(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
            // snail(&[Vec<i32>; 1] = &[Vec::new()]);
            // snail(&[vec![1]]);
            snail(&[
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25],
            ]);
            // 4, 4, 4, 3, 3, 2, 2, 1, 1
        }

        fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
            if matrix.len() == 1 {
                return matrix[0].clone();
            } else if matrix.len() < 1 {
                return vec![];
            }

            let mut x_start = 0;
            let mut y_start = 0;
            let mut x_end = matrix.len() - 1;
            let mut y_end = matrix[0].len() - 1;

            let mut result: Vec<i32> = Vec::new();

            while x_start <= x_end && y_start <= y_end {
                for y in y_start..=y_end {
                    result.push(matrix[x_start][y]);
                }
                x_start += 1;

                for x in x_start..=x_end {
                    result.push(matrix[x][y_end]);
                }
                y_end -= 1;

                if x_start <= x_end {
                    for y in (y_start..=y_end).rev() {
                        result.push(matrix[x_end][y]);
                    }
                    x_end -= 1;
                }

                if y_start <= y_end {
                    for x in (x_start..=x_end).rev() {
                        result.push(matrix[x][y_start]);
                    }
                    y_start += 1;
                }
            }
            println!("{:?}", result);
            result
        }
    }

    pub mod tortoise_racing {
        pub fn run_race() {
            race(105, 255, 84);
        }

        fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
            if v1 >= v2 {
                return None;
            }

            let b_advantage = v2 - v1;
            let time_to_close_gap = (g * 3600) / b_advantage;

            let hours = time_to_close_gap / 3600;
            let mins = (time_to_close_gap % 3600) / 60;
            let seconds = time_to_close_gap % 60;

            println!("{}{}{}", hours, mins, seconds);

            Some(vec![hours, mins, seconds])
        }
    }

    pub mod temp {
        pub fn run_sum_of_squares() {
            sum_of_squares(15);
            // sum_of_squares(16);
            // sum_of_squares(18);
            // sum_of_squares(17);
        }

        fn sum_of_squares(n: u64) -> u64 {
            if is_square(n) {
                return 1;
            }
            // set a mutable variable n
            let mut n = n;

            // while n can be divided by 4
            while n % 4 == 0 {
                // divide by 4
                n /= 4;
            }

            println!("{}", n);

            // if n remainder 8 is equal to 7 return 4
            if n % 8 == 7 {
                return 4;
            }

            // otherwise loop from 0 to the square root of n as an integer
            for num in 0..=(n as f64).sqrt() as u64 {
                // check if taking num * num from n is a square number
                if is_square(n - num * num) {
                    // if it is return 3
                    return 2;
                }
            }
            // if all else fails, the answer is 3

            return 3;
        }

        fn is_square(num: u64) -> bool {
            let sqrt = (num as f64).sqrt();
            let sqrt_int = sqrt as u64;
            sqrt_int * sqrt_int == num
        }
    }

    pub mod gangs {
        pub fn run_gangs() {
            // gangs(&[2, 3, 6, 5], 15);
            gangs(&[2, 3], 6);
        }

        use std::collections::HashSet;

        fn gangs(divisors: &[u32], k: u32) -> u32 {
            let mut result: HashSet<Vec<u32>> = HashSet::new();
            for num in 1..=k {
                let mut temp: Vec<u32> = Vec::new();
                for div in divisors {
                    if num % div == 0 {
                        temp.push(*div);
                    }
                }
                result.insert(temp);
            }
            result.len() as u32
        }
    }

    pub mod bit_counting {
        pub fn run_count_bits() {
            count_bits(1234);
        }

        fn count_bits(n: i64) -> u32 {
            let bits = format!("{:b}", n)
                .to_string()
                .chars()
                .filter(|num| *num == '1')
                .collect::<Vec<char>>()
                .len() as u32;

            return bits;
        }

        fn count_bits_cw(n: i64) -> u32 {
            n.count_ones()
        }
    }

    pub mod bouncing_balls {
        pub fn run_bouncing_balls() {
            bouncing_ball(3.0, 0.66, 1.5);
            // bouncing_ball(40.0, 0.4, 10.0);
            // bouncing_ball(30.0, 0.66, 1.5);
            // bouncing_ball(10.0, 0.6, 10.0);
            // bouncing_ball(2.0, 0.5, 1.0);
        }

        fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
            if h < 1.0 || bounce >= 1.0 || bounce <= 0.0 || window >= h {
                return -1;
            }
            let mut bounce_height = h;
            let mut bounces = 0;

            while bounce_height > window {
                bounces += 1;
                bounce_height = bounce_height * bounce;
                if bounce_height > window {
                    bounces += 1;
                }
            }
            bounces
        }
    }

    pub mod parse_age {
        pub fn run_parse_age() {
            get_age("2 years old");
        }

        fn get_age(age: &str) -> u32 {
            age.split_whitespace().collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap()
        }
    }

    pub mod array_diff {
        use std::fmt::Debug;

        pub fn run_array_diff() {
            array_diff(vec![1, 2], vec![1]);
            array_diff_b(vec![1, 2], vec![1]);
        }

        fn array_diff<T: PartialEq + Clone + Debug>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
            let mut result: Vec<T> = Vec::new();

            for num in a {
                if !b.contains(&num) {
                    result.push(num);
                }
            }
            println!("{:?}", result);
            result
        }

        fn array_diff_b<T: PartialEq + Debug>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
            a.retain(|num| !b.contains(num));
            println!("{:?}", a);
            a
        }
    }
}

pub mod codewars7 {
    pub mod create_phone_number {
        pub fn run_create_phone_number() {
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
            // create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
            // create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
        }

        fn create_phone_number(numbers: &[u8]) -> String {
            numbers
                .iter()
                .enumerate()
                .map(|(index, num)| match index {
                    0 => format!("({}", *num),
                    2 => format!("{}) ", *num),
                    6 => format!("-{}", *num),
                    _ => format!("{}", *num),
                })
                .collect::<Vec<String>>()
                .join("")
        }

        // fn create_phone_number(numbers: &[u8]) -> String {
        //     // basically just need to split the array in 3,
        //     let mut output = String::new();

        //     for (index, number) in numbers.iter().enumerate() {
        //         if index == 0 {
        //             output.push_str("(")
        //         }
        //         if index == 3 {
        //             output.push_str(") ");
        //         }

        //         if index == 6 {
        //             output.push_str("-");
        //         }
        //         output.push_str(&number.to_string());
        //     }
        //     return output;
        // }
    }

    pub mod max_dif {

        pub fn run_max_diff() {
            // max_diff(&[0, 1, 2, 3, 4, 5, 6]);
            max_diff(&[-0, 1, 2, -3, 4, 5, -6]);
        }

        fn max_diff(numbers: &[i32]) -> i32 {
            if numbers.len() == 1 {
                return 0;
            }
            let max = numbers.iter().max().unwrap();
            let min = numbers.iter().min().unwrap();
            println!("{}", (max - min).abs());
            return (max - min).abs();
        }
    }

    pub mod split_strings {
        pub fn run_solution() {
            // solution("abcdef");
            solution("abcdefg");
            // solution("");
        }

        fn solution(s: &str) -> Vec<String> {
            if s.len() < 1 {
                return vec![];
            }
            let mut result: Vec<String> = Vec::new();
            for (index, letter) in s.chars().enumerate() {
                if index % 2 == 1 {
                    let mut temp = String::new();
                    temp.push_str(&s.chars().collect::<Vec<char>>()[index - 1].to_string());
                    temp.push_str(&letter.to_string());
                    result.push(temp);
                } else if index == s.len() - 1 {
                    let mut temp = String::new();
                    temp.push_str(&letter.to_string());
                    temp.push_str("_");
                    result.push(temp);
                }
            }
            println!("{:?}", result);
            return result;
        }
    }

    pub mod number_of_trailing_zeros {
        pub fn run_zeros() {
            zeros(1000);
            zeros_cw(1000);
            // zeros_cw(30);
            // zeros(6);
        }

        fn zeros(n: u64) -> u64 {
            let mut count = 0;
            let mut factor = 5;

            while factor <= n {
                count += n / factor;
                factor *= 5;
            }
            // println!("{}", count);
            return count;
        }

        fn zeros_cw(n: u64) -> u64 {
            let n = n / 5;
            match n {
                0 => 0,
                _ => n + zeros(n),
            }
        }
    }

    pub mod string_incrementor {

        pub fn run_string_incrementor() {
            // increment_string("foo"); // foo1
            // increment_string("foobar001"); // foobar002
            // increment_string("foobar99"); // foobar100
            // increment_string(""); // 1
            // increment_string("foobar01");
            // increment_string("qRLmFQrfd5Pv05G0PbyRtcDcgTX3LNoLqQKMEnOqfI7KfxDyhKzBzHIjjHuSh05gUmd3l4qmVPFU2Ysbsrf6ci19799269");
            increment_string("1");
        }

        fn increment_string(s: &str) -> String {
            let mut output = String::new();

            if s.is_empty() {
                output.push('1');
                return output;
            }

            let chars_array: Vec<char> = s.chars().collect();
            let mut index = chars_array.len() - 1;

            if index == 0 && chars_array[index].is_digit(10) {
                return (chars_array[index].to_digit(10).unwrap() + 1).to_string();
            }

            while index > 0 && chars_array[index].is_digit(10) {
                index -= 1;
            }

            if index == chars_array.len() - 1 {
                output.push_str(s);
                output.push('1');
                println!("{}", output);
                return output;
            }

            let (word, number) = s.split_at(index + 1);
            let reversed_number: String = number.chars().rev().collect();
            let mut carry = 1;

            for digit in reversed_number.chars() {
                match digit {
                    '9' => {
                        if carry > 0 {
                            output.push('0');
                            carry = 1;
                        } else {
                            output.push('9');
                        }
                    }
                    '0' => {
                        if carry > 0 {
                            output.push(carry.to_string().chars().next().unwrap());
                            carry = 0;
                        } else {
                            output.push('0');
                        }
                    }
                    _ => {
                        let digit_value = digit.to_digit(10).unwrap() + carry;
                        output.push(std::char::from_digit(digit_value, 10).unwrap());
                        carry = 0;
                    }
                }
            }

            if carry > 0 {
                output.push(carry.to_string().chars().next().unwrap());
            }
            output.push_str(&word.chars().rev().collect::<String>());
            output.chars().rev().collect::<String>()
        }

        // fn increment_string_cw(s: &str) -> String {
        //     if let Some(last) = s.chars().last() {
        //         match last.to_digit(10) {
        //             Some(9) => format!("{}0", &increment_string(&s[..s.len() - 1])),
        //             Some(num) => format!("{}{}", &s[..s.len() - 1], num + 1),
        //             None => format!("{s}1"),
        //         }
        //     } else {
        //         format!("1")
        //     }
        // }

        // split the string at the index the last number was found
        // get the length of the number
        // add 1 to the number and covert it to a string
        // calculate

        // push the number to the output/

        // fn increment_string(s: &str) -> String {
        //     if s.len() == 0 {
        //         return "1".to_string();
        //     }
        //     let input = s.clone().to_string();
        //     let mut zeros = String::new();
        //     let mut number = String::new();
        //     let mut characters = String::new();

        //     for chrctr in input.chars() {
        //         if chrctr.is_digit(10) {
        //             if chrctr == '0' {
        //                 zeros.push(chrctr);
        //             } else {
        //                 number.push(chrctr);
        //             }
        //         } else {
        //             characters.push(chrctr);
        //         }
        //     }
        //     let length = number.len() + zeros.len();

        //     if number.len() > 0 && zeros.len() > 0 {
        //         // length is equal to zeros + number
        //         let number_plus_1 = (number.parse::<u64>().unwrap() + 1).to_string();
        //         // zeros length = length - number_plus_1.len()
        //         let padding = length - number_plus_1.to_string().len();
        //         characters.push_str(&"0".to_string().repeat(padding));
        //         characters.push_str(&number_plus_1);
        //     } else if zeros.len() > 0 {
        //         let padding = zeros.len() - 1;
        //         characters.push_str(&"0".to_string().repeat(padding));
        //         characters.push_str(&"1".to_string());
        //     } else if number.len() > 0 {
        //         let number_plus_1 = (number.parse::<u64>().unwrap() + 1).to_string();
        //         characters.push_str(&number_plus_1);
        //     } else {
        //         characters.push_str(&"1".to_string());
        //     }

        //     return characters;
        // }
    }

    pub mod validate_pin {
        pub fn run_validate_pin() {
            // validate_pin("234651");
            validate_pin("2346a");
        }

        fn validate_pin(pin: &str) -> bool {
            let filtered = pin
                .chars()
                .filter(|num| num.is_digit(10))
                .collect::<Vec<char>>();
            println!("{:?}", filtered);
            if pin.len() == 6 || pin.len() == 4 {
                if pin.len() == filtered.len() {
                    return true;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    pub mod magic_square {
        use crate::codewars2::hello_name_or_world;

        pub fn run_magic_square() {
            magic_square(3);
            // magic_square(5);
        }
        
        fn magic_square(n: u32) -> Vec<Vec<u32>> {
            let mut square = vec![vec![0 as u32; n as usize]; n as usize];
            let mut count = n * n;

            let mut vertical = 0 as usize;
            let mut horizontal = n as usize / 2;
            let mut current_number = 1;

            // square[vertical][horizontal as usize] = 1;

            while count > 0 {
                let mut vert = vertical;
                let mut hor = horizontal;
                square[vert][hor] = current_number;

                if vert > 0 {
                    // move vertical up 1 (decrement)
                    vert -= 1;
                } else {
                    vert += n as usize - 1;
                }

                if hor < n as usize - 1 {
                    hor += 1;
                } else {
                    hor = 0;
                }

                if square[vert][hor] == 0 {
                    vertical = vert;
                    horizontal = hor;
                } else {
                    if vertical < n as usize - 1 {
                        vertical += 1;
                    } else {
                        vertical = 0;
                    }
                }
                // move horizontal across 1 (increment)
                println!("{}-{}", horizontal, vertical);

                current_number += 1;
                count -= 1;
            }
            return square;
        }
    }
}
// use codewars4::integers_create_one::*;
// use codewars4::descending_order::*;
// use codewars4::max_n_occurances::*;
// use codewars4::parts_of_a_list::*;
// use codewars4::english_beggars::*;
// use codewars4::does_my_number_look_big_in_this::*;
// use codewars4::counting_duplicates::*;

// use codewars5::human_time::*;
// use codewars5::is_fibonnaci::*;
// use codewars5::spin_words::*;
// use codewars5::duplacate_encoder::*;
// use codewars5::order_please::*;
// use codewars5::mean_of_array::*;
// use codewars5::find_max::*;
// use codewars5::find_middle_element::*;
// use codewars5::next_biggest_number::*;
// use codewars5::sort_the_odd::*;
// use codewars5::dont_give_me_five::*;

// use codewars6::find_the_missing_letter::*;
// use codewars6::rot_13::*;
// use codewars6::find_the_divisors::*;
// use codewars6::printer_errors::*;
// use codewars6::count_odds::*;
// use codewars6::hit_the_target::*;
// use codewars6::snail::*;
// use codewars6::tortoise_racing::*;
// use codewars6::temp::*;
// use codewars6::gangs::*;
// use codewars6::bit_counting::*;
// use codewars6::bouncing_balls::*;
// use codewars6::parse_age::*;
// use codewars6::array_diff::*;

// use codewars6::array_diff::run_array_diff;
// use codewars7::create_phone_number::*;
// use codewars7::max_dif::*;
// use codewars7::split_strings::*;
// use codewars7::number_of_trailing_zeros::*;
// use codewars7::string_incrementor::*;
// use codewars7::validate_pin::*;
use codewars7::magic_square::*;

fn main() {
    run_magic_square()
    // run_validate_pin()
    // run_string_incrementor()
    // run_zeros()
    // run_solution()
    // run_max_diff()
    // run_create_phone_number()
    // run_array_diff()
    // run_parse_age()
    // run_bouncing_balls()
    // run_count_bits()
    // run_gangs()
    // run_sum_of_squares()
    // run_race()
    // run_snail()
    // run_solution()
    // run_odd_count()
    // run_printer_errors()
    // run_divisors()
    // run_rot13()
    // run_find_missing_letter()
    // run_dont_give_me_five()
    // run_sort_array()
    // run_next_biggest_number()
    // run_gimme()
    // run_find_max()
    // run_get_average()
    // run_order()
    // run_duplicate_encoder()
    // run_spin_words()
    // run_product_fib()
    // run_make_readable()
    // run_count_duplicates();
    // run_narcissistic()
    // run_beggars();
    // run_part_list();
    // run_delete_this();
    // run_desc_order();
    // run_list_squared();
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
