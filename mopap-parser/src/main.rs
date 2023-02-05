use std::io::Read;

use mopap_parser::parser::MopapParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    let path: String;
    path = args.next().expect("no path given");
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let ast = MopapParser::new().parse_src(&contents)?;
    println!("{:#?}", ast);
    Ok(())
}
