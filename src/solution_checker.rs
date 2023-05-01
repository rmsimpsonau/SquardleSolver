use std::collections::HashMap;

pub fn check_for_solution(solution: &mut HashMap<i8, Vec<String>>, result: &Vec<String>, word: &String, is_row: bool, word_list_index: i8) {
    for (result_index, result_char) in result.iter().enumerate() {
        if result_char.chars().nth(0).unwrap() == 'g' {
            let mut words_index = word_list_index;
            if !is_row {
                words_index += 3;
            }

            let words_list: &mut Vec<String> = solution.get_mut(&words_index).unwrap();
            let letter: String = word.chars().nth(result_index).unwrap().to_string();
            words_list[result_index] = letter.clone();

            // If the green letter is in a shared cell, add this green solution to the shared row / column
            if let 0 | 2 | 4 = result_index {
                println!("Index is 0 2 or 4: {}", result_index);
                let mut words_index = (result_index / 2) as i8;
                // If word list we are checking is a row, then we want to get the words for the intersecting column
                if is_row {
                    words_index += 3;
                }

                println!("words_index={}", words_index);

                let words_list: &mut Vec<String> = solution.get_mut(&words_index).unwrap();
                let letter: String = word.chars().nth((result_index) as usize).unwrap().to_string();

                println!("letter={}, word_list_index={}", letter, word_list_index);

                words_list[(word_list_index * 2) as usize] = letter.clone();

                println!("saved={}", words_list[(word_list_index * 2) as usize]);
            }
        }
    }
}

pub fn check_word_combination_possibilities(remaining_words: &HashMap<i8, Vec<String>>) {
    // 0 - Top Row
    // 1 - Middle Row
    // 2 - Bottom Row
    // 3 - Left Column
    // 4 - Middle Column
    // 5 - Right Column
    for (index, word) in remaining_words.iter().enumerate() {
        // TODO Future improvements
    }
}




#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::solution_checker;

    #[test]
    fn check_for_solution_test() {
        let mut solution: HashMap<i8, Vec<String>> = HashMap::new();

        let solution_vec: Vec<String> = vec![String::new(); 5];

        solution.insert(0, solution_vec.clone()); // Top Row
        solution.insert(1, solution_vec.clone()); // Middle Row
        solution.insert(2, solution_vec.clone()); // Bottom Row
        solution.insert(3, solution_vec.clone()); // Left Column
        solution.insert(4, solution_vec.clone()); // Middle Column
        solution.insert(5, solution_vec.clone()); // Right Column


        let result = vec!["g".to_string(), "g".to_string(), "g".to_string(), "x".to_string(), "x".to_string()];

        solution_checker::check_for_solution(&mut solution, &result, &"hello".to_string(), true, 0);

        let row_solution_count = solution.get(&0)
            .map(|vec| vec.iter().filter(|s| !s.is_empty()).count())
            .unwrap_or(0);

        let col_solution_count = solution.get(&3)
            .map(|vec| vec.iter().filter(|s| !s.is_empty()).count())
            .unwrap_or(0);

        assert_eq!(row_solution_count, 3);
        assert_eq!(col_solution_count, 1);

        assert_eq!(solution.get(&0).unwrap().get(0).unwrap(), "h");
        assert_eq!(solution.get(&0).unwrap().get(1).unwrap(), "e");
        assert_eq!(solution.get(&0).unwrap().get(2).unwrap(), "l");
        assert_eq!(solution.get(&3).unwrap().get(0).unwrap(), "h");
        assert_eq!(solution.get(&4).unwrap().get(0).unwrap(), "l");
    }
}