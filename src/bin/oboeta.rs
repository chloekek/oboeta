extern crate oboeta;

use std::io;

use oboeta::console::prompt_answer;
use oboeta::console::write_question;

use oboeta::card::Card;

fn main() -> io::Result<()>
{
    let cards = &[
        Card{
            question_prefix: "漢字".to_string(),
            question:        "覚える".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "おぼえる".to_string(),
        },
        Card{
            question_prefix: "漢字".to_string(),
            question:        "食べる".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "たべる".to_string(),
        },
        Card{
            question_prefix: "漢字".to_string(),
            question:        "日本".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "にほん".to_string(),
        },
    ];

    let stdin_handle = io::stdin();
    let stdout_handle = io::stdout();

    let mut stdin = stdin_handle.lock();
    let mut stdout = stdout_handle.lock();

    for card in cards {
        write_question(&mut stdout, &card)?;
        prompt_answer(&mut stdout, &mut stdin, &card)?;
    }

    Ok(())
}
