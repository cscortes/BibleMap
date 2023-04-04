//use polars::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_text(fname: &str) -> Vec<String> {
    let ofile = File::open( fname ).expect("Error: File error!");
    let reader = BufReader::new(ofile);
    
    reader.lines().map(|line| line.unwrap()).collect()
}

fn start_of(marker: &str, lines: &Vec<String>, start: usize) -> usize 
{
    for (idx,line) in lines.iter().enumerate()
    {
        if idx < start 
        {
            continue;
        }
        if line.starts_with(marker)
        {
            return idx;
        }
    }
    lines.len()
}

fn line_by_line(lines: &Vec<String>) -> Vec<(&str, i32, i32, &str)> 
{
    let re = Regex::new(r"(?P<book>\w+)\s+(?P<chapter>\d+):(?P<verse>\d+)\s*(?P<text>.+^\n)").unwrap();
    let mut rows = Vec::new();

    for line in lines 
    {
        if let Some(captures) = re.captures(&line) 
        {
            let book = captures.name("book").unwrap().as_str();
             let chapter = captures.name("chapter").unwrap().as_str().parse::<i32>().unwrap();
            let verse = captures.name("verse").unwrap().as_str().parse::<i32>().unwrap();
            let text = captures.name("text").unwrap().as_str();
            rows.push((book, chapter, verse, text));
        }
    }
    rows
}

fn find_list_books(lines: &Vec<String>) -> Vec<&String>
{
    let ot  = start_of("The Old Testament of the King James Version of the Bible", lines, 0);
    let nt  = start_of("The New Testament of the King James Bible", lines, ot);
    let ent = start_of("The Old Testament of the King James Version of the Bible", lines, nt);

    // println!("ot {}", ot);
    // println!("nt {}", nt);
    // println!("ent {}", ent);

    let mut otbooks = lines[ot+1..nt].iter().filter(|s| s.len() > 0).collect::<Vec<&String>>();
    let ntbooks = lines[nt+1..ent].iter().filter(|s| s.len() > 0).collect::<Vec<&String>>();

    println!("\nBooks in the OT: {}", otbooks.len());
    for (idx,book) in otbooks.iter().enumerate()
    {
        println!("\t{}: {}", idx + 1, book);
    }

    println!("\nBooks in the NT: {}", ntbooks.len());
    for (idx,book) in ntbooks.iter().enumerate()
    {
        println!("\t{}: {}", idx + 1, book);
    }

    otbooks.extend(ntbooks.iter());
    otbooks
}

fn main() {
    // Define a regular expression to match the book, chapter, and verse numbers
    let lines = read_text("pg10.txt");
    // println!("{:?}", lines);

    let _rows = line_by_line(&lines);
    // println!("{:?}", rows);

    let _books = find_list_books(&lines);

    // println!("List of Books: {:?}", books);

    // Iterate over the lines, extracting the matching text and creating a Polars DataFrame
    //let df = DataFrame::new(rows).unwrap();
    
    // Search for Genesis 1:10 in the DataFrame and print the result
    // let search = df.filter(col("column0").eq("Genesis").and(col("column1").eq(1)).and(col("column2").eq(10)))?;
    // println!("{:?}", search);
}
