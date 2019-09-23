use kuromoji::Tokenizer;

#[allow(dead_code)]
struct Kuroshiro;

impl Kuroshiro {
    #[allow(dead_code)]
    fn convert(text: &str) -> String {
        let mut tokenizer = Tokenizer::normal();
        let tokens = tokenizer.tokenize(text);
        let parsed_tokens = tokens
            .as_slice()
            .iter()
            .map(|token| token.detail.reading.to_owned()) //Token::new(token.clone().text, &token.clone().detail.reading)
            .collect::<Vec<String>>(); //.collect::<Vec<Token>>();
        let converted_tokens = parsed_tokens.join("");
        converted_tokens
    }

    #[allow(dead_code)]
    fn parse(text: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::normal();
        let tokens = tokenizer.tokenize(text);
        let parsed_tokens = tokens
            .as_slice()
            .iter()
            .map(|token| Token::new(token.clone().text, &token.clone().detail.reading)) //
            .collect::<Vec<Token>>();
        parsed_tokens
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
}

#[allow(dead_code)]
fn is_kanji(text: &str) -> bool {
    match to_char(text) {
        '\u{4E00}'..='\u{9FCF}' => true,
        '\u{F900}'..='\u{FAFF}' => true,
        '\u{3400}'..='\u{4DBF}' => true,
        _ => false,
    }
}
#[allow(dead_code)]
fn is_hiragana(text: &str) -> bool {
    match to_char(text) {
        '\u{3040}'..='\u{309F}' => true,
        _ => false,
    }
}
fn to_char(text: &str) -> char {
    text.chars().next().unwrap()
}
fn is_katakana(text: &str) -> bool {
    match to_char(text) {
        '\u{30A0}'..='\u{30FF}' => true,
        _ => false,
    }
}
fn main() {
    // let romaji = Kuroshiro::convert(
    //     "本項で解説する地方病とは、山梨県における日本住血吸虫症の呼称であり、\
    //      長い間その原因が明らかにならず住民を苦しめた感染症である。",
    // );
    // println!("{}", romaji);
    //  let heart: String = "項".escape_unicode().collect();
    //println!("{}", heart);
    // for unicode in "項".escape_unicode() {
    //     println!("{}", unicode);
    // }

    println!("{}", is_katakana(&"デ"));
}
