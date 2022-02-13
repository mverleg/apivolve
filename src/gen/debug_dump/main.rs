use ::apivolve::gen1::GenerateConfig;
use ::apivolve::gen1::GenerateInputFormat;
use ::apivolve::Version;

fn main() {
    let conf = GenerateConfig::new(Version::new(0, 1, 0), GenerateInputFormat::Json);
    println!("{}", serde_json::to_string(&conf).unwrap());

}
