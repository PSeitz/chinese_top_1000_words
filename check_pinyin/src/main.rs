use pinyin::ToPinyin;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io, process,
};

fn check_duplicates_sentences() -> Result<(), Box<dyn Error>> {
    let mut vals = HashMap::new();
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut line_num = 0;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let chinese = record.get(0).unwrap().to_string();
        // Check if we've already seen this character.
        if let Some(old_line) = vals.insert(chinese.to_string(), line_num) {
            println!("duplicate {} {}: {}", line_num, old_line, chinese);
        }
        line_num += 1;
    }
    Ok(())
}

fn check_duplicates_words() -> Result<(), Box<dyn Error>> {
    let mut vals = HashSet::new();
    let rdr = io::stdin();
    for result in rdr.lines() {
        let record = result?;
        let chinese = record.to_string();
        // Print the line if we haven't seen it before.
        if vals.insert(chinese.to_string()) {
            println!("{}", chinese);
        }
    }
    Ok(())
}

fn check_pinyin() -> Result<(), Box<dyn Error>> {
    let mut num_chars = 0;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let chinese = record.get(2).unwrap();
        let pinyin = record.get(3).unwrap();
        let mut gen_pinyin = String::new();
        for pinyin in chinese.to_pinyin() {
            if let Some(pinyin) = pinyin {
                gen_pinyin.push_str(pinyin.with_tone());
                gen_pinyin.push_str(" ");
            }
        }
        let gen_pinyin = gen_pinyin.trim().to_string();

        if !compare_strings_ignore_whitespace_and_case(&gen_pinyin, pinyin) {
            println!("{:?}", normalize_string(&gen_pinyin));
            println!("{:?}", normalize_string(pinyin));
        }
        num_chars += chinese.chars().count();
    }
    println!("num_chars: {}", num_chars);
    Ok(())
}
fn normalize_string(s1: &str) -> String {
    s1.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}
fn compare_strings_ignore_whitespace_and_case(s1: &str, s2: &str) -> bool {
    let normalized_s1: String = normalize_string(s1);
    let normalized_s2: String = normalize_string(s2);

    normalized_s1 == normalized_s2
}
fn main() {
    //let lines = io::stdin();
    //for line in lines.lines() {
    //println!("{}", quote_blocks(&line.unwrap()));
    //}
    if let Err(err) = check_duplicates_words() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

fn quote_blocks(input: &str) -> String {
    let parts: Vec<&str> = input.split(',').collect();

    let chinese_sep = '，';
    let num_comma_in_pinyin = input.chars().filter(|cha| cha == &chinese_sep).count();
    let pinyin_block = &parts[3..=3 + num_comma_in_pinyin].join(",");

    let english_block = &parts[4 + num_comma_in_pinyin..].join(",");

    let out = format!(
        "{},{},\"{}\",\"{}\",\"{}\"",
        parts[0], parts[1], parts[2], pinyin_block, english_block
    );
    //if input.split(',').count() != 5 {
    //panic!("{}", out);
    //}

    out
}

fn main2() {
    let input = "所以,so/thus,他累了，所以他早點睡了,Tā lèi le, suǒyǐ tā zǎodiǎn shuì le, He was tired, so he went to sleep early.";
    let output = quote_blocks(input);
    println!("{}", output);
}
