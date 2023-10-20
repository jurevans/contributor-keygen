use bip39::{Language, Mnemonic as M, MnemonicType, Seed};

pub struct Mnemonic {
    mnemonic: M,
}

impl Mnemonic {
    pub fn new(size: MnemonicType) -> Mnemonic {
        let mnemonic = M::new(size, Language::English);
        Mnemonic { mnemonic }
    }

    pub fn to_seed(&self) -> Vec<u8> {
        let seed = Seed::new(&self.mnemonic, "");
        Vec::from(seed.as_bytes())
    }

    pub fn to_phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    pub fn to_words(&self) -> Vec<&str> {
        self.to_phrase().split(" ").collect()
    }
}

pub fn print_formatted_mnemonic(words: Vec<&str>) -> () {
    let words_grouped: Vec<&[&str]> = vec![
        &words[0..4],
        &words[4..8],
        &words[8..12],
        &words[12..16],
        &words[16..20],
        &words[20..24],
    ];

    let mut i = 1;

    // Format mnemonic output as groups with indices
    for group in words_grouped {
        for word in group {
            print!("{}: {}\t", i, word);
            i = i + 1;
        }
        print!("\n");
    }

    ()
}
