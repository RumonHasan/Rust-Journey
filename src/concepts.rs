pub mod concepts_modules {
    use core::num;
    use std::cmp;
    use std::collections::btree_set::Difference;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::f32::INFINITY;
    use std::hash::Hash;
    use std::i32::MAX;
    use std::i32::MIN;
    use std::iter::Map;
    use std::string;
    use std::vec;

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut indexes_chars: Vec<i32> = Vec::new();
        let mut first: char = '\0';
        // using next method to return option first
        if let Some(first_char) = needle.chars().next() {
            first = first_char;
        }
        for (index, item) in haystack.chars().into_iter().enumerate() {
            // consuming ownership of chars of the string not haystack itself
            let injectable_index = index as i32;
            let mut local_string: String = String::new();
            if first == item {
                for sub_index in index..index + needle.len() {
                    if let Some(option_char) = haystack.chars().nth(sub_index) {
                        local_string.push(option_char);
                    }
                }
                if local_string == needle {
                    indexes_chars.push(injectable_index);
                }
            }
        }
        // need to access first elemnt like this because vec elements cannot be accessed directly
        if let Some(&first_el) = indexes_chars.first() {
            first_el as i32
        } else {
            -1
        }
    }

    pub fn is_palindrome(s: String) -> bool {
        let mut is_valid: bool = true;

        let filter_string: Vec<char> = s
            .chars()
            .filter(|&ref_char| ref_char != ' ' && ref_char.is_alphabetic())
            .map(|item| item.to_ascii_lowercase())
            .collect();

        if filter_string.is_empty() || filter_string.len() == 1 {
            is_valid = true;
        } else {
            let main_str: String = filter_string.iter().rev().collect();
            let check_str: String = filter_string.iter().collect();

            if main_str != check_str {
                is_valid = false;
            }
        }
        println!("{}", is_valid);
        is_valid
    }

    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let text_mod: Vec<String> = text
            .split_whitespace()
            .map(|chunk| String::from(chunk))
            .collect();
        println!("{:?}", text_mod);

        let mut vec_result: Vec<String> = vec![];

        for (index, item) in text_mod.iter().enumerate() {
            if item.to_string() == first && index == text_mod.len() - 3 {
                let check_second: &str = &text_mod[index + 1].to_string();
                let check_third: &str = &text_mod[index + 2].to_string();

                if check_second == &second && !check_third.is_empty() {
                    vec_result.push(check_third.to_string());
                }
            }
        }
        vec_result
    }

    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut collection: Vec<i32> = Vec::new();
        let mut mut_nums: Vec<i32> = nums.into_iter().collect(); // ownership consumed hence nums no longer exists
        mut_nums.sort_by(|a, b| a.cmp(&b));

        let mut set: HashSet<i32> = HashSet::new();
        for item in mut_nums.iter() {
            set.insert(*item);
        }
        let mut check_vec: Vec<i32> = Vec::new();
        for (index, _) in mut_nums.iter().enumerate() {
            // still can be used because it is not yet consumed
            check_vec.push((index + 1) as i32);
        }
        for item in check_vec.iter() {
            if !set.contains(&item) {
                // comparison can be done with reference
                collection.push(*item); // when pushing u need to unlock the number by dereferencing
            }
        }
        collection
    }

    pub fn first_uniq_char(s: String) -> i32 {
        let mut final_index = -1;
        let mut map: HashMap<char, i32> = HashMap::new();
        for char in s.chars() {
            if map.contains_key(&char) {
                if let Some(char) = map.get_mut(&char) {
                    *char += 1;
                }
            } else {
                map.insert(char, 1);
            }
        }
        let s_vec: Vec<char> = s.chars().collect();
        for (index, sChar) in s_vec.iter().enumerate() {
            if map.contains_key(sChar) {
                if let Some(value) = map.get(sChar) {
                    if *value == 1 {
                        final_index = index as i32;
                        break;
                    }
                }
            }
        }
        final_index
    }

    // sorting colors

    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut color_map: HashMap<i32, i32> = HashMap::new();
        for item in nums.iter() {
            if color_map.contains_key(&item) {
                if let Some(occurence) = color_map.get_mut(&item) {
                    *occurence += 1;
                }
            } else {
                color_map.insert(*item, 1); // here am not borrowing so need to dereference
            }
        }
        let check_vec: Vec<i32> = vec![0, 1, 2];
        let mut check_index = 0;
        let mut num_index = 0;

        while num_index < nums.len() {
            let check_key = check_vec[check_index as usize];
            while color_map.contains_key(&check_key) {
                nums[num_index] = check_key;
                if let Some(occurence) = color_map.get_mut(&check_key) {
                    *occurence -= 1;
                    if *occurence == 0 {
                        color_map.remove(&check_key);
                    }
                }
                num_index += 1;
            }
            check_index += 1;
        }
    }

    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        fn create_hash() -> HashSet<i32> {
            HashSet::new()
        }
        let mut col_set: HashSet<i32> = create_hash();
        let mut row_set: HashSet<i32> = create_hash();

        for (row, mat_item) in matrix.iter().enumerate() {
            for (col, item) in mat_item.iter().enumerate() {
                if *item == 0 {
                    col_set.insert(col as i32);
                    row_set.insert(row as i32);
                }
            }
        }
        for row_item in row_set.iter() {
            for col_index in 0..matrix[0].len() {
                matrix[*row_item as usize][col_index] = 0;
            }
        }

        for col_item in col_set.iter() {
            for row_index in 0..matrix.len() {
                matrix[row_index][*col_item as usize] = 0;
            }
        }
        // converting the rows and cols to 0
    }

    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut collection: Vec<Vec<String>> = Vec::new();
        let mut check_string: String = String::from("");
        products.sort_by(|a, b| a.cmp(&b));
        for curr_char in search_word.chars() {
            let mut local_vec: Vec<String> = Vec::new();
            let mut counter: i32 = 0;
            check_string.push(curr_char);
            for curr_search_item in products.iter() {
                if curr_search_item.starts_with(&check_string) {
                    local_vec.push(curr_search_item.to_string()); // here references cannot be passed so need to convert
                    counter += 1;
                }
                if counter == 3 {
                    break;
                }
            }
            collection.push(local_vec);
        }
        collection
    }

    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut peak_index = 0;
        let mut end: usize = 0;

        match nums.len() {
            1 => {
                return 0;
            }
            _ => {
                while end < nums.len() - 1 {
                    if let Some(value) = nums.get(end + 1) {
                        if nums[end] < *value {
                            let mut check_lift = false;
                            let mut check_drop = false;

                            while end < nums.len() - 1 && nums[end] < nums[end + 1] {
                                check_lift = true;
                                end += 1;
                            }

                            while end < nums.len() - 1 && nums[end] > nums[end + 1] {
                                check_drop = true;
                                if check_drop && check_lift {
                                    peak_index = end as i32;
                                    break;
                                }
                                end += 1;
                            }

                            if check_lift && !check_drop && end == nums.len() - 1 {
                                peak_index = end as i32;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                    end += 1;
                }
            }
        }
        peak_index
    }

    // moving zeroes
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut counter: i32 = 0;
        let mut new_vec: Vec<i32> = Vec::new();

        for curr_num in nums.iter() {
            if *curr_num == 0 {
                counter += 1;
            } else {
                new_vec.push(*curr_num);
            }
        }

        for _ in 0..counter {
            new_vec.push(0);
        }
        for (index, curr_num) in nums.iter_mut().enumerate() {
            *curr_num = new_vec[index];
        }
    }

    // removing all occurences of a substring by keeping track of the characters inside the substrings using stack based approach

    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = Vec::new();
        for (index, item) in nums.iter().enumerate() {
            if index == 0 {
                dp.push(*item);
            } else if index == 1 {
                if let Some(prev_dp) = dp.get(index - 1) {
                    dp.push(*prev_dp.max(item)); // max expects reference for second
                }
            } else {
                dp.push(dp[index - 1].max(dp[index - 2] + item));
            }
        }
        if let Some(last_element) = dp.last() {
            return *last_element;
        } else {
            -1
        }
    }

    pub fn number_of_substrings(s: String) -> i32 {
        let mut total: i32 = 0;
        let mut count: i32 = 0;
        let mut start: i32 = 0;

        let mut map: HashMap<char, i32> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();
        map.insert('a', 0);
        map.insert('b', 0);
        map.insert('c', 0);

        for (index, curr_char) in s_chars.iter().enumerate() {
            if let Some(prev_occurence) = map.get_mut(curr_char) {
                *prev_occurence += 1;
            }
            if let Some(occurence) = map.get(&curr_char) {
                if *occurence == 1 {
                    count += 1;
                }
            }
            while count == 3 {
                total += (s.len() as i32) - (index as i32); // conversion needed here because its usize
                if let Some(start_el) = map.get_mut(&s_chars[start as usize]) {
                    *start_el -= 1;
                    if *start_el == 0 {
                        count -= 1;
                    }
                }
                start += 1;
            }
        }
        total
    }

    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut counter = 0;
        for single_string in patterns.iter() {
            if word.contains(single_string) {
                counter += 1;
            }
        }
        counter
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut longest_common_pref: String = String::from("");
        let mut final_string: String = String::from("");

        // let sliced_vec: Vec<String> = strs
        //     .get(1..)
        //     .unwrap_or_default()
        //     .to_vec(); // this case it needs to convert the individual strings in vec of strings

        // alternative way using map
        let partial_vec: Vec<String> = strs
            .get(1..)
            .unwrap_or_default()
            .to_vec();
        println!("{:?}", partial_vec);

        let mut get_first_word: String = String::from("");
        if let Some(first) = strs.first() {
            get_first_word = first.to_string();
        }

        for curr_char in get_first_word.chars() {
            longest_common_pref.push(curr_char);
            let mut counter = 0;
            for curr_vec in partial_vec.iter() {
                if curr_vec.starts_with(&longest_common_pref) {
                    counter += 1;
                }
            }
            if counter == (partial_vec.len() as i32) {
                final_string.push(curr_char);
            }
        }

        final_string
    }
    // concept of ownership

    pub fn count_substrings(s: String) -> i32 {
        let mut counter: i32 = 0;
        let s_vec: Vec<char> = s.chars().collect();

        fn count_palindromes(mut left: i32, mut right: i32, s_vec: &Vec<char>) -> i32 {
            let mut local_count: i32 = 0;
            while
                left >= 0 &&
                right < (s_vec.len() as i32) &&
                s_vec[left as usize] == s_vec[right as usize]
            {
                left -= 1;
                right += 1;
                local_count += 1;
            }
            local_count
        }

        for (index, _) in s_vec.iter().enumerate() {
            let odd_sum: i32 = count_palindromes(index as i32, index as i32, &s_vec);
            let even_sum: i32 = count_palindromes(index as i32, (index + 1) as i32, &s_vec);
            let val: i32 = odd_sum + even_sum;
            counter += val;
        }

        counter
    }

    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut first_word: String = String::from("");
        if let Some(first) = words.first() {
            first_word = first.to_string();
        }
        let words_sliced: Vec<String> = words
            .get(1..)
            .unwrap_or_default()
            .to_vec();
        let mut map: HashMap<char, i32> = HashMap::new();

        // finding out the common char first
        for curr_char in first_word.chars() {
            let mut counter: i32 = 0;
            for word in words_sliced.iter() {
                if word.contains(curr_char) {
                    counter += 1;
                }
            }
            if counter == (words_sliced.len() as i32) {
                map.insert(curr_char, 0);
            }
        }

        // making a char map
        let mut new_map: HashMap<char, i32> = map.clone();
        for (_, value) in new_map.iter_mut() {
            *value = std::i32::MAX;
        }

        for word in words.iter() {
            for curr_char in word.chars() {
                if let Some(found_char) = map.get_mut(&curr_char) {
                    *found_char += 1;
                }
            }
            for (key, value) in map.iter() {
                if let Some(curr_value) = new_map.get_mut(key) {
                    *curr_value = (*curr_value).min(*value);
                }
            }
            // Reassign all values in the map back to 0 after processing each word
            for (_, value) in map.iter_mut() {
                *value = 0;
            }
        }
        // string creation
        for (key, value) in new_map.into_iter() {
            for _ in 0..value {
                result.push(key.to_string());
            }
        }
        result
    }

    pub fn play_ground() {
        let mut array: Vec<i32> = vec![1, 2, 3, 4, 5];
        let el: i32 = array[1];
        array[2] = 4; // gets a mutable reference to the vector // can be borrowed as a mutable reference once at a time
        for (index, item) in array.iter_mut().enumerate() {
            *item = index as i32;
        }
        println!("{:?}", array);
    }

    pub fn reverse_vowels(s: String) -> String {
        let mut result: String = String::from("");
        let mut str_vec: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, char> = HashMap::new();
        let vowel_array: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'I', 'E', 'O', 'U'];

        // injecting vowels
        for index in 0..10 {
            map.insert(vowel_array[index], vowel_array[index]);
        }

        let mut left: usize = 0;
        let mut right: usize = str_vec.len() - 1;

        while left < right {
            if let Some(left_char) = str_vec.get(left) {
                if let Some(right_char) = str_vec.get(right) {
                    if map.contains_key(left_char) && !map.contains_key(right_char) {
                        right -= 1;
                    } else if map.contains_key(right_char) && !map.contains_key(left_char) {
                        left += 1;
                    } else if map.contains_key(right_char) && map.contains_key(left_char) {
                        let temp_char: char = *left_char;
                        str_vec[left] = *right_char;
                        str_vec[right] = temp_char;
                        left += 1;
                        right -= 1;
                    } else {
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
        result = str_vec
            .into_iter()
            .map(|char| char.to_string())
            .collect(); // ownership is taken here
        result
    }

    pub fn remove_duplicates(s: String) -> String {
        let mut final_string: String = String::from("");
        let mut stack: Vec<char> = Vec::new();

        for curr_char in s.chars() {
            if let Some(stack_last) = stack.last() {
                if *stack_last == curr_char {
                    stack.pop();
                } else {
                    // after popping the index order changes to the next char so need to be pushed again
                    stack.push(curr_char);
                }
            } else {
                stack.push(curr_char);
            }
        }
        final_string = stack
            .into_iter()
            .map(|curr_char| curr_char)
            .collect();
        final_string
    }

    // to check whether the words are valid anagram or not
    pub fn is_anagram(mut s: String, mut t: String) -> bool {
        let mut check: bool = false;

        fn return_sorted_order(local_string: &mut String) -> String {
            let mut char_convert: Vec<char> = local_string.chars().collect();
            char_convert.sort_by(|a, b| a.cmp(&b));
            let local_string: String = char_convert
                .into_iter()
                .map(|value| value)
                .collect();
            local_string
        }

        let s_one: String = return_sorted_order(&mut s);
        let t_one: String = return_sorted_order(&mut t);

        if s_one == t_one {
            check = true;
        }

        check
    }

    // delete and earn
    pub fn delete_earn(mut nums: Vec<i32>) -> i32 {
        // function to return tuple for hash and vec

        // basic tuple approach by congragating hashmap and hashset into one combo
        fn convert_to_hash(local_nums: &mut Vec<i32>) -> (Vec<i32>, HashMap<i32, i32>) {
            let mut local_set: HashSet<i32> = HashSet::new();
            let mut occurence_hash: HashMap<i32, i32> = HashMap::new();

            for curr_num in local_nums.iter() {
                if let Some(found) = occurence_hash.get_mut(&curr_num) {
                    *found += 1;
                } else {
                    occurence_hash.insert(*curr_num, 1);
                }
            }

            for curr_num in local_nums.iter() {
                local_set.insert(*curr_num);
            }
            let mut return_vec: Vec<i32> = Vec::new();
            for val in local_set {
                return_vec.push(val);
            }
            (return_vec, occurence_hash)
        }

        let tuple_vec: (Vec<i32>, HashMap<i32, i32>) = convert_to_hash(&mut nums);

        let mut dp_vec: Vec<i32> = Vec::with_capacity(tuple_vec.0.len());
        // populating the element
        for _ in 0..tuple_vec.0.len() {
            dp_vec.push(0);
        }
        let mut new_vec: Vec<i32> = tuple_vec.0.clone();
        new_vec.sort_by(|val_one, val_two| val_one.cmp(&val_two));

        // populating dp vec
        for (index, item) in new_vec.iter().enumerate() {
            if (index as i32) == 0 {
                if let Some(map_val) = tuple_vec.1.get(&item) {
                    dp_vec[index] = *item * *map_val;
                }
            }
            if (index as i32) == 1 {
                if let Some(found_key) = tuple_vec.1.get(&item) {
                    if new_vec[index - 1] + 1 == new_vec[index] {
                        dp_vec[index] = dp_vec[index - 1].max(*found_key * *item);
                    } else {
                        dp_vec[index] = dp_vec[index - 1] + *found_key * *item;
                    }
                }
            }
            if (index as i32) > 1 {
                if let Some(found_key) = tuple_vec.1.get(&item) {
                    if new_vec[index - 1] + 1 == new_vec[index] {
                        let sub_val: i32 = dp_vec[index - 2] + *found_key * *item;
                        dp_vec[index] = sub_val.max(dp_vec[index - 1]);
                    } else {
                        dp_vec[index] = dp_vec[index - 1] + *found_key * *item;
                    }
                }
            }
        }

        if let Some(last) = dp_vec.last() {
            return *last;
        } else {
            return 0;
        }
    }

    // rank transform of an array
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut cloned_prev: Vec<i32> = arr.clone();
        let mut rank_clone: Vec<i32> = cloned_prev.clone();
        arr.sort_by(|a, b| a.cmp(&b));
        let mut rank_hash: HashMap<i32, i32> = HashMap::new();

        for (index, item) in arr.iter().enumerate() {
            if (index as i32) > 0 {
                if let Some(last_el) = arr.get(index - 1) {
                    if *last_el == *item {
                        if let Some(last_rank) = rank_hash.get(last_el) {
                            rank_hash.insert(*item, *last_rank);
                        }
                    } else {
                        if let Some(last_rank) = rank_hash.get(last_el) {
                            rank_hash.insert(*item, *last_rank + 1);
                        }
                    }
                }
            }
            if (index as i32) == 0 {
                rank_hash.insert(*item, (index as i32) + 1);
            }
        }
        for (index, item) in cloned_prev.iter_mut().enumerate() {
            if let Some(found_el) = rank_hash.get(&item) {
                rank_clone[index] = *found_el;
            }
        }
        rank_clone
    }

    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut total_counter: i32 = 0;
        let new_vec: Vec<i32> = nums
            .into_iter()
            .map(|val| {
                if val % 2 == 0 {
                    return 0;
                } else {
                    return 1;
                }
            })
            .collect();

        let mut end: usize = 0;
        let mut start: usize = 0;
        let mut subarray_counter: i32 = 0;
        let mut local_counter: i32 = 0;
        fn odd_check(num: &i32) -> bool {
            if num % 2 == 1 { true } else { false }
        }
        while end < new_vec.len() {
            if odd_check(&new_vec[end]) {
                local_counter += 1;
            }
            if local_counter == k {
                // it is only updated if the subarray counter again reaches k
                subarray_counter = 0;
            }
            while local_counter >= k {
                if odd_check(&new_vec[start]) {
                    local_counter -= 1;
                }
                subarray_counter += 1;
                start += 1;
            }
            total_counter += subarray_counter;
            end += 1;
        }
        total_counter
    }

    pub fn min_steps(mut s: String, mut t: String) -> i32 {
        let mut count: i32 = 0;

        fn gen_occurence_hash(string: &String) -> HashMap<char, i32> {
            let mut map: HashMap<char, i32> = HashMap::new();
            let mut string_vec: Vec<char> = string.chars().collect();
            string_vec.sort_by(|a, b| a.cmp(&b));
            for curr_char in string_vec.iter() {
                if let Some(freq) = map.get_mut(&curr_char) {
                    *freq += 1;
                } else {
                    map.insert(*curr_char, 1);
                }
            }
            map
        }

        let s_hash: HashMap<char, i32> = gen_occurence_hash(&s);
        let mut t_hash: HashMap<char, i32> = gen_occurence_hash(&t);
        let mut s_set: HashSet<char> = HashSet::new();
        for s_char in s.chars() {
            s_set.insert(s_char);
        }
        let s_new: Vec<char> = s_set
            .into_iter()
            .map(|val| val)
            .collect();

        for s_char in s_new.iter() {
            if t_hash.contains_key(&s_char) {
                if let Some(s_freq) = s_hash.get(&s_char) {
                    if let Some(t_freq) = t_hash.get_mut(&s_char) {
                        *t_freq = *t_freq - *s_freq;
                    }
                }
            }
        }
        for (_, value) in t_hash {
            if value >= 0 {
                count += value;
            }
        }
        count
    }

    pub fn sort_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut even_vec: Vec<i32> = Vec::new();
        let mut odd_vec: Vec<i32> = Vec::new();
        fn odd_check(num: &i32) -> bool {
            if *num % 2 == 0 {
                return true;
            } else {
                return false;
            }
        }
        for curr_num in nums.iter() {
            if odd_check(curr_num) {
                even_vec.push(*curr_num);
            } else {
                odd_vec.push(*curr_num);
            }
        }

        even_vec.append(&mut odd_vec); // in this case append is taking the ownership of the odd vec
        even_vec
    }

    // house robber ii
    pub fn house_robber_ii(nums: Vec<i32>) -> i32 {
        // edge case for one element
        if nums.len() == (1 as usize) {
            if let Some(first) = nums.get(0) {
                return *first;
            }
        }

        fn make_vec(index: i32, array: &Vec<i32>) -> Vec<i32> {
            if index == 0 {
                return array
                    .get(index as usize..array.len() - 1)
                    .unwrap_or_default()
                    .to_vec();
            }
            return array
                .get(index as usize..)
                .unwrap_or_default()
                .to_vec();
        }

        let first_vec: Vec<i32> = make_vec(0, &nums);
        let two_vec: Vec<i32> = make_vec(1, &nums);

        fn get_total_robber(array: Vec<i32>) -> Vec<i32> {
            let mut dp: Vec<i32> = vec![0; array.len()];
            for (index, item) in array.iter().enumerate() {
                let curr_num: i32 = *item;
                if (index as i32) == 0 {
                    dp[index] = curr_num;
                }
                if (index as i32) == 1 {
                    dp[index] = curr_num.max(dp[index - 1]);
                }
                if (index as i32) > 1 {
                    dp[index] = dp[index - 1].max(curr_num + dp[index - 2]);
                }
            }
            dp
        }

        let mut dp_one: Vec<i32> = get_total_robber(first_vec);
        let mut dp_two: Vec<i32> = get_total_robber(two_vec);

        dp_one.append(&mut dp_two);
        let mut max: i32 = 0;
        for item in dp_one.into_iter() {
            if max < item {
                max = item;
            }
        }
        max
    }

    pub fn play_ground_two() {
        println!("Come and Pkay");

        let array: Vec<i32> = vec![1, 2, 3, 45, 5, 66];

        fn update_vec(local_vec: &Vec<i32>) -> Vec<i32> {
            let mut new: Vec<i32> = Vec::new();
            for item in local_vec.iter() {
                new.push(*item + 1);
            }
            new
        }

        let new: Vec<i32> = update_vec(&array);
        println!("{:?}{:?}", new, array);
    }

    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut portion: i32 = 0;
        let mut freq_map: HashMap<char, i32> = HashMap::new();
        for item in s.chars() {
            if let Some(freq) = freq_map.get_mut(&item) {
                *freq += 1;
            } else {
                freq_map.insert(item, 1);
            }
        }
        for (key, value) in freq_map {
            if key == letter {
                portion = (((value as f64) / (s.len() as f64)) * 100.0).floor() as i32; // reconverting is back to i32
            }
        }
        portion
    }

    // using a sliding window technique in order to solve the minimum recolors and retain the total count
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut counter: i32 = std::i32::MAX;
        let mut w_count: i32 = 0;
        let mut end: usize = 0;
        let mut start: usize = 0;
        let block_vec: Vec<char> = blocks.chars().collect();

        for index in 0..k {
            if let Some(curr_char) = block_vec.get(index as usize) {
                if *curr_char == 'W' {
                    w_count += 1;
                }
            }
        }
        // sliding the window and acquiring the letters
        end = k as usize;
        counter = counter.min(w_count);
        while end < block_vec.len() {
            if let Some(w_letter) = block_vec.get(start) {
                if *w_letter == 'W' {
                    w_count -= 1;
                }
            }
            if let Some(w_letter_end) = block_vec.get(end) {
                if *w_letter_end == 'W' {
                    w_count += 1;
                }
            }
            counter = counter.min(w_count); // recalculating the minimum count in order to keep track of any other combinations that might come
            start += 1;
            end += 1;
        }
        counter
    }

    // top example for mutable reference in rust where a tuple and max val is passed as mutable reference
    pub fn get_longest_palindrome(s: String) -> String {
        let s_vec: Vec<char> = s.chars().collect();
        let mut range_tuple: (i32, i32) = (0, 0);
        let mut max: i32 = 1;
        fn count_palindromes(
            mut left: i32,
            mut right: i32,
            array: &Vec<char>,
            tuple: &mut (i32, i32),
            max: &mut i32
        ) {
            while
                left >= 0 &&
                right < (array.len() as i32) &&
                array[left as usize] == array[right as usize]
            {
                if right - left + 1 > *max {
                    *max = right - left + 1;
                    tuple.1 = right as i32;
                    tuple.0 = left as i32;
                }
                left -= 1;
                right += 1;
            }
        }

        for (index, _) in s_vec.iter().enumerate() {
            count_palindromes(index as i32, index as i32, &s_vec, &mut range_tuple, &mut max);
            count_palindromes(index as i32, (index + 1) as i32, &s_vec, &mut range_tuple, &mut max);
        }
        let mut result: Vec<char> = Vec::new();
        if let Some(slice_str) = s_vec.get(range_tuple.0 as usize..(range_tuple.1 + 1) as usize) {
            result = slice_str.to_vec();
        }
        let string: String = result
            .iter()
            .map(|val| val.to_string())
            .collect();
        string
    }

    // finding intersection between two arrays
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::new();
        // populate one and check against second hash
        for item in nums1.iter() {
            set.insert(*item);
        }
        let mut result: HashSet<i32> = HashSet::new();
        for item_two in nums2.iter() {
            let curr_num: i32 = *item_two;
            if set.contains(&curr_num) {
                result.insert(curr_num);
            }
        }
        let array: Vec<i32> = result.into_iter().collect();
        array
    }

    // partition labels
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s_vec: Vec<char> = s.chars().into_iter().collect();
        let mut range: Vec<i32> = Vec::new();
        let mut map: HashMap<char, Vec<i32>> = HashMap::new();
        let mut start: i32 = 0;
        let mut end: i32 = 0;

        for (index, item) in s_vec.iter().enumerate() {
            let curr_index: i32 = index as i32;
            if !map.contains_key(item) {
                map.insert(*item, vec![curr_index, curr_index]);
            } else {
                if let Some(el_range) = map.get_mut(item) {
                    el_range[1] = curr_index;
                }
            }
        }

        let mut intervals: Vec<Vec<i32>> = Vec::new();
        for (_, value) in map.into_iter() {
            intervals.push(value.to_vec());
        }
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        //finding the sizes
        for (index, item) in intervals.iter().enumerate() {
            let local_start: i32 = item[0];
            let local_end: i32 = item[1];
            if (index as i32) == 0 {
                start = local_start;
                end = local_end;
            }
            if (index as i32) > 0 {
                if end < local_start {
                    let local_range: i32 = end - start + 1;
                    range.push(local_range);
                    start = local_start;
                    end = local_end;
                }
                // updating the end as the interval progresses
                if end < local_end {
                    end = local_end;
                }
            }
            if index == intervals.len() - 1 {
                let local_range: i32 = end - start + 1;
                range.push(local_range);
            }
        }
        range
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut sum: i32 = 0;
        map.insert(0, 1);
        for item in nums.iter() {
            let curr_num: i32 = *item;
            sum += curr_num;
            if map.contains_key(&(sum - k)) {
                // here the prefix sum is checked and calculated and sum is incremented accordingly
                if let Some(found_val) = map.get(&(sum - k)) {
                    count += *found_val;
                }
            }
            if let Some(found_sum) = map.get(&sum) {
                map.insert(sum, *found_sum + 1);
            } else {
                map.insert(sum, 1);
            }
        }
        count
    }

    // getting pivot index
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let pivot_index: i32 = -1;
        let sum: i32 = nums.iter().sum();
        let mut sum_left: i32 = 0;
        for (index, item) in nums.iter().enumerate() {
            if sum_left == sum - sum_left - *item {
                return index as i32;
            }
            sum_left += *item;
        }
        pivot_index
    }

    pub fn repeated_character(s: String) -> char {
        let last_char: char = 'c';
        let mut map: HashMap<char, i32> = HashMap::new();
        for curr_char in s.chars() {
            if let Some(char_val) = map.get_mut(&curr_char) {
                *char_val += 1;
                return curr_char;
            } else {
                map.insert(curr_char, 1);
            }
        }
        last_char
    }

    // longest character replacement using map to store
    pub fn longest_char_replacement(s: String, k: i32) -> i32 {
        let mut total_len: i32 = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut start: usize = 0;
        let mut end: usize = 0;
        let s_vec: Vec<char> = s.chars().collect();

        while end < s_vec.len() {
            if let Some(end_char) = map.get_mut(&s_vec[end]) {
                *end_char += 1;
            } else {
                map.insert(s_vec[end], 1);
            }
            if end - start + 1 - (*map.values().max().unwrap() as usize) <= (k as usize) {
                total_len = total_len.max((end - start + 1) as i32);
            }
            //checking for character replacement then add the object
            while end - start + 1 - (*map.values().max().unwrap() as usize) > (k as usize) {
                if let Some(map_val) = map.get_mut(&s_vec[start]) {
                    *map_val -= 1;
                    if *map_val == 0 {
                        map.remove(&s_vec[start]);
                    }
                }
                start += 1;
            }

            end += 1;
        }
        total_len
    }

    // function to find intersection between two arrays while keeping the count of the lowest count of the elements
    pub fn intersect_array(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        // ugly approach
        let mut array: Vec<i32> = Vec::new();
        let set_one: HashSet<i32> = nums1
            .iter()
            .map(|val| *val)
            .collect();
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for item in nums2.iter() {
            if set_one.contains(item) {
                map.insert(*item, vec![0, 0]);
            }
        }
        // function to get the occurence of the numbers based on the first or second sector of the map
        fn get_occurence(
            ref_map: &mut HashMap<i32, Vec<i32>>,
            num_vec: &mut Vec<i32>,
            map_type: String
        ) {
            if map_type == "one" {
                for item in num_vec.iter() {
                    if let Some(found_vec) = ref_map.get_mut(item) {
                        found_vec[0] += 1;
                    }
                }
            } else {
                for item in num_vec.iter() {
                    if let Some(found_vec) = ref_map.get_mut(item) {
                        found_vec[1] += 1;
                    }
                }
            }
        }
        get_occurence(&mut map, &mut nums1, String::from("one"));
        get_occurence(&mut map, &mut nums2, String::from("two"));

        // forming the array based on iter count
        for (key, value) in map {
            for _ in 0..value[0].min(value[1]) {
                array.push(key);
            }
        }

        array
    }

    pub fn fucking_function() -> Vec<i32> {
        let mut array = [1, 2, 3, 4];
        array[1] = 1;
        let array_one: Vec<i32> = vec![1, 2, 3, 4];

        for (index, item) in array.iter_mut().enumerate() {
            *item += 1;
        }
        println!("{:?}", array);

        let mut new_array: Vec<i32> = array
            .iter()
            .cloned()
            .map(|a| a + 1)
            .collect();

        Vec::new()
    }

    // finding shortest distance to char
    pub fn shortest_distance_to_char(mut s: String, mut c: char) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(s.len());
        let mut left: Vec<i32> = Vec::with_capacity(s.len());
        let mut right: Vec<i32> = Vec::with_capacity(s.len());
        let mut common_len: usize = s.len();

        // for filling 0 up
        fn predefine_vec(local_vec: &mut Vec<i32>, common_len: &mut usize) {
            for _ in 0..*common_len {
                local_vec.push(0);
            }
        }
        predefine_vec(&mut left, &mut common_len);
        predefine_vec(&mut right, &mut common_len);

        fn fill_vec(
            local_str: &mut String,
            array: &mut Vec<i32>,
            common_len: &mut usize,
            type_iter: String,
            check_char: &mut char
        ) {
            let mut distance: i32 = 0;
            // switch char here
            if type_iter == String::from("right") {
                for (curr_index, curr_item) in local_str.bytes().enumerate().rev() {
                    if (curr_item as char) == *check_char {
                        distance = *common_len as i32;
                        array[curr_index] = distance;
                    } else {
                        distance -= 1;
                        array[curr_index] = distance.max(0);
                    }
                }
            } else {
                for (curr_index, curr_item) in local_str.chars().enumerate() {
                    let mut index: usize = 0;
                    if type_iter == String::from("left") {
                        index = curr_index;
                    } else {
                        index = *common_len - 1 - curr_index;
                    }
                    if curr_item == *check_char {
                        distance = *common_len as i32;
                        array[index] = distance;
                    } else {
                        distance -= 1;
                        array[index] = distance.max(0);
                    }
                }
            }
        }

        fill_vec(&mut s, &mut left, &mut common_len, String::from("left"), &mut c);
        fill_vec(&mut s, &mut right, &mut common_len, String::from("right"), &mut c);

        // comparison
        for index in 0..common_len {
            result.push(
                ((common_len as i32) - left[index]).min((common_len as i32) - right[index])
            );
        }
        result
    }

    // uncommong stuff
    pub fn uncommon_from_sentences(mut s1: String, mut s2: String) -> Vec<String> {
        // clean approach
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut result: Vec<String> = Vec::new();

        fn get_occurence(local_str: &mut String, map: &mut HashMap<String, i32>) {
            let array: Vec<String> = local_str
                .split_whitespace()
                .map(|val| val.to_string())
                .collect();
            for word in array.into_iter() {
                match map.get_mut(&word) {
                    Some(occurence) => {
                        *occurence += 1;
                    }
                    None => {
                        map.insert(word, 1);
                    }
                }
            }
        }

        get_occurence(&mut s1, &mut map);
        get_occurence(&mut s2, &mut map);

        for (key, value) in map {
            if value == 1 {
                result.push(key);
            }
        }

        result
    }

    // pushing dominoes
    pub fn push_dominoes(dominoes: String) -> String {
        let common_len = dominoes.len();
        let dominoes_array: Vec<char> = dominoes.chars().into_iter().collect();
        let mut right: Vec<i32> = vec![0; common_len];
        let mut left: Vec<i32> = vec![0; common_len];
        let mut char_force: i32 = 0;

        // right side
        for (index, item) in dominoes_array.iter().enumerate() {
            let curr_item: char = *item;
            if curr_item == '.' {
                char_force -= 1;
            }
            if curr_item == 'L' {
                char_force = 0;
            }
            if curr_item == 'R' {
                char_force = common_len as i32;
                right[index] = common_len as i32;
            }
            right[index] = char_force.max(0);
        }

        char_force = 0;
        // left
        for (index, item) in dominoes_array.iter().enumerate().rev() {
            let curr_item = *item;
            if curr_item == '.' {
                char_force -= 1;
            }
            if curr_item == 'R' {
                char_force = 0;
            }
            if curr_item == 'L' {
                char_force = common_len as i32;
                left[index] = common_len as i32;
            }
            left[index] = char_force.max(0);
        }

        // creating a the new dominoes string by comparing the vecs
        let mut result_dominoes: Vec<char> = vec!['.';  common_len];

        for (index, item) in result_dominoes.iter_mut().enumerate() {
            let left_val: i32 = left[index];
            let right_val: i32 = right[index];
            if right_val == 0 && left_val > 0 {
                *item = 'L';
            } else if left_val == 0 && right_val > 0 {
                *item = 'R';
            } else if (left_val == 0 && right_val == 0) || left_val == right_val {
                *item = '.';
            } else if left_val > 0 && right_val > 0 {
                if left_val > right_val {
                    *item = 'L';
                } else {
                    *item = 'R';
                }
            }
        }
        let mut str_result: String = String::from("");
        for curr_char in result_dominoes.iter() {
            str_result.push(*curr_char);
        }

        str_result
    }

    // checking for word pattern
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut check: bool = true;
        let pattern_new: Vec<char> = pattern.chars().collect();
        let s_array: Vec<String> = s
            .split_whitespace()
            .into_iter()
            .map(|val| val.to_string())
            .collect();
        if pattern_new.len() != s_array.len() {
            return false;
        }
        let mut map: HashMap<char, String> = HashMap::new();
        // populating map
        for (index, curr_char) in pattern_new.iter().enumerate() {
            let local_char = *curr_char;
            match map.get_mut(&local_char) {
                Some(inner_string) => {
                    *inner_string = s_array[index].to_string();
                }
                None => {
                    map.insert(local_char, s_array[index].to_string());
                }
            }
        }
        for index in 0..pattern_new.len() {
            let pattern_char: char = pattern_new[index];
            let s_array_word: String = s_array[index].to_string();
            match map.get(&pattern_char) {
                Some(mapped_word) => {
                    if mapped_word.to_string() != s_array_word {
                        check = false;
                        break;
                    }
                }
                None => {}
            }
        }
        check
    }

    // frequency sort
    pub fn frequency_sort_two(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut array: Vec<i32> = Vec::new();
        for curr_num in nums.iter() {
            match map.get_mut(&curr_num) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(*curr_num, 1);
                }
            }
        }
        // sort the map
        let mut map_vec: Vec<(i32, i32)> = map.into_iter().collect();
        // if the frequency is equal then compare the numbers and rearrange them in decreasing else arrange em by increasing of frequency
        map_vec.sort_by(|(a, a_freq), (b, b_freq)| {
            if a_freq == b_freq { b.cmp(&a) } else { a_freq.cmp(&b_freq) }
        });
        // array formation
        for item in map_vec.into_iter() {
            for _ in 0..item.1 {
                array.push(item.0);
            }
        }
        array
    }

    // getting teh subarray ranges
    pub fn subarray_ranges(nums: Vec<i32>) -> i64 {
        let mut total: i64 = 0;
        for (index, item) in nums.iter().enumerate() {
            let curr_item: i32 = *item;
            let mut max: i32 = curr_item;
            let mut min: i32 = curr_item;
            for sub_index in index + 1..nums.len() {
                max = max.max(nums[sub_index]);
                min = min.min(nums[sub_index]);
                let diff: i64 = (max - min) as i64;
                total += diff;
            }
        }
        total
    }

    // min length of counter
    pub fn min_len(s: String) -> i32 {
        let s_vec: Vec<char> = s.chars().into_iter().collect();
        let mut stack: Vec<char> = Vec::new();

        for curr_char in s_vec.iter() {
            if *curr_char == 'B' || *curr_char == 'D' {
                let mut check: bool = false;
                if let Some(last_char) = stack.last() {
                    if *curr_char == 'B' && *last_char == 'A' {
                        check = true;
                    }
                    if *curr_char == 'D' && *last_char == 'C' {
                        check = true;
                    }
                }
                if check {
                    stack.pop();
                } else {
                    stack.push(*curr_char);
                }
            } else {
                stack.push(*curr_char);
            }
        }
        stack.len() as i32
    }

    //[1,1,0,1,1,1]
    pub fn max_consequtive_ones(nums: Vec<i32>) -> i32 {
        let mut counter: i32 = 0;
        let mut end: usize = 0;
        while end < nums.len() {
            let curr_num: i32 = nums[end];
            match curr_num == 1 {
                true => {
                    let mut local_counter: i32 = 0;
                    while end < nums.len() && nums[end] == 1 {
                        if curr_num == 0 {
                            break;
                        }
                        end += 1;
                        local_counter += 1;
                    }
                    counter = counter.max(local_counter);
                }
                false => {
                    end += 1;
                }
            }
        }
        counter
    }

    // checking for subsequence
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return false;
        }
        fn make_vec(string: String) -> Vec<char> {
            string.chars().collect()
        }
        // consumed
        let s_vec: Vec<char> = make_vec(s);
        let t_vec: Vec<char> = make_vec(t);

        let mut s_index: usize = 0;
        for t_item in t_vec.iter() {
            let t_char: char = *t_item;
            if t_char == s_vec[s_index] {
                s_index += 1;
            }
            if s_index == s_vec.len() {
                // if its equal stopping additional add ons for boundary error prevention
                break;
            }
        }
        if (s_index as usize) == s_vec.len() {
            return true;
        }
        false
    }

    // license key formatting
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut license_vec: Vec<char> = Vec::new();
        let s_vec: Vec<char> = s
            .chars()
            .filter(|&local_char| local_char != '-')
            .collect(); // get chars and add only the ones u need
        let mut local_counter = 0;
        for curr_char in s_vec.iter().rev() {
            if local_counter == k {
                local_counter = 0;
                license_vec.insert(0, '-');
            }
            // keeps on inserting if there is no hash needed
            license_vec.insert(0, *curr_char);

            local_counter += 1;
        }
        // loop approach
        let mut license_iter = license_vec.iter_mut();
        loop {
            match license_iter.next() {
                Some(curr_char) => {
                    if curr_char.is_alphabetic() {
                        *curr_char = curr_char.to_ascii_uppercase();
                    }
                }
                None => {
                    break;
                }
            }
        }
        let result: String = license_vec.into_iter().collect();
        result
    }

    // ransom note problem
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // function to generate occurence
        fn get_occurence(local_string: String) -> HashMap<char, i32> {
            let mut map: HashMap<char, i32> = HashMap::new();
            let array: Vec<char> = local_string.chars().collect();
            let mut array_iter = array.iter();
            loop {
                match array_iter.next() {
                    Some(curr_char) => {
                        match map.get_mut(curr_char) {
                            Some(curr_occurence) => {
                                *curr_occurence += 1;
                            }
                            None => {
                                map.insert(*curr_char, 1);
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            map
        }
        let ransom_hash: HashMap<char, i32> = get_occurence(ransom_note);
        let magazine_hash: HashMap<char, i32> = get_occurence(magazine);

        for (key, value) in magazine_hash.iter() {
            if ransom_hash.contains_key(&key) {
                if let Some(key_val) = ransom_hash.get(&key) {
                    if *key_val > *value {
                        return false;
                    }
                }
            }
        }
        for (key, char) in ransom_hash {
            if !magazine_hash.contains_key(&key) {
                return false;
            }
        }
        true
    }

    // finding winners
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut winner_vec: Vec<i32> = Vec::new();
        let mut loser_vec: Vec<i32> = Vec::new();
        // populating map
        for curr_num in matches.iter() {
            let num: Vec<i32> = curr_num.to_vec();
            let (winner, loser) = (num[0], num[1]);
            if !map.contains_key(&winner) {
                map.insert(winner, vec![0, 0]);
            }
            if !map.contains_key(&loser) {
                map.insert(loser, vec![0, 0]);
            }
        }

        // getting indexes and finding the values
        for curr_num in matches.into_iter() {
            let (winner, loser) = (curr_num[0], curr_num[1]);
            // getting the winners
            if map.contains_key(&winner) {
                match map.get_mut(&winner) {
                    Some(inner_vec) => {
                        inner_vec[0] = inner_vec[0] + 1;
                    }
                    None => {}
                }
            }
            // getting losers
            if map.contains_key(&loser) {
                match map.get_mut(&loser) {
                    Some(inner_vec) => {
                        inner_vec[1] = inner_vec[1] + 1;
                    }
                    None => {}
                }
            }
        }
        for (key, value) in map {
            if value[1] == 0 {
                winner_vec.push(key);
            }
            if value[1] == 1 {
                loser_vec.push(key);
            }
        }

        fn sort_stuff(array: &mut Vec<i32>) {
            array.sort_by(|a, b| a.cmp(&b))
        }
        //fetching winner and loser from vec

        sort_stuff(&mut winner_vec);
        sort_stuff(&mut loser_vec);
        result.push(winner_vec);
        result.push(loser_vec);
        result
    }

    // counting vowel substrings
    pub fn count_vowel_substrings(word: String) -> i32 {
        let mut total: i32 = 0;
        let mut end: usize = 0;
        // creates a default vowel map
        fn get_map() -> HashMap<char, i32> {
            let mut map: HashMap<char, i32> = HashMap::new();
            map.insert('a', 0);
            map.insert('i', 0);
            map.insert('e', 0);
            map.insert('o', 0);
            map.insert('u', 0);
            map
        }
        let word_array: Vec<char> = word.chars().collect();

        while end < word_array.len() {
            let mut map: HashMap<char, i32> = get_map();
            let curr_char = word_array[end];
            let mut sub_counter = 0;
            if map.contains_key(&curr_char) {
                for index in end..word_array.len() {
                    let sub_char: char = word_array[index];
                    match map.contains_key(&sub_char) {
                        true =>
                            match map.get_mut(&sub_char) {
                                Some(occurence) => {
                                    *occurence += 1;
                                }
                                None => {
                                    break;
                                }
                            }
                        false => {
                            break;
                        }
                    }
                    let check: bool = map.values().all(|val| *val > 0); // can be destructured
                    if check {
                        sub_counter += 1;
                    }
                }
            }
            total += sub_counter;
            end += 1;
        }
        total
    }

    // getting subsequence vals based on the subsequence entered and by cancelling chars
    pub fn is_find_word_in_sub(array: Vec<String>) -> i32 {
        let mut min_counter: i32 = std::i32::MAX;
        let word: String = array[0].clone();
        let dict: Vec<String> = array[1]
            .split(',')
            .map(|val| val.to_string())
            .collect();

        fn is_sub(dict_word: String, word: &mut String) -> bool {
            let dict_chars: Vec<char> = dict_word.chars().collect();
            let mut index: usize = 0;
            for curr_char in word.chars() {
                if curr_char == dict_chars[index] {
                    index += 1;
                }
                if index == dict_chars.len() {
                    break;
                }
            }
            index == dict_chars.len()
        }
        for dict_word in dict.iter() {
            let curr_word: String = dict_word.to_string();
            let curr_len = curr_word.len();
            let found_check = is_sub(curr_word, &mut word.to_string());
            if found_check {
                let found_len = (word.len() - curr_len) as i32;
                min_counter = min_counter.min(found_len);
            }
        }
        println!("{}", min_counter);
        min_counter
    }

    // first palindromic substring
    pub fn first_palindromic_substring(words: Vec<String>) -> String {
        fn second_check(curr_string: Vec<char>) -> bool {
            let rev_string: String = curr_string
                .iter()
                .rev()
                .map(|val| val.to_string())
                .collect();
            let normal_string: String = curr_string
                .iter()
                .map(|char| char.to_string())
                .collect();
            normal_string == rev_string
        }
        // regular pal check
        // fn palindrome_check(curr_string: Vec<char>)-> bool{
        //     let check = true;
        //     let half_len: usize = (curr_string.len() / 2 as usize) as usize;
        //     let mut start: usize = 0;
        //     let mut end: usize = curr_string.len() - 1 as usize;
        //     while start <= half_len && end >= half_len{
        //         if let Some(first) = curr_string.get(start){
        //             if let Some(last) = curr_string.get(end){
        //                 if *first != *last{
        //                     return false;
        //                 }
        //             }
        //         }
        //         start += 1;
        //         end -= 1;
        //     }
        //     check
        // }
        let word_iter = words.iter();
        for word in word_iter {
            let curr_string = word.to_string();
            if second_check(curr_string.chars().collect()) {
                return word.to_string();
            }
        }
        String::from("")
    }

    // longest sub without repeating chars using sliding window logic
    pub fn longest_sub_without_repeating_chars(s: String) -> i32 {
        let mut length: i32 = 0;
        let array: Vec<char> = s.chars().collect();
        let mut set: HashSet<char> = HashSet::new();
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut end: usize = 0;
        let mut start: usize = 0;

        while end < array.len() {
            // map to keep track of chars
            match map.get_mut(&array[end]) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(array[end], 1);
                }
            }
            set.insert(array[end]);
            if set.len() == end - start + (1 as usize) {
                length = length.max(set.len() as i32);
            }
            // checking length against empty set in order to define whether the length can be updated or not
            while !set.is_empty() && set.len() < end - start + (1 as usize) && start < array.len() {
                if let Some(occurence) = map.get_mut(&array[start]) {
                    *occurence -= 1;
                    if *occurence == 0 {
                        set.remove(&array[start]);
                        map.remove(&array[start]);
                    }
                }
                start += 1;
            }

            end += 1;
        }
        length
    }
    // minimum size of substring will be equal to the number of unique substring
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut counter: i32 = 0;
        let set: HashSet<i32> = nums
            .iter()
            .map(|val| *val)
            .collect();
        let k: usize = set.len();
        for index in 0..nums.len() {
            let mut sub_set: HashSet<i32> = HashSet::new();
            // checking all the subarrays from the starting points using set since it starts from index
            for sub_index in index..nums.len() {
                let curr_num: i32 = nums[sub_index];
                sub_set.insert(curr_num);
                if sub_set.len() == k {
                    counter += 1;
                }
            }
        }
        counter
    }

    // max area for containing water using two pointer approach to check the both sides
    pub fn container_water(height: Vec<i32>) -> i32 {
        let mut max_height: i32 = 0;
        let mut start: usize = 0;
        let mut end: usize = height.len() - 1;

        // two pointer approach
        while start < end {
            let start_height: i32 = height[start];
            let end_height: i32 = height[end];
            let index_gap: i32 = (end - start) as i32;
            let min_height: i32 = start_height.min(end_height);
            max_height = max_height.max(min_height * index_gap);
            if start_height < end_height {
                start += 1;
            } else {
                end -= 1;
            }
        }
        max_height
    }

    // common sequence of substring
    pub fn longest_substring(word: String) -> i32 {
        let mut max_len: i32 = 0;
        let vowel_array: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let mut index: usize = 0;
        let word_vec: Vec<char> = word.chars().collect();
        if word_vec.len() < 5 {
            return max_len;
        }
        while index < word_vec.len() {
            let curr_char = word_vec[index];
            if curr_char == 'a' {
                let start_index: usize = index;
                let mut check_index: usize = 0;
                let mut len_checker: i32 = 0;
                let mut check: bool = true;
                for (v_index, vowel) in vowel_array.iter().enumerate() {
                    let curr_vowel: char = *vowel;
                    // failed check
                    if index < word_vec.len() && curr_vowel != word_vec[index] {
                        check_index = v_index;
                        check = false;
                        break;
                    }
                    // checking for vowel first
                    while index < word_vec.len() && word_vec[index] == curr_vowel {
                        index += 1;
                        len_checker += 1;
                        check_index = v_index;
                    }
                }
                if check && len_checker >= 5 && check_index == 4 {
                    max_len = max_len.max((index - start_index) as i32);
                }
            } else {
                index += 1;
            }
        }
        max_len
    }

    // checking unique occurence
    pub fn unique_occurence(arr: Vec<i32>) -> bool {
        fn get_occurence(arr: Vec<i32>) -> HashMap<i32, i32> {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for curr_num in arr.iter() {
                match map.get_mut(curr_num) {
                    Some(occurence) => {
                        *occurence += 1;
                    }
                    None => {
                        map.insert(*curr_num, 1);
                    }
                }
            }
            map
        }
        let value_map: HashMap<i32, i32> = get_occurence(arr.clone());
        let occ_map: HashMap<i32, i32> = get_occurence(
            value_map
                .values()
                .to_owned()
                .map(|val| *val)
                .collect()
        );
        let check = occ_map.values().all(|val| val == &1);
        check
    }

    // getting the longest pal that can be build from the data
    pub fn longest_palindrome(s: String) -> i32 {
        let mut long_palindrome: i32 = 0;
        let array: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        for curr_char in array.iter() {
            match map.get_mut(curr_char) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(*curr_char, 1);
                }
            }
        }
        let mut check: bool = false;
        for (_, value) in map.iter() {
            if value % 2 == 1 {
                long_palindrome += *value - 1;
                check = true;
            }
            if value % 2 == 0 {
                long_palindrome += *value;
            }
        }
        if check {
            long_palindrome += 1;
        }
        long_palindrome
    }

    // longest word in dict
    pub fn longest_word_in_dict(mut words: Vec<String>) -> String {
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut collection: Vec<String> = Vec::new();
        let set: HashSet<String> = words
            .iter()
            .map(|word| word.to_string())
            .collect();
        let mut lng: usize = 0;
        for curr_word in words.iter() {
            let check_word: Vec<char> = curr_word.chars().collect();
            let mut local_str: String = String::from("");
            let mut check: bool = true;
            for curr_char in check_word.iter() {
                local_str.push(*curr_char);
                if !set.contains(&local_str) {
                    check = false;
                    break;
                }
            }
            if check {
                if curr_word.len() > lng {
                    lng = curr_word.len();
                }
                collection.push(curr_word.to_string());
            }
        }
        collection.retain(|word| word.len() == lng);
        collection.sort();
        // checking if there is a first rreturn or ignore
        match collection.first() {
            Some(word) => { word.to_string() }
            None => { String::from("") }
        }
    }

    // getting length of last word
    pub fn length_of_last_word(s: String) -> i32 {
        let s_array: Vec<String> = s
            .split_whitespace()
            .map(|slice| slice.to_string())
            .collect();
        let mut length_of_last_word: i32 = 0;
        for curr_word in s_array.iter().rev() {
            let curr_word_str = curr_word.to_string();
            length_of_last_word = curr_word_str.len() as i32;
            break;
        }
        length_of_last_word
    }

    // getting the peak index in a mountain array// solution must be on Ologn
    pub fn peak_index(arr: Vec<i32>) -> i32 {
        let mut peak: i32 = 0;
        let mut end: usize = 0;
        while end < arr.len() {
            let curr_el: i32 = arr[end];
            if let Some(next_el) = arr.get(end + 1) {
                if *next_el > curr_el {
                    while end + 1 < arr.len() && *arr.get(end + 1).unwrap() > arr[end] {
                        end += 1;
                    }
                    peak = end as i32;
                    break;
                }
            }
            end += 1;
        }
        peak
    }
    // can place flowers
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut check = false;
        let mut counter: i32 = n;
        if n == 0 {
            return true;
        }
        let mut bed: Vec<i32> = flowerbed
            .iter()
            .map(|val| *val)
            .collect();
        if bed.len() == 1 {
            if bed[0] == 0 {
                return counter <= 1;
            }
            return false;
        }
        // main code
        for index in 0..bed.len() {
            if index == 0 && bed[index] == 0 {
                if let Some(next) = bed.get(index + 1) {
                    if *next == 0 {
                        bed[index] = 1;
                        counter -= 1;
                    }
                }
            }
            if index >= 2 && index == bed.len() - 1 && bed[index] == 0 {
                if let Some(second_last) = bed.get(index - 1) {
                    if *second_last == 0 {
                        bed[index] = 1;
                        counter -= 1;
                    }
                }
            }
            if index > 1 && index < bed.len() - 1 && bed[index] == 0 {
                if let (Some(before), Some(after)) = (bed.get(index - 1), bed.get(index + 1)) {
                    if *before == 0 && *after == 0 {
                        bed[index] = 1;
                        counter -= 1;
                    }
                }
            }
            if counter == 0 {
                return true;
            }
        }
        if counter == 0 {
            check = true;
        }
        check
    }

    // getting halves of length of strings
    pub fn halves_in_string(s: String) -> bool {
        let mut s_vec: Vec<char> = s.chars().collect();
        let mut vowel_array: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let half_len = s.len() / 2;
        let mut counter_one = 0;
        let mut counter_two = 0;

        fn get_vowel_counter(counter: &mut i32, array: &mut [char], vowels: &mut Vec<char>) {
            for curr_char in array.iter() {
                if vowels.contains(curr_char) {
                    *counter += 1;
                }
            }
        }
        get_vowel_counter(&mut counter_one, &mut s_vec[0..half_len], &mut vowel_array);
        get_vowel_counter(&mut counter_two, &mut s_vec[half_len..s.len()], &mut vowel_array);
        counter_one == counter_two
    }

    // using single number
    pub fn single_num_iii(nums: Vec<i32>) -> Vec<i32> {
        let mut collection: Vec<i32> = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for curr_num in nums.iter() {
            let curr = *curr_num;
            match map.get_mut(&curr) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(curr, 1);
                }
            }
        }
        for (key, value) in map {
            if value == 1 {
                collection.push(key);
            }
        }
        collection
    }

    // word count
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut counter: i32 = 0;
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        fn populate_map(
            map: &mut HashMap<String, Vec<i32>>,
            type_str: String,
            words: &Vec<String>
        ) {
            for curr_string in words.iter() {
                let curr = curr_string.to_string();

                match map.get_mut(&curr) {
                    Some(occurrence) => {
                        if type_str == "one".to_string() {
                            occurrence[0] += 1;
                        } else if type_str == "two".to_string() {
                            occurrence[1] += 1;
                        }
                    }
                    None => {
                        if type_str == "one".to_string() {
                            map.insert(curr, vec![1, 0]);
                        } else {
                            map.insert(curr, vec![0, 1]);
                        }
                    }
                }
            }
        }
        populate_map(&mut map, "one".to_string(), &words1);
        populate_map(&mut map, "two".to_string(), &words2);
        for (_, value) in map {
            if value[0] == 1 && value[1] == 1 {
                counter += 1;
            }
        }
        counter
    }

    // fn to get rep chars
    pub fn rep_chars(s: String) -> char {
        let mut res_char: char = 'a'; // filler char
        let s_vec: Vec<char> = s.chars().into_iter().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        for curr_char in s_vec.into_iter() {
            match map.get_mut(&curr_char) {
                Some(occurence) => {
                    *occurence += 1;
                    if *occurence == 2 {
                        return curr_char;
                    }
                }
                None => {
                    map.insert(curr_char, 1);
                }
            }
        }
        res_char
    }

    // finding the most common word after removing banned words
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        // fn initial_attempt(){
        //     let mut max_occurence: i32 = 1;
        //     let spaced_out: Vec<String> = paragraph
        //         .split_whitespace()
        //         .map(|val| val.to_string().to_ascii_lowercase())
        //         .collect();
        //     let mut map: HashMap<String, i32> = HashMap::new();
        //     let mut new_array: Vec<String> = Vec::new();

        //     // extracting
        //     for curr_word in spaced_out.iter(){
        //         let curr: String = curr_word.to_string();
        //         let mut local_string: String = String::from("");
        //         for curr_char in curr.chars(){
        //             if curr_char.is_alphabetic(){
        //                 local_string.push(curr_char);
        //             }
        //         }
        //         new_array.push(local_string);
        //     }

        //     for curr_word in new_array.iter() {
        //         if !banned.contains(curr_word) {
        //             match map.get_mut(curr_word) {
        //                 Some(occurence) => {
        //                     *occurence += 1;
        //                     max_occurence = *occurence;
        //                 }
        //                 None => {
        //                     map.insert(curr_word.to_string(), 1);
        //                 }
        //             }
        //         }
        //     }
        //     for (key, value) in map {
        //         if value == max_occurence {
        //             return key;
        //         }
        //     }
        // }

        String::from("")
    }

    // max sub array using the simple concept of prefix sum
    pub fn max_subarray(nums: Vec<i32>) -> i32 {
        let mut max_sum: i32 = std::i32::MIN;
        let mut dp: Vec<i32> = Vec::new();
        for _ in 0..nums.len() {
            dp.push(0);
        }
        for (index, curr_num) in nums.iter().enumerate() {
            let num: i32 = *curr_num;
            if index == 0 {
                dp[index] = num;
                max_sum = max_sum.max(dp[index]);
            }
            if index > 0 {
                // check for index out of bound
                if let Some(prev_dp) = dp.get(index - 1) {
                    let local_sum: i32 = *prev_dp + num;
                    dp[index] = local_sum.max(num);
                    max_sum = max_sum.max(dp[index]);
                }
            }
        }
        max_sum
    }

    // calculating the max possible sum of the subarray
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_possible: i32 = std::i32::MIN;
        let mut min_possible: i32 = std::i32::MAX;
        let mut dp: Vec<i32> = Vec::new();
        let mut dp_two: Vec<i32> = Vec::new();
        fn populate_vec(dp: &mut Vec<i32>, length: usize) {
            for _ in 0..length {
                dp.push(0);
            }
        }
        populate_vec(&mut dp, nums.len());
        populate_vec(&mut dp_two, nums.len());
        // check for min sum
        for (index, item) in nums.iter().enumerate() {
            let curr_num = *item;
            if index == 0 {
                dp[index] = curr_num;
                dp_two[index] = curr_num;
                min_possible = min_possible.min(dp[index]);
                max_possible = max_possible.max(dp_two[index]);
            }
            if index > 0 {
                let sum_dp: i32 = dp[index - 1] + curr_num;
                let sum_dp_two: i32 = dp_two[index - 1] + curr_num;
                dp[index] = sum_dp.min(curr_num);
                dp_two[index] = sum_dp_two.max(curr_num);
                min_possible = min_possible.min(dp[index]);
                max_possible = max_possible.max(dp_two[index]);
            }
        }
        if max_possible.abs() > min_possible.abs() {
            max_possible.abs()
        } else {
            min_possible.abs()
        }
    }

    // is toeplix matrix for checking top left value of the matrix
    pub fn is_toeplix_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut check = true;
        for i in 0..matrix.len() {
            if i == 0 {
                continue;
            }
            for j in 0..matrix[i].len() {
                if j == 0 {
                    continue;
                }
                let curr_el = matrix[i][j];
                if let Some(top_left) = matrix.get(i - 1).and_then(|row| row.get(j - 1)) {
                    if curr_el != *top_left {
                        check = false;
                        break;
                    }
                }
            }
        }
        check
    }

    // getting the word pattern
    pub fn word_pattern_check(pattern: String, s: String) -> bool {
        let mut check: bool = true;
        let s_array: Vec<String> = s
            .split_whitespace()
            .map(|val| val.to_string())
            .collect();
        let p_array: Vec<char> = pattern.chars().collect();
        let mut map: HashMap<char, String> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();

        if s_array.len() != p_array.len() {
            // edge case for different lengths
            return false;
        }

        // add pattern
        for index in 0..s_array.len() {
            let curr_word = &s_array[index];
            let curr_pat = p_array[index];
            if map.contains_key(&curr_pat) {
                continue;
            }
            if set.contains(curr_word) {
                continue;
            }
            set.insert(curr_word.to_string());
            map.insert(curr_pat, curr_word.to_string());
        }

        for (index, word) in s_array.iter().enumerate() {
            let curr_word = word.to_string();
            let curr_pat_char = p_array[index];
            if !map.contains_key(&curr_pat_char) {
                check = false;
                break;
            }
            if let Some(map_word) = map.get(&curr_pat_char) {
                if curr_word != map_word.to_string() {
                    check = false;
                    break;
                }
            }
        }
        // checking pattern
        check
    }

    // max vowels
    pub fn max_vowels_in_given_sub(s: String, k: i32) -> i32 {
        // fn initial_unoptimized_approach(s:&String)->i32{
        //     let mut map:HashMap<char, i32> = HashMap::new();
        //     let array: Vec<char> = s.to_string().chars().collect();
        //     map.insert('a', 0);
        //     map.insert('i', 0);
        //     map.insert('e', 0);
        //     map.insert('o', 0);
        //     map.insert('u', 0);

        //     // initial render
        //     for index in 0..k{
        //       let curr_char: char = array[index as usize];
        //       if map.contains_key(&curr_char){
        //         match map.get_mut(&curr_char){
        //             Some(occurence)=>{
        //                 *occurence += 1;
        //             }
        //             None => {
        //             }
        //         }
        //       }
        //     }
        //     let mut max_count: i32 = map.values().sum();
        //     let mut start: usize = 0;
        //     for index in k as usize..(array.len() as usize){
        //         if map.contains_key(&array[start]){
        //             if let Some(occurence) = map.get_mut(&array[start]){
        //                 *occurence -= 1;
        //             }
        //         }
        //         if map.contains_key(&array[index as usize]){
        //             if let Some(occurence) = map.get_mut(&array[index as usize]){
        //                 *occurence += 1;
        //             }
        //         }
        //         max_count = max_count.max(map.values().sum());
        //         start += 1;
        //     }
        //     max_count
        // }
        // initial_unoptimized_approach(&s);

        // optimized approach
        fn is_vowel(char: &char) -> bool {
            let check_char: char = *char;
            check_char == 'a' ||
                check_char == 'e' ||
                check_char == 'i' ||
                check_char == 'o' ||
                check_char == 'u'
        }
        let mut counter: i32 = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..k {
            let curr_char = chars[i as usize];
            if is_vowel(&curr_char) {
                counter += 1;
            }
        }
        let mut max_counter: i32 = counter;
        let mut start: usize = 0;
        for i in k as usize..chars.len() {
            if is_vowel(&chars[start]) {
                counter -= 1;
            }
            if is_vowel(&chars[i]) {
                counter += 1;
            }
            max_counter = max_counter.max(counter);
            start += 1;
        }
        max_counter
    }

    pub fn max_freq(nums: Vec<i32>) -> i32 {
        let mut total: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut num_iter = nums.iter();
        let mut max_freq: i32 = 0;
        loop {
            match num_iter.next() {
                Some(num) => {
                    match map.get_mut(num) {
                        Some(occurence) => {
                            *occurence += 1;
                            max_freq = max_freq.max(*occurence);
                        }
                        None => {
                            map.insert(*num, 1);
                            max_freq = max_freq.max(1);
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        for (_, value) in map {
            if value == max_freq {
                total += value;
            }
        }
        total
    }
    // returning the number of subarrays that passes the count threshold
    pub fn num_of_subarrays_two(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut sub_array_count: i32 = 0;
        let mut total: i32 = 0;
        // initial check
        for i in 0..k {
            let curr_i = i as usize;
            let curr_num = arr[curr_i];
            total += curr_num;
        }
        let curr_avg: i32 = total / k;
        if curr_avg >= threshold {
            sub_array_count += 1;
        }
        let mut start: usize = 0;
        let mut check_iter = arr.iter().skip(k as usize);

        // window iterator // slower approach if iterator through every window
        let win_iter = arr.windows(k as usize);
        for curr_win in win_iter {
            let win_sum: i32 = curr_win.into_iter().sum();
            if win_sum / k >= threshold {
                sub_array_count += 1;
            }
        }
        // sliding window approach
        loop {
            match check_iter.next() {
                Some(curr_val) => {
                    total -= arr[start];
                    total += *curr_val;
                    if total / k >= threshold {
                        sub_array_count += 1;
                    }
                    start += 1;
                }
                None => {
                    break;
                }
            }
        }
        sub_array_count
    }

    pub fn count_good_sub(s: String) -> i32 {
        let mut count = 0;
        let array: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut start: usize = 0;
        let s_iter = array.iter().skip(3);

        //edge case
        if array.len() < 3 {
            return 0;
        }

        // first three
        for i in 0..3 {
            let index = i as usize;
            match map.get_mut(&array[index]) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(array[index], 1);
                }
            }
        }
        if map.len() == 3 {
            count += 1;
        }
        for item in s_iter {
            let curr_char = *item;
            if let Some(first_char) = map.get_mut(&array[start]) {
                *first_char -= 1;
                if *first_char == 0 {
                    map.remove(&array[start]);
                }
            }
            match map.get_mut(item) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(curr_char, 1);
                }
            }
            if map.len() == 3 {
                count += 1;
            }
            start += 1;
        }
        count
    }

    // getting kth distinct string
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut iter = arr.iter();
        let mut track_vec: Vec<(String, i32)> = Vec::new();
        loop {
            match iter.next() {
                Some(val) => {
                    let str: String = val.to_string();
                    match map.get_mut(val) {
                        Some(occurence) => {
                            *occurence += 1;
                        }
                        None => {
                            map.insert(str, 1);
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        let iter_two = arr.iter();
        let mut order: i32 = 1;
        for (_, curr_str) in iter_two.enumerate() {
            if map.contains_key(curr_str) {
                if let Some(occurence) = map.get(curr_str) {
                    if *occurence == 1 {
                        track_vec.push((curr_str.to_string(), order));
                        order += 1;
                    }
                }
            }
        }
        if track_vec.len() < (k as usize) {
            String::from("")
        } else {
            let mut result: String = String::from("");
            for item in track_vec.iter() {
                if item.1 == k {
                    result = item.0.to_string();
                }
            }
            result
        }
    }
    // finding relative ranks
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_score: Vec<i32> = score.clone();
        sorted_score.sort_by(|a, b| b.cmp(&a));
        let mut collection: Vec<String> = Vec::new();
        let mut map: HashMap<i32, String> = std::collections::HashMap::new();
        for (index, item) in sorted_score.iter().enumerate() {
            if index == 0 {
                map.insert(*item, String::from("Gold Medal"));
            }
            if index == 1 {
                map.insert(*item, String::from("Silver Medal"));
            }
            if index == 2 {
                map.insert(*item, String::from("Bronze Medal"));
            }
            if index > 2 {
                map.insert(*item, (index + 1).to_string());
            }
        }
        for item in score.iter() {
            if map.contains_key(item) {
                if let Some(map_value) = map.get(item) {
                    collection.push(map_value.to_string());
                }
            }
        }
        collection
    }

    // getting the min set size
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut set_size: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut arr_iter = arr.iter();
        let half_len: i32 = (arr.len() / 2) as i32;
        loop {
            match arr_iter.next() {
                Some(curr_value) => {
                    match map.get_mut(curr_value) {
                        Some(occurence) => {
                            *occurence += 1;
                        }
                        None => {
                            map.insert(*curr_value, 1);
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        // need to acquire the maximum set occurence
        let mut check_vec: Vec<(i32, i32)> = Vec::new();
        let mut total: i32 = 0;
        for (key, value) in map {
            check_vec.push((key, value));
        }
        check_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for curr_iterm in check_vec {
            let curr_size = curr_iterm.1;
            total += curr_size;
            set_size += 1;
            if total >= half_len {
                break;
            }
        }
        set_size
    }

    // getting the two most k freq elements
    pub fn top_k_frequent_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut collection: Vec<i32> = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut freq_counter: i32 = 0;
        let num_iter = nums.iter();
        for curr_num in num_iter {
            if let Some(occurence) = map.get_mut(&curr_num) {
                *occurence += 1;
            } else {
                map.insert(*curr_num, 1);
            }
        }
        let mut oc_vec: Vec<(i32, i32)> = Vec::new();
        for (key, value) in map {
            oc_vec.push((key, value));
        }
        oc_vec.sort_by(|a, b| b.1.cmp(&a.1));
        for curr_item in oc_vec.iter() {
            if freq_counter == k {
                break;
            }
            collection.push(curr_item.0);
            freq_counter += 1;
        }
        collection
    }

    // finding all anagrams in a string
    pub fn find_all_anagrams(s: String, p: String) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::new();
        let mut map: HashMap<char, i32> = HashMap::new();
        for curr_char in p.chars() {
            if let Some(occurence) = map.get_mut(&curr_char) {
                *occurence += 1;
            } else {
                map.insert(curr_char, 1);
            }
        }
        let s_vec: Vec<_> = s.chars().collect();
        let mut s_map: HashMap<char, i32> = HashMap::new();
        for index in 0..p.len() {
            if let Some(s_char) = s_vec.get(index) {
                match s_map.get_mut(s_char) {
                    Some(occurence) => {
                        *occurence += 1;
                    }
                    None => {
                        s_map.insert(*s_char, 1);
                    }
                }
            }
        }
        if map == s_map {
            indices.push(0);
        }
        // sliding iteration
        let mut start: usize = 0;
        for index in p.len()..s.len() {
            if let Some(start_char) = s_map.get_mut(&s_vec[start]) {
                *start_char -= 1;
                if *start_char == 0 {
                    s_map.remove(&s_vec[start]);
                }
            }
            start += 1;
            if let Some(end_char_val) = s_map.get_mut(&s_vec[index]) {
                *end_char_val += 1;
            } else {
                s_map.insert(s_vec[index], 1);
            }
            if map == s_map {
                indices.push(start as i32);
            }
        }
        indices
    }

    // number of zero filled subarrays // 46 out of 48 passed... not complete
    pub fn zero_filled_subarrays(mut nums: Vec<i32>) -> i64 {
        let mut counter: i32 = 0;
        let mut started: usize = 0;
        let mut end: usize = 0;
        nums.push(1); // offsetting value in order to match the final count
        while end < nums.len() {
            if let Some(curr_num) = nums.get(end) {
                if *curr_num == 0 {
                    started = end;
                    while end < nums.len() && nums[end] == 0 {
                        end += 1;
                        // condition for when it hits the end
                        if end == nums.len() - 1 {
                            break;
                        }
                    }
                    let curr_len: usize = end - started;
                    let curr_sub_count: i32 = ((curr_len as i32) * ((curr_len as i32) + 1)) / 2;
                    counter += curr_sub_count;
                    started = end;
                } else {
                    end += 1;
                }
            } else {
                end += 1;
                started = end;
            }
        }
        counter as i64
    }

    // getting the max power
    pub fn max_power(s: String) -> i32 {
        let mut count: i32 = 0;
        let mut s_vec: Vec<char> = s.chars().collect();
        s_vec.push('#'); // just for the last length index
        let mut started: usize = 0;
        let mut curr_char: char = s_vec[0];
        for index in 1..s_vec.len() {
            if let Some(local_char) = s_vec.get(index) {
                if *local_char != curr_char {
                    let curr_len = index - started;
                    count = count.max(curr_len as i32);
                    curr_char = *local_char;
                    started = index;
                }
            }
        }
        count
    }

    //23456

    // basic problem to get the largest value based on key and value being the same
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut lucky_one: i32 = -1;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for curr_num in arr.iter() {
            let num: i32 = *curr_num;
            match map.get_mut(curr_num) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(num, 1);
                }
            }
        }
        for (key, value) in map {
            if key == value {
                lucky_one = lucky_one.max(key);
            }
        }
        lucky_one
    }

    // longest consequtive sequence using sets
    pub fn longest_consequtive_sequence(nums: Vec<i32>) -> i32 {
        let mut max_len: i32 = 0;
        let set: HashSet<i32> = nums
            .iter()
            .map(|val| *val)
            .collect();
        for curr_num in &set {
            let left: i32 = curr_num - 1;
            if !set.contains(&left) {
                let mut right: i32 = *curr_num + 1;
                let mut local_len: i32 = 0;
                while set.contains(&right) {
                    right += 1;
                    local_len += 1;
                }
                max_len = max_len.max(local_len + 1);
            }
        }
        max_len
    }

    // similar pairs with sets
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count: i32 = 0;
        let mut map: HashMap<String, i32> = HashMap::new();
        for curr_word in words.iter() {
            let mut local_set: HashSet<char> = HashSet::new();
            for curr_char in curr_word.to_string().chars() {
                local_set.insert(curr_char);
            }
            let mut local_array: Vec<char> = Vec::new();
            for char in local_set {
                local_array.push(char);
            }
            local_array.sort_by(|a, b| a.cmp(&b));
            let new_str: String = local_array
                .into_iter()
                .map(|char| char)
                .collect();
            match map.get_mut(&new_str) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(new_str, 1);
                }
            }
        }
        for (_, value) in map {
            if value > 1 {
                let pair_count = (value * (value - 1)) / 2;
                count += pair_count;
            }
        }
        count
    }

    // push dominoes retry using vector intervals
    pub fn push_dom(dominoes: String) -> String {
        let doms: Vec<char> = dominoes.chars().collect();
        let mut dominoes_str: String = String::from("");
        let common_len: usize = dominoes.len();
        let mut right_doms: Vec<i32> = vec![0; common_len];
        let mut left_doms: Vec<i32> = vec![0; common_len];
        let mut dom_force: i32 = 0;
        // populating the left and right doms with the force
        for (index, curr_piece) in doms.iter().enumerate() {
            let piece: char = *curr_piece;
            if piece == '.' {
                dom_force -= 1;
            }
            if piece == 'R' {
                dom_force = common_len as i32;
            }
            if piece == 'L' {
                dom_force = 0;
            }
            right_doms[index] = dom_force.max(0);
        }
        // populating the left side
        dom_force = 0;
        for (index, curr_piece) in doms.iter().enumerate().rev() {
            let piece: char = *curr_piece;
            if piece == '.' {
                dom_force -= 1;
            }
            if piece == 'L' {
                dom_force = common_len as i32;
            }
            if piece == 'R' {
                dom_force = 0;
            }
            left_doms[index] = dom_force.max(0);
        }
        // compare and populate the new dom array
        for common_index in 0..doms.len() {
            let right_doms_val: i32 = right_doms[common_index];
            let left_doms_val: i32 = left_doms[common_index];
            if (right_doms_val == 0 && left_doms_val == 0) || right_doms_val == left_doms_val {
                dominoes_str.push('.');
            }
            if right_doms_val > left_doms_val {
                dominoes_str.push('R');
            }
            if left_doms_val > right_doms_val {
                dominoes_str.push('L');
            }
        }
        dominoes_str
    }

    // vowel end and start
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut counter: i32 = 0;
        let vowels: Vec<char> = vec!['a', 'i', 'e', 'o', 'u'];
        for i in left..right + 1 {
            let index: usize = i as usize;
            let curr_word: Vec<char> = words[index].to_string().chars().collect();
            if let Some(first) = curr_word.first() {
                if let Some(last) = curr_word.last() {
                    if vowels.contains(first) && vowels.contains(last) {
                        counter += 1;
                    }
                }
            }
        }
        counter
    }

    pub fn random_shit() {
        let mut vector_something: Vec<i32> = Vec::new();

        fn something(vector: &mut Vec<i32>) {
            for index in 0..100 {
                vector.push(index);
            }
        }
        something(&mut vector_something);
        let mut array: Vec<i32> = Vec::new();

        for (index, curr_item) in vector_something.into_iter().enumerate() {
            array.push(curr_item);
        }

        println!("{:?}", array);
    }

    // finding the shortest distance to char
    pub fn shortest_to_char_try(s: String, c: char) -> Vec<i32> {
        let mut right: Vec<i32> = vec![0; s.len()];
        let mut left: Vec<i32> = vec![0; s.len()];
        let mut final_array: Vec<i32> = vec![0; s.len()];
        let mut char_force: i32 = 0;
        let s_vec: Vec<char> = s.chars().collect();
        let s_iter = s_vec.iter();
        // right
        for (index, curr_char) in s_iter.enumerate() {
            if *curr_char == c {
                char_force = s.len() as i32;
            }
            if *curr_char != c {
                char_force -= 1;
            }
            right[index] = char_force.max(0);
        }
        // left
        for (index, curr_char) in s_vec.iter().enumerate().rev() {
            if *curr_char == c {
                char_force = s.len() as i32;
            }
            if *curr_char != c {
                char_force -= 1;
            }
            left[index] = char_force.max(0);
        }
        for i in 0..final_array.len() {
            let right_val = right[i];
            let left_val = left[i];
            let min_val: i32 = ((s.len() as i32) - right_val).min((s.len() as i32) - left_val);
            final_array[i] = min_val;
        }
        final_array
    }

    // top k most frequent words
    pub fn top_k_frequent(mut words: Vec<String>, k: i32) -> Vec<String> {
        words.sort_by(|a, b| a.cmp(&b)); // initial sort to get the first lexicological arrangment
        let mut map: HashMap<String, i32> = HashMap::new();
        for word in words.iter() {
            match map.get_mut(word) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(word.to_string(), 1);
                }
            }
        }
        let mut map_tuple: Vec<(String, i32)> = map
            .into_iter()
            .map(|val| (val.0.to_string(), val.1))
            .collect();
        // here while sorting keep it mind the frequency of the words
        map_tuple.sort_by(|a, b| if a.1 != b.1 { b.1.cmp(&a.1) } else { a.0.cmp(&b.0) });
        let mut counter: i32 = k;
        let mut collection: Vec<String> = Vec::new();
        for curr_iterm in map_tuple.iter() {
            collection.push(curr_iterm.0.to_string());
            counter -= 1;
            if counter == 0 {
                break;
            }
        }
        collection
    }

    // reverse string in a set
    pub fn reverse_words(s: String) -> String {
        let s_vec: Vec<String> = s
            .split_whitespace()
            .map(|val| val.to_string())
            .collect();
        let mut rev_vec: Vec<String> = Vec::new();
        let mut result: String = String::from("");
        for curr_word in s_vec.iter() {
            let mut word: Vec<char> = curr_word.to_string().chars().collect();
            for index in 0..word.len() / 2 {
                let word_len = word.len();
                word.swap(index, word_len - index - 1); // swap function to reverse the letter
            }
            let word_string: String = word
                .iter()
                .map(|val| val)
                .collect();
            rev_vec.push(word_string);
        }

        for (index, word) in rev_vec.iter().enumerate() {
            result.push_str(word);
            if index != rev_vec.len() - 1 {
                result.push(' ');
            }
        }
        result
    }

    // partition string checking for unique chars everytime
    pub fn partition_string_two(s: String) -> i32 {
        let mut counter: i32 = 0;
        let mut s_vec: Vec<char> = s.chars().collect();
        s_vec.push('#');
        let mut set: HashSet<char> = HashSet::new();

        for (index, curr_char) in s_vec.iter().enumerate() {
            if set.contains(curr_char) {
                counter += 1;
                set = HashSet::new();
            }

            set.insert(*curr_char);
            if !set.is_empty() && index == s_vec.len() - 1 {
                counter += 1;
            }
        }
        counter
    }

    // length of LIS
    pub fn longest_increasing_subsequence(nums: Vec<i32>) -> i32 {
        let mut max_len: i32 = 0;
        let mut dp: Vec<i32> = vec![1; nums.len()];
        for index in 0..nums.len() {
            let base_el: i32 = nums[index];
            for sub_index in 0..index {
                let check_el: i32 = nums[sub_index];
                if base_el > check_el {
                    // updates from the previous dp vals in order to increase the subsequence
                    dp[index] = (dp[sub_index] + 1).max(dp[index]);
                }
            }
        }
        if let Some(max_val) = dp.into_iter().max() {
            max_len = max_val;
        }
        max_len
    }

    // kth largest in a sorted order
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut el: i32 = 0;
        nums.sort();
        let rev_vec: Vec<i32> = nums
            .iter()
            .rev()
            .map(|val| *val)
            .collect();
        let mut counter: i32 = 0;
        for curr_num in rev_vec.iter() {
            counter += 1;
            if counter == k {
                el = *curr_num;
                break;
            }
        }
        el
    }

    // largest good integer
    pub fn largest_good_integer(num: String) -> String {
        let num_vec: Vec<char> = num.chars().collect();
        let mut curr_num_check: i32 = -1;
        let mut collection: Vec<i32> = Vec::new();
        let mut end: usize = 1;
        let mut counter: i32 = 1;
        let mut start: char = num_vec[0];
        while end < num_vec.len() {
            let curr_char = num_vec[end];
            if curr_char == start {
                counter += 1;
            }
            if curr_char != start {
                start = curr_char;
                counter = 1;
            }
            if counter == 3 {
                if let Some(digit) = curr_char.to_digit(10) {
                    collection.push(digit as i32);
                }
            }
            end += 1;
        }
        for curr_num in collection.iter() {
            if *curr_num > curr_num_check {
                curr_num_check = *curr_num;
            }
        }
        let mut result: String = String::from("");
        if curr_num_check == -1 {
            return result;
        }
        let char_digit = std::char::from_digit(curr_num_check as u32, 10).unwrap_or('?');

        for _ in 0..3 {
            result.push(char_digit);
        }
        result
    }

    // has all codes
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let s_vec: Vec<char> = s.chars().collect();
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut binary_str: String = String::from("");
        // edge case
        if s.len() < (k as usize) {
            return false;
        }
        for index in 0..k {
            let curr_char = s_vec[index as usize];
            binary_str.push(curr_char);
        }
        map.insert(binary_str.clone(), 1);
        // sliding window
        for index in k as usize..s_vec.len() {
            binary_str.drain(..1);
            binary_str.push(s_vec[index]);
            match map.get_mut(&binary_str) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(binary_str.clone(), 1);
                }
            }
        }
        let map_size: i32 = map.len() as i32;
        let need_size: i32 = (2u32).pow(k as u32) as i32;
        need_size == map_size
    }

    // checking maximum number of unique palindromic subseqeunce of length 3
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s_vec: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut set: HashSet<String> = HashSet::new();
        let mut left_set: HashSet<char> = HashSet::new();
        for letter in s_vec.iter() {
            match map.get_mut(letter) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(*letter, 1);
                }
            }
        }
        // main iteration for checking
        let characters: [char; 26] = [
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
            'g',
            'h',
            'i',
            'j',
            'k',
            'l',
            'm',
            'n',
            'o',
            'p',
            'q',
            'r',
            's',
            't',
            'u',
            'v',
            'w',
            'x',
            'y',
            'z',
        ];
        for curr_char in s_vec.iter() {
            if map.contains_key(curr_char) {
                // reducing right key if its available
                if let Some(occurence) = map.get_mut(curr_char) {
                    *occurence -= 1;
                    if *occurence == 0 {
                        map.remove(curr_char);
                    }
                }
            }
            // for an O(n) the iteration of all the 26 letters is necessary in order to check for possible unique substrings and are stored in sets in order to check the final size of the palindromic substring.
            for index in 0..26 {
                let curr_letter: char = characters[index as usize];
                if left_set.contains(&curr_letter) && map.contains_key(&curr_letter) {
                    let mut res: String = String::from("");
                    res.push(curr_letter);
                    res.push(*curr_char);
                    res.push(curr_letter);
                    set.insert(res.to_string());
                }
            }
            left_set.insert(*curr_char);
        }
        set.len() as i32
    }

    // strictly largest max ascending sum
    pub fn max_ascending_sum(mut nums: Vec<i32>) -> i32 {
        let mut max_size: i32 = 0;
        nums.push(0);
        let mut local_total = nums[0];

        for index in 1..nums.len() {
            let curr_num = nums[index];
            // strictly ascending
            if curr_num > nums[index - 1] {
                local_total += curr_num;
            }
            if curr_num <= nums[index - 1] {
                max_size = max_size.max(local_total);
                local_total = curr_num;
            }
        }
        max_size
    }

    // solve for every string in the list of arrays
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut total: i32 = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        for curr_char in chars.chars() {
            match map.get_mut(&curr_char) {
                Some(ocurrence) => {
                    *ocurrence += 1;
                }
                None => {
                    map.insert(curr_char, 1);
                }
            }
        }
        for curr_word in words.iter() {
            let word = curr_word.to_string();
            let mut copy_map: HashMap<char, i32> = map.clone();
            let mut check: bool = true;
            for curr_letter in word.chars() {
                if !copy_map.contains_key(&curr_letter) {
                    check = false;
                    break;
                }
                if copy_map.contains_key(&curr_letter) {
                    if let Some(occurence) = copy_map.get_mut(&curr_letter) {
                        *occurence -= 1;
                        if *occurence == 0 {
                            copy_map.remove(&curr_letter);
                        }
                    }
                }
            }
            if check {
                total += word.len() as i32;
            }
        }
        total
    }

    // min deletions -> optimization needed
    pub fn min_deletions(s: String) -> i32 {
        let mut min_counter: i32 = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, i32> = HashMap::new();
        for curr_char in s_chars.iter() {
            match map.get_mut(curr_char) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(*curr_char, 1);
                }
            }
        }
        let mut map_freq: Vec<i32> = map
            .values()
            .map(|val| *val)
            .collect();
        map_freq.sort_by(|a, b| b.cmp(a));
        // checking against has
        let mut oc_map: HashMap<i32, i32> = HashMap::new();
        for num in map_freq.iter() {
            match oc_map.get_mut(&num) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    oc_map.insert(*num, 1);
                }
            }
        }
        // main algorithm to check
        for index in 0..map_freq.len() {
            while *oc_map.get(&map_freq[index]).unwrap_or(&0) > 1 {
                if let Some(main_oc) = oc_map.get_mut(&map_freq[index]) {
                    *main_oc -= 1;
                }
                map_freq[index] -= 1;
                min_counter += 1;
                if map_freq[index] > 0 {
                    if let Some(check_oc) = oc_map.get_mut(&map_freq[index]) {
                        *check_oc += 1;
                    } else {
                        oc_map.insert(map_freq[index], 1);
                    }
                }
            }
        }
        min_counter
    }

    // number of zero filled subarrays
    pub fn zero_filled(mut nums: Vec<i32>) -> i64 {
        // note: becareful of integer over flow during calculations
        // substring calc from a mutable reference
        fn formulaic_approach(mut nums: &mut Vec<i32>) -> i64 {
            let mut count: i64 = 0;
            fn calc_substring(zero_count: &mut i64) -> i64 {
                let mut sub_count: i64 = 0;
                sub_count += ((*zero_count * (*zero_count + 1)) / 2) as i64;
                sub_count
            }
            // one pass iteration
            let mut zero_count: i64 = 0;
            for index in 0..nums.len() {
                let curr_num = nums[index];
                if curr_num == 0 {
                    zero_count += 1;
                } else if curr_num != 0 {
                    // basic substring formula to calculate the number of subs
                    count += calc_substring(&mut zero_count);
                    zero_count = 0;
                }
            }
            count += calc_substring(&mut zero_count);
            count
        }

        // intuitive approach
        let mut zero_count: i64 = 0;
        let mut total_count: i64 = 0;
        for curr_num in nums.iter() {
            let num: i32 = *curr_num;
            if num == 0 {
                zero_count += 1;
                total_count += zero_count;
            }
            if num != 0 {
                zero_count = 0;
            }
        }
        total_count as i64
    }

    // problem to calculate the number of car fleet
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut stack_fleet: Vec<(i32, i32)> = Vec::new();
        // making a sorted position and speed tuple vec
        let mut fleet: Vec<(i32, i32)> = Vec::new();
        for index in 0..position.len() {
            let pos: i32 = position[index];
            let speed: i32 = speed[index];
            fleet.push((pos, speed));
        }
        fleet.sort_by(|a, b| a.0.cmp(&b.0));
        if fleet.len() == 1 {
            return 1;
        }
        println!("{:?}- main fleet", fleet);
        // reverse iteration in order to populate stack
        let mut curr_time: f32 = 0.0;
        let mut next_time: f32 = 0.0;
        for (index, curr_vector) in fleet.iter().enumerate().rev() {
            if index == fleet.len() - 1 {
                next_time = ((target as f32) - (curr_vector.0 as f32)) / (curr_vector.1 as f32);
                stack_fleet.push(*curr_vector); // last one will always be the lead car that all needs to follow
            }
            if index == fleet.len() - 2 {
                stack_fleet.push(*curr_vector); // first push incase there is no intersection then add to stack for next comparison
                curr_time = ((target as f32) - (curr_vector.0 as f32)) / (curr_vector.1 as f32);
                if curr_time <= next_time {
                    stack_fleet.pop();
                }
            }
            if index < fleet.len() - 2 {
                let latest_stack_fleet = stack_fleet[stack_fleet.len() - 1];
                // next should be updated as per last value in stack
                next_time =
                    ((target as f32) - (latest_stack_fleet.0 as f32)) /
                    (latest_stack_fleet.1 as f32);
                stack_fleet.push(*curr_vector);
                curr_time = ((target as f32) - (curr_vector.0 as f32)) / (curr_vector.1 as f32);
                if curr_time <= next_time {
                    stack_fleet.pop();
                }
            }
        }
        stack_fleet.len() as i32
    }

    // counting the car collisions
    pub fn count_car_collisions(directions: String) -> i32 {
        let mut counter: i32 = 0;
        let dir_vec: Vec<char> = directions.chars().collect();
        let mut extra: i32 = 0;
        let mut prev: char = dir_vec[0];
        for index in 1..dir_vec.len() {
            let curr: char = dir_vec[index];
            if prev == 'R' && curr == 'L' {
                // double collision then turn to S
                counter += 2;
                counter += extra;
                extra = 0; // turns stationery no collision
                prev = 'S';
            } else if
                // stationary single collision
                (curr == 'L' && prev == 'S') ||
                (curr == 'S' && prev == 'R')
            {
                counter += 1 + extra;
                prev = 'S';
                extra = 0;
            } else if curr == 'R' && prev == 'R' {
                extra += 1; // collision force gathers till it hits something
                prev = curr;
            } else {
                prev = curr;
            }
        }
        counter
    }

    // boats to save people
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut boat_counter: i32 = 0;
        let mut sorted_people: Vec<i32> = people.clone();
        sorted_people.sort_by(|a, b| a.cmp(&b));

        let mut start: usize = 0;
        let mut end: usize = sorted_people.len() - 1;

        while start < end {
            let start_people: i32 = sorted_people[start];
            let end_people: i32 = sorted_people[end];
            let curr_capacity: i32 = start_people + end_people;

            if curr_capacity > limit {
                boat_counter += 1;
                end -= 1;
            } else if curr_capacity <= limit {
                boat_counter += 1;
                end -= 1;
                start += 1;
            }
        }

        boat_counter
    }

    // longest continuous sub that ordered alphabetically
    pub fn longest_continuous_alphbetic_sum(s: String) -> i32 {
        let mut len: i32 = 1;
        let s_vec: Vec<char> = s.chars().collect();
        let mut prev_char_byte: u8 = s_vec[0] as u8;
        let mut local_counter: i32 = 1;

        for i in 1..s_vec.len() {
            let curr_char: char = s_vec[i];
            let curr_char_byte: u8 = curr_char as u8;

            if curr_char_byte - 1 == prev_char_byte {
                local_counter += 1;
                prev_char_byte = curr_char_byte;
            } else {
                len = len.max(local_counter);
                local_counter = 1;
                prev_char_byte = curr_char_byte;
            }
            // edge case
            if i == s_vec.len() - 1 {
                len = len.max(local_counter);
            }
        }

        len
    }

    // max number of vowel letters in a substring
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max_len: i32 = 0;
        let vec: Vec<char> = s.chars().collect();
        let mut v_set: HashSet<char> = HashSet::new();
        v_set.insert('a');
        v_set.insert('i');
        v_set.insert('e');
        v_set.insert('o');
        v_set.insert('u');

        let mut flex_vec: Vec<char> = Vec::new();
        let mut v_counter: i32 = 0;
        for i in 0..k as usize {
            flex_vec.push(vec[i]);
            if v_set.contains(&vec[i]) {
                v_counter += 1;
            }
        }
        max_len = max_len.max(v_counter);
        for i in k as usize..vec.len() {
            let curr_char: char = vec[i];
            if let Some(first) = flex_vec.first() {
                if v_set.contains(first) {
                    v_counter -= 1;
                }
            }
            flex_vec.remove(0);
            flex_vec.push(curr_char);
            if v_set.contains(&curr_char) {
                v_counter += 1;
            }
            max_len = max_len.max(v_counter);
        }
        max_len
    }

    // getting the longest ones
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: i32 = 0;
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut zero_counter: i32 = 0;

        while end < nums.len() {
            let curr_num: i32 = nums[end];
            if curr_num == 0 {
                zero_counter += 1;
            }
            while zero_counter > k {
                if nums[start] == 0 {
                    zero_counter -= 1;
                }
                start += 1;
            }
            count = count.max((end as i32) - (start as i32) + 1);
            end += 1;
        }
        count
    }
    // removing stars from the strings
    pub fn remove_stars(s: String) -> String {
        let s_vec: Vec<char> = s.chars().collect();
        let s_iter = s_vec.iter();
        let mut stack: Vec<char> = Vec::new();
        for curr_char in s_iter {
            let local_char: char = *curr_char;
            if local_char == '*' {
                stack.pop();
            } else {
                stack.push(local_char);
            }
        }
        let res: String = stack
            .into_iter()
            .map(|a| a.to_string())
            .collect();
        res
    }

    // check distances
    pub fn check_distance(s: String, distance: Vec<i32>) -> bool {
        let mut check: bool = true;
        let s_vec: Vec<char> = s.chars().collect();
        let alphabets: Vec<char> = ('a'..='z').collect();
        let mut alpha_vec: Vec<(char, i32)> = Vec::new();
        for i in 0..alphabets.len() {
            alpha_vec.push((alphabets[i], distance[i]));
        }
        let mut map: HashMap<char, (i32, i32)> = HashMap::new();

        for (index, curr_char) in s_vec.iter().enumerate() {
            if map.contains_key(curr_char) {
                if let Some(tuple) = map.get_mut(curr_char) {
                    tuple.1 = index as i32;
                }
            } else {
                map.insert(*curr_char, (index as i32, 0));
            }
        }
        // positioning
        for curr_alpha in alpha_vec.iter() {
            let curr_char: char = curr_alpha.0;
            let curr_char_val: i32 = curr_alpha.1;
            if map.contains_key(&curr_char) {
                if let Some(tup_val) = map.get(&curr_char) {
                    let diff: i32 = tup_val.1 - tup_val.0 - 1;
                    if diff != curr_char_val {
                        check = false;
                        break;
                    }
                }
            }
        }

        check
    }

    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let word_vec: Vec<char> = word1.chars().collect();
        let word_vec2: Vec<char> = word2.chars().collect();
        let mut res: String = String::from("");

        let mut long_len: usize = word1.len();
        if word2.len() > word1.len() {
            long_len = word2.len();
        }
        let mut check: bool = false;
        let mut check_two: bool = false;

        for _ in 0..long_len {
            let one_char: char = word_vec[i];
            let two_char: char = word_vec2[j];
            if !check {
                res.push(one_char);
            }
            if !check_two {
                res.push(two_char);
            }

            if i == word_vec.len() - 1 {
                check = true;
            }
            if !check {
                i += 1;
            }
            if j == word_vec2.len() - 1 {
                check_two = true;
            }
            if !check_two {
                j += 1;
            }
        }
        res
    }

    // rearrange nums
    pub fn rearrange_nums(nums: Vec<i32>) -> Vec<i32> {
        // without swapping
        let mut sorted_nums: Vec<i32> = nums.clone();
        sorted_nums.sort_by(|a, b| a.cmp(&b));
        let mut arr_vec: Vec<i32> = Vec::new();
        let half_len: usize = sorted_nums.len() / 2;
        let mut f_h: Vec<i32> = Vec::new();
        let mut s_h: Vec<i32> = Vec::new();
        for i in 0..half_len + 1 {
            f_h.push(sorted_nums[i]);
        }
        for i in half_len + 1..sorted_nums.len() {
            s_h.push(sorted_nums[i]);
        }
        let mut f_index: usize = 0;
        let mut s_index: usize = 0;
        let mut check: bool = false;
        let mut check_two: bool = false;

        let mut long_len: usize = f_h.len();
        if long_len < s_h.len() {
            long_len = s_h.len();
        }

        for _ in 0..long_len {
            if !check {
                arr_vec.push(f_h[f_index]);
            }
            if !check_two {
                arr_vec.push(s_h[s_index]);
            }
            if s_index == s_h.len() - 1 {
                check_two = true;
            }
            if f_index == f_h.len() - 1 {
                check = true;
            }

            if !check {
                f_index += 1;
            }

            if !check_two {
                s_index += 1;
            }
        }

        arr_vec
    }

    // finding the longest harmonious sequence
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut longest_len: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut set: HashSet<i32> = HashSet::new();
        for item in nums.iter() {
            set.insert(*item);
        }
        for i in 0..nums.len() {
            match map.get_mut(&nums[i]) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(nums[i], 1);
                }
            }
        }
        let mut set_vec: Vec<i32> = set
            .into_iter()
            .map(|val| val)
            .collect();
        set_vec.sort_by(|a, b| b.cmp(&a));
        let mut end: usize = 1;
        while end < set_vec.len() {
            if set_vec[end - 1] - set_vec[end] == 1 {
                longest_len = longest_len.max(
                    map.get(&set_vec[end]).unwrap() + map.get(&set_vec[end - 1]).unwrap()
                );
            }
            end += 1;
        }
        longest_len
    }

    // getting the long pressed name
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut type_pointer: usize = 1;
        let mut name_pointer: usize = 1;
        let n_vec: Vec<char> = name.chars().collect();
        let t_vec: Vec<char> = typed.chars().collect();
        if n_vec.is_empty() {
            return false;
        }
        if n_vec.len() > t_vec.len() {
            return false;
        }

        while type_pointer < t_vec.len() {
            let t_char: char = t_vec[type_pointer];

            if name_pointer < n_vec.len() && type_pointer < t_vec.len() {
                let n_char: char = n_vec[name_pointer];
                let n_prev: char = if name_pointer > 0 { n_vec[name_pointer - 1] } else { '\0' };
                if n_char == t_char {
                    name_pointer += 1;
                    type_pointer += 1;
                } else if t_char == n_prev {
                    type_pointer += 1;
                } else {
                    return false;
                }
            } else {
                return true;
            }
        }
        name.len() == name_pointer
    }

    // replace words
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let s_vec: Vec<&str> = sentence.split_whitespace().collect();
        let mut result_vec: Vec<String> = Vec::new();
        for word in s_vec.iter() {
            result_vec.push(word.to_string());
        }
        for (index, word) in s_vec.iter().enumerate() {
            let curr_word = word.to_string();
            let mut d_match: String = String::from("");
            for d_word in dictionary.iter() {
                let d = d_word.to_string();
                if curr_word.starts_with(&d) {
                    if d_match.is_empty() {
                        d_match = d.to_string();
                    } else {
                        if d_match.len() > d.to_string().len() {
                            d_match = d.to_string();
                        }
                    }
                }
            }
            if let Some(res_word) = result_vec.get_mut(index) {
                if !d_match.is_empty() {
                    *res_word = d_match;
                }
            }
        }
        result_vec.join("")
    }

    // sliding window binary sum
    pub fn num_subarrays_with_sum(mut nums: Vec<i32>, mut goal: i32) -> i32 {
        let mut counter: i32 = 0;
        fn get_sub_ocurence(nums: &mut Vec<i32>, goal: &mut i32) -> i32 {
            let mut counter: i32 = 0;
            let mut start: usize = 0;
            let mut total: i32 = 0;
            let mut end: usize = 0;
            while end < nums.len() {
                total += nums[end];
                while total > *goal {
                    if start < nums.len() {
                        total -= nums[start];
                        start += 1;
                    } else {
                        break;
                    }
                }
                if start <= end {
                    counter += (end - start + 1) as i32;
                }
                end += 1;
            }
            return counter;
        }
        let total_sub: i32 = get_sub_ocurence(&mut nums, &mut goal);
        goal -= 1;
        let second_sub: i32 = get_sub_ocurence(&mut nums, &mut goal);
        counter = total_sub - second_sub;
        counter
    }

    // getting the longest substring with k repeating characters using hashmap and sliding window
    pub fn longest_substring_with_k(s: String, k: i32) -> i32 {
        let mut max_len: usize = 0;
        let s_vec: Vec<char> = s.chars().collect();
        let set: HashSet<char> = s_vec
            .iter()
            .map(|val| *val)
            .collect();
        let unique_size: usize = set.len();
        let mut unique_index: usize = 1;

        while unique_index <= unique_size {
            let mut map: HashMap<char, i32> = HashMap::new();
            let mut atleast_k: i32 = 0;
            let mut u_count: i32 = 0;
            let mut start: usize = 0;

            for (index, curr_char) in s_vec.iter().enumerate() {
                if let Some(occurence) = map.get_mut(curr_char) {
                    *occurence += 1;
                } else {
                    map.insert(*curr_char, 1);
                }
                // populating unique and atleast k
                if let Some(k_check) = map.get(curr_char) {
                    if *k_check == k {
                        atleast_k += 1;
                    }
                }
                if let Some(occ) = map.get(curr_char) {
                    if *occ == 1 {
                        u_count += 1;
                    }
                }
                // if it exceeds unique count
                while u_count > (unique_index as i32) {
                    if let Some(atk) = map.get(&s_vec[start]) {
                        if *atk == k {
                            atleast_k -= 1;
                        }
                    }
                    if let Some(oc) = map.get_mut(&s_vec[start]) {
                        *oc -= 1;
                        if *oc == 0 {
                            u_count -= 1;
                        }
                    }
                    start += 1;
                }
                if (u_count as usize) == unique_index && (atleast_k as usize) == unique_index {
                    max_len = max_len.max(index - start + 1);
                }
            }

            unique_index += 1;
        }
        max_len as i32
    }

    // longest turbulent
    pub fn turbulent(arr: Vec<i32>) -> i32 {
        let mut maxlen: i32 = 0;
        let mut end: usize = 1;
        let mut start: usize = 0;
        let mut currsign: char = '.';
        if arr.len() == 1 {
            return 1;
        }
        while end < arr.len() {
            let currel: i32 = arr[end];
            let currprev: i32 = arr[end - 1];

            if currprev > currel && currsign != '>' {
                maxlen = maxlen.max((end - start + 1) as i32);
                end += 1;
                currsign = '>';
            } else if currprev < currel && currsign != '<' {
                maxlen = maxlen.max((end - start + 1) as i32);
                end += 1;
                currsign = '<';
            } else {
                if currel == currprev {
                    maxlen = maxlen.max((end - start) as i32);
                    start = end;
                    end = end + 1;
                    currsign = '.';
                }
                if currsign == '>' || currsign == '<' {
                    start = end - 1;
                    end += 1;
                }
            }
        }
        return maxlen;
    }

    // asteroid collision problem
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        for asteroid in asteroids.iter() {
            if stack.is_empty() {
                stack.push(*asteroid);
            } else {
                let last_roid: i32 = *stack.last().unwrap();
                // collision occurs
                if last_roid > 0 && *asteroid < 0 {
                    let mut collision_check: bool = false;
                    while !stack.is_empty() && stack[stack.len() - 1] > 0 && *asteroid < 0 {
                        if stack[stack.len() - 1].abs() < asteroid.abs() {
                            stack.pop();
                        } else if stack[stack.len() - 1].abs() > asteroid.abs() {
                            collision_check = true;
                            break;
                        } else if stack[stack.len() - 1].abs() == asteroid.abs() {
                            stack.pop();
                            collision_check = true;
                            break;
                        } else {
                            collision_check = true;
                            break;
                        }
                    }
                    if !collision_check {
                        stack.push(*asteroid);
                    }
                } else {
                    stack.push(*asteroid);
                }
            }
        }
        println!("{:?}", stack);
        stack
    }

    // removing least number of unique integers
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut min_count: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in arr.iter() {
            *map.entry(*num).or_insert(0) += 1;
        }
        let mut sorted_freq: Vec<(i32, i32)> = map
            .iter()
            .map(|val| (*val.0, *val.1))
            .collect();
        sorted_freq.sort_by(|a, b| a.1.cmp(&b.1));
        // removing k elements then counting distinct
        let mut cut_off: usize = 0;
        let mut old_limit: i32 = k;
        for (index, item) in sorted_freq.iter_mut().enumerate() {
            k -= item.1;
            item.1 -= old_limit - k;
            old_limit = k;
            if k <= 0 {
                cut_off = index;
                break;
            }
        }
        // if there is a negative value then change cut off val
        if k < 0 {
            min_count = sorted_freq[cut_off..sorted_freq.len()].len() as i32;
        } else {
            sorted_freq.iter().for_each(|val| {
                if val.1 > 0 {
                    min_count += 1;
                }
            });
        }
        min_count
    }
    // getting the maximum sub circular subarray sum
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total: i32 = nums.iter().fold(0, |avv, val| avv + val);
        // global linear max
        let mut global_max: i32 = *nums.first().unwrap();
        let mut curr_max: i32 = global_max;
        // global linear min
        let mut global_min: i32 = *nums.first().unwrap();
        let mut curr_min: i32 = global_min;

        for index in 1..nums.len() {
            let curr_num: i32 = nums[index];
            // for global max
            curr_max = (curr_max + curr_num).max(curr_num);
            if curr_max > global_max {
                global_max = curr_max;
            }
            // getting global mins
            curr_min = (curr_num + curr_min).min(curr_num);
            if global_min > curr_min {
                global_min = curr_min;
            }
        }
        total = if total - global_min > global_max && total != global_min {
            total - global_min
        } else {
            global_max
        };
        total
    }

    // finding occurences of element
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut ans_queries: Vec<i32> = vec![-1; queries.len()];
        let mut freq_vec: Vec<i32> = Vec::new();

        for (index, item) in nums.iter().enumerate() {
            if *item == x {
                freq_vec.push(index as i32);
            }
        }
        for index in 0..ans_queries.len() {
            let query: i32 = queries[index];
            if (query as usize) <= freq_vec.len() {
                ans_queries[index] = freq_vec[(query - 1) as usize];
            }
        }
        ans_queries
    }

    // finding a common prefix
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut stack: Vec<i32> = vec![0; a.len()];
        let mut i: usize = 0;
        let mut counter: i32 = 0;
        fn get_oc(local_map: &mut HashMap<i32, i32>, value: &i32) {
            match local_map.get_mut(value) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    local_map.insert(*value, 1);
                }
            }
        }
        for index in 0..a.len() {
            let a_el: i32 = a[index];
            let b_el: i32 = b[index];
            get_oc(&mut map, &a_el);
            get_oc(&mut map, &b_el);
            if a_el != b_el {
                if *map.get(&a_el).unwrap() > 1 {
                    counter += 1;
                }
                if *map.get(&b_el).unwrap() > 1 {
                    counter += 1;
                }
            } else {
                counter += 1;
            }
            stack[i] = counter;
            println!("{:?}-{:?}-{}", stack, map, counter);
            i += 1;
        }
        stack
    }

    // to reorg string
    pub fn reorg_string(s: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut dp: Vec<char> = vec!['a'; s.len()];
        // logic
        for s_char in s.chars() {
            match map.get_mut(&s_char) {
                Some(occurence) => {
                    *occurence += 1;
                }
                None => {
                    map.insert(s_char, 1);
                }
            }
        }
        // get max char occurence
        let mut max_char: char = 'x';
        let mut max_freq_char: i32 = 0;
        for (key, value) in map.iter() {
            if *value > max_freq_char {
                max_freq_char = *value;
                max_char = *key;
            }
        }
        // place max char
        let mut dp_index: usize = 0;
        let mut char_c: i32 = 0;
        while dp_index < dp.len() {
            dp[dp_index] = max_char;
            char_c += 1;
            if let Some(oc) = map.get_mut(&max_char) {
                *oc -= 1;
            }
            dp_index += 2;
            if char_c == max_freq_char {
                break;
            }
        }
        // if the map is 0 then continue or else return max as it did not place the most frequent char
        let max_val: i32 = *map.get(&max_char).unwrap();
        if max_val == 0 {
            map.remove(&max_char);
        } else {
            return String::from("");
        }
        // placing the remaining chars
        for (key, _) in map.clone() {
            while let Some(val) = map.get(&key) {
                if *val > 0 {
                    if dp_index >= dp.len() {
                        dp_index = 1;
                    }
                    dp[dp_index] = key;
                    // map syntax for adding and modifying in place without additional borrowing
                    map.entry(key).and_modify(|v| *v -= 1); // for subtracting within the same object
                    dp_index += 2;
                } else {
                    map.remove(&key);
                    break;
                }
            }
        }
        let result: String = dp.iter().collect();
        result
    }


    // next great element I finding the next greater el in nums2 using nums1 as reference.. using map
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {

        // n * m solution using map
        fn o_n_m_solution(nums1: Vec<i32>, nums2: Vec<i32>)-> Vec<i32>{
            let mut ans: Vec<i32> = vec![-1; nums1.len()];
            let mut map: HashMap<i32, usize> = HashMap::new();
            for (index, num) in nums1.iter().enumerate(){
                map.insert(*num, index);
            }
            for (index, num) in nums2.iter().enumerate(){
                if map.contains_key(&num){
                    let mut curr_num: i32 = *num;
                    for sub_index in (index + 1)..nums2.len(){
                        let sub_num: i32 = nums2[sub_index];
                        if sub_num > curr_num{
                            curr_num = sub_num;
                            ans[*map.get(&num).unwrap()] = curr_num;
                            break;
                        }
              
                    }
                }
                
            }
            ans
        }

        // monotonic decreasing stack
        let mut stack: Vec<i32> = Vec::new();
        let mut ans: Vec<i32> = vec![-1; nums1.len()];
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i,item) in nums1.iter().enumerate(){
            map.insert(*item, i);
        }
        for item in nums2.iter(){
            let curr_num: i32 = *item;
            while !stack.is_empty() && stack[stack.len() - 1] < curr_num{
                if let Some(popped_val) = stack.pop(){
                    if map.contains_key(&popped_val){
                        ans[*map.get(&popped_val).unwrap()] = curr_num; 
                    }
                }
            }
            stack.push(curr_num);
        }
        ans

    }

    // finding a pattern and replacing the words accordingly
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        // function to check pattern of the word against the pattern
        fn pattern_check(word: &String, pattern: &String)-> bool{  
            if word.len() != pattern.len(){
                return false;
            }
            // set check
            let set_word: HashSet<char> = word.chars().collect();
            let set_p: HashSet<char> = pattern.chars().collect();
            if set_word.len() != set_p.len(){
                return false;
            }

            let mut word_map: HashMap<char,char> = HashMap::new();
            let word_char: Vec<char> = word.chars().collect();
            let p_char: Vec<char> = pattern.chars().collect();

            println!("{:?}-{:?}", word_char, p_char);
            // abc abb
            for (index , word_letter) in word_char.iter().enumerate(){
                let w_char: char = *word_letter;
                let p_char: char = p_char[index];
                println!("{}-{}", w_char, p_char);
                if word_map.contains_key(&p_char){
                    if *word_map.get(&p_char).unwrap() != w_char{
                        return false;
                    }
                 }else{
                    word_map.insert(p_char, w_char);
                 }
            }
            println!("{:?}-map", word_map);

            true
        }

        for word in words.iter(){
            let curr_word: String = word.to_string();
            if pattern_check(&curr_word, &pattern){
                ans.push(curr_word);
            }
        }
        println!("{:?}", ans);
        ans
    }

    // specialised group distributing the  numbers into group sizs
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for (index, curr_num) in group_sizes.iter().enumerate(){
            let num: i32 = *curr_num;
            map.entry(num).or_insert_with(Vec::new).push(index as i32);
        }
        // distribution based on value
        for (key, value) in map{
            let mut m_count: i32 = 0;
            let mut sub_stack: Vec<i32> = Vec::new();
            for item in value.iter(){
                sub_stack.push(*item);
                m_count += 1;
                if m_count == key{
                    m_count = 0;
                    ans.push(sub_stack.clone());
                    sub_stack = Vec::new();
                }
            }
        }
        ans
    }

    // min sequence not sub
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut vec: Vec<i32> = nums.clone();
        let mut ans: Vec<i32> = Vec::new();
        vec.sort_by(|a, b| b.cmp(a));
        let total: i32 = vec.iter().fold(0, |acc, curr| acc + curr);
        let mut sub_total: i32 = 0;
        for curr_num in vec.iter(){
            sub_total += *curr_num;
            let diff: i32 = total - sub_total;
            ans.push(*curr_num);
            if sub_total > diff{
                return ans
            }
        }
        ans
    }

    // find length using double db manipulations
    pub fn find_length_dp_sub(nums1: Vec<i32>, nums2: Vec<i32>)-> i32{
        let mut max_len: i32 = 0;
        // populate dp -> have to add one extra space for bottom right indices
        let mut dp: Vec<Vec<i32>> = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        // iterating through the matrix to check for substrings 
        for i in (0..dp.len() - 1).rev(){
            let num_one: i32 = nums1[i];
            for j in (0..dp[i].len() - 1).rev(){
                let num_two: i32 = nums2[j];
                if num_one == num_two{
                    dp[i][j] += dp[i + 1][j + 1] + 1;
                    max_len = max_len.max(dp[i][j]);
                }
            }
        }
        max_len
    }
    // getting min subarray length
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_len: i32 = i32::MAX;
        let mut end: usize = 0;
        let mut start: usize = 0;
        let mut total: i32 = 0;
        while end < nums.len(){
            total += nums[end];
            while total >= target{ // to let the total reduce as soon as it hits the target or is equal to
                min_len = min_len.min((end - start + 1) as i32);
                total -= nums[start];
                start += 1;
            }
            end += 1;
        }
        if min_len == 2147483647{
            return 0;
        }
        min_len
    }

    // cannot be typed words
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut count: i32 = 0;
        let str_vec: Vec<String> = text.split_whitespace().map(|val| val.to_string()).collect();
        let set: HashSet<char> = broken_letters.chars().collect();
        for word in str_vec.iter(){
            let mut check: bool = true;
            for curr_letter in word.chars(){
                if set.contains(&curr_letter){
                    check = false;
                    break;
                }
            }
            if check{
                count += 1;
            }
        }
        count
    }

    // next greater element II
    pub fn next_greater_el_ii(nums: Vec<i32>)->Vec<i32>{
        let mut ans: Vec<i32> = vec![-1; nums.len()];
        let mut i: usize = 0;
        let mut new: Vec<i32> = vec![0; nums.len() * 2];
        for curr in new.iter_mut(){
            *curr = nums[i];
            i += 1;
            if i == nums.len(){
                i = 0;
            }
        }
        for (i, curr) in new.iter().enumerate(){
            for check_index in (i + 1)..new.len(){
                let check_num: i32 = new[check_index];
                if check_num > *curr{
                    ans[i] = check_num;
                    break;
                }
            }
            if i == nums.len() - 1{
                break;
            }
        }
        ans
    }


    // grumpy bookstore owner
    pub fn grumpy_owner(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32)-> i32{
        let mut total_sat: i32 = 0;
        for (i, curr_sat) in customers.iter().enumerate(){
            let grump: i32 = grumpy[i];
            if grump == 0{
                total_sat += *curr_sat;
            }
        }
        let mut start: usize = 0;
        let mut curr_total_sat: i32 = 0;
        for i in 0..minutes as usize{
            if let Some(curr_grump) = grumpy.get(i){
                if *curr_grump == 1{
                    curr_total_sat += customers.get(i).unwrap();
                }
            }
        }
        let mut curr_sat_max: i32 = curr_total_sat;
        for i in (minutes as usize)..customers.len(){
            if grumpy[start] == 1{
                curr_total_sat -= customers[start];
            }
            if grumpy[i] == 1{
                curr_total_sat += customers[i];
            }
            curr_sat_max = curr_sat_max.max(curr_total_sat);
            start += 1;
        }
        total_sat += curr_sat_max;
        total_sat
    }




    // removing duplicates
    pub fn remove_duplicate_char(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();
        if let Some(first_char) = s.chars().next() {
            stack.push((first_char, 1));
        }
        let s_vec: Vec<char> = s.chars().collect();
        let mut res: String = String::from("");
        for index in 1..s_vec.len(){
            let curr_char: char = s_vec[index];
           if let Some(last) = stack.last_mut(){
            if last.0 == curr_char{
                last.1 += 1;
                if last.1 == k{
                    stack.pop();
                }
            }else{
                stack.push((curr_char, 1));
            }
           }else{
            stack.push((curr_char, 1));
           }
        }
        for item in stack.iter(){
            for i in 0..item.1{
                res.push(item.0);
            }
        }
        res
    }
}
//[5,6,1,5,6,4,1,5]
/*
5, -3, 5



*/

