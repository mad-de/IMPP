use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Input,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub id: i32,
    pub answer: String,
    pub question: String,
    pub category: String,
    pub extra: String,
}

pub static BEGIN_CHARS: &str = "<td class=\"s[1-9]{1}\">";
pub static BEGIN_ALT_CHARS: &str = "<td class=\"s[1-9]{1} softmerge\">";
pub static BEGIN_ALT_EXTRA_CHARS: &str =
    r#"<div class="softmerge-inner" style="width: 512px; left: -1px;">"#;

// Main function to import a http request from a Google Sheet
pub fn import_googlesheet(httprequest: String, path: &str) -> i32 {
    // Return Vec with our Questions database. Hand in Vector for easier handling.
    let questions_db = extract_from_raw_data([httprequest, String::from("")].to_vec());
    let file_path = path.to_owned() + "database.json";
    // Serialize our Questions database to json
    let data = String::from(
        serde_json::to_string(&questions_db).expect("Transferring Vector to JSON failed."),
    );
    fs::write(file_path.clone(), &data).expect("Writing the database file did not work.");
    // If saving file is not possible, process will break with an Error. If we get here, return true;
    i32::try_from(questions_db.len()).expect("could not convert uszie to i32")
}

pub fn get_database_status(path: &str) -> bool {
    let file_path = path.to_owned() + "database.json";
    Path::new(&file_path).exists()
}

// Main function to generate a random question number
pub fn generate_random_question(category: String, path: &str) -> i32 {
    i32::try_from(generate_random_question_number(
        &import_json_question_db(path),
        &category,
    ))
    .expect("Random number could not be converted to i32.")
}

// Main function to return a String with all details of our question.
pub fn get_question_details(our_question_num: i32, jeopardy_mode: bool, path: &str) -> [String; 4] {
    let question_details = get_question_vector(
        &import_json_question_db(path),
        jeopardy_mode,
        usize::try_from(our_question_num)
            .expect("Our question number could not be converted to usize."),
    );
    let array: [String; 4] = [
        String::from(&question_details[0].question),
        String::from(&question_details[0].answer),
        String::from(&question_details[0].category),
        String::from(&question_details[0].extra),
    ];
    array
}

// Main function to return a vector with max 4 distractors to our question
pub fn get_mc_distractors(
    question_num: i32,
    distractor_amount: i32,
    jeopardy_mode: bool,
    path: &str,
) -> Vec<Question> {
    generate_mc_distractors(
        &import_json_question_db(&path),
        usize::try_from(question_num)
            .expect("Our question number could not be converted to usize."),
        jeopardy_mode,
        usize::try_from(distractor_amount)
            .expect("Our distractor amount could not be converted to usize."),
    )
}

// Main function to return a vector with all categories
pub fn get_categories(path: &str) -> HashSet<String> {
    let questions_db = import_json_question_db(&path);
    let mut categories = HashSet::new();
    for item in &questions_db {
        if !categories.contains(&item.category) {
            categories.insert(String::from(&item.category));
        }
    }
    categories
}

// read our questions_db. When calling the function, make sure that this database exists.
pub fn import_json_question_db(path: &str) -> Vec<Question> {
    let file_path = path.to_owned() + "database.json";
    let questions_db: Vec<Question> = serde_json::from_str(
        &fs::read_to_string(file_path.clone()).expect("Opening the database file did not work."),
    )
    .expect("Converting the database did not work.");
    questions_db
}

// Cut the title from a given httprequest
pub fn return_title(input: String) -> String {
    let string_array: [String; 2] = [input, String::from("")];
    let begin_chars = "<title>";
    let end_chars = "</title>";
    let mut pos = string_array[0].find(begin_chars).unwrap() + begin_chars.chars().count();

    let (_old_string, new_string) = string_array[0].split_at(pos); // cut everything before our string
    pos = new_string.find(end_chars).unwrap(); // find end of our string
    let (this_string, _this_rest_string) = new_string.split_at(pos);

    String::from(this_string)
}

// Return a vector with all details of one question
pub fn get_question_vector(
    questions_db: &[Question],
    jeopardy_mode: bool,
    our_question_num: usize,
) -> Vec<Question> {
    let mut this_questions_vec = vec![];
    let mut this_question = String::from(&questions_db[our_question_num].question);
    let mut this_answer = String::from(&questions_db[our_question_num].answer);
    if jeopardy_mode == true {
        this_question = String::from(&questions_db[our_question_num].answer);
        this_answer = String::from(&questions_db[our_question_num].question);
    }
    let question0 = Question {
        id: questions_db[our_question_num].id,
        question: this_question,
        answer: this_answer,
        category: String::from(&questions_db[our_question_num].category),
        extra: String::from(&questions_db[our_question_num].extra),
    };
    this_questions_vec.push(question0);
    this_questions_vec
}

