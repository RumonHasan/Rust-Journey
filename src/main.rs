use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;

use concepts::concepts_modules::can_be_typed_words;
use concepts::concepts_modules::find_and_replace_pattern;
use concepts::concepts_modules::find_length_dp_sub;
use concepts::concepts_modules::find_the_prefix_common_array;
use concepts::concepts_modules::group_the_people;
use concepts::concepts_modules::grumpy_owner;
use concepts::concepts_modules::min_sub_array_len;
use concepts::concepts_modules::min_subsequence;
use concepts::concepts_modules::next_greater_el_ii;
use concepts::concepts_modules::next_greater_element;
use concepts::concepts_modules::occurrences_of_element;
use concepts::concepts_modules::remove_duplicate_char;
use concepts::concepts_modules::reorg_string;

use crate::concepts::concepts_modules::asteroid_collision;
use crate::concepts::concepts_modules::find_least_num_of_unique_ints;
use crate::concepts::concepts_modules::find_lhs;
use crate::concepts::concepts_modules::is_long_pressed_name;
use crate::concepts::concepts_modules::longest_substring_with_k;
use crate::concepts::concepts_modules::max_subarray_sum_circular;
use crate::concepts::concepts_modules::num_subarrays_with_sum;
use crate::concepts::concepts_modules::rearrange_nums;
use crate::concepts::concepts_modules::replace_words;

// using the files
mod rust_leetcode;
mod concepts;

