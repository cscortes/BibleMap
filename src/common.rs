#[derive(Debug)]
struct BookIndex {
    is_book: bool,
    title: String,
    line_num: usize,
    bidx: usize,    
}

#[derive(Clone,Debug)]
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

#[derive(Debug)]
pub struct Test {
    book: String,
    verses: Vec<String>
}

#[derive(Debug)]
pub struct TestSuite {
    tests: Vec<Test>
}
