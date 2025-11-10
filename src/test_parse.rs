use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MarkdownParser;

pub fn parser(input: &str) -> String {
    let pairs = MarkdownParser::parse(Rule::document, input);
    let mut content = String::new();
    content.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>Parsed Markdown</title>\n</head>\n<body style=\"margin: auto; padding: 0; max-width: 800px;\">\n");

    match pairs {
        Ok(pairs) => {
            for pair in pairs {
                content.push_str(&parse_element(pair));
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            content.push_str(&format!("<pre>Parse error:\n{}</pre>\n", e));
        }
    }

    content.push_str("</body>\n<script src=\"https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js\"></script>\n</html>");
    content
}

fn parse_element(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::document => {
            let mut result = String::new();
            for inner_pair in pair.into_inner() {
                result.push_str(&parse_element(inner_pair));
            }
            result
        }
        Rule::block => {
            let mut result = String::new();
            for inner_pair in pair.into_inner() {
                result.push_str(&parse_element(inner_pair));
            }
            result
        }
        Rule::heading => parse_heading(pair),
        Rule::paragraph => parse_paragraph(pair),
        Rule::code => parse_code_block(pair),
        Rule::blockquote => parse_blockquote(pair),
        Rule::list => parse_list(pair),
        Rule::horizontal_rule => "<hr>\n".to_string(),
        _ => String::new(),
    }
}

fn parse_heading(pair: pest::iterators::Pair<Rule>) -> String {
    let mut tag = String::new();
    let mut content = String::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::heading_start => {
                tag = format!("h{:?}", inner_pair.as_str().len());
            }
            Rule::line_content => {
                content.push_str(parse_line_content(inner_pair).as_str());
            }
            _ => {}
        }
    }
    simple_html_element_builder(tag, content.as_str())
}

fn parse_paragraph(pair: pest::iterators::Pair<Rule>) -> String {
    let mut content = String::new();
    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::paragraph_line {
            for nested_pair in inner_pair.into_inner() {
                match nested_pair.as_rule() {
                    Rule::line_content => {
                        content.push_str(parse_line_content(nested_pair).as_str());
                    }
                    _ => {}
                }
            }
        }
        content.push_str("\n");
    }
    content
}

fn parse_code_block(pair: pest::iterators::Pair<Rule>) -> String {
    let mut content = String::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::code_content => {
                content.push_str(inner_pair.as_str());
            }
            _ => {}
        }
    }
    format!(
        "<pre style=\"background-color: #fff; padding: 1rem; overflow-x: auto;\" class=\"prettyprint\" ><code>\n{}\n</code></pre>\n",
        content
    )
}

// TODO: Parse markdown blockquotes
fn parse_blockquote(pair: pest::iterators::Pair<Rule>) -> String {
    String::from(pair.into_inner().as_str())
}

#[derive(Debug, PartialEq, Clone)]
enum ListType {
    Unordered,
    Ordered,
    None,
}

impl ListType {
    fn opening_tag(&self) -> String {
        match self {
            ListType::Unordered => "<ul>".to_string(),
            ListType::Ordered => "<ol>".to_string(),
            ListType::None => format!(""),
        }
    }

    fn closing_tag(&self) -> String {
        match self {
            ListType::Unordered => "</ul>".to_string(),
            ListType::Ordered => "</ol>".to_string(),
            ListType::None => format!(""),
        }
    }
}

fn parse_list(pair: pest::iterators::Pair<Rule>) -> String {
    let mut content = String::new();
    let mut depth_stack: Vec<(usize, ListType)> = Vec::new();

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() != Rule::list_point {
            continue;
        }

        let mut current_depth = 0;
        let mut list_type = ListType::None;
        let mut item_content = String::new();

        for child in inner_pair.into_inner() {
            match child.as_rule() {
                Rule::indent => {
                    current_depth = child.as_str().len();
                }
                Rule::list_start => {
                    if let Some(marker) = child.into_inner().next() {
                        list_type = match marker.as_rule() {
                            Rule::ordered => ListType::Ordered,
                            Rule::unordered => ListType::Unordered,
                            _ => ListType::None,
                        };
                    }
                }
                Rule::line_content => {
                    item_content = parse_line_content(child);
                }
                _ => {}
            }
        }

        while depth_stack.len() > 0 && depth_stack.last().unwrap().0 > current_depth {
            let (prev_depth, prev_type) = depth_stack.pop().unwrap();
            content.push_str(&format!(
                "{}{}\n",
                " ".repeat(prev_depth),
                prev_type.closing_tag()
            ));
        }

        if depth_stack.is_empty()
            || depth_stack.last().unwrap().1 != list_type
            || depth_stack.last().unwrap().0 < current_depth
        {
            if depth_stack.is_empty() || depth_stack.last().unwrap().0 < current_depth {
                content.push_str(&format!(
                    "{}{}\n",
                    " ".repeat(current_depth),
                    list_type.opening_tag()
                ));
                depth_stack.push((current_depth, list_type.clone()));
            }
        }

        content.push_str(&format!(
            "{}<li>{}</li>\n",
            " ".repeat(current_depth + 2),
            item_content
        ));
    }

    while let Some((depth, list_type)) = depth_stack.pop() {
        content.push_str(&format!(
            "{}{}\n",
            " ".repeat(depth),
            list_type.closing_tag()
        ));
    }

    content
}

