module 覚えた.card;

struct Card
{
    wchar[] questionPrefix;
    wchar[] question;
    wchar[] answerPrefix;
    wchar[] answer;
}

Card[] readCards(const(string)[] paths)
{
    import std.algorithm : joiner, map;
    import std.array : array;
    import std.stdio : File;
    auto files = paths.map!(p => File(p));
    auto lines = files.map!(f => f.byLine).joiner;
    auto cards = lines.map!parseCard;
    return cards.array;
}

pure @safe
Card parseCard(scope const(char)[] line)
{
    import std.format : formattedRead;
    Card card;
    line.formattedRead!"%s　%s　%s　%s"(card.tupleof);
    return card;
}
