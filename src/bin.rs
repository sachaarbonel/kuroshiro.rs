use kuroshiro::Kuroshiro;

fn main() {
    let ruby = Kuroshiro::output_ruby(
        "本項で解説する地方病とは、山梨県における日本住血吸虫症の呼称であり、\
         長い間その原因が明らかにならず住民を苦しめた感染症である。",
    );

    println!("{}", ruby);
}
