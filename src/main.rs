#[macro_use] extern crate lazy_static;

extern crate clap;
use clap::{Arg, App};

use regex::Regex;

// use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::fs::{self, create_dir};
use std::collections::HashMap;




fn compile_mds(dir: &Path) -> Result<(), Error> {
    // Verify /var/cache/zkru exists
    if !Path::new("/var/cache/zkru").exists() {
        create_dir("/var/cache/zkru").expect("failed to create /var/cache/zkru");
    }

    // Iterate each file in given directory and compile it to HTML.
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry?;
        let filename = entry.file_name();

        // Check if file is markdown file.
        if let Some(s) = filename.to_str() {
            if s.ends_with(".md") {
                convert_md_to_html(&s, entry.path())?;
            }
        }
        
    }

    Ok(())
}


// Need to figure out lifetime parameters for ID/Title to make them into &str instead of strings.
#[derive(Debug)]
struct ZKNote {
    id: Option<String>,
    title: Option<String>,
    tags: Vec<String>,
    html: String,
}

impl ZKNote {
    fn new() -> ZKNote {
        ZKNote {
            id: None,
            title: None,
            tags: Vec::new(),
            html: String::new(),
        }
    }
}

fn convert_md_to_html(_filename: &str, p: PathBuf) -> Result<(), Error> {
    
    // Open file and start reading contents
    let contents: String = fs::read_to_string(p)?;
    
    // Will store our html file. Maybe change this from a String? I'm not sure what is more effecient.
    let mut html_lines = Vec::new();

    lazy_static! {

        // HTML patterns, sort these by priority as they will be find/replace within the body of the markdown.
        static ref HTML_PATTERNS: Vec<Regex> = vec![
            Regex::new(r"<([^:]+):([^>]+)>").unwrap(), // matches variable declarations <var:val>, group[1]=var, group[1]=val
            
            Regex::new(r"#{1}([^#\s]+)").unwrap(), // matchs tag declarations #tag1 #tag2, group[1]=tag
            
            Regex::new(r"(?m)^(#+)\s+(.+)$").unwrap(), // matches headings, group[1] = number of #, group[2] = heading text
            
            Regex::new(r"(\*+)([^\*\n]+)(\*+)").unwrap(), // matches **bold** and *italic*, group[1] = number of *, group[2] = text
            
            Regex::new(r"(?m)^(?:\*{3}|={3}|-{3})$").unwrap(), // matches horizontal rules (===,---,***)
            
            Regex::new(r#"\[(.+)\]\((https?://\S+)\s"(.+)"\)"#).unwrap(), // matches url insert ([Link Text](http://example.com "Optional Title")), group[1] = Link Name, group[2] = link, group[3] = Alt text/mouseover text
            
            Regex::new(r#"!\[(.+)\]\((\S+)\s"(.+)"\)"#).unwrap(), // matches img insert (![Alt Text](/path/to/image.jpg "Optional Title")) group[1]=alt text, group[2]=image path, group[3]=title
            
            Regex::new(r"(?m)^(\d+)\.(.+)$").unwrap(), // Matches list items denoted with numbers 1. test 2. test2 etc
            
            Regex::new(r"(?m)^[*-]{1}\s(.*)$").unwrap(), // Matches list items denoted with - and *
            
            Regex::new(r"(?s)`{3}(.*)`{3}").unwrap(), // Matches code blocks denoted with ``` code ```
            
            Regex::new(r"`{1}([^`\n]+)`{1}").unwrap(), // Matches inline code denoted with ` code `
            
            Regex::new(r"(?m)^(>+)(.*)").unwrap(), // Matches blockquote
        ];

    }

    let mut note = ZKNote::new();    

    // Likely remove lines that have var or tag definitions, all the other lines should be treated as a single string once these are removed since there are multiple multi line markdown things we need to format in
    for x in contents.split("\n") {

        let mut current_line: String = x.to_string();

        println!("{:?}", current_line);
        // Iterate all HTML patterns, which are sorted descending by priority.
        for (i, pattern) in HTML_PATTERNS.iter().enumerate() {

            // Loop until everything possible in a line has been matched.
            loop {

                // possibly clone current line here so we aren't borrowing as mutable && immutable at the same time?
                // let cap = match pattern.captures(&current_line) {
                //     Some(c) => c,
                //     None => break,
                // };

                let cap = if let Some(c) = pattern.captures(&current_line) {c} else {break};
                println!("cap: {:?}", cap);
                
                // Depending which pattern index we are on we need to perform different operations.
                match i {

                    // var declaration regex
                    0 => {
                        // Set variable value on current note.
                        let val: String = cap.get(2).unwrap().as_str().trim().to_string();
                        match cap.get(1).unwrap().as_str().trim() {
                            "id" => {note.id = Some(val)},
                            "title" => {note.title = Some(val)},
                            _ => {},
                        }
                        // Replace match range with a blank str.
                        current_line.replace_range(
                            cap.get(0).unwrap().range(),
                            ""
                        )
                    }

                    // tag declaration regex
                    1 => {
                        
                    }
                    _ => {},
                }
            }
        }
        println!("{:?} >> {:?}\n", note, current_line);
        html_lines.push(current_line);
        
        break;
    }
    
    Ok(())
}



fn main() {
    let matches = App::new("Zkru")
                        .version("0.1")
                        .author("Cameron Williams")
                        .about("ZK Compiler")
                        .arg(
                            Arg::with_name("dir")
                                    .required(true)
                                    .help("zettelkasten directory path")
                        ).get_matches();

    
    let path = Path::new(matches.value_of("dir").unwrap());
    println!("Running on {:?}", path);

    // Compile the markdown files in the ZK.
    compile_mds(&path).unwrap(); 
    
}
