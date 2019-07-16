use self::lib::*;
use preferences::{AppInfo, Preferences, PreferencesMap};
use rand::Rng;
use std::io;

const APP_INFO: AppInfo = AppInfo {
    name: "preferences",
    author: "Rust language community",
};

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
> To go back to this menu type 'exit', to quit the program type 'quit'
We have {} items in our database."#,
        number_of_questions
    );
}

fn fetch_data(url: &str) -> Result<Vec<String>, ()> {
    let body = ureq::get(url).call().into_string().unwrap();
    let string_array: [String; 2] = [body, String::from("")]; // Initialize array
    Ok(string_array.to_vec())
}

fn main() {
    let mut input_url = String::new();
    let prefs_key = "preferences/apps/impp";
    let load_preferences = PreferencesMap::<String>::load(&APP_INFO, prefs_key);
    if load_preferences.is_ok() {
        let preferences_index = "primary_db";
        for (index, string) in load_preferences.unwrap() {
            if index == "primary_db" {
                input_url = string;
            };
        }
    } else {
        loop {
            input_url = String::from("");
            println!(
            "You seem to be here for the first time. Please specify the URL of your spreadsheet:"
        );
            io::stdin()
                .read_line(&mut input_url)
                .expect("Failed to read line");

            input_url = input_url.trim().to_string();

            println!("Loading your database...");

            let init_raw_data = fetch_data(&input_url).unwrap();
            let init_questions_db = extract_from_raw_data(init_raw_data);

            if init_questions_db.len() == 0 {
                println!("Your database seems to be empty. Are you sure you want to continue? y/n");
                input_url = String::new();
                io::stdin()
                    .read_line(&mut input_url)
                    .expect("Failed to read line");

                input_url = input_url.trim().to_string();
                if input_url == "y" {
                    break;
                }
            } else {
                break;
            }
        }

        // Edit the preferences (std::collections::HashMap)
        let mut insert_preferences: PreferencesMap<String> = PreferencesMap::new();
        let input_url_2 = &input_url;
        insert_preferences.insert("primary_db".into(), input_url_2.into());
        let save_result = insert_preferences.save(&APP_INFO, prefs_key);
        assert!(save_result.is_ok());
    }

    let raw_data = fetch_data(&input_url).unwrap();
    let questions_db = extract_from_raw_data(raw_data);

    // START LOOP
    loop {
        print_welcome_msg(questions_db.len());
        let mut input_root = String::new();
        let mut input_curr = String::new();
        io::stdin()
            .read_line(&mut input_root)
            .expect("Failed to read line");

        input_root = input_root.trim().to_string();
        if input_root.contains("quit") {
            break;
        } else if input_root.contains("db") {
            for item in &questions_db {
                println!(
                    "Frage: \'{}\', Antwort: \'{}\', Kategorie: \'{}\', Extra: \'{}\'",
                    item.question, item.answer, item.category, item.extra
                );
            }
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
                    input_curr = String::from("");
                    io::stdin()
                        .read_line(&mut input_curr)
                        .expect("Failed to read line");
                    input_curr = input_curr.trim().to_string();
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
                    input_curr = String::from("");
                    io::stdin()
                        .read_line(&mut input_curr)
                        .expect("Failed to read line");
                    input_curr = input_curr.trim().to_string().to_uppercase();
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
