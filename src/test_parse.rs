use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HelloParser;

pub fn parser() {
    let pairs = HelloParser::parse(Rule::header, "# Hello World\n#### Header 4\n")
        .unwrap()
        .flatten();
    
    let mut text = String::new();
    
    println!("{:?}", pairs);
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::tag => {
                let header_level = pair.as_str().chars().count().to_string();
                println!("<h{h}>{t}</h{h}>", h=header_level, t=text);
            },
            Rule::header => {
                text = String::from(pair.clone().into_inner().last().unwrap().as_str());
            }
            _ => continue,
        }
    };
}
