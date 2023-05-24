#[derive(Debug)]
pub struct BookIndex {
    pub bidx: usize,    
    pub title: String,
    pub is_book: bool,
    pub line_num: usize,
}

#[derive(Clone,Debug)]
pub struct VerseInfo {
    pub chapter: usize,
    pub verse: usize,
    pub text: String
}    

#[derive(Debug)]
pub struct TextIndex {
    pub start_num: usize,
    pub end_num: usize,
    pub body_text: String,
    pub verses: Vec<VerseInfo>
}

#[derive(Debug)]
pub struct Test {
    pub book: String,
    pub verses: Vec<String>
}

#[derive(Debug)]
pub struct TestSuite {
    pub tests: Vec<Test>
}

