pub mod leetcode_module{
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::vec;

    pub fn is_valid_stack(s:String)-> bool{
        let check:bool = true;
        let mut stack_map: HashMap<char, char> = HashMap::new();
        stack_map.insert(']', '[');
        stack_map.insert('}', '{');
        stack_map.insert(')', '(');

        // let bracket_val = stack_map.get(&')'); // if this needs to be converted to string then it needs to be used map for checking condition if it exists or not
        // let bracket_val_string: String = bracket_val.map(|&curr_val| curr_val.to_string()).unwrap_or_default();
        // println!("{:?}", bracket_val_string);

        let mut check_stack:Vec<char> = Vec::new();

        for char in s.chars(){
            if !check_stack.is_empty() {
                if let Some(last_stack_bracket) = stack_map.get_mut(&char){
                    if last_stack_bracket == &check_stack[check_stack.len() - 1]{
                        check_stack.pop();
                        continue; // continues because u dont wanna push
                    }
                }
            }
            check_stack.push(char);
    
        }
        check_stack.len() == 0  && check
    }

    // getting formal group anagram
    pub fn is_group_anagram(strs: &mut Vec<String>)-> Vec<Vec<String>>{

        let mut anagram_map:HashMap<String, Vec<String>> = HashMap::new();
        let mut result_vec: Vec<Vec<String>>  = Vec::new();

        for item in strs.iter(){
            let mut curr_mod_item: Vec<char> = item.chars().collect(); 
            curr_mod_item.sort();
            let sorted_curr_item: String = curr_mod_item.iter().collect(); // here getting immutable reference// also remember a string cannot be built from iter_mut
            if anagram_map.contains_key(&sorted_curr_item){
                if let Some(anagram_array) = anagram_map.get_mut(&sorted_curr_item){ // mutable reference is needed in order to edit the map pair key value
                    anagram_array.push(item.to_string());
                }
            }else{
                anagram_map.insert(sorted_curr_item, vec![item.to_string()]);// creates a new string that is owned by the hashmap here
            }
            
        }
        for( key, value) in &anagram_map{ // vectors in map have ownership tied to the map
            result_vec.push(value.clone());
        }
       result_vec
    }

    pub fn frequency_sort(s:String)-> String{
        let mut map:HashMap<char, i32> = HashMap::new();
        for char in s.chars(){
            if map.contains_key(&char){
                if let Some(prev_occurence) = map.get_mut(&char){
                   *prev_occurence +=1; 
                }
            }else{
                map.insert(char, 1);
            }
        }
        let mut result_string = String::new();
        let mut sorted_vec: Vec<_> = map.into_iter().collect(); // map is consumed here but no longer is able to consume map and returns an array of tuples
        sorted_vec.sort_by(|a, b| b.1.cmp(&a.1)); 
        for (key, value) in sorted_vec{
            for index in 0..value{
                result_string.push(key);
            }
        }
        result_string
    }

    pub fn longest_subarray_after_deletion(nums:Vec<i32>) -> i32{
        let mut max_len = 0;
        let mut start_index = 0;
        let mut counter = 0;
        for (end_index, &item) in nums.iter().enumerate(){
            if item == 0{
                counter += 1;
            }
            while counter > 1{
                if nums[start_index] == 0{
                    counter -= 1;
                }
                start_index += 1;
            }
            let curr_len = (end_index - start_index) as i32 ;
            max_len = curr_len.max(max_len);
        }
        max_len
    }