// aebaaabbc
// n * (n + 1) / 2 => 3 * 4 => 12 / 2 = 6
// [5,5,4,3]
/* 
3 5 3 4 start

3 3 4 5  9



3 3 4

3 3








    [3,2,2,1]
    [3,1,2, 1] => {1} 1
    [3,1,2, 1] 

    1,2,2,3 - limit - 3
    1 2 2 - c = 1

    1 2 2 2


     1 1 3 3 = 3



    1 1 = c = 1



*/
// pattern check
// abcabcabc => 9/2 = 4

//"aeiaaioaaaaeiiiiouuuooaauuaeiu"

// notes

// remember cloned is used on the iterator to copy values but clone is used on the value itself
// map is used to create iterator

// #[derive(Debug)]
// struct User{
//     username: String,
//     age: i32
// }
// pub fn play_ground_three(){
//     let mut array_vec: Vec<User> = Vec::new();
//     for index in 0..10{
//         array_vec.push(
//             User { username: String::from("rumon"), age: 27 + index }
//         )
//     }
//     let single_user = array_vec.iter_mut().find(|user| user.age == 28).unwrap(); // this has the direct link for an iterable reference to the User
//     single_user.age = 10;
//     println!("{:?}", array_vec);
// }

// Correct, you cannot directly use the index obtained from enumerate() to modify the vector in place. The reason is that the indexing operation array[index] requires a mutable reference to the vector, and when using enumerate(), you only get a mutable reference to each element, not to the vector itself.
// If you need to modify elements based on their index, you can use iter_mut().enumerate() and then calculate the modified value separately before assigning it to the vector. Here's an example:

