use self::lib::*;
use rand::Rng;
use regex::Regex;

mod lib;

fn print_welcome_msg(number_of_questions: usize) {
    println!(
        r#"-----------------------
- Welcome to the IMPP -
-----------------------
> To see all questions in the database type 'db'.
> To play a game of normal single-choice questions type 'mc' or 'mc -n'
|- For a game of automatic single-choice questions add '-a' (eg. 'mc -a')
|- For a reversed (Jeopardy-Style) game add '-a' (eg. 'mc -a -j')
|- To only get questions about a specific topic add '-t "topic"
> To edit your database url type 'ed'
> To go back to this menu type 'exit', to quit the program type 'quit'
We have {} items in our database."#,
        number_of_questions
    );
}

fn fetch_data(url: &str) -> Result<Vec<String>, ()> {
    let body = ureq::get(url).call().into_string().unwrap();
    let string_array: [String; 2] = [body, String::from("")];
    Ok(string_array.to_vec())
}

fn check_database(database_url: &str) -> bool {
    println!("Loading your database...");

    let raw_data = fetch_data(&database_url).unwrap();
    let questions_db = extract_from_raw_data(raw_data);
    if questions_db.len() == 0 {
        return false;
    } else {
        return true;
    }
}

fn get_new_spreadsheet_url() -> String {
    let mut spreadsheet_url: String;
    loop {
        spreadsheet_url = get_input("Please specify the URL of your spreadsheet:");
        if check_database(&spreadsheet_url) {
            insert_pref_key("primary_db", &spreadsheet_url);
            break;
        } else {
            let check_input = get_input(
                "Your database seems to be empty. Are you sure you want to continue? y/n",
            );
            if check_input.contains("y") {
                insert_pref_key("primary_db", &spreadsheet_url);
                break;
            }
        }
    }
    return spreadsheet_url;
}

fn extract_topic(input_string: &str) -> &str {
    if Regex::new("-t \"(.*?)\"").unwrap().is_match(input_string) {
        let re = Regex::new("-t \"(.*?)\"").unwrap();
        let this_return = re.captures(input_string).unwrap();
        return this_return.get(1).unwrap().as_str();
    } else {
        return ("");
    }
}

fn generate_random_question_number(questions_db: &Vec<Question>, topic: &str) -> usize // Todo: add weighting option https://rust-num.github.io/num/rand/distributions/struct.WeightedChoice.html | return a state if topic is not found
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
        while (questions_db[this_number].category != topic) {
            // check if our random number has the right category
            this_number = rand::thread_rng().gen_range(0, questions_db.len());
        }
    }
    return this_number;
}

