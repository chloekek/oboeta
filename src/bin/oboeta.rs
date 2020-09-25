extern crate oboeta;

use std::io;

use oboeta::console::prompt_answer;
use oboeta::console::write_mistake;
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
        Card{
            question_prefix: "漢字".to_string(),
            question:        "今日".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "きょう".to_string(),
        },
        Card{
            question_prefix: "漢字".to_string(),
            question:        "日本語".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "にほんご".to_string(),
        },
        Card{
            question_prefix: "漢字".to_string(),
            question:        "人".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "ひと".to_string(),
        },
        Card{
            question_prefix: "漢字".to_string(),
            question:        "木".to_string(),
            answer_prefix:   "平仮名".to_string(),
            answer:          "き".to_string(),
        },
    ];

    let stdin_handle = io::stdin();
    let stdout_handle = io::stdout();

    let mut stdin = stdin_handle.lock();
    let mut stdout = stdout_handle.lock();

    for card in cards {
        write_question(&mut stdout, &card)?;
        let answer = prompt_answer(&mut stdout, &mut stdin, &card)?;
        if answer != card.answer {
            write_mistake(&mut stdout, &card)?;
        }
    }

    Ok(())
}