    pub fn iterator_playground(){
        println!("Iterarot Playground");
        let array:[i32; 4] = [1,3,4,5];
        let mut consumed_vec: Vec<i32> = array.iter().map(|&x| x).collect(); // entirely consumed
        let mut check_vec: Vec<i32> = Vec::new();
        println!("{:?}", consumed_vec);

        for (index, &item) in consumed_vec.iter().enumerate(){ // immutable reference
            if item > 10 as i32{
                check_vec.push(index as i32);
            }
        };

        // hash with number and vec Strings
        let mut hash: HashMap<i32, Vec<String>> = HashMap::new();
        let array: [i32; 4] = [1,2,3,4];

        for &item in array.iter(){
            hash.insert(item, vec![String::from("Rumon"), String::from("Kris")]);
        };

        for (key, value) in hash.iter_mut(){
            value.push(String::from("rumon"));
        }

        // increasing a particular one
        let mut mini_vec: Vec<i32> = vec![1];
        for item in mini_vec.iter(){
            if hash.contains_key(&item){
                // mutable reference
                if let Some(array) = hash.get_mut (item){
                    array.push(String::from("value"));
                }
            }
        }
        let  vec_freq: Vec<i32> = vec![1,2,2,2,2,4,5];
        {
            
        }
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut hashmap: HashMap<&str, Vec<i32>> = HashMap::new();
    
        hashmap.insert("first", vec![1, 2, 3]);
        hashmap.insert("second", vec![4, 5, 6]);
    
        // Using flat_map to flatten the values of the HashMap into a single iterator
        let flattened_iterator: Vec<&i32> = hashmap.values().flat_map(|v| v.iter()).collect(); // for nested iterators

        let mut array: Vec<i32> = vec![1,2,3,4,5];
        let mut get_vec: Vec<i32> = array.into_iter().map(|item| item).collect();
        let filtered: Vec<i32> = get_vec.into_iter().filter(|&item| item > 2).collect();
        

        println!("{:?} flat", flattened_iterator);

        for (index , &item) in vec_freq.iter().enumerate(){ // here is returning immutable reference to vec_freq
            // reference of the key is checked here order to access the dereferencing occurence value and for iteration
            if map.contains_key(&item){
                if let  Some(prev_occurence) = map.get_mut(&item){
                    *prev_occurence += 1;
                }
            }else{
                map.insert(item, 1);
            }
        }

        // filter vector and return something new but consume the ownership of the vector

        let filtered_vec: Vec<&i32> = vec_freq.iter().filter(|&&item| item > 2).collect(); // 

        println!("{:?}", filtered_vec);


        println!("{:?}", check_vec);

    }

    // pub fn play_ground(){
    //     let mut vector_array:Vec<i32> = Vec::new();
    //     for index in 0..10{
    //         let curr_num = index;
    //         vector_array.push(curr_num);
    //     };
    //     // into iter mutationm
    //     // for item in vector_array.into_iter(){ if into_iter is used then it consumes the ownership of the value entirely 
    //     //     println!("{}", item);
    //     // }
    //     let mut map: HashMap<i32, i32> = HashMap::new();
    //     let mut new_vec:Vec<i32> = Vec::new();
        
    //     for (index, item) in vector_array.iter_mut().enumerate(){
    //         *item += *item * 10;
    //         new_vec.push(*item);
    //         new_vec.push(*item);
    //     };

    //     // storing occurence 
    //     for item in new_vec.iter(){
    //         if map.contains_key(&item){
    //             if let Some(option_item) = map.get_mut(&item){ // getting mutable ref and here after dereferencing the value can be edited
    //                 *option_item += 1;
    //             }
    //         }else{
    //             map.insert(*item, 1);
    //         }
    //     }
    //     println!("{:?}", map);

    // }

}



// notes:
/*
IMPORTANT ITERATORS AND METHODS
into_iter() -> consumed the ownership to wherever its used after that the ownership or vector is no longer valid
iter() -> returns immutable references to the value of the vectors.. have to use ampersand when referencing element
iter_mut() -> returns mutable reference to a value but u have to destructuring using asterisk in order to make any changes.
collect() -> returns the collection of something based on the type of iterator passed and filtered
Some() -> returns an Option<> value when returning a value from a datastructure such as a HashMap or a HashSet.
get_mut() -> returns mutable reference to the value of the HashMap;
get() -> returns immutable reference

asterisk (dereferencing) -> when the value of a hashmap is a vector then u do not need to deference with asterisk in order to edit the value unless its a regular single char or string value
        --> for example if there is a hashmap that needs to get its occurence increased based on the previous value then dereferencing can be used to increase the value


filter -> needs double ampersand if u are referencing a vector element from a vec... type reference needed to 
map -> needs single ampersand for referencing a single element but the type will also be reference since its taking reference 

When you use map with into_iter, the closure provided to map is consuming each element from the iterator and applying the transformation. This means the closure receives ownership of each element and is free to move or consume it. The result of map is a new collection of owned values.

On the other hand, when you use filter with into_iter, the closure provided to filter is evaluating a predicate for each element. 
The closure receives ownership of each element, but the result is a boolean 
indicating whether the element should be included in the filtered collection or not. In this case, the closure needs to return a boolean, so you use & to take a reference to the owned value, allowing you to perform the comparison without consuming the element.


first, nth, get, -> all these methods return option that needs to be dereferenced by Some
last_mut gets optionable reference

position -> trait returns the position of index of the first element that is found based on the condition passed in the callback

in vectors or strings u cannot index the first or any particular element as it will return an option so u have to use nth and dereference the option
*/