mod test_parse;

use std::fs;

fn main() {
    let md = fs::read_to_string("src/test.md").unwrap();
    let result = test_parse::parser(md.as_str());
    fs::write("src/test.html", result).unwrap();
}
