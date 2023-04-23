use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

mod common;

use common::{ TextInfo, Test };

mod test_bible_verses;
use test_bible_verses::TestSuite;




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
    let cv_pat = Regex::new(r"(?P<chapter>\d+):(?P<verse>\d+)").unwrap();
    let cvm_pat = Regex::new(r"(?P<chapter>\d+):(?P<verse>\d+)\s*(?P<text>[^~]+)").unwrap();

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


        let mut next_book = next_book.line_num;

        if book.title.contains("Malachi")
        {
            next_book -= 12;
        }

        text_indexes.push(TextIndex { 
            start_num: book.line_num, 
            end_num: next_book,
            body_text: "".to_owned(),
            verses: Vec::new() 
        });
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
    }

    // find verses
    for text_idx in text_indexes.iter_mut()
    {
        let mut body =cv_pat.replace_all(&text_idx.body_text, "~${chapter}:${verse}");
        body += "~";

        for cap in cvm_pat.captures_iter(&body)
        {            
            let info = VerseInfo {
                chapter: cap.name("chapter").unwrap().as_str().parse::<usize>().unwrap(),
                verse: cap.name("verse").unwrap().as_str().parse::<usize>().unwrap(),
                text: cap.name("text").unwrap().as_str().trim().to_owned(),
            };
            text_idx.verses.push(info.clone());
        }
    }

    text_indexes
}

fn findx(t1:&String, t2:&String)
{
    // assert!(t1.len() == t2.len());

    println!("t1: {}", t1);
    println!("t2: {}", t2);

    println!("t1.len == t2.len, {} == {}", t1.len(), t2.len());

    for (c1, c2) in zip(t1.chars(), t2.chars())
    {
        println!("c1 == c2, {} == {}", c1, c2);
        assert!(c1 == c2 );
    }
}


fn main() {
    // Define a regular expression to match the book, chapter, and verse numbers
    let lines = read_text("pg10.txt");

    let mut book_indexes = find_list_books(&lines);

    // Find Text, between "books",
    // need to know where the first books (line (indexes))
    
    let text_indexes = book_texts(&lines, &mut book_indexes);

    // Final Output
    // print_bible(&book_indexes, &text_indexes);

    // Tests
    //// tests(book_indexes, text_indexes);

    let test_suite = TestSuite::new();
    print!("{:?}", test_suite);

    test_suite.run(book_indexes, &text_indexes);

    // TODO: Use Polars
    // Iterate over the lines, extracting the matching text and creating a Polars DataFrame
    // let df = DataFrame::new(rows).unwrap();
    
    // Search for Genesis 1:10 in the DataFrame and print the result
    // let search = df.filter(col("column0").eq("Genesis").and(col("column1").eq(1)).and(col("column2").eq(10)))?;
    // println!("{:?}", search);
}

fn print_bible(book_indexes: &Vec<BookIndex>, text_indexes: &Vec<TextIndex>) {
    for book in book_indexes.iter()
    {
        for tex in text_indexes.iter()
        {
            if book.line_num == tex.start_num
            {
                println!("\n{}",book.title);
                for vinfo in tex.verses.iter()
                {
                    println!("{}:{} {}", vinfo.chapter, vinfo.verse, vinfo.text);
                }
            }
        }
    }
}
