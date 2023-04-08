//use polars::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
struct BookIndex {
    is_book: bool,
    title: String,
    line_num: usize,
    bidx: usize,    
}

fn read_text(fname: &str) -> Vec<String> 
{
    let ofile = File::open( fname ).expect("Error: File error!");
    let reader = BufReader::new(ofile);
    reader.lines().map(|line| line.unwrap()).collect()
}

fn normalize_string(word: &String) -> String 
{
    let listwords: Vec<_> = word
                .split(" ")
                .filter(|&x| x.len() > 0)
                .collect();

    listwords.join(" ")
}

fn start_of(marker: &str, lines: &[String], start: usize) -> usize 
{
    for idx in start..lines.len()
    {
        let line = lines.iter().nth(idx).unwrap();
        if marker.eq(normalize_string(line).as_str())
        {
            return idx;
        }
    }
    // Return Invalid Value
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

fn find_list_books(lines: &[String]) -> Vec<BookIndex>
{
    let mut books = Vec::new(); 
    // Finds the list of Old and New Testament books

    let ot  = start_of("The Old Testament of the King James Version of the Bible", lines, 0);
    let nt  = start_of("The New Testament of the King James Bible", lines, ot);
    let ent = start_of("The Old Testament of the King James Version of the Bible", lines, nt);

    books.push(BookIndex { is_book: false, title: "Start Old Testament".to_owned(), line_num: ot, bidx: 1 });
    books.push(BookIndex { is_book: false, title: "Start New Testament".to_owned(), line_num: nt, bidx: 2 });
    books.push(BookIndex { is_book: false, title: "End New Testament".to_owned(), line_num: ent, bidx: 3 });

    let mut otbooks = lines[ot+1..nt].iter().filter(|s| s.len() > 0).collect::<Vec<&String>>();
    let ntbooks = lines[nt+1..ent].iter().filter(|s| s.len() > 0).collect::<Vec<&String>>();

    println!("\nBooks in the OT: {}", otbooks.len());
    for (idx,book) in otbooks.iter().enumerate()
    {
        println!("\t{}: {}", idx + 1, book);
        books.push(BookIndex { is_book: true, title: book.to_string(), line_num: 0, bidx: idx + 1 });
    }

    println!("\nBooks in the NT: {}", ntbooks.len());
    for (idx,book) in ntbooks.iter().enumerate()
    {
        println!("\t{}: {}", idx + 1, book);
        books.push(BookIndex { is_book: true, title: book.to_string(), line_num: 0, bidx: idx + 1 });
    }

    books
}

fn book_texts(lines: &[String], book_indexes : &mut Vec<BookIndex>)  
{
    let mut ent = lines.len();

    // find line_num by book == false and bidx == 3
    for book in book_indexes.as_slice()
    {
        if (book.bidx == 3) && (book.is_book == false) 
        {
            ent = book.line_num;
            break;
        }
    }

    // previous code should have found the "end of list of books"
    assert!(ent != lines.len());


    let first = ent;

    for book in book_indexes.as_slice()
    {
        if book.is_book == false
        {
            continue;
        }

        let next = start_of(book.title.as_str(), lines, first);
        println!("Next line number {} : {}", next, book.title);

    }


}

fn main() {
    // Define a regular expression to match the book, chapter, and verse numbers
    let lines = read_text("pg10.txt");
    let _rows = line_by_line(&lines);

    let mut book_indexes = find_list_books(&lines);
    println!("List of Books: {:?}", book_indexes);

    // Find Text, between "books",
    // need to know where the first books (line (indexes))
    book_texts(&lines, &mut book_indexes);

    // Iterate over the lines, extracting the matching text and creating a Polars DataFrame
    // let df = DataFrame::new(rows).unwrap();
    
    // Search for Genesis 1:10 in the DataFrame and print the result
    // let search = df.filter(col("column0").eq("Genesis").and(col("column1").eq(1)).and(col("column2").eq(10)))?;
    // println!("{:?}", search);
}
