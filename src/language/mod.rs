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
	pub fn get_word_idx(self, word: &str) -> Option<u16> {
		self.word_list().iter().position(|w| *w == word).map(|i| i as u16)
	}

	pub fn find_language_from_word_seed(words: &[&str]) -> Option<Language> {
		let _languages: Vec<Language> = vec![Language::English];

		for language in _languages {
			//n^2
			let _full_match = words.iter().all(|item| language.word_list().contains(item));
			if _full_match {
				return Some(language);
			}
		}
		None
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
	fn words_by_prefix() {
		let lang = Language::English;

		let res = lang.get_word_idx("abbey");

		println!("Hello, {:?}!", res.unwrap());

		let res = lang.words_by_prefix("");
		assert_eq!(res.len(), 1626);

		let res = lang.words_by_prefix("woof");
		assert!(res.is_empty());
	}
    #[test]
	fn find_language() {
		let lang = Language::find_language_from_word_seed(&["because", "decay", "vacation", "gigantic", "nail", "binocular", "mittens", "pipeline", "tweezers", "refer", "teardrop", "ecstatic", "kiwi", "pawnshop", "highway", "enlist", "enhanced", "tinted", "biweekly", "pimple", "orphans", "tipsy", "seasons", "sidekick"]);
		assert_eq!(lang, Some(Language::English));
		let lang = Language::find_language_from_word_seed(&["beasdcause", "decay", "vacation", "gigantic", "nail", "binocular", "mittens", "pipeline", "tweezers", "refer", "teardrop", "ecstatic", "kiwi", "pawnshop", "highway", "enlist", "enhanced", "tinted", "biweekly", "pimple", "orphans", "tipsy", "seasons", "sidekick"]);
		assert_eq!(lang, None);
	}
}