use kuromoji::Tokenizer;

pub struct Kuroshiro;

impl Kuroshiro {
    pub fn output_ruby(text: &str) -> String {
        let tokens = parse(text);
        let ruby_tokens = tokens
            .as_slice()
            .iter()
            .map(|token| token.clone().to_ruby())
            .collect::<Vec<String>>();
        let ruby_output = ruby_tokens.join("");
        ruby_output
    }

    pub fn convert(text: &str) -> String {
        let tokens = parse(text);
        let ruby_tokens = tokens
            .as_slice()
            .iter()
            .map(|token| token.clone().reading)
            .collect::<Vec<String>>();
        let ruby_output = ruby_tokens.join("");
        ruby_output
    }
}

#[derive(Clone)]
struct Token {
    text: String,
    reading: String,
}

impl Token {
    fn new(text: &str, reading: &String) -> Token {
        Token {
            text: String::from(text),
            reading: reading.to_owned(),
        }
    }

    fn alphabet(&self) -> Alphabet {
        define_alphabet(self.text.as_str())
    }

    fn to_ruby(&self) -> String {
        match self.alphabet() {
            Alphabet::Kanji => format!("<ruby>{}<rt>{}</rt></ruby>", self.text, self.to_hiragana()),
            _ => format!("{}", self.text),
        }
    }

    fn to_hiragana(&self) -> String {
        if let Alphabet::Katakana = define_alphabet(&self.reading.clone().as_str()) {
            to_hiraganas(self.reading.as_str())
        } else if self.reading.as_str() == "UNK" {
            //TODO: very ugly
            self.reading.clone()
        } else {
            self.text.clone()
        }
    }
}

#[derive(Debug)]
enum Alphabet {
    Kanji,
    Hiragana,
    Katakana,
    Other,
    Punctuation,
    KanjiRadicals,
}

fn to_char(text: &str) -> char {
    text.chars().next().unwrap()
}

fn parse(text: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::normal();
    let tokens = tokenizer.tokenize(text);
    let parsed_tokens = tokens
        .as_slice()
        .iter()
        .map(|token| Token::new(token.clone().text, &token.clone().detail.reading))
        .collect::<Vec<Token>>();
    parsed_tokens
}

fn to_katakana(hiragana: &str) -> String {
    let hiragana_ch = hiragana.chars().next().unwrap() as u32;
    std::char::from_u32(hiragana_ch + 96).unwrap().to_string()
}

fn to_hiragana(katakana: &char) -> String {
    let katakana_ch = *katakana as u32;
    std::char::from_u32(katakana_ch - 96).unwrap().to_string()
}

fn to_hiraganas(katakanas: &str) -> String {
    String::from(katakanas)
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|ch| to_hiragana(ch))
        .collect::<Vec<String>>()
        .join("")
}

fn define_alphabet(text: &str) -> Alphabet {
    match to_char(text) {
        '\u{3000}'..='\u{303F}' => Alphabet::Punctuation,
        '\u{4E00}'..='\u{9FCB}' => Alphabet::Kanji,
        '\u{F900}'..='\u{FAFA}' => Alphabet::Kanji,
        '\u{3400}'..='\u{4DB5}' => Alphabet::Kanji,
        '\u{2E80}'..='\u{2FD5}' => Alphabet::KanjiRadicals,
        '\u{3041}'..='\u{3096}' => Alphabet::Hiragana,
        '\u{30A0}'..='\u{30FF}' => Alphabet::Katakana,
        _ => Alphabet::Other,
    }
}
