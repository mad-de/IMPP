use self::lib::*;
use regex::Regex;
use std::io;

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

fn get_input(message: &str) -> String {
    if !(message == "") {
        println!("{}", message);
    }
    let mut this_input = String::from("");
    io::stdin()
        .read_line(&mut this_input)
        .expect("Failed to read line");
    return this_input.trim().to_string();
}

fn extract_topic(input_string: &str) -> &str {
    if Regex::new("-t \"(.*?)\"").unwrap().is_match(input_string) {
        let re = Regex::new("-t \"(.*?)\"").unwrap();
        let this_return = re.captures(input_string).unwrap();
        return this_return.get(1).unwrap().as_str();
    } else {
        return "";
    }
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
                let mut mc_questions_vec = generate_mc_questions(
                    &questions_db,
                    question_num,
                    input_root.contains("-j"),
                    num_mc_questions,
                );
                mc_questions_vec = order_vec_by_rand(mc_questions_vec);
                let mut this_question: String;
                let mut this_answer: String;
                // let mut this_gen_answer;

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
                if input_root.contains("-a") || input_curr.contains("m") && !(mc_questions_vec.len() == 1) {
                    let mut f = 0;
                    while f < mc_questions_vec.len() {
                        println!("{}) {}", characters[f], mc_questions_vec[f].answer);
                        f = f + 1;
                    }
                    input_curr = get_input("").to_string().to_uppercase();
                    if characters.contains(&input_curr) {
                        let index = characters
                            .iter()
                            .position(|r| r.to_string() == input_curr)
                            .unwrap();
                        if index < mc_questions_vec.len() {
                            if this_question
                                == mc_questions_vec[index].question
                            {
                                println!("{}) is correct!", input_curr);
                            } else {
                                println!("Wrong!");
                            }
                        }
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

    mod return_by_topic {
        use super::*;

        #[test]
        fn extract_topic_test_string() {
            assert!(extract_topic("-t \"test topic\"") == "test topic");
        }

        #[test]
        fn extract_topic_empty_string() {
            assert!(extract_topic("-s \"fail this test\"").is_empty());
        }

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
        fn return_question_for_invalid_topic() {
            let database_url = "https://docs.google.com/spreadsheets/d/14fNP2Elca82rryRJ8-a_XwH3_oZgrJyXqh7r7Q7GuEc/edit#gid=0";
            let raw_data_test = fetch_data(&database_url).unwrap();
            let questions_db = extract_from_raw_data(raw_data_test);
            let this_number = generate_random_question_number(
                &questions_db,
                extract_topic(&"-t \"wrong category\""),
            );
            assert!(!(questions_db[this_number].question.is_empty()));
        }
    }
}