// always remember to have a check for vectors whether they are empty or not before comparing any characters for comparison
// otherwise rust is natually inclined to create a panic warning

/*for index in 1..mut_nums.len(){
//     check_vec.push(index);

 this structure iterates over the actual values hence they cannot be used as indices or numbers that can be pushed into a separate vector
 // enumerate() is the way to get indices separately
// }*/

// right operate is a reference in rust
// if its more than one addition u can always format or when using plus use ampersand as the right side needs to be referenced
// since am creating new chars I dont wanna create references

// collect is for building strings or vecs or other stuffs and it cannot be build from vecs

// split_whitespace method works only only on string slices
// since split returns string slices it needs to be converted to String type by map iterator
// for comparison purposes it is more prudent to use &str string slices instead of proper String UTF-8 type

// sort_by -> sorts the elements in place and does not return a new vector

// remmeber dereferencing allows u to change the original value of the variable even if u return a mutable reference to it

// is_empty method can be used to check for both string and vectors whether they are empty or not

//If item in the HashMap was of type char instead of i32, and your nums vector contained characters, you wouldn't need to use the asterisk (*) to dereference when inserting into the HashMap.

// filter method: accepts dereferencing

// into_iter() -> inherently performs the clone trait which is a deep copy and it takes into an ownership of the elements hence u need not have to clone it

// only map and hashsets and hashmap accepts references for getting val

// pub fn playground(){
//     let mut gen_vec = vec![10,10,20,2,2,3,4,4,2,3,1];
//     let filtered_vec: Vec<i32> = gen_vec.iter().filter(|item| **item > 10).cloned().collect(); // cloned is shalow copy
//     let mut map_vec: Vec<i32> = gen_vec.iter_mut().map(|item| {*item += 1; *item}).collect();

//     for item in map_vec.iter_mut(){
//     }
//     println!("{:?}", map_vec);
//     gen_vec.retain(|&item| item > 10); // retain method is used to keep the elements in place and add conditions
//     println!("{:?}{:?}", filtered_vec, gen_vec);
// }
