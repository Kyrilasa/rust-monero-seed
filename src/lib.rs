mod electrum;
mod language;
//TODO checksum
pub fn checksum_test(words: &[&str], language: language::Language) -> bool {
    let last_word = words.last();
    
    //TODO get unique prefix length
    let unique_prefix_length = 3;

    true
}


//TODO
pub fn create_checksum_idx(words: &[&str], language: language::Language) -> u32 {
    1
}

pub fn words_to_bytes(words: &[&str]) -> usize {
    /// precheck begin
    if words.len() % 4 != 0
    {
        //TODO error handling
        println!("Invalid seed: not a multiple of 4");
    }


    let expected = words.len() * 8 * 3 / 32;
    let mut has_checksum = true;
    
    if words.len() != expected/2 && words.len() != expected &&
      words.len() != expected + 1
    {
        //TODO error handling
        println!("Invalid seed: not a multiple of 4");
    }

    has_checksum = words.len() == (expected + 1);

    /// precheck end



    let language = language::Language::find_language_from_word_seed(words);
    if language.is_none() {
        println!("Can not find matching language!");
        1
    } else {


        let word_idx_array = get_word_idx_array(words, language.unwrap());
        println!("Joined, {}", words.join(""));
        let hex_seed = generate_private_key_from_word_idx_array(&word_idx_array);
        println!("Hello, {:?}!", hex_seed);
        1

    }
}

//TODO
pub fn bytes_to_words(hex_seed: String) -> bool {
    false
}

pub fn get_word_idx_array(words: &[&str], language: language::Language) -> Vec<u32> {
    let mut word_idx_vec:Vec<u32> = Vec::new();
    for word in words {
        let word_idx = language.get_word_idx(word);
        if word_idx.is_some() {
            word_idx_vec.push(u32::from(word_idx.unwrap()))
        } 
    }
    word_idx_vec
}

pub fn generate_private_key_from_word_idx_array(word_idx_array: &Vec<u32>) -> String {
    //TODO checksum
    if true
    {
    //   if (!checksum_test(seed, language))
    //   {
    //     // Checksum fail
    //     MERROR("Invalid seed: invalid checksum");
    //     return false;
    //   }
    //   seed.pop_back();
    }

    let word_list_length = 1626;
    let mut hex_seed = "".to_string();
    for i in 0..word_idx_array.len() / 3 {
        let mut w:[u32;4] = [0; 4];
        w[1] = word_idx_array[i*3];
        w[2] = word_idx_array[i*3 + 1];
        w[3] = word_idx_array[i*3 + 2];
        w[0]= w[1] + word_list_length * (((word_list_length - w[1]) + w[2]) % word_list_length) + word_list_length * word_list_length * (((word_list_length - w[2]) + w[3]) % word_list_length);

        if !(w[0]% word_list_length == w[1]) {
            println!("Error invalid seed!");
        }

        let hex_triplet_sum:&str = &format!("{:x}", w[0].swap_bytes());
        hex_seed.push_str(hex_triplet_sum);

    }

    // TODO 12 word hex construction
    hex_seed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_idx_test() {
        let result = get_word_idx_array(&["because", "decay", "vacation", "gigantic", "nail", "binocular", "mittens", "pipeline", "tweezers", "refer", "teardrop", "ecstatic", "kiwi", "pawnshop", "highway", "enlist", "enhanced", "tinted", "biweekly", "pimple", "orphans", "tipsy", "seasons", "sidekick"], language::Language::English);
        assert_eq!(result, vec![155, 299, 1498, 546, 906, 177, 872, 1072, 1437, 1148, 1364, 381, 758, 1044, 612, 413, 410, 1390, 183, 1069, 1011, 1391, 1232, 1257]);
        let result = get_word_idx_array(&["buffet", "abnormal", "baptism", "opened", "putty", "tribal", "inbound", "video", "ajar", "until", "arrow", "axis", "evolved", "bemused", "rising", "gorilla", "gown", "ablaze", "snake", "purged", "southern", "demonstrate", "perfect", "poaching", "baptism"], language::Language::English);
        assert_eq!(result, vec![207, 4, 150, 1000, 1118, 1417, 665, 1523, 49, 1467, 103, 136, 437, 162, 1177, 567, 572, 3, 1284, 1116, 1302, 308, 1056, 1085, 150]);
    }
    
    #[test]
    fn generate_spend_secret_key_test() {
        let result = generate_private_key_from_word_idx_array(&vec![155, 299, 1498, 546, 906, 177, 872, 1072, 1437, 1148, 1364, 381, 758, 1044, 612, 413, 410, 1390, 183, 1069, 1011, 1391, 1232, 1257]);
        assert_eq!(result, "570cf6bc5606648d8cee8939588159656ade2fbc3c6979ab3c82ef731fa1404");
        let result = generate_private_key_from_word_idx_array(&vec![207, 4, 150, 1000, 1118, 1417, 665, 1523, 49, 1467, 103, 136, 437, 162, 1177, 567, 572, 3, 1284, 1116, 1302, 308, 1056, 1085, 150]);
        assert_eq!(result, "9d4a2517f04d212f9d550918fbd13905e70115a01dfe91a6c0de731dc07da404");
    }
    
}
