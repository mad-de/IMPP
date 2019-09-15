use preferences::{AppInfo, Preferences, PreferencesMap};
use rand::Rng;
use regex::Regex;
//use webpage::{Webpage, WebpageOptions};
// WORKAROUND for MacOS
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub category: String,
    pub extra: String,
}

#[derive(Debug)]
pub enum Error {
    Input,
}

const APP_INFO: AppInfo = AppInfo {
    name: "IMPP",
    author: "mad-de & others",
};

pub static BEGIN_CHARS: &'static str = "<td class=\"s[1-9]{1}\">";
pub static BEGIN_ALT_CHARS: &'static str = "<td class=\"s[1-9]{1} softmerge\">";
pub static BEGIN_ALT_EXTRA_CHARS: &'static str =
    r#"<div class="softmerge-inner" style="width: 512px; left: -1px;">"#;
pub static PREF_KEY_ADDR: &'static str = "preferences/apps/impp";

pub fn return_pref_key(pref_key: &str) -> String {
    let mut spreadsheet_url = String::from("");
    let load_preferences = PreferencesMap::<String>::load(&APP_INFO, PREF_KEY_ADDR);
    if load_preferences.is_ok() {
        for (index, string) in load_preferences.unwrap() {
            if index == pref_key {
                spreadsheet_url = String::from(string);
            }
        }
    }
    return spreadsheet_url;
}

pub fn insert_pref_key(pref_key: &str, string: &str) -> bool {
    let mut insert_preferences: PreferencesMap<String> = PreferencesMap::new();
    insert_preferences.insert(pref_key.into(), string.into());
    let save_result = insert_preferences.save(&APP_INFO, PREF_KEY_ADDR);
    assert!(save_result.is_ok());
    return true;
}

pub fn generate_mc_questions(
    questions_db: &Vec<Question>,
    our_question_num: usize,
    jeopardy_mode: bool,
    num_mc_questions: usize,
) -> Vec<Question> // Return a vector with x items with number 0 being the correct answer.
{
    // check how many answers of our category are in our vector
    let mut this_num_mc = num_mc_questions;
    let mut i = 0;
    let mut count_category_items = 0;
    let mut temp_question_num = rand::thread_rng().gen_range(0, questions_db.len());
    while i < questions_db.len() {
        if questions_db[i].category == questions_db[our_question_num].category {
            count_category_items = count_category_items + 1;
        }
        i = i + 1;
    }

    if count_category_items < num_mc_questions {
        this_num_mc = count_category_items;
    }

    let mut new_questions_db = vec![];

    // Push our correct question as question [0]
    if !(jeopardy_mode) {
        let question0 = Question {
            question: String::from(&questions_db[our_question_num].question),
            answer: String::from(&questions_db[our_question_num].answer),
            category: String::from(&questions_db[our_question_num].category),
            extra: String::from(&questions_db[our_question_num].extra),
        };
        new_questions_db.push(question0);
    } else {
        let question0 = Question {
            question: String::from(&questions_db[our_question_num].answer),
            answer: String::from(&questions_db[our_question_num].question),
            category: String::from(&questions_db[our_question_num].category),
            extra: String::from(&questions_db[our_question_num].extra),
        };
        new_questions_db.push(question0);
    }

    let mut curr_questions = vec![];
    curr_questions.push(String::from(&questions_db[our_question_num].question));
    i = 1;
    while i < this_num_mc {
        if !(curr_questions.contains(&questions_db[temp_question_num].question))
            && &questions_db[temp_question_num].category == &questions_db[our_question_num].category
        {
            if !(jeopardy_mode) {
                let question1 = Question {
                    question: String::from(&questions_db[temp_question_num].question),
                    answer: String::from(&questions_db[temp_question_num].answer),
                    category: String::from(&questions_db[temp_question_num].category),
                    extra: String::from(&questions_db[temp_question_num].extra),
                };
                new_questions_db.push(question1);
            } else {
                let question1 = Question {
                    question: String::from(&questions_db[temp_question_num].answer),
                    answer: String::from(&questions_db[temp_question_num].question),
                    category: String::from(&questions_db[temp_question_num].category),
                    extra: String::from(&questions_db[temp_question_num].extra),
                };
                new_questions_db.push(question1);
            }
            curr_questions.push(String::from(&questions_db[temp_question_num].question));
            i = i + 1;
        }
        temp_question_num = rand::thread_rng().gen_range(0, questions_db.len());
    }
    return new_questions_db;
}

