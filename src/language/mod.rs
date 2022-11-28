mod mnemonic_english;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    #[inline]
	pub fn word_list(self) -> &'static [&'static str; 1626] {
		match self {
			Language::English => &mnemonic_english::WORDS,
		}
	}

	pub fn words_by_prefix(self, prefix: &str) -> &[&'static str] {
		let first = match self.word_list().iter().position(|w| w.starts_with(prefix)) {
			Some(i) => i,
			None => return &[],
		};
		let count = self.word_list()[first..].iter().take_while(|w| w.starts_with(prefix)).count();
		&self.word_list()[first..first + count]
	}

    #[inline]
	pub fn find_word(self, word: &str) -> Option<u16> {
		self.word_list().iter().position(|w| *w == word).map(|i| i as u16)
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
	fn words_by_prefix() {
		let lang = Language::English;

		let res = lang.find_word("abbey");

		println!("Hello, {:?}!", res.unwrap());

		let res = lang.words_by_prefix("");
		assert_eq!(res.len(), 1626);

		let res = lang.words_by_prefix("woof");
		assert!(res.is_empty());
	}
}