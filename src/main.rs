use rand::Rng;
use regex::Regex;
use std::convert::TryFrom;
use std::io;
use std::time::{Instant};
use ureq;

#[allow(dead_code)]
mod lib_impp;

fn print_welcome_msg() {
    println!(
        r#"-----------------------
- Welcome to the IMPP -
----------------------- 
> To play a game of normal single-choice questions type 'mc' or 'mc -n'
|- For a game of automatic single-choice questions add '-a' (eg. 'mc -a')
|- For a reversed (Jeopardy-Style) game add '-a' (eg. 'mc -a -j')
|- To only get questions about a specific topic add '-t "topic"
> To edit your database url type 'ed'
> To go back to this menu type 'exit', to quit the program type 'quit'"#,
    );
}

fn fetch_data(url: &str) -> String {
    let my_string = ureq::get(url)
        .call()
        .into_string()
        .expect("Could not open URL");
    my_string
}

fn get_new_spreadsheet_url() {
    let mut spreadsheet_url: String;
    spreadsheet_url = get_input("Please specify the URL of your spreadsheet:");
    // TODO: Check Result
    let now = Instant::now();
    println!("Importing database");
    let import_count = lib_impp::import_googlesheet(fetch_data(&spreadsheet_url), "");
    println!("\nImported {} objects. Import took {} seconds.", import_count, now.elapsed().as_secs());
}

fn get_input(message: &str) -> String {
    if message != "" {
        println!("{}", message);
    }
    let mut this_input = String::from("");
    io::stdin()
        .read_line(&mut this_input)
        .expect("Failed to read line");
    this_input.trim().to_string()
}

fn extract_topic(input_string: &str) -> &str {
    if Regex::new("-t \"(.*?)\"").unwrap().is_match(input_string) {
        let re = Regex::new("-t \"(.*?)\"").unwrap();
        let this_return = re.captures(input_string).unwrap();
        this_return.get(1).unwrap().as_str()
    } else {
        ""
    }
}

fn main() {
    // Check if we have a database file
    if !lib_impp::get_database_status("") == true {
        println!("You seem to be here for the first time.");
    }

    // START LOOP
    loop {
        print_welcome_msg();
        let mut input_curr = String::new();
        let input_root = get_input("");
        if input_root.contains("quit") {
            break;
        }
        else if input_root.contains("ed") {
            get_new_spreadsheet_url();
        } else if input_root.contains("mc") {
            loop {
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
                let question_num = lib_impp::generate_random_question(
                    String::from(extract_topic(&input_root)),
                    "",
                );
                let question_vector: Vec<lib_impp::Question> = lib_impp::get_question_vector(
                    &lib_impp::import_json_question_db(""),
                    input_root.contains("-j"),
                    usize::try_from(question_num)
                        .expect("Our question number could not be converted to usize."),
                );
                let mc_distractors =
                    lib_impp::get_mc_distractors(question_num, 4, input_root.contains("-j"), "");
                let our_distractors_length = mc_distractors.len();

                println!("------------------------------------------------------------------------------------------------------");

                // Print question
                if input_root.contains("-a") {
                    println!("Frage: \'{}\' \n", &question_vector[0].question);
                } else {
                    println!("Frage: \'{}\' (type \'m\' for multiple choice mode or any key to reveal the answer)", &question_vector[0].question);
                    input_curr = get_input("");
                }
                // print mc questions (only if more than one mc answer is avaliable)
                if input_root.contains("-a")
                    || input_curr.contains('m') && our_distractors_length != 0
                {
                    let our_rand_number =
                        rand::thread_rng().gen_range(0, our_distractors_length + 1);
                    let mut i = 0;
                    let mut j = 0;
                    while i < our_distractors_length + 1 {
                        let mut this_mc_answer = &question_vector[0].answer;
                        if i != our_rand_number {
                            this_mc_answer = &mc_distractors[j].answer;
                            j += 1;
                        }
                        println!("{}) {}", characters[i], this_mc_answer);
                        i += 1;
                    }

                    // Check answer input
                    input_curr = get_input("").to_string().to_uppercase();
                    if characters.contains(&input_curr) {
                        let index = characters.iter().position(|r| *r == input_curr).unwrap();
                        if index < our_distractors_length + 1 {
                            if our_rand_number == index {
                                print!("{} is correct! ", &input_curr);
                            } else {
                                print!("{} is wrong! ", &input_curr);
                            }
                        }
                    }
                }
                // Return correct answer
                println!("The correct answer is: {}", &question_vector[0].answer);
                if &question_vector[0].extra != "" {
                    println!("Extra info: {}", &question_vector[0].extra);
                }

                if input_curr.to_uppercase().contains("EXIT") {
                    break;
                }
            }
        }
    }
}
