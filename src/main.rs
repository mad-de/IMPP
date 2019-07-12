extern crate ureq;

use std::io;
use rand::Rng;
// #[derive(Debug)]

struct Question {
    question: String,
    answer: String,
    category: String,
    extra: String,
}

fn extract_field_value (string_array: &mut [String]) {
    let begin_chars = "<td class=\"s4\">";
    let mut end_chars = "</td>";
    let begin_alt_chars = "<td class=\"s4 softmerge\">";
    let begin_alt_2_chars = "<td class=\"s5 softmerge\">";
    let begin_alt_extra_chars = "<div class=\"softmerge-inner\" style=\"width: 512px; left: -1px;\">";
    let end_alt_chars = "</div>";
    let mut pos_alt = 10000;
    let mut pos_alt_2 = 10000;    
    let mut pos = string_array[0].find(begin_chars).unwrap() + begin_chars.chars().count(); // Finds first encounter of a substring in our string
    if string_array[0].contains(begin_alt_chars)
    {
    	pos_alt = string_array[0].find(begin_alt_chars).unwrap() + begin_alt_chars.chars().count() + begin_alt_extra_chars.chars().count(); // Finds first encounter of a alternative substring in our string
    }
    if string_array[0].contains(begin_alt_2_chars)
    {
    	pos_alt_2 = string_array[0].find(begin_alt_2_chars).unwrap() + begin_alt_2_chars.chars().count() + begin_alt_extra_chars.chars().count(); // Finds first encounter of a alternative substring in our string
    }
    if pos_alt_2 < pos_alt {
    	     pos_alt = pos_alt_2;
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
}

fn main() {
    // CREATE DATABASE
    // "https://docs.google.com/spreadsheets/d/e/2PACX-1vQkvkSX1lrkGyvetFg90smmaGD0rVTz4QteaWNzPkqgUDNDUPjozmp0wlWudKdw-C1F9vTB6N37oYDx/pubhtml"; // Medikamente - s3
    let url = "https://docs.google.com/spreadsheets/d/e/2PACX-1vTo-d-1ObJn_cXyN2uINb1x8nW58qj5oY5hzLqYL4YJTwIjwY-sBrcM2tzGv564b5VzoPHOJSiaUcSW/pubhtml"; // Vokabeln - s4 + s5
    let body = ureq::get(url).call().into_string().unwrap();

    let string_start = "<td class=\"s4\">";
    let mut string_array: [String; 2] = [body, String::from("")]; // Initialize array
    let mut this_question : String;	
    let mut this_answer : String;
    let mut this_category : String;
    let mut this_extra : String;

    let mut questions_db: Vec<Question> = Vec::new();
    while string_array[0].contains(string_start) {
	let mut initial = true;
	while ( string_array[1] == "" || string_array[1] == "EOL" || initial == true ) && string_array[0].contains(string_start) { // search for the first normally formatted field
	     extract_field_value(&mut string_array);
	     initial = false;
	}
	if string_array[1] == "EOL" {
	// if we get the last EOL break
	}
	else
	{
	    this_question = string_array[1].to_string();
	    extract_field_value(&mut string_array);
	    if string_array[1] != "EOL" {
		this_answer = string_array[1].to_string();
	    }
	    else {
	    	this_answer = String::from("");
	    }
	    extract_field_value(&mut string_array);
	    if string_array[1] != "EOL" {
	    	this_category = string_array[1].to_string();
	    }
	    else {
	    	this_category = String::from("");
	    }
	    extract_field_value(&mut string_array);

	    if string_array[1] != "EOL" {
	    	this_extra = string_array[1].to_string();
	    }
	    else {
	    	this_extra = String::from("");
	    }

	    let question1 = Question {
	    	question: this_question,
	    	answer: this_answer.clone(),
	    	category: this_category.clone(),
	    	extra: this_extra.clone(),
	    };
            if question1.question.is_empty() && question1.answer.is_empty() {
            }
	    else {
	    	questions_db.push(question1);
	    }
	}
    }

    // START LOOP
    loop {        
    	println!("-----------------------\n- Welcome to the IMPP -\n-----------------------\nTo see all questions in the database type db.\nTo play a game of normal mc questions type \'mc\' or \'mc -n\' \nWe have {} items in our database.", questions_db.len());
    	let mut input = String::new();
    	io::stdin().read_line(&mut input).expect("Failed to read line");
    	/* let input = match input.trim() {
    		Ok(str) => str,
    		Err(_) => continue,
	}; */

    	input = input.trim().to_string();
    	if input == "db" {
    	    for item in &questions_db {
	    	println!("Frage: \'{}\', Antwort: \'{}\', Kategorie: \'{}\', Extra: \'{}\'", item.question, item.answer, item.category, item.extra);
            }
        }
    	else if input == "mc" || input == "mc -n" {
	    loop {
    	    	println!("------------------------------------------------------------------------------------------------------");
    	    	let question_num = rand::thread_rng().gen_range(0, questions_db.len());
    	    	let correct_answer_num : usize;
    	    	let mut num_mc = 0;
    	    	let num_mc_questions = 5;
    	    	let mut temp_question_num : usize;
    	    	let characters: [String; 10] = [String::from("A"), String::from("B"), String::from("C"), String::from("D"), String::from("E"), String::from("F"),
 			String::from("G"), String::from("H"), String::from("I"), String::from("J")];
    	    	println!("Frage: \'{}\' (type \'m\' for multiple choice mode or any key to reveal the answer)", &questions_db[question_num].question);
    	    	input = String::from("");
            	io::stdin().read_line(&mut input).expect("Failed to read line");
            	input = input.trim().to_string();


    	    	if input == "m" || input == "mc -n" {
    		    correct_answer_num = rand::thread_rng().gen_range(0, num_mc_questions);
		    // Generate answers
    		    while num_mc < num_mc_questions {
    		    	if num_mc == correct_answer_num {
    			    println!("{}) {}", characters[num_mc], &questions_db[question_num].answer);
    		    	}
    		    	else {
			    temp_question_num = question_num; // Set temporary question to current question so while has to fail initially
			    while &questions_db[temp_question_num].question == &questions_db[question_num].question || &questions_db[temp_question_num].category != &questions_db[question_num].category {
				temp_question_num = rand::thread_rng().gen_range(0, questions_db.len());
			    }
    			    println!("{}) {}", characters[num_mc], &questions_db[temp_question_num].answer);
			}
    		    	num_mc += 1;
	    	    }
    	    	    input = String::from("");
            	    io::stdin().read_line(&mut input).expect("Failed to read line");
            	    input = input.trim().to_string();
		    if input == characters[correct_answer_num] {
    		    	println!("Correct!");
		    }
		    else {
    		    	println!("Wrong!");
		    }
    		    println!("The correct answer is: {}) {}", characters[correct_answer_num], &questions_db[question_num].answer);
		    if &questions_db[question_num].extra != "" {
		    println!("Extra info: {}", &questions_db[question_num].extra);
		    }
	        }
	    	else {
		     println!("Antwort: \'{}\', Kategorie: \'{}\', Extra: \'{}\'\n", &questions_db[question_num].answer, &questions_db[question_num].category, &questions_db[question_num].extra);
	       	}
	     }
    	}
     }
}
