


pub impl TestSuite {
    // mod common;
    use common::TextIndex;

    pub fn new () -> TestSuite {

        // Read file, filter empty lines
        let alllines = read_text("tests/bible_tests.txt"); 
        let tlines: Vec<&String> = alllines.iter()
        .filter(|&line| line.trim().len() >0).collect();

        // init tests
        let mut tests = Vec::new();
        let mut idx = 0; 

        while idx < tlines.len() 
        {
            let mut line = String::from(tlines[idx].trim());

            if line.contains("T~")
            {
                let replaced = line.replace("T~", "").to_string();
                let title = String::from(replaced.trim());
                let mut t = Test { 
                    book: title, 
                    verses: Vec::new()  
                };

                idx += 1;
                while idx < tlines.len() && !tlines[idx].contains("T~")
                {
                    line = String::from(tlines[idx].trim());
                    t.verses.push(line);
                    idx += 1;
                }

                tests.push(t);
            }
        }

        assert!(tests.len() == 66, "Should have 66 books in the KJV!");
        TestSuite { tests: tests }
    }


    pub fn run (&self, books: Vec<BookIndex>, texts: &Vec<TextIndex>) 
    {
        let cv_pat = Regex::new(r"(?P<chapter>\d+):(?P<verse>\d+)\s*(?P<text>[^~]+)\s*").unwrap();
        for test in self.tests.iter()
        {
            println!("TEST: Search for [{}]", test.book);

            let bookinfo : Vec<&BookIndex> = books.iter()
                .filter(|b| b.is_book && b.title == test.book)
                .collect();
            let thisbook = bookinfo.first().unwrap();

            // should be able to find a book
            assert!(bookinfo.len() == 1);

            let these_texts : Vec<&TextIndex> = texts.iter()
            .filter(|&text| text.start_num == thisbook.line_num ).collect();

            // should have many texts 
            assert!(these_texts.len() == 1);
            let this_text = these_texts.first().unwrap();

            for iverse in test.verses.iter()
            {
                if let Some(cap) = cv_pat.captures(iverse)
                {
                    let chapter = cap.name("chapter").unwrap().as_str().parse::<usize>().unwrap();
                    let verse   = cap.name("verse").unwrap().as_str().parse::<usize>().unwrap();
                    let text   = String::from(cap.name("text").unwrap().as_str());

                    println!("TEST TEXT: {}", text);

                    let search  = this_text.verses.iter()
                    .filter(|&v| v.chapter == chapter && v.verse == verse )
                    .collect::<Vec<&VerseInfo>>();

                    assert!( search.len() == 1 );
                    let found = search.first().unwrap();

                    if found.text != text
                    {
                        findx(&found.text, &text);
                        assert!(false, "ERROR: Didn't find test verse!")
                    }
                }
                else {
                    assert!(false, "Can't find verse!");
                }
            }
        }
    }
}