// Extract database from http request string
pub fn extract_from_raw_data(mut string_array: Vec<String>) -> Vec<Question> {
    let mut this_id: i32 = 0;
    let mut this_question: String;
    let mut this_answer: String;
    let mut this_category: String;
    let mut this_extra: String;
    let mut questions_db = vec![];

    while Regex::new(BEGIN_CHARS).unwrap().is_match(&string_array[0]) {
        let mut initial = true;
        while (string_array[1] == "" || string_array[1] == "EOL" || initial)
            && Regex::new(BEGIN_CHARS).unwrap().is_match(&string_array[0])
        {
            // search for the first normally formatted field
            extract_field_value(&mut string_array).unwrap();
            initial = false;
        }
        if string_array[1] == "EOL" {
            // if we get the last EOL break
        } else {
            this_question = string_array[1].to_string();
            extract_field_value(&mut string_array).unwrap();
            if string_array[1] != "EOL" {
                this_answer = string_array[1].to_string();
            } else {
                this_answer = String::from("");
            }
            extract_field_value(&mut string_array).unwrap();
            if string_array[1] != "EOL" {
                this_category = string_array[1].to_string();
            } else {
                this_category = String::from("");
            }
            extract_field_value(&mut string_array).unwrap();

            if string_array[1] != "EOL" {
                this_extra = string_array[1].to_string();
            } else {
                this_extra = String::from("");
            }

            let question1 = Question {
                id: this_id,
                question: this_question,
                answer: this_answer.clone(),
                category: this_category.clone(),
                extra: this_extra.clone(),
            };
            if question1.question.is_empty() && question1.answer.is_empty() {
            } else {
                questions_db.push(question1);
                this_id = this_id + 1;
                // Only for Desktop version
                print!("\rImporting dataset #{}", this_id);
            }
        }
    }

    questions_db
}

// Extract a value from a string
pub fn extract_field_value(string_array: &mut [String]) -> Result<(), Error> {
    if string_array.is_empty() {
        return Err(Error::Input);
    }
    let mut end_chars = "</td>";
    let end_alt_chars = "</div>";
    let mut pos_alt = 10000;
    let mut pos = Regex::new(BEGIN_CHARS)
        .unwrap()
        .find(&string_array[0])
        .unwrap()
        .end(); // Finds first encounter of a substring in our string
    if Regex::new(BEGIN_ALT_CHARS)
        .unwrap()
        .is_match(&string_array[0])
    {
        pos_alt = Regex::new(BEGIN_ALT_CHARS)
            .unwrap()
            .find(&string_array[0])
            .unwrap()
            .end()
            + BEGIN_ALT_EXTRA_CHARS.chars().count();
    }
    if pos_alt < pos {
        pos = pos_alt;
        end_chars = end_alt_chars;
    }

    let (_old_string, new_string) = string_array[0].split_at(pos); // cut everything before our string
    pos = new_string.find(end_chars).unwrap(); // find end of our string
    let (this_string, this_item) = new_string.split_at(pos); // extradite string and generate new string
    let clone_this_string = String::from(this_string); // copy string with mut until I figure out a nicer way to do it
    string_array[0] = String::from(this_item); // Return values
    string_array[1] = clone_this_string;
    Ok(())
}

// Generate a random question number
pub fn generate_random_question_number(questions_db: &[Question], topic: &str) -> usize // Todo: add weighting option https://rust-num.github.io/num/rand/distributions/struct.WeightedChoice.html | return a state if topic is not found
{
    let mut this_number = rand::thread_rng().gen_range(0, questions_db.len());
    let mut i = 0;
    let mut category_exists = false;
    while i < questions_db.len() {
        // does the topic even exist? TODO: can this be replaced with a contains() somehow?
        if questions_db[i].category == topic {
            category_exists = true;
        }
        i += 1;
    }

    if !topic.is_empty() && category_exists {
        while questions_db[this_number].category != topic {
            // check if our random number has the right category
            this_number = rand::thread_rng().gen_range(0, questions_db.len());
        }
    }
    this_number
}

