#[macro_use] extern crate lazy_static;

extern crate clap;
use clap::{Arg, App};

use regex::Regex;

use std::io::prelude::*;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::fs::{self, create_dir};



lazy_static! {

    // HTML patterns, sort these by priority as they will be find/replace within the body of the markdown.
    static ref HTML_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"<([^:]+):([^>]+)>").unwrap(),                      // 0. matches variable declarations <var:val>, group[1]=var, group[1]=val
        Regex::new(r"#{1}([^#\s]+)").unwrap(),                          // matchs tag declarations #tag1 #tag2, group[1]=tag
        Regex::new(r"(?m)^(#+)\s+(.+)$").unwrap(),                      // matches headings, group[1] = number of #, group[2] = heading text
        Regex::new(r"(\*+)([^\*\n]+)(\*+)").unwrap(),                   // matches **bold** and *italic*, group[1] = number of *, group[2] = text
        Regex::new(r"(?m)^(?:\*{3}|={3}|-{3})$").unwrap(),              // matches horizontal rules (===,---,***)
        Regex::new(r#"\[(.+)\]\((https?://\S+)\s"(.+)"\)"#).unwrap(),   // 5. matches url insert ([Link Text](http://example.com "Optional Title")), group[1] = Link Name, group[2] = link, group[3] = Alt text/mouseover text
        Regex::new(r#"!\[(.+)\]\((\S+)\s"(.+)"\)"#).unwrap(),           // matches img insert (![Alt Text](/path/to/image.jpg "Optional Title")) group[1]=alt text, group[2]=image path, group[3]=title
        Regex::new(r"(?m)^(\d+)\.(.+)$").unwrap(),                      // Matches list items denoted with numbers 1. test 2. test2 etc, group[1]=point #, group[2]=text
        Regex::new(r"(?m)^[*-]{1}\s(.*)$").unwrap(),                    // Matches list items denoted with - and *, group[1]=text
        Regex::new(r"(?s)`{3}(.*)`{3}").unwrap(),                       // Matches code blocks denoted with ``` code ``` group[1] = code
        Regex::new(r"`{1}([^`\n]+)`{1}").unwrap(),                      // 10. Matches inline code denoted with ` code ` group[1] = code
        Regex::new(r"(?m)^(>+)(.*)").unwrap(),                          // Matches blockquote group[1]=# of >, group[2]=text
    ];

}



fn is_empty(s: &str) -> bool {
    for i in s.chars() {
        if i.is_alphanumeric() {
            return false
        }
    }
    true
}


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
    let mut html = String::new();

    

    let mut note = ZKNote::new();    

    for x in contents.split("\n") {

        let mut current_line: String = x.to_string();

        println!("Doing line >> {:?}", current_line);
        // Iterate all HTML patterns, which are sorted descending by priority.
        for (i, pattern) in HTML_PATTERNS.iter().enumerate() {

            // Loop until everything possible in a line has been matched.
            loop {

                // possibly clone current line here so we aren't borrowing as mutable && immutable at the same time?
                let cap = if let Some(c) = pattern.captures(&current_line) {c} else {break};
                println!("Matched regex:{} => {:?}", i, cap);
                
                // Depending which pattern index we are on we need to perform different operations.
                match i {

                    // var regex
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

                    // tag regex
                    1 => {
                        note.tags.push(
                            cap.get(1).unwrap().as_str().to_string()
                        );
                        current_line.replace_range(
                            cap.get(0).unwrap().range(),
                            ""
                        );
                    },
                    
                    // header regex
                    2 => {
                        // Replace # Header line with <h{n of #}>{Text}</h{n of #}> .
                        current_line.replace_range(
                            cap.get(0).unwrap().range(),
                            format!(
                                "<h{0}>{1}</h{0}>",
                                cap.get(1).unwrap().as_str().len(),
                                cap.get(2).unwrap().as_str(),
                            ).as_str()
                        );
                    },

                    // inline code regex
                    10 => {
                        current_line.replace_range(
                            cap.get(0).unwrap().range(),
                            format!(
                                "<code>{0}</code>",
                                cap.get(1).unwrap().as_str(),
                            ).as_str()
                        )
                    }
                    _ => {},
                }
            }
        }
        
        // If statement removes and random spaces at top of HTML file leftover from removing var and tag declarations.
        if html.len() > 0 || !is_empty(&current_line) {
            html.push_str(format!("{}\n", &current_line).as_str());
        }
        
        
    }


    // will need to adjust the output dir to /var/cache once out of dev
    let mut output_html = fs::OpenOptions::new()
                                        .read(false)
                                        .write(true)
                                        .create(true)
                                        .truncate(true)
                                        .open("./test_outputs/Test.html")?;

    output_html.write_all(html.as_bytes())?;
    println!("{:?}", output_html);

    
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
