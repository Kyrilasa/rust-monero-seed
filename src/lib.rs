mod electrum;
mod language;

pub fn words_to_bytes(words: &[&str]) -> usize {
    use bip39::{Mnemonic, MnemonicType, Language};
    println!("Hello, {:?}!", words);
    println!("Joined, {}", words.join(""));

    /// create a new randomly generated mnemonic phrase
    let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    /// get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);
    //for c in words.join("").bytes(){
    //    println!("Byte: {}", c);
    //}
    1
}


// for (unsigned int i=0; i < seed.size() / 3; i++)
//       {
//         uint32_t w[4];
//         w[1] = matched_indices[i*3];
//         w[2] = matched_indices[i*3 + 1];
//         w[3] = matched_indices[i*3 + 2];

//         w[0]= w[1] + word_list_length * (((word_list_length - w[1]) + w[2]) % word_list_length) +
//           word_list_length * word_list_length * (((word_list_length - w[2]) + w[3]) % word_list_length);

//         if (!(w[0]% word_list_length == w[1]))
//         {
//           memwipe(w, sizeof(w));
//           MERROR("Invalid seed: mumble mumble");
//           return false;
//         }

//         w[0] = SWAP32LE(w[0]);
//         dst.append((const char*)&w[0], 4);  // copy 4 bytes to position
//         memwipe(w, sizeof(w));
//       }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = electrum::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn words_to() {
        let result = words_to_bytes(&["asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd","asd", "asdasd"]);
        assert_eq!(result, 1);
    }
}