// Generate MC distractors from the same category in our database
pub fn generate_mc_distractors(
    questions_db: &[Question],
    our_question_num: usize,
    jeopardy_mode: bool,
    num_mc_questions: usize,
) -> Vec<Question> // Return a vector with x items with number 0 being the correct answer.
{
    // check how many answers of our category are in our vector
    let mut this_num_mc = num_mc_questions + 1;
    let mut i = 0;
    let mut count_category_items = 0;
    let mut temp_question_num = rand::thread_rng().gen_range(0, questions_db.len());
    while i < questions_db.len() {
        if questions_db[i].category == questions_db[our_question_num].category {
            count_category_items += 1;
        }
        i += 1;
    }

    if count_category_items < num_mc_questions {
        this_num_mc = count_category_items;
    }

    // Build two arrays (one where all the answers are saved which we don't want to use anymore and one where all answers are saved.
    let mut new_questions_db = vec![];
    let mut curr_questions = vec![];
    curr_questions.push(String::from(&questions_db[our_question_num].question));
    i = 1;
    while i < this_num_mc {
        if !(curr_questions.contains(&questions_db[temp_question_num].question))
            && questions_db[temp_question_num].category == questions_db[our_question_num].category
        {
            if !(jeopardy_mode) {
                let question1 = Question {
                    id: questions_db[temp_question_num].id,
                    question: String::from(&questions_db[temp_question_num].question),
                    answer: String::from(&questions_db[temp_question_num].answer),
                    category: String::from(&questions_db[temp_question_num].category),
                    extra: String::from(&questions_db[temp_question_num].extra),
                };
                new_questions_db.push(question1);
            } else {
                let question1 = Question {
                    id: questions_db[temp_question_num].id,
                    question: String::from(&questions_db[temp_question_num].answer),
                    answer: String::from(&questions_db[temp_question_num].question),
                    category: String::from(&questions_db[temp_question_num].category),
                    extra: String::from(&questions_db[temp_question_num].extra),
                };
                new_questions_db.push(question1);
            }
            curr_questions.push(String::from(&questions_db[temp_question_num].question));
            i += 1;
        }
        temp_question_num = rand::thread_rng().gen_range(0, questions_db.len());
    }
    new_questions_db
}

#[cfg(test)]
mod base_function_tests {
    use super::*;

    mod extract_field_value {
        use super::*;

        #[test]
        fn from_empty_string_array() {
            let mut arr = vec![];
            assert!(extract_field_value(&mut arr).is_err());
        }
    }
    mod check_database {
        use super::*;

        #[test]
        fn known_database_result_num() {
            assert!(&import_json_question_db("src/tests/").len() == &usize::try_from(10).unwrap());
        }
    }
}

#[cfg(test)]
mod module_tests {
    use super::*;

    mod main_modules {
        use super::*;
        use std::fs;

        #[test]
        fn return_correct_title() {
            let sample_table =
                String::from(fs::read_to_string("src/tests/sample_table.txt").unwrap());
            assert!(return_title(sample_table) == "IMPP sample table".to_string());
        }

        // Check if result from an import equals our sample json file
        #[test]
        fn import_googlesheet_correct() {
            let sample_table =
                String::from(fs::read_to_string("src/tests/sample_table.txt").unwrap());
            import_googlesheet(sample_table, &"target/");
            assert!(
                String::from(fs::read_to_string("target/database.json").unwrap())
                    == String::from(fs::read_to_string("src/tests/sample_database.json").unwrap())
            );
        }

        #[test]
        fn generate_random_question_number_for_category() {
            assert!(generate_random_question(String::from("Endocrinology"), "src/tests/") == 9);
        }

        #[test]
        fn get_known_question_details() {
            assert!(
                get_question_details(2, false, "src/tests/")
                    == ["Fabella sign", "Displacement of the fabella that is seen in cases of synovial effusion and popliteal fossa masses", "Radiologic sign", ""]
            );
        }

        #[test]
        fn get_known_question_details_jeopardy_mode_true() {
            assert!(
                get_question_details(2, true, "src/tests/")
                    == ["Displacement of the fabella that is seen in cases of synovial effusion and popliteal fossa masses", "Fabella sign", "Radiologic sign", ""]
            );
        }

        #[test]
        fn count_distractors_none() {
            assert!(get_mc_distractors(9, 4, false, "src/tests/").len() == 0);
        }

        #[test]
        fn count_distractors_all() {
            assert!(get_mc_distractors(1, 4, false, "src/tests/").len() == 4);
        }

        #[test]
        fn count_distractors_size3() {
            assert!(get_mc_distractors(1, 3, false, "src/tests/").len() == 3);
        }

        #[test]
        fn count_known_categories() {
            assert!(get_categories("src/tests/").len() == 3);
        }
        #[test]
        fn test_database_exists() {
            assert!(get_database_status("src/tests/") == true);
        }
    }
}
