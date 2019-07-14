#[derive(Debug)]
pub struct Question {
    pub question: String,
    pub answer: String,
    pub category: String,
    pub extra: String,
}

pub static BEGIN_CHARS: &'static str = r#"<td class="s4">"#;

#[derive(Debug)]
pub enum Error {
    Input,
}

pub fn extract_field_value(string_array: &mut [String]) -> Result<(), Error> {
    if string_array.is_empty() {
        return Err(Error::Input);
    }
    let mut end_chars = "</td>";
    let begin_alt_chars = "<td class=\"s4 softmerge\">";
    let begin_alt_2_chars = "<td class=\"s5 softmerge\">";
    let begin_alt_extra_chars =
        "<div class=\"softmerge-inner\" style=\"width: 512px; left: -1px;\">";
    let end_alt_chars = "</div>";
    let mut pos_alt = 10000;
    let mut pos_alt_2 = 10000;

    let mut pos = string_array[0].find(BEGIN_CHARS).unwrap() + BEGIN_CHARS.chars().count(); // Finds first encounter of a substring in our string

    if string_array[0].contains(begin_alt_chars) {
        pos_alt = string_array[0].find(begin_alt_chars).unwrap()
            + begin_alt_chars.chars().count()
            + begin_alt_extra_chars.chars().count(); // Finds first encounter of a alternative substring in our string
    }
    if string_array[0].contains(begin_alt_2_chars) {
        pos_alt_2 = string_array[0].find(begin_alt_2_chars).unwrap()
            + begin_alt_2_chars.chars().count()
            + begin_alt_extra_chars.chars().count(); // Finds first encounter of a alternative substring in our string
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
    Ok(())
}

pub fn extract_from_raw_data(mut string_array: Vec<String>) -> Vec<Question> {
    let mut this_question: String;
    let mut this_answer: String;
    let mut this_category: String;
    let mut this_extra: String;
    let mut questions_db = vec![];

    while string_array[0].contains(BEGIN_CHARS) {
        let mut initial = true;
        while (string_array[1] == "" || string_array[1] == "EOL" || initial == true)
            && string_array[0].contains(BEGIN_CHARS)
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
