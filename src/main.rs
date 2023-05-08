use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::common::BookIndex;
mod common;

fn read_text(fname: &str) -> Vec<String> 
{
    let ofile = File::open( fname ).expect("Error: Reading file error!");
    let reader = BufReader::new(ofile);
    reader.lines().map(|line| line.unwrap()).collect()
}

#[test]
fn test_read_text()
{
    let lines = read_text("pg10.txt");

    // Able to read it
    assert_ne!(lines.len(), 0);
    // come up with correct number of lines
    assert_eq!(lines.len(), 100264);
}

#[test]
#[should_panic(expected = "Error: Reading file error!")]
fn test_read_text_no_file() 
{
    let lines = read_text("pg10_no_exists.txt");

    // Not able to read it
    assert_eq!(lines.len(), 0);
}

fn normalize_string(word: &String) -> String 
{
    let listwords: Vec<_> = word
                .split(" ")
                .filter(|&x| x.len() > 0)
                .collect();

    listwords.join(" ")
}

#[test]
fn normalize_string_tests()
{
    assert_eq!(normalize_string(&"hello".to_string()), "hello");
    assert_eq!(normalize_string(&" hello ".to_string()), "hello");
    assert_eq!(normalize_string(&"hello there".to_string()), "hello there");
    assert_eq!(normalize_string(&" hello there ".to_string()), "hello there");
    assert_eq!(normalize_string(&"  hello  there  ".to_string()), "hello there");
    assert_eq!(normalize_string(&"  hello  there  again  ".to_string()), "hello there again");
}

fn start_of(marker: &str, lines: &[String], start: usize) -> Result<usize, & 'static str> 
{
    // Assume that our list is zero based
    //
    for (idx, line) in lines.iter().skip(start).enumerate()
    {
        let norm_line = normalize_string(&line);
        if norm_line.contains(marker)
        {
            return Ok(start+idx);
        }
    }

    // Return Invalid Value
    Err("Failed: to find marker in lines!")
}

#[test]
fn test_start_index_bad_index()
{
    let lines = read_text("pg10.txt");
    // really bad index
    assert_eq!(start_of("Genesis", &lines, 9999999), 
            Err("Failed: to find marker in lines!"));
}

#[test]
fn test_start_index_bad_marker()
{
    let lines = read_text("pg10.txt");
    // bad marker
    assert_eq!(start_of("Zenesis", &lines, 9999999), 
            Err("Failed: to find marker in lines!"));
}

#[test]
fn test_start_index_good_beginning()
{
    let lines = read_text("pg10.txt");

    // really bad index
    assert_eq!(start_of("The Project Gutenberg eBook of The King James Bible", &lines, 0), 
            Ok(0));
}

#[test]
fn test_start_index_start_10()
{
    let lines = read_text("pg10.txt");

    // Find the first King James marker
    assert_eq!(start_of("Title: The King James Bible", &lines, 0), 
            Ok(10));
}

#[test]
fn test_start_index_start_10_skip_9()
{
    let lines = read_text("pg10.txt");

    // skipping lines, but should find King James at the same index
    assert_eq!(start_of("Title: The King James Bible", &lines, 9), 
            Ok(10));
}

#[test]
fn test_start_index_start_10_skip_10()
{
    let lines = read_text("pg10.txt");

    // skipping lines, including King James Marker, find next index, but err
    // because there is no next
    assert_eq!(start_of("Title: The King James Bible", &lines, 910), 
        Err("Failed: to find marker in lines!"));
}

#[test]
fn test_start_index_start_24()
{
    let lines = read_text("pg10.txt");

    // skipping lines, but exactly match 1st find
    assert_eq!(start_of("The First Book of Moses: Called Genesis", &lines, 24), 
            Ok(24));
}

#[test]
fn test_start_index_start_101_skip_25()
{
    let lines = read_text("pg10.txt");

    // skipping lines, 1 past first find, but should find King James 
    // at the next index
    assert_eq!(start_of("The First Book of Moses: Called Genesis", &lines, 25), 
            Ok(101));
}

fn find_list_books(lines: &[String]) -> Result<Vec<common::BookIndex>,& 'static str>
{
    let mut books = Vec::new(); 
    // Finds the list of Old and New Testament books

    let ot  = start_of("The Old Testament of the King James Version of the Bible", lines, 0)?;
    let nt  = start_of("The New Testament of the King James Bible", lines, ot)?;
    let ent = start_of("The Old Testament of the King James Version of the Bible", lines, nt)?;
    let end = start_of("*** END OF THE PROJECT GUTENBERG EBOOK (THE KING JAMES BIBLE ***", lines, ent)?;

    books.push(common::BookIndex { is_book: false, title: "Start Old Testament Books".to_owned(), line_num: ot, bidx: 1 });
    books.push(common::BookIndex { is_book: false, title: "Start New Testament Books".to_owned(), line_num: nt, bidx: 2 });
    books.push(common::BookIndex { is_book: false, title: "End New Testament Books".to_owned(), line_num: ent, bidx: 3 });
    books.push(common::BookIndex { is_book: false, title: "End Bible Text".to_owned(), line_num: end, bidx: 4 });

    // Old Testament books
    let otbooks = lines[ot+1..nt].iter()
                .filter(|s| s.len() > 0).collect::<Vec<&String>>();
    println!("\nBooks in the OT: {}", otbooks.len());
    for (idx,book) in otbooks.iter().enumerate()
    {
        println!("\t{}: {}", idx + 1, book);
        books.push(common::BookIndex { is_book: true, title: book.to_string(), line_num: 0, bidx: idx + 1 });
    }

    // New Testament books
    let ntbooks = lines[nt+1..ent].iter()
                .filter(|s| s.len() > 0).collect::<Vec<&String>>();
    println!("\nBooks in the NT: {}", ntbooks.len());
    for (idx,book) in ntbooks.iter().enumerate()
    {
        println!("\t{}: {}", idx + 1, book);
        books.push(common::BookIndex { is_book: true, title: book.to_string(), line_num: 0, bidx: idx + 1 });
    }

    Ok(books)
}

#[test]
fn test_find_list_books()
{
    let lines = read_text("pg10.txt");
    let books_found = find_list_books(&lines).unwrap();
    let bible_books : Vec<&BookIndex> = books_found.iter()
                    .filter(|book| book.is_book).collect();

    assert_eq!(bible_books.len(), 66);
}