// main function is necesary to make the code run
// cannot assign values ot a different type after once its declared... unless its mutable
fn main() {

    //merge_alternately(String::from("ab"), String::from("pqrs"));
    // rearrange_nums(vec![1,2,5,9]);
    // find_lhs(vec![1,3,2,2,5,2,3,7]);
    //is_long_pressed_name(String::from("alex"), String::from("aaleexa"));
    //replace_words(vec!["cat".to_string(),"bat".to_string(),"rat".to_string()], String::from("the cattle was rattled by the battery"));
    //num_subarrays_with_sum(vec![1,0,1,0,1], 2);
    //longest_substring_with_k(String::from("ababbc"), 2);
    //asteroid_collision(vec![10, -10]);
    //find_least_num_of_unique_ints(vec![-3,-2,-3], 3);
    //max_subarray_sum_circular(vec![-3,-2,-3]);
    // occurrences_of_element(vec![1,3,2,2,3,3,1,3,1], vec![5,6,1,5,6,4,1,5], 3);
    // check_str_twot();
    //find_the_prefix_common_array(vec![1,2, 3], vec![3,2, 1]);
    //reorg_string(String::from("ogccckcwmbmxtsbmozli"));
    // next_greater_element(vec![4,1,2], vec![1,3,4,2]);
    //find_and_replace_pattern(vec!["abc","deq","mee","aqq","dkd","ccc"].iter().map(|curr_string| String::from(*curr_string)).collect(), "abb".to_string());
    //group_the_people(vec![3,3,3,3,3,1,3]);
    // min_subsequence(vec![4,3,10,9,8]);
    //find_length_dp_sub(vec![1,2,3,2,1], vec![3,2,1,4,7]);
    //min_sub_array_len(7, vec![2,3,1,2,4,3]);
    //can_be_typed_words(String::from("hello world"), String::from("ad"));
    //next_greater_el_ii(vec![1,2,3,4,3]);
    //grumpy_owner(vec![1,0,1,2,1,1,7,5], vec![0,1,0,1,0,1,0,1], 3);
    remove_duplicate_char(String::from("abcd"), 2);

    // in rust str is a sequence of immutable chars but String is mutable
    let mut anagram_input: Vec<String> = vec!["bat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|char| char.to_string())
        .collect();

    let mut x = 4;
    println!("x is: {}", x);
    x = 5;
    println!("x new value: {}", x);
    //reverse_vowels(String::from("leetcode"));
    // here the value is redeclared that is okay even if they are immutable
    // let x = 4;
    // let x = x + 1;

    // making a scope/
    // within a scope the value is independent and is not affected by external scope
    {
        let x = 2;
        println!("x is: {}", x);
    }
    // if a new variable is created then the type can be changed easily....
    // using a constant// writing convention for writing constants
    const SECONDS_IN_MINUTE: u32 = 60; // the type here cannot be changed as its constant throughout the entire problem
    println!("seonds: {}", SECONDS_IN_MINUTE);

    // data types: primitive data types//
    // rust has two primary data types, scalar and compound data types.

    // Scalar data type i32, u32... f32 and f64 is default type for scalar data type
    let true_or_false: bool = false; // also 0 and 1 can be used for false and true
    let letter: char = 'a'; // for char have to use single quote in order to represent a char

    // compound type::: we have tuple and array

    // tuple: implicit declaration and is defined by the value type it has stored
    let mut tup: (i32, bool, char) = (1, true, 's');
    let mut tup2: (i8, bool, char) = (1, true, 's'); // here those are two different types of tuple;
    // how to print tuples
    // remember u cannot add elements to tuple since it has definite type to begin with
    tup.1 = false; // can be accessed individual value of tuple
    // println!("{}", tup.1);

    // arrays
    let arr = [1, 23, 4, 4, 4, 32];
    // explicity declaration of array // arrays need to be manually initialised in rust since there is a length defined
    let mut new_array: [i32; 5] = [2, 3, 4, 5, 5];
    new_array[3] = 17;
    // println!("{}", new_array[3]);

    // if there is a type already defined in a variable that type cannot be changed when adding the element to a different variable

    // getting input output// reference is for & for adding mutable reference
    let mut input = String::new(); // mutable string
    // when read line is used... the type in input is expected to be a string
    // io::stdin().read_line(&mut input).expect("failed to read line"); // creating a mutable ref to this input variable which will enable us to directly modify the mutable input
    // println!("{}", input);

    // for arithmetic operations rust varibles need to have same type //
    // for floating point the type needs to be f8 or f64 and so on
    let x: u8 = 255;
    let y: u8 = 10;
    let z: u8 = x / y;
    // println!("{}", z);

    // type casting
    let x: f32 = 10.4f32; // or it can be done using underscore;
    let h: i64 = 123_000 as i64;
    // println!("{}", x);

    let x = (i32::MAX as i64) + 1;
    // println!("{} : overflow", x);

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("error");
    // println!("{}", input);

    // // parse string and give us an integer if its possible to get into type 64
    // let int_input: i64 = input.trim().parse().unwrap(); // converts the input into possible integer in order to be parsed then the operation is added
    // println!("{}", int_input + 2);

    // for condition operations the type needs to be the same also;
    // let condition = (2 as f32) <= 2.2;
    // let cond2 = condition && true;

    // let food = "cookie";
    // if food == "cookie"{
    //     println!("{}", "I like cookies");
    // }else{
    //     println!("nothing");
    // }

    // returning values from a function

    // let result = add_numbers(20, 30);
    // println!("{}", result);

    // number is a statement but inside is an expression.. very rust specific feature
    let number = {
        let x = 3;
        x + 1 // if u put a semi colon then it will say it did not return anything since its a statement
    };
    // println!("{}", number);

    let result: Vec<String> = divide_string("abcdefghij", 3, 'x');
    let add_sum: Vec<i32> = two_sum(vec![2, 7, 11, 15], 9);
    let mut vector_matric = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    // matrix_set_zeroes(&mut vector_matric);
    // let result_hash: (HashMap<i32, i32>, Vec<i32>) = play_with_hash(&mut vec![1,2,3,4,2,2,2,1,1,1,23]); // the result will have a tuple
    // println!("{:?} result hash map comes here", result_hash.1); // here tuples act like objects
    // check_map_iter(&mut vec![1,3,4,5,6,7]);

    two_sum_two(vec![2, 7, 11, 15], 9);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut target_map: HashMap<i32, i32> = HashMap::new();
    for (index, item) in nums.iter().enumerate() {
        let compliment = target - item;
        if let Some(prev_index) = target_map.get(&compliment) {
            return vec![*prev_index, index as i32];
        }
        target_map.insert(*item, index as i32);
    }
    Vec::new()
}

fn two_sum_match(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        let compliment = target - num;
        match hash_map.get(&compliment) {
            Some(&prev_index) => {
                return vec![prev_index, index as i32];
            }
            None => {
                hash_map.insert(num, index as i32);
            }
        }
    }
    Vec::new()
}

// play with hash maps
fn play_with_hash(arr: &mut Vec<i32>) -> (HashMap<i32, i32>, Vec<i32>) {
    // functions gets mutable vector then it will return an array of its occurences
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for (index, &item) in arr.iter().enumerate() {
        // remember the item is always a reference
        if let Some(&count_found) = hash_map.get(&item) {
            hash_map.insert(item, count_found + 1);
        } else {
            hash_map.insert(item, 1);
        }
    }
    let mut hash_values = hash_map.values();
    let mut value_vec: Vec<i32> = Vec::new();
    for (index, &item) in hash_values.into_iter().enumerate() {
        println!("{}", item);
        value_vec.push(item);
    }

    (hash_map, value_vec) // they can use a; tuple to return multiple values
}

