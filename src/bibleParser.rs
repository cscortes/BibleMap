use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_text(fname: &str) -> Vec<String> 
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

pub fn normalize_string(word: &String) -> String 
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

pub fn start_of(marker: &str, lines: &[String], start: usize) -> Result<usize, String> 
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
    let errmsg = format!("Failed: to find marker [{marker}]!");
    Err(errmsg.to_string())
}

#[test]
fn test_start_index_bad_index()
{
    let lines = read_text("pg10.txt");
    // really bad index
    assert_eq!(start_of("Genesis", &lines, 9999999), 
            Err("Failed: to find marker [Genesis]!".to_string()));
}

#[test]
fn test_start_index_bad_marker()
{
    let lines = read_text("pg10.txt");
    // bad marker
    assert_eq!(start_of("Zenesis", &lines, 9999999), 
            Err("Failed: to find marker [Zenesis]!".to_string()));
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
        Err("Failed: to find marker [Title: The King James Bible]!".to_string()));
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
