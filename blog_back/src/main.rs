use pulldown_cmark::{html, Parser};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: mkblog <input.md> [output.html]");
    }
    // this is probably redundant
    let output_path = if args.len() == 2 {
        "output.html"
    } else {
        &args[1]
    };

    let input_path = &args[1];
    let mut input_file = File::open(input_path)?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;

    let parser = Parser::new(&input_content);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let template_path = "blogTemplate.html";
    let mut template_file = File::open(template_path)?;
    let mut template_content = String::new();
    template_file.read_to_string(&mut template_content)?;

    let delimiter = '|';
    let (template_part1, template_part2) = template_content.split_once(delimiter).unwrap_or((&template_content[..], &template_content[..]));


    let mut output_file = File::create(output_path)?;
    output_file.write_all(template_part1.as_bytes())?;
    output_file.write_all(html_output.as_bytes())?;
    output_file.write_all(template_part2.as_bytes())?;
    //    io::stdout().write_all(html_output.as_bytes())?;
    Ok(())
}
