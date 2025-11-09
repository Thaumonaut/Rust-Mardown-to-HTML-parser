# Overview

This project was to learn how to parse a language and convert it to another. I am converting markdown files into html files in the project. I wanted to learn this because I had an idea for my own language I wanted to write but I first need to learn how to parse text and how to handle the text after it has been parsed. With this I hope to make my own Markup language that can compile to html and be run on the web.

The software is incomplete but it does parse basic markdown like Headers, Lists, Inline styling (Bold, Italics, Code), Code Blocks (kinda), Links, Images, and regular text.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

I build this software using the rust language and the pest parser library. I also used this project as a chance to test working with rust in VS Code, Zed, and RustRover. Each IDE had their strengths and drawbacks but I ended up writing most of my code in Zed. I also used Claude Code to ask questions when my parser was not working to help me with debugging and testing different solutions for problems I faced.

# Useful Websites

Here are some of the websites that helped me to build this project.

- [Markdown Guide](https://www.markdownguide.org/)
- [Pest Book: Grammar](https://pest.rs/book/grammars/syntax.html)
- [Rust Book](https://doc.rust-lang.org/book/title-page.html)

# Future Work

Here are some of the features I want to include in the future:

- Parse and covert Blockquotes to html
- Handle escape characters in markdown
- Parse and covert tables