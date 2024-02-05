pub mod concepts_modules {
    use std::collections::HashMap;
    use std::collections::HashSet;
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
            if curr_item == 'L'{
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
            if curr_item == 'R'{
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

        for (index, item) in result_dominoes.iter_mut().enumerate(){
            let left_val: i32 = left[index];
            let right_val: i32 = right[index];
            if right_val == 0 && left_val > 0{
                *item = 'L';
            }else if left_val == 0 && right_val > 0{
                *item = 'R';
            }else if (left_val == 0 && right_val == 0) || (left_val == right_val){
                *item = '.';
            }else if left_val > 0 && right_val > 0{
                if left_val > right_val{
                    *item = 'L';
                }else{
                    *item = 'R';
                }
            }

        }
        let mut str_result: String = String::from("");
        for curr_char in result_dominoes.iter(){
            str_result.push(*curr_char);
        }

        str_result
    }

    // checking for word pattern
    pub fn word_pattern(pattern: String, s: String)->bool{
        let mut check: bool = true;
        let pattern_new: Vec<char> = pattern.chars().collect();
        let s_array: Vec<String> = s.split_whitespace().into_iter().map(|val|val.to_string()).collect();
        if pattern_new.len() != s_array.len(){
            return false;
        }
        let mut map: HashMap<char, String> = HashMap::new();
        // populating map
        for (index, curr_char) in pattern_new.iter().enumerate(){
            let local_char = *curr_char;
            match map.get_mut(&local_char){
                Some(inner_string)=> {
                    *inner_string = s_array[index].to_string();
                },
                None=>{
                    map.insert(local_char, s_array[index].to_string());
                }
            }
        }
        for index in 0..pattern_new.len(){
            let pattern_char: char = pattern_new[index];
            let s_array_word: String = s_array[index].to_string();
            match map.get(&pattern_char){
                Some(mapped_word)=>{
                    if mapped_word.to_string() != s_array_word{
                        check = false;
                        break;
                    }
                },
                None=>{}
            }
        }
        check
    }

    pub fn frequency_sort_two(nums: Vec<i32>)->Vec<i32>{
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut array: Vec<i32> = Vec::new();
        for curr_num in nums.iter(){
            match map.get_mut(&curr_num){
                Some(occurence)=> {
                    *occurence += 1;
                },
                None=>{
                    map.insert(*curr_num, 1);s
                }
            }
        }
        // sort the map
        let mut map_vec: Vec<(i32, i32)> = map.into_iter().collect(); 
        // if the frequency is equal then compare the numbers and rearrange them in decreasing else arrange em by increasing of frequency
        map_vec.sort_by(|(a,a_freq),(b,b_freq)| {
            if a_freq == b_freq{
                b.cmp(&a)
            }else{
                a_freq.cmp(&b_freq)
            }
        });
        // array formation
        for item in map_vec.into_iter(){
            for _ in 0..item.1{
                array.push(item.0);
            }
        }
        array
    }   
}

// notes

// remember cloned is used on the iterator to copy values but clone is used on the value itself

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
