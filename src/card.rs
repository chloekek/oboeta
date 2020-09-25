use std::io;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub struct Card
{
    pub question_prefix: String,
    pub question:        String,
    pub answer_prefix:   String,
    pub answer:          String,
}

impl Card
{
    pub fn from_files<I: IntoIterator<Item=P>, P: AsRef<Path>>(paths: I)
        -> io::Result<Vec<Self>>
    {
        let mut cards = Vec::new();
        for path in paths {
            let file = File::open(path)?;
            let buf  = BufReader::new(file);
            for line in buf.lines() {
                if let Some(card) = Self::parse(&line?) {
                    cards.push(card);
                }
            }
        }
        Ok(cards)
    }

    pub fn parse(line: &str) -> Option<Self>
    {
        let mut segments = line.split('　');
        let question_prefix = segments.next()?.to_string();
        let question        = segments.next()?.to_string();
        let answer_prefix   = segments.next()?.to_string();
        let answer          = segments.next()?.to_string();
        Some(Card{question_prefix, question, answer_prefix, answer})
    }
}
