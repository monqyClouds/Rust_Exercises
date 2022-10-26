fn main() {}

fn to_pig_string(text: String) {
    let mut pig_string = String::new();

    for word in text.split_whitespace() {
        let mut deref_word = *word;
        let first_letter = word.chars()[0];

        if first_letter == "a"
            || first_letter == "e"
            || first_letter == "i"
            || first_letter == "o"
            || first_letter == "u"
        {
            deref_word.push("-" + removed_char + "hay");
        } else {
            let removed_char = deref_word.remove(0);
            deref_word.push("-" + removed_char + "ay");
        }

        pig_string.push(deref_word + " ");

        // match first_letter {
        //     "a" => {

        //     },
        //     "e" => {
        //         deref_word.push("-" + removed_char + "hay");
        //     },
        //     "i" => {
        //         deref_word.push("-" + removed_char + "hay");
        //     },
        //     "o" => {
        //         deref_word.push("-" + removed_char + "hay");
        //     },
        //     "u" => {
        //         deref_word.push("-" + removed_char + "hay");
        //     },
        //     _ => {

        //     }
        // }
    }

    pig_string.trim()
}
