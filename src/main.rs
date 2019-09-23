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

// struct Token<'a> {
//     text: &'a str,
//     reading: String,
// }

// impl<'a> Token<'a> {
//     fn new(text: &'a str, reading: &String) -> Token<'a> {
//         Token {
//             text: text,
//             reading: reading.to_owned(),
//         }
//     }
// }
fn main() {
    let romaji = Kuroshiro::convert(
        "本項で解説する地方病とは、山梨県における日本住血吸虫症の呼称であり、\
         長い間その原因が明らかにならず住民を苦しめた感染症である。",
    );
    println!("{}", romaji);
}
