extern crate oboeta;
extern crate rand;

use oboeta::console;
use std::env;
use std::io;

use oboeta::console::clear_screen;
use oboeta::console::prompt_answer;
use oboeta::console::write_mistake;
use oboeta::console::write_question;

use oboeta::card::Card;
use rand::seq::SliceRandom;
use std::io::Write;

fn main() -> io::Result<()>
{
    let paths = env::args().skip(1);
    let mut cards = Card::from_files(paths)?;

    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);

    let stdin_handle = io::stdin();
    let stdout_handle = io::stdout();

    let mut stdin = stdin_handle.lock();
    let mut stdout = stdout_handle.lock();

    let mut correct   = Vec::new();
    let mut incorrect = Vec::new();

    for card in cards.iter() {
        clear_screen(&mut stdout)?;
        write!(stdout, "\n\n\n\n\n\n\n\n\n")?;
        write_question(&mut stdout, card)?;
        write!(stdout, "\n\n")?;
        let answer = prompt_answer(&mut stdout, &mut stdin, card)?;
        if answer == card.answer {
            correct.push(card);
        } else {
            write_mistake(&mut stdout, &mut stdin, card)?;
            incorrect.push(card);
        }
    }

    clear_screen(&mut stdout)?;

    for card in correct {
        writeln!(
            stdout,
            "{}正解、{}：{}、{}：{}{}",
            console::GREEN,
            card.question_prefix,
            card.question,
            card.answer_prefix,
            card.answer,
            console::RESET,
        )?;
    }

    for card in incorrect {
        writeln!(
            stdout,
            "{}不正解、{}：{}、{}：{}{}",
            console::RED,
            card.question_prefix,
            card.question,
            card.answer_prefix,
            card.answer,
            console::RESET,
        )?;
    }

    Ok(())
}
