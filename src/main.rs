mod solution_checker;
use std::env;

/*
TODO List
 - Add checking for multiple red / yellow / oranges
 - Print out solution in appropriate grid
 - Add algorithm for checking all possibilities
 - Serve up on webpage
*/

mod squardle_solver {

    use std::collections::HashMap;
    use std::{fs, io};
    use rand::Rng;
    use colored::Colorize;
    use crate::solution_checker;

    pub fn start() {
        let words_list_filename: String = String::from("./src/words-list");

        let contents = fs::read_to_string(words_list_filename)
            .expect("Should have been able to read the file");

        // Create a map that stores a vector of strings for each row/column. Each row/column has a
        // unique set of remaining words
        let mut remaining_words: HashMap<i8, Vec<String>> = HashMap::new();

        // Read the words and assign them to each of the row/column vectors
        let mut words: Vec<String> = Vec::new();

        for word in contents.split_whitespace() {
            words.push(String::from(word));
        }

        remaining_words.insert(0, words.clone()); // Top Row
        remaining_words.insert(1, words.clone()); // Middle Row
        remaining_words.insert(2, words.clone()); // Bottom Row
        remaining_words.insert(3, words.clone()); // Left Column
        remaining_words.insert(4, words.clone()); // Middle Column
        remaining_words.insert(5, words.clone()); // Right Column

        // Create map to store solution of each index
        let mut solution: HashMap<i8, Vec<String>> = HashMap::new();

        let solution_vec: Vec<String> = vec![String::new(); 5];

        solution.insert(0, solution_vec.clone()); // Top Row
        solution.insert(1, solution_vec.clone()); // Middle Row
        solution.insert(2, solution_vec.clone()); // Bottom Row
        solution.insert(3, solution_vec.clone()); // Left Column
        solution.insert(4, solution_vec.clone()); // Middle Column
        solution.insert(5, solution_vec.clone()); // Right Column

        // Start with row index 0
        let mut current_column_row_idx = 0;

        println!("Squardle Solver Instructions");
        println!(" - Daryl gives a word to guess and waits for the results of that guess");
        println!(" - User enters results in the follow format with a space between each:");
        println!("   - Green letter  = g");
        println!("   - Yellow letter = y");
        println!("   - Orange letter = o");
        println!("   - Red letter    = r");
        println!("   - White letter  = w");
        println!("   - Black letter  = b");
        println!("   - Multiple arrows on yellow, orange or red letters = add the number of arrows at the end");
        println!("     - Ex: Red letter with 2 arrows: r2\n");
        println!(" - Other actions:");
        println!("   - Skip a column/row = s\n\n");


        loop {
            let list_of_words: &Vec<String>;

            let row_solution_count = solution.get(&current_column_row_idx)
                .map(|vec| vec.iter().filter(|s| !s.is_empty()).count())
                .unwrap_or(0);

            let col_solution_count = solution.get(&(current_column_row_idx + 3))
                .map(|vec| vec.iter().filter(|s| !s.is_empty()).count())
                .unwrap_or(0);

            // If we already have the row/ solution, get a word from the opposite
            if row_solution_count == 5 {
                println!("Getting word for column index {} since row is solved", &current_column_row_idx);
                list_of_words = remaining_words.get(&(current_column_row_idx + 3))
                    .expect("Should get words for column index");
            } else if col_solution_count == 5 {
                println!("Getting word for row index {} since column is solved", &current_column_row_idx);
                list_of_words = remaining_words.get(&current_column_row_idx)
                    .expect("Should get words for row index");
            } else {
                if remaining_words.get(&current_column_row_idx).unwrap().len() <=
                    remaining_words.get(&(&current_column_row_idx + 3)).unwrap().len() {
                    println!("Getting word for row index {}", &current_column_row_idx);
                    list_of_words = remaining_words.get(&current_column_row_idx)
                        .expect("Should get words for row index");
                } else {
                    println!("Getting word for column index {}", &current_column_row_idx);
                    list_of_words = remaining_words.get(&(&current_column_row_idx + 3))
                        .expect("Should get words for column index");
                }
            }

            // Get a random word for Daryl to guess from remaining words in list
            let solver_guess_index: usize = rand::thread_rng().gen_range(0..=list_of_words.len() - 1);
            let solver_guess_string: String = list_of_words.get(solver_guess_index)
                .expect("Should get guess string for solver to guess")
                .clone(); // Clone so we don't get a reference to it

            println!("Daryl's Guess \n=== {} ===\nRow / Column {}", &solver_guess_string.to_uppercase().purple().italic(), &current_column_row_idx);

            let mut print_requested = false;

            /* Row Results */
            println!("Enter ROW results ({})reen, ({})ellow, ({})range, ({})ed, ({})hite, ({})lack, ({})kip:",
                     'g'.to_string().green(), 'y'.to_string().yellow(), 'o'.to_string().bright_red(),
                     'r'.to_string().red(), 'w'.to_string().white(), 'b'.to_string().bright_black(),
                     's'.to_string().blue()
            );
            let mut row_result_of_guess = String::new();

            io::stdin()
                .read_line(&mut row_result_of_guess)
                .expect("Failed to read line");


            if row_result_of_guess.contains('p') {
                print_requested = true;
                // For debugging, you can enter the letter p and word list will be printed for the
                // current row and column
                let mut row_result_vec: Vec<String> = Vec::new();
                for result in row_result_of_guess.split_whitespace() {
                    row_result_vec.push(String::from(result));
                }
                let word_list_index: i8 = row_result_vec[1].parse().unwrap();
                println!("{}", remaining_words.get(&word_list_index).unwrap().join(", "));
            } else if row_result_of_guess.contains('s') {
                println!("Skipping column/row index {}", current_column_row_idx);
            } else {
                // Put the results of the guess that user entered into a vector so it's easier to work with
                let mut row_result_vec: Vec<String> = Vec::new();
                for result in row_result_of_guess.split_whitespace() {
                    row_result_vec.push(String::from(result));
                }

                // If the results are not valid, tell the user and start again at the top of the loop
                if !verify_result_valid(&row_result_vec) {
                    continue;
                }

                /* Column Results */
                println!("Enter COLUMN results ({})reen, ({})ellow, ({})range, ({})ed, ({})hite, ({})lack, ({})kip:",
                         'g'.to_string().green(), 'y'.to_string().yellow(), 'o'.to_string().bright_red(),
                         'r'.to_string().red(), 'w'.to_string().white(), 'b'.to_string().bright_black(),
                         's'.to_string().blue()
                );
                let mut col_result_of_guess = String::new();

                io::stdin()
                    .read_line(&mut col_result_of_guess)
                    .expect("Failed to read line");

                // Put the results of the guess that user entered into a vector so it's easier to work with
                let mut col_result_vec: Vec<String> = Vec::new();
                for result in col_result_of_guess.split_whitespace() {
                    col_result_vec.push(String::from(result));
                }

                // If the results are not valid, tell the user and start again at the top of the loop
                if !verify_result_valid(&col_result_vec) {
                    continue;
                }

                // Iterate
                let indexes: [i8; 3] = [0, 1, 2];
                for n in indexes.iter() {
                    // This is the current row index so all letters apply for the row
                    let row_words: &mut Vec<String> = remaining_words.get_mut(n)
                        .expect("Should get words list from remaining words by index");

                    remove_invalid_words(row_words, &row_result_vec, &solver_guess_string,
                                                          *n, current_column_row_idx,
                                                          true, true);

                    remove_invalid_words(row_words, &col_result_vec, &solver_guess_string,
                                                          *n, current_column_row_idx,
                                                          true, false);

                    let col_words: &mut Vec<String> = remaining_words.get_mut(&(n + 3))
                        .expect("Should get words list from remaining words by index");

                    remove_invalid_words(col_words, &col_result_vec, &solver_guess_string,
                                                          *n, current_column_row_idx,
                                                          false, false);

                    remove_invalid_words(col_words, &row_result_vec, &solver_guess_string,
                                                          *n, current_column_row_idx,
                                                          false, true);
                }


                // Add solution letters to map
                solution_checker::check_for_solution(&mut solution, &row_result_vec, &solver_guess_string, true, current_column_row_idx);
                solution_checker::check_for_solution(&mut solution, &col_result_vec, &solver_guess_string, false, current_column_row_idx);

                println!("== SOLUTION ==");
                let mut solution_idx = 0;
                while solution_idx <= 5 {
                    print!(" {} = ", solution_idx);
                    let solution_vec = solution.get(&solution_idx).unwrap();
                    for letter in solution_vec {
                        if letter.is_empty() {
                            print!("?");
                        } else {
                            print!("{}", letter);
                        }
                    }
                    solution_idx += 1;
                    println!();
                }
                println!();
            }

            // If user requested words to be printed, don't skip to the next column
            if !print_requested {
                // Increment the index, setting it to 0 if we are current on the last row/column
                if current_column_row_idx == 2 {
                    current_column_row_idx = 0;
                } else {
                    current_column_row_idx += 1;
                }

                let indexes: [i8; 3] = [0, 1, 2];
                for n in indexes.iter() {
                    let row_words: &Vec<String> = remaining_words.get(&n)
                        .expect("Should get words for row");
                    let col_words: &Vec<String> = remaining_words.get(&(n + 3))
                        .expect("Should get words for row");
                    println!("Index {}", n);
                    println!("- Row words:    {}", row_words.len());
                    println!("- Column words: {}", col_words.len());
                    println!();
                }
            }
        }
    }

