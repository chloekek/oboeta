use std::io;

use card::Card;
use std::io::BufRead;
use std::io::Write;

pub const RESET: &'static str = "\x1B[0m";
pub const RED: &'static str = "\x1B[31m";
pub const YELLOW: &'static str = "\x1B[33m";
pub const CYAN: &'static str = "\x1B[36m";
pub const WHITE: &'static str = "\x1B[37m";

pub fn write_question<W: Write>(w: &mut W, c: &Card) -> io::Result<()>
{
    // CJK characters are twice as wide.
    // Include extra space for the colon.
    let width = 2 * c.question_prefix.chars().count()
              + 2 * c.question.chars().count()
              + 2;

    // Compute padding to center the question.
    let padding = 80 / 2 - width / 2 - 2;

    // Print the question enclosed in a box.
    write!(w, "{0: <1$}", "", padding)?;
    writeln!(w, "{0}┌{1:─<2$}┐{3}", CYAN, "", width, RESET)?;
    write!(w, "{0: <1$}", "", padding)?;
    writeln!(w, "{0}│{1}{2}：{3}{4}{5}│{6}",
                CYAN, WHITE, c.question_prefix,
                YELLOW, c.question, CYAN, RESET)?;
    write!(w, "{0: <1$}", "", padding)?;
    writeln!(w, "{0}└{1:─<2$}┘{3}", CYAN, "", width, RESET)?;

    Ok(())
}

pub fn prompt_answer<W: Write, R: BufRead>(w: &mut W, r: &mut R, c: &Card) -> io::Result<String>
{
    // CJK characters are twice as wide.
    // Include extra space for the colon.
    let width = 2 * c.answer_prefix.chars().count()
              + 2 * c.answer.chars().count()
              + 2;

    // Compute padding to center the answer.
    let padding = 80 / 2 - width / 2 - 1;

    // Display prompt with answer prefix.
    write!(w, "{0: <1$}", "", padding)?;
    write!(w, "{0}{1}：{2}", WHITE, c.answer_prefix, YELLOW)?;
    w.flush()?;

    // Accept input into string.
    let mut line = String::new();
    r.read_line(&mut line)?;
    line.pop();

    // Reset color after accepting input.
    write!(w, "{0}", RESET)?;
    w.flush()?;

    Ok(line)
}

pub fn write_mistake<W: Write>(w: &mut W, c: &Card) -> io::Result<()>
{
    // CJK characters are twice as wide.
    // Include extra space for the colon.
    let width = 2 * c.answer_prefix.chars().count()
              + 2 * c.answer.chars().count()
              + 2;

    // Compute padding to center the answer.
    let padding = 80 / 2 - width / 2 - 1;

    // Display prompt with actual answer.
    write!(w, "{0: <1$}", "", padding)?;
    writeln!(w, "{0}{1}：{2}{3}", RED, c.answer_prefix, c.answer, RESET)?;

    Ok(())
}
