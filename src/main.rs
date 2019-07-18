use self::lib::*;
use rand::Rng;

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
            } } else if input_root.contains("ed") {
             get_new_spreadsheet_url();
        println!("Database changed, please start program again.");
break;
        } else if input_root.contains("mc") {
            loop {
                println!("------------------------------------------------------------------------------------------------------");
                let question_num = rand::thread_rng().gen_range(0, questions_db.len());
                let correct_answer_num: usize;
                let mut num_mc = 0;
                let num_mc_questions = 5;
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
mod database_tests {
    use super::*;

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