fn simple_html_element_builder(tag: String, content: &str) -> String {
    format!("<{t}>{c}</{t}>\n", t = tag, c = content)
}

fn parse_line_content(pair: pest::iterators::Pair<Rule>) -> String {
    let mut content = String::new();
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::bold_italic => {
                let mut inner = inner_pair.into_inner();
                inner.next();
                if let Some(content_pair) = inner.next() {
                    match content_pair.as_rule() {
                        Rule::italic => {
                            let italic_content = content_pair.into_inner().nth(1).unwrap().as_str();
                            content.push_str(&simple_html_element_builder(
                                "strong".to_string(),
                                &simple_html_element_builder("em".to_string(), italic_content),
                            ));
                        }
                        _ => {
                            content.push_str(&simple_html_element_builder(
                                "strong".to_string(),
                                &simple_html_element_builder(
                                    "em".to_string(),
                                    content_pair.as_str(),
                                ),
                            ));
                        }
                    }
                }
            }
            Rule::bold => {
                let mut bold_html = String::new();
                let mut inner = inner_pair.into_inner();
                inner.next();
                if let Some(bold_content_pair) = inner.next() {
                    for item in bold_content_pair.into_inner() {
                        match item.as_rule() {
                            Rule::italic => {
                                let italic_content = item.into_inner().nth(1).unwrap().as_str();
                                bold_html.push_str(&simple_html_element_builder(
                                    "em".to_string(),
                                    italic_content,
                                ));
                            }
                            Rule::bold_text => {
                                bold_html.push_str(item.as_str());
                            }
                            _ => {}
                        }
                    }
                }
                content.push_str(&simple_html_element_builder(
                    "strong".to_string(),
                    &bold_html,
                ));
            }
            Rule::italic => {
                let inner_content = inner_pair.into_inner().nth(1).unwrap().as_str();
                content.push_str(&simple_html_element_builder(
                    "em".to_string(),
                    inner_content,
                ));
            }
            Rule::inline_code => {
                let inner_content = inner_pair.into_inner().nth(1).unwrap().as_str();
                content.push_str(&simple_html_element_builder(
                    "code".to_string(),
                    inner_content,
                ));
            }
            Rule::strikethrough => {
                let inner_content = inner_pair.into_inner().nth(1).unwrap().as_str();
                content.push_str(&simple_html_element_builder("s".to_string(), inner_content));
            }
            Rule::inline_link => {
                let mut tag: (String, String, Option<String>) =
                    (String::new(), String::new(), None);
                for child in inner_pair.into_inner().flatten() {
                    match child.as_rule() {
                        Rule::url => {
                            tag.0.push_str(child.as_str());
                        }
                        Rule::title => {
                            tag.1.push_str(child.as_str());
                        }
                        Rule::alt => {
                            tag.2 = Some(child.as_str().to_string());
                        }
                        _ => {}
                    }
                }
                content.push_str(
                    format!("<a href=\"{url}\">{title}</a>", url = tag.0, title = tag.1).as_str(),
                )
            }
            Rule::inline_image => {
                let mut tag: (String, String, Option<String>) =
                    (String::new(), String::new(), None);
                for child in inner_pair.into_inner().flatten() {
                    match child.as_rule() {
                        Rule::url => {
                            tag.0.push_str(child.as_str());
                        }
                        Rule::title => {
                            tag.2 = Some(child.as_str().to_string());
                        }
                        Rule::alt => {
                            tag.1.push_str(child.as_str());
                        }
                        _ => {}
                    }
                }
                content.push_str(
                    format!(
                        "<img src=\"{url}\" alt=\"{alt}\"/>",
                        url = tag.0,
                        alt = tag.2.unwrap_or_default()
                    )
                    .as_str(),
                )
            }
            Rule::char => {
                content.push_str(inner_pair.as_str());
            }
            Rule::WHITESPACE => {
                content.push_str(inner_pair.as_str());
            }
            _ => {}
        }
    }
    content
}