    pub fn remove_invalid_words(word_list: &mut Vec<String>, result: &Vec<String>, solver_guess_string: &String,
                            word_list_index: i8, result_list_index: i8, word_list_is_row: bool, result_list_is_row: bool) {
        // Adjacent if we are looking at columns that do not have the same index
        let word_and_result_matching_index: bool = word_list_index == result_list_index;
        // Perpendicular if the word list is for a column and the result list is for a row or vice versa
        let word_and_result_parallel: bool = word_list_is_row == result_list_is_row;

        /*

        Diagram to help visualize how this works because my brain can't do it:

        0 1 2
        XXXXX 0
        X X X
        XXXXX 1
        X X X
        XXXXX 2

         */

        word_list.retain(|word| {
            for (result_index, result_char) in result.iter().enumerate() {
                let word_char: char = word.chars().nth(result_index).unwrap();
                let guess_char: char = solver_guess_string.chars().nth(result_index).unwrap();

                match result_char.as_str() {
                    // GREEN LETTER
                    "g" => {
                        /* Perpendicular word row and result column
                                  r
                                  r
                                  r
                                  r
                              w w r w w
                         */
                        if word_and_result_matching_index && word_and_result_parallel {
                            // Current Row or Column
                            if word_char != guess_char {
                                return false;
                            }
                        } else if !word_and_result_parallel && word_list_index * 2 == result_index as i8 {
                            if word.chars().nth(result_list_index as usize * 2).unwrap() != guess_char {
                                // The word and result are perpendicular and the green letter is in the word
                                // Make sure the character is in the word at the correct index
                                return false;
                            }
                        }
                    },
                    // YELLOW LETTER
                    s if s.starts_with("y") => {
                        let min_num_yellow_letters: usize = s.trim_start_matches("y").parse::<i32>().unwrap_or(1) as usize;

                        if word_and_result_matching_index && word_and_result_parallel {
                            // Current Row or Column
                            if word_list_is_row {
                                // Current row
                                // Go through each letter for each word in the list. If it has the
                                // yellow letter and the index of the letter is not the same index
                                // as where the yellow letter clue is, then it is found and valid
                                let mut letter_indexes: Vec<usize> = Vec::new();
                                for (letter_index, letter) in word.chars().enumerate() {
                                    if letter == guess_char {
                                        letter_indexes.push(letter_index);
                                    }
                                }

                                if letter_indexes.len() == 0 || letter_indexes.len() < min_num_yellow_letters {
                                    // get rid of words that do not have the letter
                                    return false;
                                } else {
                                    for i in letter_indexes {
                                        if i == result_index {
                                            // get rid of words that have the letter in the same index
                                            return false;
                                        }
                                    }
                                }
                            } else {
                                // Current column
                                // Get rid of all words that include this yellow letter in this column
                                // because it can't be in this column
                                if word.contains(guess_char) {
                                    return false;
                                }
                            }
                        } else if word_and_result_parallel && word_list_is_row{
                            // Parallel rows
                            // Only keep words that do NOT have this letter at this index
                            if word_char == guess_char
                            {
                                return false
                            }
                        } else if word_and_result_parallel && !word_list_is_row {
                            // Parallel columns
                            // We need to keep this so we can check for it later
                            // TODO add a way to keep this to check for it later because this column
                            // or the other adjacent column has to have this letter
                        } else if !word_and_result_parallel && word_list_is_row {
                            /* Perpendicular word row and result column
                                        r
                                w w w w r
                                        r
                                        r
                                        r
                             */
                            if word_list_index * 2 == result_index as i8 {
                                // The yellow letter is IN the word row. So we only want to keep words
                                // that have the yellow letter but NOT at the result_index
                                let mut letter_indexes: Vec<usize> = Vec::new();
                                for (letter_index, letter) in word.chars().enumerate() {
                                    if letter == guess_char {
                                        letter_indexes.push(letter_index);
                                    }
                                }

                                if letter_indexes.len() == 0 || letter_indexes.len() < min_num_yellow_letters {
                                    // get rid of words that do not have the letter
                                    return false;
                                } else {
                                    for i in letter_indexes {
                                        if i == (result_list_index as usize * 2) {
                                            // get rid of words that have the letter in the same index
                                            return false;
                                        }
                                    }
                                }
                            } else {
                                // The yellow letter is NOT in the word's row,
                                if word.chars().nth(result_list_index as usize * 2).unwrap() == guess_char {
                                    return false;
                                }
                            }
                        } else if !word_and_result_parallel && !word_list_is_row {
                            /* Perpendicular result row and word column
                                        w
                                r r r r r
                                        w
                                        w
                                        w
                             */
                            if word_list_index * 2 == result_index as i8 {
                                // The yellow letter is IN the word column. So don't want to keep any
                                // words with the yellow letter
                                if word.contains(guess_char) {
                                    return false;
                                }
                            } else {
                                // The yellow letter is NOT in the word's row
                                // TODO keep track of these at some point to help with deducing words
                            }
                        }
                    },
                    // RED LETTER
                    s if s.starts_with("r") => {
                        let min_num_red_letters: usize = s.trim_start_matches("r").parse::<i32>().unwrap_or(1) as usize;

                        if word_and_result_matching_index && word_and_result_parallel {
                            // Current Row or Column
                            if word_list_is_row {
                                // Current column
                                // Get rid of all words that include this red letter in this column
                                // because it can't be in this column
                                if word.contains(guess_char) {
                                    return false;
                                }
                            } else {
                                // Current row
                                // Go through each letter for each word in the list. If it has the
                                // red letter and the index of the letter is not the same index
                                // as where the red letter clue is, then it is found and valid
                                let mut letter_indexes: Vec<usize> = Vec::new();
                                for (letter_index, letter) in word.chars().enumerate() {
                                    if letter == guess_char {
                                        letter_indexes.push(letter_index);
                                    }
                                }

                                if letter_indexes.len() == 0 || letter_indexes.len() < min_num_red_letters {
                                    // get rid of words that do not have the letter
                                    return false;
                                } else {
                                    for i in letter_indexes {
                                        if i == result_index {
                                            // get rid of words that have the letter in the same index
                                            return false;
                                        }
                                    }
                                }
                            }
                        } else if word_and_result_parallel && word_list_is_row{
                            // Parallel row
                            // We need to keep this so we can check for it later
                            // TODO add a way to keep this to check for it later because this column
                            // or the other adjacent column has to have this letter
                        } else if word_and_result_parallel && !word_list_is_row {
                            // Parallel column
                            // Only keep words that do NOT have this letter at this index
                            if word_char == guess_char
                            {
                                return false
                            }
                        } else if !word_and_result_parallel && word_list_is_row {
                            /* Perpendicular word row and result column
                                        r
                                        r
                                w w w w r
                                        r
                                        r
                             */
                            if word_list_index * 2 == result_index as i8 {
                                // The red letter is IN the word row. So don't want to keep any
                                // words with the red letter
                                if word.contains(guess_char) {
                                    return false;
                                }
                            } else {
                                // The red letter is NOT in the word's row
                                // TODO keep track of these at some point to help with deducing words
                            }
                        } else if !word_and_result_parallel && !word_list_is_row {
                            /* Perpendicular result row and word column
                                    w
                                    w
                                r r r r r
                                    w
                                    w
                             */
                            if word_list_index * 2 == result_index as i8 {
                                // The red letter is IN the word row. So we only want to keep words
                                // that have the red letter but NOT at the result_index
                                let mut letter_indexes: Vec<usize> = Vec::new();
                                for (letter_index, letter) in word.chars().enumerate() {
                                    if letter == guess_char {
                                        letter_indexes.push(letter_index);
                                    }
                                }

                                if letter_indexes.len() == 0 || letter_indexes.len() < min_num_red_letters {
                                    // get rid of words that do not have the letter
                                    return false;
                                } else {
                                    for i in letter_indexes {
                                        if i == (result_list_index as usize * 2) {
                                            // get rid of words that have the letter in the same index
                                            return false;
                                        }
                                    }
                                }
                            } else {
                                // The red letter is NOT in the word's row,
                                if word.chars().nth(result_list_index as usize * 2).unwrap() == guess_char {
                                    return false;
                                }
                            }
                        }
                    },
                    // WHITE LETTER
                    "w" => {
                        if word_and_result_matching_index && word_and_result_parallel {
                            // Get rid of all words in this row/column because this letter cannot be here
                            // The white letter is in the row/column
                            if word.contains(guess_char) {
                                return false;
                            }
                        } else if !word_and_result_parallel {
                            if word_list_index * 2 == result_index as i8 {
                                if word.contains(guess_char) {
                                    return false;
                                }
                            }
                        } else if word_and_result_parallel {
                            // Parallel rows/columns
                            // Only keep words that do NOT have this letter at this index
                            if word_char == guess_char
                            {
                                return false
                            }
                        }
                    },
                    // BLACK LETTER
                    "b" => {
                        // Get rid of all words that include this black letter because it doesn't exist
                        // in the puzzle at all
                        if word.contains(guess_char) {
                            return false;
                        }
                    },
                    // ORANGE LETTER
                    s if s.starts_with("o") => {
                        // Pull out the row and column arrows from the string.
                        // Possible results include:
                        //  "o", "o12", "o21", "o22", "o31", "o13", "o32", "o23", "o33"

                        let mut row_min_num_orange_letters = 1;
                        let mut col_min_num_orange_letters = 1;
                        for (i, c) in s.chars().enumerate() {
                            if c.is_digit(10) {
                                if i == 1 {
                                    row_min_num_orange_letters = c.to_digit(10).unwrap();
                                } else if i == 2{
                                    col_min_num_orange_letters = c.to_digit(10).unwrap();
                                }
                            }
                        }

                        // Assign the minimum number of orange letters based on the row or column value found
                        let mut min_num_orange_letters = col_min_num_orange_letters as usize;
                        if word_list_is_row {
                            min_num_orange_letters = row_min_num_orange_letters as usize;
                        }

                        println!("Found min num orange letters row={}, col={}", row_min_num_orange_letters, col_min_num_orange_letters);

                        if word_and_result_matching_index && word_and_result_parallel {
                            // Current Row or Column
                            // Go through each letter for each word in the list. If it has the
                            // orange letter and the index of the letter is not the same index
                            // as where the orange letter clue is, then it is found and valid
                            let mut letter_indexes: Vec<usize> = Vec::new();
                            for (letter_index, letter) in word.chars().enumerate() {
                                if letter == guess_char {
                                    letter_indexes.push(letter_index);
                                }
                            }

                            if letter_indexes.len() == 0 || letter_indexes.len() < min_num_orange_letters {
                                // get rid of words that do not have the letter or don't have enough of the letter
                                return false;
                            } else {
                                for i in letter_indexes {
                                    if i == result_index {
                                        // get rid of words that have the letter at the same index
                                        return false;
                                    }
                                }
                            }
                        } else if word_and_result_parallel {
                            // Parallel rows
                            // TODO - Use this to help determine other words
                        } else if !word_and_result_parallel {
                            /* Perpendicular word row and result column
                                        r          r                   r
                                w w w w r          w w w w w           r
                                        r    or    r           or  w w w w w
                                        r          r                   r
                                        r          r                   r
                             */
                            if word_list_index * 2 == result_index as i8 {
                                // The orange letter is IN the word row. So we only want to keep words
                                // that have the orange letter but NOT at the result_index
                                let mut letter_indexes: Vec<usize> = Vec::new();
                                for (letter_index, letter) in word.chars().enumerate() {
                                    if letter == guess_char {
                                        letter_indexes.push(letter_index);
                                    }
                                }

                                if letter_indexes.len() == 0 || letter_indexes.len() < min_num_orange_letters {
                                    // get rid of words that do not have the letter
                                    return false;
                                } else {
                                    for i in letter_indexes {
                                        if i == (result_list_index as usize * 2) {
                                            // get rid of words that have the letter in the same index
                                            return false;
                                        }
                                    }
                                }
                            } else {
                                // It's not in the same row / column as the word
                                // TODO - Use this to help determine other words
                            }
                        }
                    }
                    _ => {}//println!("Invalid result {}", result_char)
                }
            }
            return true
        });
    }