fn generate_mc_questions(
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
    while (i < questions_db.len()) {
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
    while (i < this_num_mc) {
            if !(curr_questions.contains(&questions_db[temp_question_num].question))
                && &questions_db[temp_question_num].category
                    == &questions_db[our_question_num].category
            {
                let question1 = Question {
                    question: String::from(&questions_db[temp_question_num].question),
                    answer: String::from(&questions_db[temp_question_num].answer),
                    category: String::from(&questions_db[temp_question_num].category),
                    extra: String::from(&questions_db[temp_question_num].extra),
                };
                new_questions_db.push(question1);
                curr_questions.push(String::from(&questions_db[temp_question_num].question));
            i = i + 1;
            }
            temp_question_num = rand::thread_rng().gen_range(0, questions_db.len());
    }
    return new_questions_db;
}

fn main() {
    let mut spreadsheet_url = return_pref_key("primary_db"); // load our spreasheet url
    if spreadsheet_url.is_empty() {
        println!("You seem to be here for the first time.");
        spreadsheet_url = get_new_spreadsheet_url();
    }

    let raw_data = fetch_data(&spreadsheet_url).unwrap();
    let questions_db = extract_from_raw_data(raw_data);
    // START LOOP
    loop {
        print_welcome_msg(questions_db.len());
        let mut input_curr = String::new();
        let input_root = get_input("");
        if input_root.contains("quit") {
            break;
        } else if input_root.contains("db") {
            for item in &questions_db {
                println!(
                    "Frage: \'{}\', Antwort: \'{}\', Kategorie: \'{}\', Extra: \'{}\'",
                    item.question, item.answer, item.category, item.extra
                );
            }
        } else if input_root.contains("ed") {
            get_new_spreadsheet_url();
            println!("Database changed, please restart program.");
            break;
        } else if input_root.contains("mc") {
            loop {
                println!("------------------------------------------------------------------------------------------------------");
                let num_mc_questions = 5;
                let question_num =
                    generate_random_question_number(&questions_db, extract_topic(&input_root));
                let mc_questions_vec = generate_mc_questions(
                    &questions_db,
                    question_num,
                    input_root.contains("-j"),
                    num_mc_questions,
                );
                let correct_answer_num: usize;
                let mut num_mc = 0;
                let mut temp_question_num: usize;
                let mut this_question: String;
                let mut this_answer: String;
                let mut this_gen_answer;
                let characters: [String; 10] = [
                    String::from("A"),
                    String::from("B"),
                    String::from("C"),
                    String::from("D"),
                    String::from("E"),
                    String::from("F"),
                    String::from("G"),
                    String::from("H"),
                    String::from("I"),
                    String::from("J"),
                ];
                // Switch answer and question in jeopardy-mode
                if input_root.contains("-j") {
                    this_question = String::from(&questions_db[question_num].answer);
                    this_answer = String::from(&questions_db[question_num].question);
                } else {
                    this_question = String::from(&questions_db[question_num].question);
                    this_answer = String::from(&questions_db[question_num].answer);
                }
                if input_root.contains("-a") {
                    println!("Frage: \'{}\' \n", this_question);
                } else {
                    println!("Frage: \'{}\' (type \'m\' for multiple choice mode or any key to reveal the answer)", this_question);
                    input_curr = get_input("");
                }
                if input_root.contains("-a") || input_curr.contains("m") {
                    correct_answer_num = rand::thread_rng().gen_range(0, num_mc_questions);
                    // Generate answers
                    while num_mc < num_mc_questions {
                        if num_mc == correct_answer_num {
                            println!("{}) {}", characters[num_mc], this_answer);
                        } else {
                            temp_question_num = question_num; // Set temporary question to current question so the while has to fail initially
                            while &questions_db[temp_question_num].question
                                == &questions_db[question_num].question
                                || &questions_db[temp_question_num].category
                                    != &questions_db[question_num].category
                            {
                                temp_question_num =
                                    rand::thread_rng().gen_range(0, questions_db.len());
                            }

                            if input_root.contains("-j") {
                                this_gen_answer =
                                    String::from(&questions_db[temp_question_num].question);
                            } else {
                                this_gen_answer =
                                    String::from(&questions_db[temp_question_num].answer);
                            }

                            println!("{}) {}", characters[num_mc], this_gen_answer);
                        }
                        num_mc += 1;
                    }
                    input_curr = get_input("").to_string().to_uppercase();
                    if input_curr == characters[correct_answer_num] {
                        println!("{}) is correct!", characters[correct_answer_num]);
                    } else {
                        println!(
                            "Wrong! The right one is {})",
                            characters[correct_answer_num]
                        );
                    }
                }
                println!("The correct answer is: {}", this_answer);
                if &questions_db[question_num].extra != "" {
                    println!("Extra info: {}", &questions_db[question_num].extra);
                }
                if input_curr.to_uppercase().contains("EXIT") {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod main_tests {
    use super::*;

    mod check_string_functions {
        use super::*;

        #[test]
        fn extract_topic_test_string() {
            assert!(extract_topic("-t \"test topic\"") == "test topic");
        }

        #[test]
        fn extract_topic_empty_string() {
            assert!(extract_topic("-s \"fail this test\"").is_empty());
        }
    }

    mod check_generate_mc_functions {
        use super::*;

        #[test]
        fn return_question_by_topic() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            let questions_db = extract_from_raw_data(raw_data_test);
            let this_number = generate_random_question_number(
                &questions_db,
                extract_topic(&"-t \"Endocrinology\""),
            );
            assert!(questions_db[this_number].question == "SHBG");
        }

        #[test]
        fn return_question_invalid_topic() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            let questions_db = extract_from_raw_data(raw_data_test);
            let this_number = generate_random_question_number(
                &questions_db,
                extract_topic(&"-t \"wrong category\""),
            );
            assert!(!(questions_db[this_number].question.is_empty()));
        }

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
