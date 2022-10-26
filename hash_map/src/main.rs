use std::collections::HashMap;

fn main() {
    let mut vec = vec![9, 9, 3, 7, 1, 5, 3, 1, 11, 4, 22, 5, 22];

    vec.sort();

    // Get median
    let median = get_median(&vec);

    // Get mode
    let mode = get_mode(&vec);

    println!("Median is {}, Mode is {}", median, mode);

    let pig_string = to_pig_string("how are you".to_string());

    println!("pig string of \"how are you\" is: \"{}\"", pig_string);
}

// fn sort_arr(arr: &mut Vec<i32>) -> Vec<i32> {
//     arr.sort()
// }

fn get_median(vec: &Vec<i32>) -> f64 {
    let mid_index = vec.len() / 2;

    if vec.len() % 2 == 1 {
        vec[mid_index] as f64
    } else {
        let mid_index2 = mid_index + 1;

        let median = (&vec[mid_index] + &vec[mid_index2]) as f64 / 2.0;

        median
    }
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut int_map = HashMap::new();

    let mut max_occurence = 0;
    let mut mode = 0;

    for int in vec {
        let count = int_map.entry(int).or_insert(0);
        *count += 1;

        if count >= &mut max_occurence {
            max_occurence = *count;
            mode = *int;
        }
    }

    mode
}

fn to_pig_string(text: String) -> String {
    let mut pig_string = String::new();

    for word in text.split_whitespace() {
        let first_letter = &word[0..1];
        let mut deref_word = String::from(word);

        if first_letter == "a"
            || first_letter == "e"
            || first_letter == "i"
            || first_letter == "o"
            || first_letter == "u"
        {
            deref_word += "-hay";
        } else {
            let removed_char = deref_word.remove(0);
            deref_word = deref_word + "-" + &removed_char.to_string() + "ay";
        }

        let new_word = deref_word + " ";
        pig_string += &new_word;
    }

    pig_string.trim().to_owned()
}

// fn generate_company() {

// }
