// use clap::parser;
// use std::fs;



// #[derive(Parser, Debug)]
// #[command(name = "wordcounter")]
// #[command(about = "Counts words in a file", long_about = None)]
// #[command(version = "1.0")]
// struct Args {
//     file: String,
// }

// fn main (){
//     let args = Args:: parse();

//     let content = fs:: read_to_string(&args.file)
//         .expect("Failed to read the file");

//     let word_count = content.split_whitespace().count();

//     let word_count = content.split_whitespace().count();
//     println!("file: {}", args.file);
//     println!("word count: {}", word_count);
// }



// use clap::Parser;
// use std::fs;

// #[derive(Parser, Debug)]
// #[command(name = "wordcounter")]
// #[command(about = "Counts words in a file", long_about = None)]
// #[command(version = "1.0")]
// struct Args {
    
//     file: String,
// }

// fn main() {
//     let args = Args::parse();


//     let bytes = fs::read(&args.file)
//         .expect("Failed to read the file");
//     let content = String::from_utf8_lossy(&bytes);

//     let word_count = content.split_whitespace().count();

//     println!("file: {}", args.file);
//     println!("word count: {}", word_count);
// }


use clap :: Parser;
use std::{fs , path::PathBuf};
use serde_json::json;



#[derive(Parser, Debug)]

#[command (name = "word_counter", author,version,about = "counts words in a textfile")]

struct Args{
    files: PathBuf,

    #[arg(short = 'c', long)]
    chars: bool,

    #[arg(short = 'l', long)]
    lines: bool,

    #[arg(short = 'j', long)]
    json: bool,

}

fn main () -> Result<(), Box<dyn std::error::Error>>
{
    let args = Args::parse();


    let bytes = fs::read(&args.files)?;
    let content = String::from_utf8_lossy(&bytes);

    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    let lines = content.lines().count();
    if args.json{

        let output = json!({
            "file": args.files.to_string_lossy(),
            "words": words,
            "chars": chars,
            "lines": lines,
        });
    
    println!("{}", serde_json::to_string_pretty(&output)?);
    return Ok(());


}
println!("File: {}", args.files.to_string_lossy());
    println!("Words: {}", words);
    if args.chars {
        println!("Characters: {}", chars);
    }
    if args.lines {
        println!("Lines: {}", lines);
    }

    Ok(())
}
