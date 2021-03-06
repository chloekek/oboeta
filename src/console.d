module 覚えた.console;

import std.stdio : File;
import 覚えた.card : Card;

enum CLEAR  = "\x1Bc";

enum RESET  = "\x1B[0m";
enum RED    = "\x1B[31m";
enum GREEN  = "\x1B[32m";
enum YELLOW = "\x1B[33m";
enum CYAN   = "\x1B[36m";
enum WHITE  = "\x1B[37m";

/++
 + Compute the width of a string in terminal columns.
 +/
private pure @safe
size_t strwidth(scope const(wchar)[] s)
{
    // TODO: Use Unicode property “East Asian Width”.
    import std.algorithm : map, sum;
    return s.map!(c => (c >= 'a' && c <= 'z') ||
                       (c >= 'A' && c <= 'Z') ||
                       c == ' '
                       ? 1 : 2).sum;
}

void clearScreen(ref File w)
{
    w.write(CLEAR);
    w.flush();
}

void writeQuestion(ref File w, ref const(Card) c)
{
    import std.range : repeat;

    // Include extra space for the colon.
    const width = strwidth(c.questionPrefix)
                + strwidth(c.question)
                + 2;

    // Compute padding to center the question.
    const padding = 80 / 2 - width / 2 - 2;

    // Print the question enclosed in a box.
    w.write(repeat(' ', padding));
    w.writefln!"%s┌%s┐%s"(CYAN, repeat('─', width), RESET);
    w.write(repeat(' ', padding));
    w.writefln!"%s│%s%s：%s%s%s│%s"
              (CYAN, WHITE, c.questionPrefix,
               YELLOW, c.question, CYAN, RESET);
    w.write(repeat(' ', padding));
    w.writefln!"%s└%s┘%s"(CYAN, repeat('─', width), RESET);
}

wstring promptAnswer(ref File w, ref File r, ref const(Card) c)
{
    import std.range : repeat;
    import std.utf : toUTF16;

    // Include extra space for the colon.
    const width = strwidth(c.answerPrefix)
                + strwidth(c.answer)
                + 2;

    // Compute padding to center the answer.
    const padding = 80 / 2 - width / 2 - 1;

    // Display prompt with answer prefix.
    w.write(repeat(' ', padding));
    w.writef!"%s%s：%s"(WHITE, c.answerPrefix, YELLOW);
    w.flush();

    // Accept input into string.
    const line = r.readln()[0 .. $ - 1].toUTF16;

    // Reset color after accepting input.
    w.write(RESET);
    w.flush();

    return line;
}

void writeMistake(ref File w, ref File r, ref const(Card) c)
{
    import std.range : repeat;

    // CJK characters are twice as wide.
    // Include extra space for the colon.
    const width = strwidth(c.answerPrefix)
                + strwidth(c.answer)
                + 2;

    // Compute padding to center the answer.
    const padding = 80 / 2 - width / 2 - 1;

    // Display actual answer.
    w.write(repeat(' ', padding));
    w.writefln!"%s%s：%s%s"(RED, c.answerPrefix, c.answer, RESET);

    // Wait for user input.
    r.readln();
}
