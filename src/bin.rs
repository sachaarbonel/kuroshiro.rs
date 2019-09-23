//use kuroshiro::Kuroshiro;
fn to_katakana(hiragana: &str) -> String {
    let hiragana_ch = hiragana.chars().next().unwrap() as u32;
    std::char::from_u32(hiragana_ch + 96).unwrap().to_string()
}
fn to_hiragana(katakana: &str) -> String {
    let katakana_ch = katakana.chars().next().unwrap() as u32;
    std::char::from_u32(katakana_ch - 96).unwrap().to_string()
}
fn main() {
    // let ruby = Kuroshiro::output_ruby(
    //     "本項で解説する地方病とは、山梨県における日本住血吸虫症の呼称であり、\
    //      長い間その原因が明らかにならず住民を苦しめた感染症である。",
    // );
    println!("{}", to_hiragana("ア"));
    println!("{}", to_katakana("あ"));
}
