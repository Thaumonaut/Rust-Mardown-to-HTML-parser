use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MarkdownParser;

pub fn parser(input: &str) -> String {
    let pairs = MarkdownParser::parse(Rule::document, input);
    let mut content = String::new();
    content.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>Parsed Markdown</title>\n</head>\n<body>\n");

    match pairs {
        Ok(pairs) => {
            for pair in pairs {
                content.push_str(&parse_element(pair));
            }
        },
        Err(e) => {
            eprintln!("Parse error: {}", e);
            content.push_str(&format!("<pre>Parse error:\n{}</pre>\n", e));
        }
    }

    content.push_str("</body>\n</html>");
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
        },
        Rule::block => {
            let mut result = String::new();
            for inner_pair in pair.into_inner() {
                result.push_str(&parse_element(inner_pair));
            }
            result
        },
        Rule::heading => {
            parse_heading(pair)
        },
        Rule::paragraph => {
            parse_paragraph(pair)
        },
        Rule::code => {
            parse_code_block(pair)
        },
        Rule::blockquote => {
            parse_blockquote(pair)
        },
        Rule::list_point => {
            parse_list(pair)
        },
        Rule::horizontal_rule => {
            "<hr>\n".to_string()
        },
        _ => {
            // eprintln!("Unhandled rule: {:?}", pair.as_rule());
            String::new()
        }
    }
}

fn parse_heading(pair: pest::iterators::Pair<Rule>) -> String {
  let mut tag = String::new();
  let mut content = String::new();
  for inner_pair in pair.into_inner() {

  match inner_pair.as_rule() {
      Rule::heading_start => {
        tag = format!("h{:?}", inner_pair.as_str().len());
      },
      Rule:: line_content => {
        content.push_str(parse_line_content(inner_pair).as_str());
      },
      _ => {}
    }
  }
  simple_html_element_builder(tag, content.as_str())
}

fn parse_paragraph(pair: pest::iterators::Pair<Rule>) ->  String {
  let mut content = String::new();
  for inner_pair in pair.into_inner() {
    if inner_pair.as_rule() == Rule::paragraph_line {
      for nested_pair in inner_pair.into_inner() {
        match nested_pair.as_rule() {
          Rule::line_content => {
            content.push_str(parse_line_content(nested_pair).as_str());
          },
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
      },
      _ => {}
    }
  }
  format!("<pre style=\"background-color: #111; color: #0f0; padding: 10px; border-radius: 5px; border: 1px solid #ccc;\"><code>\n{}\n</code></pre>\n", content)
}

fn parse_blockquote(pair: pest::iterators::Pair<Rule>) -> String {String::from(pair.into_inner().as_str())}
fn parse_list(pair: pest::iterators::Pair<Rule>) ->       String {String::from(pair.into_inner().as_str())}


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
                            content.push_str(&simple_html_element_builder("strong".to_string(), &simple_html_element_builder("em".to_string(), italic_content)));
                        },
                        _ => {
                            content.push_str(&simple_html_element_builder("strong".to_string(), &simple_html_element_builder("em".to_string(), content_pair.as_str())));
                        }
                    }
                }
            },
            Rule::bold => {
                let mut bold_html = String::new();
                let mut inner = inner_pair.into_inner();
                inner.next();
                if let Some(bold_content_pair) = inner.next() {
                    for item in bold_content_pair.into_inner() {
                        match item.as_rule() {
                            Rule::italic => {
                                let italic_content = item.into_inner().nth(1).unwrap().as_str();
                                bold_html.push_str(&simple_html_element_builder("em".to_string(), italic_content));
                            },
                            Rule::bold_text => {
                                bold_html.push_str(item.as_str());
                            },
                            _ => {}
                        }
                    }
                }
                content.push_str(&simple_html_element_builder("strong".to_string(), &bold_html));
            },
            Rule::italic => {
                let inner_content = inner_pair.into_inner().nth(1).unwrap().as_str();
                content.push_str(&simple_html_element_builder("em".to_string(), inner_content));
            },
            Rule::inline_code => {
                let inner_content = inner_pair.into_inner().nth(1).unwrap().as_str();
                content.push_str(&simple_html_element_builder("code".to_string(), inner_content));
            },
            Rule::strikethrough => {
                let inner_content = inner_pair.into_inner().nth(1).unwrap().as_str();
                content.push_str(&simple_html_element_builder("s".to_string(), inner_content));
            },
            Rule::inline_link => {
              let mut tag: (String, String, Option<String>) = (String::new(), String::new(), None);
              for child in inner_pair.into_inner().flatten() {
                match child.as_rule() {
                  Rule::url => {
                    tag.0.push_str(child.as_str());
                  },
                  Rule::title => {
                    tag.1.push_str(child.as_str());
                  },
                  Rule::alt => {
                    tag.2 = Some(child.as_str().to_string());
                  },
                  _ => {}
                }
              }
              content.push_str(format!("<a href=\"{url}\">{title}</a>", url = tag.0, title = tag.1).as_str())
            },
            Rule::inline_image => {
              let mut tag: (String, String, Option<String>) = (String::new(), String::new(), None);
              for child in inner_pair.into_inner().flatten() {
                match child.as_rule() {
                  Rule::url => {
                    tag.0.push_str(child.as_str());
                  },
                  Rule::title => {
                    tag.2 = Some(child.as_str().to_string());
                  },
                  Rule::alt => {
                    tag.1.push_str(child.as_str());
                  },
                  _ => {}
                }
              }
              content.push_str(format!("<img src=\"{url}\" alt=\"{alt}\"/>", url = tag.0, alt = tag.2.unwrap_or_default()).as_str())
            },
            Rule::char => {
                content.push_str(inner_pair.as_str());
            },
            Rule::WHITESPACE => {
                content.push_str(inner_pair.as_str());
            },
            _ => {}
        }
    }
    content
}
