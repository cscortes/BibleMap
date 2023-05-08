#[derive(Debug)]
pub struct BookIndex {
    pub bidx: usize,    
    pub title: String,
    pub is_book: bool,
    pub line_num: usize,
}

#[derive(Clone,Debug)]
pub struct VerseInfo {
    chapter: usize,
    verse: usize,
    text: String
}    

#[derive(Debug)]
pub struct TextIndex {
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
