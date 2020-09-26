extern crate oboeta;

use std::io;

use oboeta::console::clear_screen;
use oboeta::console::prompt_answer;
use oboeta::console::write_mistake;
use oboeta::console::write_question;

use oboeta::card::Card;
use std::io::Write;

fn main() -> io::Result<()>
{
    let paths = &["cards/kanji-words.txt",
                  "cards/kanji-kunyomi.txt",
                  "cards/kanji-onyomi.txt"];
    let cards = Card::from_files(paths)?;

    let stdin_handle = io::stdin();
    let stdout_handle = io::stdout();

    let mut stdin = stdin_handle.lock();
    let mut stdout = stdout_handle.lock();

    for card in cards {
        clear_screen(&mut stdout)?;
        write!(stdout, "\n\n\n\n\n\n\n\n\n")?;
        write_question(&mut stdout, &card)?;
        write!(stdout, "\n\n")?;
        let answer = prompt_answer(&mut stdout, &mut stdin, &card)?;
        if answer != card.answer {
            write_mistake(&mut stdout, &mut stdin, &card)?;
        }
    }

    Ok(())
}