fn check_map_iter(mutable_array: &mut Vec<i32>) {
    let mut vec_basic: Vec<i32> = vec![1, 3, 4, 5, 6, 7, 6];
    let result_vec: Vec<i32> = vec_basic
        .iter()
        .map(|val| val + 1)
        .collect(); // here collect is needed at the end to get teh values
    for (index, item) in mutable_array.iter_mut().enumerate() {
        // iter gives u immutable references  but iter_mut lets u change those references into mutable values
        if *item > 2 {
            *item += 10;
        }
    }

    // easier way to change mutable vals in array just using the length of the mutable reference and changing the variables on the spot
    for index in 0..mutable_array.len() {
        mutable_array[index] += 10;
    }

    println!("{:?}", mutable_array);
}

fn random_camel() -> Vec<i32> {
    let mut array: Vec<i32> = Vec::new();
    println!("{:?}", array);
    Vec::new()
}

// set matrix elements to 0
// remember when indexing elements... they are usize and they need to be reinitialized to usize in order to make iterations
fn matrix_set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    // here is already returns a mutable reference to the matric hence u need not use iter_mut to modify
    let mut row_set: HashSet<i32> = HashSet::new();
    let mut col_set: HashSet<i32> = HashSet::new();
    // getting the row and col set
    for row_index in 0..matrix.len() {
        for (col_index, &curr_number) in matrix[row_index].iter().enumerate() {
            let curr_val: i32 = matrix[row_index][col_index];
            match curr_val == 0 {
                true => {
                    row_set.insert(row_index as i32);
                    col_set.insert(col_index as i32);
                }
                false => {}
            }
        }
    }
    // turning individual cols and rows into 0 of the matrix .. in rust vector indexing is done through i32
    for row_val in row_set {
        for index in 0..matrix[0].len() {
            matrix[row_val as usize][index] = 0;
        }
    }
    // changing the cols
    for col_val in col_set {
        for row_index in 0..matrix.len() {
            matrix[row_index][col_val as usize] = 0;
        }
    }
}

fn partition_string_match(s: String) -> i32 {
    let mut counter: i32 = 0;
    let mut map: HashMap<char, i32> = HashMap::new();
    for char in s.chars() {
        match map.contains_key(&char) {
            true => {
                map.clear();
                map.insert(char, 1);
                counter += 1;
            }
            false => {
                map.insert(char, 1);
            }
        }
    }
    counter + 1
}
// note if its vectors then u need reference in order to copy
fn partition_string(s: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    let mut counter: i32 = 0;
    for char in s.chars() {
        if map.contains_key(&char) {
            println!("{:?}", map);
            counter += 1;
            map = HashMap::new(); // redefining the hashmap
            map.insert(char, 1);
        } else {
            map.insert(char, 1);
        }
    }
    counter + 1
}

// naming is snake case in rust no camel case

fn divide_string(s: &str, k: i32, fill: char) -> Vec<String> {
    // let s: Vec<char> = s.chars().collect();// injects the char in vector array
    let mut empty_char_vec: Vec<char> = Vec::new();
    for char in s.chars() {
        empty_char_vec.push(char);
    }
    let k = k as usize;
    let mut local_string = String::new();
    let mut counter: usize = 0;
    let mut result_vec: Vec<String> = Vec::new();
    for index in 0..empty_char_vec.len() {
        local_string.push(empty_char_vec[index]);
        counter += 1;
        if counter == k {
            result_vec.push(local_string);
            local_string = String::new();
            counter = 0;
        }
    }
    if !local_string.is_empty() {
        result_vec.push(local_string);
    }
    if let Some(last_elem) = result_vec.last_mut() {
        // if there is a last element it binds the mutable reference to the last element
        if last_elem.len() < k {
            let len_diff: usize = k - last_elem.len();
            let mut add_string = String::new();
            for _index in 0..len_diff {
                add_string.push(fill);
            }
            last_elem.push_str(&add_string);
        }
    }
    result_vec
}

// here the vec does not need to be mutated
fn two_sum_two(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for (index, &item) in nums.iter().enumerate() {
        let compliment = target - item;
        if hash_map.contains_key(&compliment) {
            if let Some(&prev_index) = hash_map.get(&compliment) {
                return vec![prev_index, index as i32];
            }
        } else {
            hash_map.insert(item, index as i32);
        }
    }
    Vec::new()
}
