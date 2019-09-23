use kuromoji::Tokenizer;
struct Kuroshiro;
impl Kuroshiro {
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
}

struct Token<'a> {
    text: &'a str,
    reading: String,
}

impl<'a> Token<'a> {
    fn new(text: &'a str, reading: &String) -> Token<'a> {
        Token {
            text: text,
            reading: reading.to_owned(),
        }
    }
}

fn is_kanji(text: &str) -> bool {
    match toChar(text) {
        '\u{4E00}'...'\u{9FCF}' => true,
        '\u{F900}'...'\u{FAFF}' => true,
        '\u{3400}'...'\u{4DBF}' => true,
        _ => false,
    }
}

fn isHiragana(text: &str) -> bool {
    match toChar(text) {
        '\u{3040}'...'\u{309F}' => true,
        _ => false,
    }
}
fn toChar(text: &str) -> char {
    text.chars().next().unwrap()
}
fn isKatakana(text: &str) -> bool {
    match toChar(text) {
        '\u{30A0}'...'\u{30FF}' => true,
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

    println!("{}", isKatakana(&"デ"));
}
