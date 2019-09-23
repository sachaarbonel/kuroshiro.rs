use kuromoji::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::normal();
    let tokens = tokenizer.tokenize(
        "本項で解説する地方病とは、山梨県における日本住血吸虫症の呼称であり、\
         長い間その原因が明らかにならず住民を苦しめた感染症である。",
    );
    for token in tokens {
        println!("text {:?} reading {:?} ", token.text, token.detail.reading);
    }
}