pub fn order_vec_by_rand(mut questions_db: Vec<Question>) -> Vec<Question> {
    questions_db.sort_by(|_a, _b| {
        rand::thread_rng()
            .gen_range(0, 10)
            .cmp(&rand::thread_rng().gen_range(0, 10))
    });
    if rand::thread_rng().gen_range(0, 10) > 5 {
        questions_db.reverse();
    }
    return questions_db;
}

//pub fn fetch_data(url: &str) -> Result<Vec<String>, ()> {
pub fn fetch_data(mut url: &str) -> Result<Vec<String>, ()> {
    //    let body = Webpage::from_url(url, WebpageOptions::default()).expect("Could not read from URL").http.body;
    if url == "" {} // fix warning remove later.
    // path: src/vokabelliste.txt OR src/sample_table.txt
    url = "src/vokabelliste.txt";
    let mut file = File::open(url).expect("Unable to open");
    let mut body = String::new();
    file.read_to_string(&mut body).expect("Could not read file");
    //println!("{}", &body);
    let string_array: [String; 2] = [body, String::from("")];
    //    let string_array: [String; 2] = [body.to_string(), String::from("")];
    Ok(string_array.to_vec())
}

pub fn check_database(database_url: &str) -> bool {
    println!("Loading your database...");

    let raw_data = fetch_data(&database_url).unwrap();
    let questions_db = extract_from_raw_data(raw_data);
    if questions_db.len() == 0 {
        return false;
    } else {
        return true;
    }
}

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

pub fn generate_random_question_number(questions_db: &Vec<Question>, topic: &str) -> usize // Todo: add weighting option https://rust-num.github.io/num/rand/distributions/struct.WeightedChoice.html | return a state if topic is not found
{
    let mut this_number = rand::thread_rng().gen_range(0, questions_db.len());
    let mut i = 0;
    let mut category_exists = false;
    while i < questions_db.len() {
        // does the topic even exist? TODO: can this be replaced with a contains() somehow?
        if questions_db[i].category == topic {
            category_exists = true;
        }
        i = i + 1;
    }

    if !(topic.is_empty()) && category_exists {
        while questions_db[this_number].category != topic {
            // check if our random number has the right category
            this_number = rand::thread_rng().gen_range(0, questions_db.len());
        }
    }
    return this_number;
}

pub fn extract_from_raw_data(mut string_array: Vec<String>) -> Vec<Question> {
    let mut this_question: String;
    let mut this_answer: String;
    let mut this_category: String;
    let mut this_extra: String;
    let mut questions_db = vec![];
    while Regex::new(BEGIN_CHARS).unwrap().is_match(&string_array[0]) {
        let mut initial = true;
        while (string_array[1] == "" || string_array[1] == "EOL" || initial == true)
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
                question: this_question,
                answer: this_answer.clone(),
                category: this_category.clone(),
                extra: this_extra.clone(),
            };
            if question1.question.is_empty() && question1.answer.is_empty() {
            } else {
                questions_db.push(question1);
            }
        }
    }

    questions_db
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extract_field_value {
        use super::*;

        #[test]
        fn from_empty_string_array() {
            let mut arr = vec![];
            assert!(extract_field_value(&mut arr).is_err());
        }
    }

    mod check_generate_mc_functions {
        use super::*;

        #[test]
        fn return_only_one_answer() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            let questions_db = extract_from_raw_data(raw_data_test);
            assert!(generate_mc_questions(&questions_db, 9, false, 5).len() == 1);
        }

        #[test]
        fn return_four_answers() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            let questions_db = extract_from_raw_data(raw_data_test);
            assert!(generate_mc_questions(&questions_db, 6, false, 5).len() == 4);
        }

        #[test]
        fn jeopardy_mode_works() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            let questions_db = extract_from_raw_data(raw_data_test);
            assert!(generate_mc_questions(&questions_db, 9, true, 5)[0].answer == "SHBG");
        }

    }
    mod check_database {
        use super::*;

        #[test]
        fn known_database_check_positive() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            assert!(check_database(database_url));
        }
        #[test]
        fn known_database_result_num() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            assert!(extract_from_raw_data(raw_data_test).len() == 10);
        }
    }
}
