module 覚えた.main;

void main(const(string)[] args)
{
    import std.random : randomShuffle;
    import std.stdio : stdin, stdout;
    import 覚えた.card : Card, readCards;
    import 覚えた.console : clearScreen, promptAnswer;
    import 覚えた.console : writeMistake, writeQuestion;

    import console = 覚えた.console;

    auto inputCards = readCards(args[1 .. $]);
    const cards = randomShuffle(inputCards);

    const(Card)[] correct;
    const(Card)[] incorrect;

    foreach (card; cards) {
        clearScreen(stdout);
        stdout.write("\n\n\n\n\n\n\n\n\n");
        writeQuestion(stdout, card);
        stdout.write("\n\n");
        const answer = promptAnswer(stdout, stdin, card);
        if (answer == card.answer) {
            correct ~= card;
        } else {
            writeMistake(stdout, stdin, card);
            incorrect ~= card;
        }
    }

    clearScreen(stdout);

    foreach (card; correct)
        stdout.writefln!"%s正解、%s：%s、%s：%s%s"(
            console.GREEN,
            card.questionPrefix,
            card.question,
            card.answerPrefix,
            card.answer,
            console.RESET,
        );

    foreach (card; incorrect)
        stdout.writefln!"%s不正解、%s：%s、%s：%s%s"(
            console.RED,
            card.questionPrefix,
            card.question,
            card.answerPrefix,
            card.answer,
            console.RESET,
        );
}
