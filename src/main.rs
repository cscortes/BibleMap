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

#[derive(Debug)]
struct VerseInfo {
    chapter: usize,
    verse: usize,
    text: String
}

#[derive(Debug)]
struct TextIndex {
    start_num: usize,
    end_num: usize,
    body_text: String,
    verses: Vec<VerseInfo>
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
            return idx+1;
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
    let end = start_of("*** END OF THE PROJECT GUTENBERG EBOOK THE KING JAMES BIBLE ***", lines, ent);

    books.push(BookIndex { is_book: false, title: "Start Old Testament Books".to_owned(), line_num: ot, bidx: 1 });
    books.push(BookIndex { is_book: false, title: "Start New Testament Books".to_owned(), line_num: nt, bidx: 2 });
    books.push(BookIndex { is_book: false, title: "End New Testament Books".to_owned(), line_num: ent, bidx: 3 });
    books.push(BookIndex { is_book: false, title: "End Bible Text".to_owned(), line_num: end, bidx: 4 });

    let otbooks = lines[ot..nt-1].iter()
                .filter(|s| s.len() > 0).collect::<Vec<&String>>();
    let ntbooks = lines[nt..ent-1].iter()
                .filter(|s| s.len() > 0).collect::<Vec<&String>>();

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

fn book_texts(lines: &[String], book_indexes : &mut Vec<BookIndex>) -> Vec<TextIndex>  
{
    let mut text_indexes = Vec::new();

    let mut ent = lines.len();
    let mut end = lines.len();

    // find line_num by book == false and bidx == 3
    for book in book_indexes.as_slice()
    {
        if book.is_book == false
        {
            if book.bidx == 3
            {
                ent = book.line_num;
            }
            if book.bidx == 4
            {
                end = book.line_num;
                break;
            }
        }
    }

    // previous code should have found the "end of list of books"
    assert!(ent != lines.len());

    let mut realbooks : Vec<&mut BookIndex> = book_indexes.iter_mut()
            .filter(|b| b.is_book)
            .collect();

    // Find all line numbers of books
    let mut first = ent;
    for book in realbooks.iter_mut()
    {
        let next = start_of(book.title.as_str(), lines, first);
        // println!("Next line number {} : {}", next, book.title);
        book.line_num = next;
        first = next+1;
    }

    for idx in 0..realbooks.len()-1
    {
        let book = realbooks.iter().nth(idx).unwrap();
        let next_book = realbooks.iter().nth(idx+1).unwrap();

        text_indexes.push(TextIndex { 
            start_num: book.line_num, 
            end_num: next_book.line_num,
            body_text: "".to_owned(),
            verses: Vec::new() });
    }

    let last = realbooks.iter().last().unwrap();
    text_indexes.push(TextIndex { 
        start_num: last.line_num, 
        end_num: end, 
        body_text: "".to_owned(),
        verses: Vec::new()
    });

    // find text
    for text_idx in text_indexes.iter_mut()
    {
        text_idx.body_text = lines[text_idx.start_num+1..text_idx.end_num-1].join(" ");
        // println!("{}", text_idx.verses);
    }

    let cv_pat = Regex::new(r"(?P<chapter>\d+):(?P<verse>\d+)").unwrap();
    let cvm_pat = Regex::new(r"(?P<chapter>\d+):(?P<verse>\d+)\s*(?P<text>[^~]+)").unwrap();

    // find verses
    for text_idx in text_indexes.iter_mut()
    {
        let mut body =cv_pat.replace_all(&text_idx.body_text, "~${chapter}:${verse}");
        body += "~";

        for cap in cvm_pat.captures_iter(&body)
        {            
            text_idx.verses.push(VerseInfo {
                chapter: cap.name("chapter").unwrap().as_str().parse::<usize>().unwrap(),
                verse: cap.name("verse").unwrap().as_str().parse::<usize>().unwrap(),
                text: cap.name("text").unwrap().as_str().to_string(),
            });
        }
    }

    text_indexes
}

fn main() {
    // Define a regular expression to match the book, chapter, and verse numbers
    let lines = read_text("pg10.txt");
    let _rows = line_by_line(&lines);

    let mut book_indexes = find_list_books(&lines);

    // Find Text, between "books",
    // need to know where the first books (line (indexes))
    let mut text_indexes = book_texts(&lines, &mut book_indexes);


    // Final Output
    for book in book_indexes.iter()
    {
        for tex in text_indexes.iter()
        {
            if (book.line_num == tex.start_num) 
            {
                println!("\n{}",book.title);
                for vinfo in tex.verses.iter()
                {
                    println!("{}:{} {}", vinfo.chapter, vinfo.verse, vinfo.text);
                }
            }
        }

    }



    // Iterate over the lines, extracting the matching text and creating a Polars DataFrame
    // let df = DataFrame::new(rows).unwrap();
    
    // Search for Genesis 1:10 in the DataFrame and print the result
    // let search = df.filter(col("column0").eq("Genesis").and(col("column1").eq(1)).and(col("column2").eq(10)))?;
    // println!("{:?}", search);
}
