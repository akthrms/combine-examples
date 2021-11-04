use combine::optional;
use combine::parser::char::{char, digit};
use combine::{EasyParser, ParseError, Parser, Stream};

#[derive(Debug, PartialEq, PartialOrd)]
struct Date {
    year: i64,
    month: i64,
    day: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Time {
    hour: i64,
    minute: i64,
    second: i64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Datetime {
    date: Date,
    time: Time,
}

fn four_digits<Input>() -> impl Parser<Input, Output = i64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (digit(), digit(), digit(), digit()).map(|n| {
        (n.0.to_digit(10).unwrap() * 1000
            + n.1.to_digit(10).unwrap() * 100
            + n.2.to_digit(10).unwrap() * 10
            + n.3.to_digit(10).unwrap()) as i64
    })
}

fn two_digits<Input>() -> impl Parser<Input, Output = i64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (digit(), digit()).map(|n| (n.0.to_digit(10).unwrap() * 10 + n.1.to_digit(10).unwrap()) as i64)
}

fn date<Input>() -> impl Parser<Input, Output = Date>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        four_digits(),
        char('-'),
        two_digits(),
        char('-'),
        two_digits(),
    )
        .map(|(year, _, month, _, day)| Date { year, month, day })
}

fn time<Input>() -> impl Parser<Input, Output = Time>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        two_digits(),
        char(':'),
        two_digits(),
        char(':'),
        two_digits(),
    )
        .map(|(hour, _, minute, _, second)| Time {
            hour,
            minute,
            second,
        })
}

fn datetime<Input>() -> impl Parser<Input, Output = Datetime>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (date(), optional(char('T')), time(), optional(char('Z')))
        .map(|(date, _, time, _)| Datetime { date, time })
}

fn main() {
    let result = datetime().easy_parse("2000-01-01T01:02:03Z");
    println!("{:?}", result.unwrap().0);
}
