use regex::Regex;
use preferences::{AppInfo, Preferences, PreferencesMap};
use std::io;

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
    name: "preferences",
    author: "Rust language community",
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

pub fn get_input(message: &str) -> String {
    if !(message == "") {
println!("{}", message);}
    let mut this_input = String::from("");
    io::stdin()
        .read_line(&mut this_input)
        .expect("Failed to read line");
    return this_input.trim().to_string();
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
}