    pub fn verify_result_valid(result: &Vec<String>) -> bool {
        let mut valid = true;
        if result.len() != 5 {
            valid = false;
            println!("{} {} {} {}", "!!!".bright_red(), "Expected 5 results but only found".bright_red(), result.len().to_string().bright_red(), "!!!".bright_red());
        } else {
            let valid_entries: Vec<String> =
                vec![
                    "g".to_string(),
                    "b".to_string(),
                    "w".to_string(),
                    "y".to_string(),
                    "y2".to_string(),
                    "y3".to_string(),
                    "r".to_string(),
                    "r2".to_string(),
                    "r3".to_string(),
                    "o".to_string(),
                    "o2".to_string(),
                    "o3".to_string()
                ];
            for entry in result {
                if !valid_entries.contains(entry) {
                    valid = false;
                    println!("\n{} {} {} {}", "!!!".bright_red(), "Invalid result entry found:".bright_red(), entry.bright_red(), "!!!".bright_red());
                }
            }
        }

        if !valid {
            println!();
        }

        return valid;
    }
}

fn main() {
    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "full");
    squardle_solver::start();
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::squardle_solver;

    // Green result tests
    #[test]
    fn remove_invalid_words_green_current_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["g".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_green_current_column() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["g".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_green_adjacent_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["g".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_green_adjacent_col() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["g".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_green_perpendicular() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["g".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "every".to_string();

        /*
        w
        w
        g r r r r
        w
        w
         */

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    // Yellow result tests
    #[test]
    fn remove_invalid_words_yellow_current_row() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string(), "silly".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "y".to_string()];
        let solver_guess_string = "crepe".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_multiple_yellow_current_row() {
        let mut word_list = vec!["spend".to_string(), "sever".to_string(), "dirge".to_string(), "silly".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "y2".to_string(), "x".to_string(), "y2".to_string()];
        let solver_guess_string = "crepe".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["sever".to_string()]);
    }

    #[test]
    fn remove_invalid_words_yellow_current_column() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["y".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_yellow_adjacent_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["y".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index = 0;
        let result_list_index = 2;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_yellow_adjacent_col() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "y".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "smell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_yellow_perpendicular_result_row() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "y".to_string()];
        let solver_guess_string = "crepe".to_string();

        let word_list_index: i8 = 1;
        let result_list_index: i8 = 2;
        let word_list_is_row: bool = false;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_yellow_perpendicular_result_column() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "x".to_string(), "y".to_string(), "x".to_string()];
        let solver_guess_string = "spire".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "crepe".to_string()]);
    }

    // Red result tests
    #[test]
    fn remove_invalid_words_red_current_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "r".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_red_current_column() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string(), "silly".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "r".to_string()];
        let solver_guess_string = "crepe".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_red_adjacent_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "r".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_red_adjacent_col() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "r".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_red_perpendicular_column_result() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "r".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_red_perpendicular_row_result() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "plier".to_string()];
        let result = vec!["x".to_string(), "r".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 2;
        let word_list_is_row: bool = false;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string()]);
    }

    // Black result tests
    #[test]
    fn remove_invalid_words_black_current_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "b".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_black_current_column() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["b".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_black_adjacent_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "b".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_black_adjacent_col() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "b".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["dirge".to_string()]);
    }

    // White result tests
    #[test]
    fn remove_invalid_words_white_current_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "w".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_white_current_column() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["w".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "prize".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_white_adjacent_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "w".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_white_adjacent_col() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "w".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["prize".to_string(), "dirge".to_string()]);
    }

    // Orange result tests
    #[test]
    fn remove_invalid_words_orange_current_row() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string(), "silly".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "o".to_string()];
        let solver_guess_string = "crepe".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_orange_current_column() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "o".to_string()];
        let solver_guess_string = "crepe".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string()]);
    }

    #[test]
    fn remove_invalid_words_orange_adjacent_row() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "o".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        /*

        w w w w w
        r o r r r

         */

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_orange_adjacent_col() {
        let mut word_list = vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["x".to_string(), "o".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "spell".to_string();

        /*

          w r
          w o
          w r
          w r
          w r

         */

        let word_list_index = 0;
        let result_list_index = 1;
        let word_list_is_row = false;
        let result_list_is_row = false;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["spend".to_string(), "prize".to_string(), "dirge".to_string()]);
    }

    #[test]
    fn remove_invalid_words_multiple_orange_current_row() {
        let mut word_list = vec!["spend".to_string(), "crepe".to_string(), "dirge".to_string(), "silly".to_string()];
        let result = vec!["o21".to_string(), "x".to_string(), "x".to_string(), "x".to_string(), "x".to_string()];
        let solver_guess_string = "elope".to_string();

        let word_list_index: i8 = 0;
        let result_list_index: i8 = 0;
        let word_list_is_row: bool = true;
        let result_list_is_row: bool = true;
        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string, word_list_index,
                                              result_list_index, word_list_is_row, result_list_is_row);
        assert_eq!(word_list, vec!["crepe".to_string()]);
    }

    #[test]
    fn remove_invalid_words_real_life_red_perp() {
        let mut word_list = vec!["imago".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["g".to_string(), "r".to_string(), "r".to_string(), "b".to_string(), "b".to_string()];
        let solver_guess_string = "waive".to_string();

        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string,
                                              1, 0,
                                              false, true);


        assert!(!word_list.contains(&"imago".to_string()));
    }

    #[test]
    fn remove_invalid_words_real_life_white_perp() {
        let mut word_list = vec!["vista".to_string(), "prize".to_string(), "dirge".to_string()];
        let result = vec!["w".to_string(), "b".to_string(), "w".to_string(), "b".to_string(), "b".to_string()];
        let solver_guess_string = "thick".to_string();

        squardle_solver::remove_invalid_words(&mut word_list, &result, &solver_guess_string,
                                              1, 0,
                                              false, true);


        assert!(!word_list.contains(&"vista".to_string()));
    }

    #[test]
    fn remove_invalid_words_real_life_greens() {
        let words_list_filename: String = String::from("./src/words-list");

        let contents = fs::read_to_string(words_list_filename)
            .expect("Should have been able to read the file");

        // Read the words and assign them to each of the row/column vectors
        let mut words: Vec<String> = Vec::new();
        for word in contents.split_whitespace() {
            words.push(String::from(word));
        }

        let solver_guess_string1 = "ohias".to_string();

        let row_result1 = vec!["o".to_string(), "b".to_string(), "r".to_string(), "w".to_string(), "o".to_string()];

        squardle_solver::remove_invalid_words(&mut words, &row_result1, &solver_guess_string1,
                                              1, 1,
                                              false, true);

        assert!(words.contains(&"stoic".to_string()));
    }

    #[test]
    fn verify_result_valid_correct() {
        let result: Vec<String> = vec!["r".to_string(), "y2".to_string(), "b".to_string(), "o".to_string(), "w".to_string()];
        assert!(squardle_solver::verify_result_valid(&result));
    }

    #[test]
    fn verify_result_valid_not_enough_letters() {
        let result: Vec<String> = vec!["r".to_string(), "y2".to_string(), "b".to_string(), "o".to_string()];
        assert!(!squardle_solver::verify_result_valid(&result));
    }

    #[test]
    fn verify_result_valid_wrong_letters() {
        let result: Vec<String> = vec!["p".to_string(), "y2".to_string(), "b".to_string(), "o".to_string(), "o".to_string()];
        assert!(!squardle_solver::verify_result_valid(&result));
    }
}

// VALUE
//
// Enter ROW results (g, y, o, r, w, b):
// b b y o g
// Enter COLUMN results (g, y, o, r, w, b):
// b b w r y
