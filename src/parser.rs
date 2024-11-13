use crate::token::Token;
use nom::bytes::complete::take_while1;
use nom::character::complete::{alphanumeric1, digit1, space0};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, separated_pair, tuple};
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

fn parse_keyword_set(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("set"), |_| Token::KeywordSet),
        map(tag("define"), |_| Token::KeywordSet),
        map(tag("declare"), |_| Token::KeywordSet),
    ))(input)
}

fn parse_keyword_display(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("show"), |_| Token::KeywordDisplay),
        map(tag("display"), |_| Token::KeywordDisplay),
        map(tag("print"), |_| Token::KeywordDisplay),
        map(tag("output"), |_| Token::KeywordDisplay),
        map(tag("say"), |_| Token::KeywordDisplay),
    ))(input)
}

fn parse_keyword_loop(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("repeat"), |_| Token::KeywordLoop),
        map(tag("loop"), |_| Token::KeywordLoop),
    ))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, Token> {
    map(alphanumeric1, |s: &str| Token::Identifier(s.to_string()))(input)
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    map_res(digit1, |s: &str| s.parse::<i32>().map(Token::Number))(input)
}

fn parse_text(input: &str) -> IResult<&str, Token> {
    delimited(
        tag("'"),
        map(take_while1(|c| c != '\''), |s: &str| Token::Text(s.to_string())),
        tag("'"),
    )(input)
}

fn parse_boolean(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("true"), |_| Token::Boolean(true)),
        map(tag("false"), |_| Token::Boolean(false)),
    ))(input)
}

fn parse_comma(input: &str) -> IResult<&str, Token> {
    alt((
        map(tuple((tag(","), space0, tag("and"))), |_| Token::Group),
        map(preceded(space0, tag(",")), |_| Token::Group),               // Match a single comma with optional leading spaces
        map(preceded(space0, tag(";")), |_| Token::Group),               // Match a semicolon with optional leading spaces
        map(preceded(space0, tag("and")), |_| Token::Group),             // Match "and" with optional leading space
    ))(input)
}

fn parse_period(input: &str) -> IResult<&str, Token> {
    map(tag("."), |_| Token::End)(input)
}

fn parse_keyword_function(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("function"), |_| Token::KeywordFunction),
        map(tag("method"), |_| Token::KeywordFunction),
    ))(input)
}
fn parse_keyword_class(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("class"), |_| Token::KeywordClass),
        map(tag("object"), |_| Token::KeywordClass),
    ))(input)
}

fn parse_keyword_import(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("import"), |_| Token::KeywordImport),
        map(tag("include"), |_| Token::KeywordImport),
        map(tag("use"), |_| Token::KeywordImport),
    ))(input)
}

fn parse_comment(input: &str) -> IResult<&str, Token> {
    delimited(
        tag("("),
        map(take_while1(|c| c != ')'), |s: &str| Token::Comment(s.to_string())),
        tag(")"),
    )(input)
}
fn parse_keyword_try(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("try"), |_| Token::KeywordTry),
        map(tag("attempt"), |_| Token::KeywordTry),
    ))(input)
}

fn parse_keyword_catch(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("catch"), |_| Token::KeywordCatch),
        map(tag("handle"), |_| Token::KeywordCatch),
    ))(input)
}

fn parse_conditional_start(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("when"), |_| Token::KeywordIf),
        map(tag("given that"), |_| Token::KeywordIf),
        map(tag("suppose that"), |_| Token::KeywordIf),
        map(tag("supposing"), |_| Token::KeywordIf),
        map(tag("in the case of"), |_| Token::KeywordIf),
        map(tag("should it be"), |_| Token::KeywordIf),
        map(tag("if"), |_| Token::KeywordIf),
        map(tag("while"), |_| Token::KeywordIf),
        map(tag("until"), |_| Token::KeywordIf),
    ))(input)
}

fn parse_conditional_alternative(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("otherwise"), |_| Token::KeywordElse),
        map(tag("alternatively"), |_| Token::KeywordElse),
        map(tag("in other cases"), |_| Token::KeywordElse),
        map(tag("else"), |_| Token::KeywordElse),
        map(tag("then"), |_| Token::KeywordElse),
    ))(input)
}

fn parse_calculation(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("add"), |_| Token::Add),
        map(tag("subtract"), |_| Token::Subtract),
        map(tag("multiply"), |_| Token::Multiply),
        map(tag("divide"), |_| Token::Divide),
    ))(input)
}

fn parse_skip_words(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("the"), |_| Token::Skip),
        map(tag("in"), |_| Token::Skip),
        map(tag("is"), |_| Token::Skip),
        map(tag("by"), |_| Token::Skip),
        map(tag("from"), |_| Token::Skip),
        map(tag("that"), |_| Token::Skip)
    ))(input)
}

fn parse_equals(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("equals"), |_| Token::Equals),
        map(tag("as"), |_| Token::Equals),
        map(tag("to"), |_| Token::Equals),
    ))(input)
}

fn parse_comparison(input: &str) -> IResult<&str, Token> {
    alt((
        map(alt((tag("greater than"), tag("bigger than"), tag("larger than"))), |_| Token::GreaterThan),
        map(alt((tag("less than"), tag("smaller than"))), |_| Token::LessThan),
        map(alt((tag("greater or equals"), tag("at least"))), |_| Token::GreaterOrEquals),
        map(alt((tag("less or equals"), tag("at most"))), |_| Token::LessOrEquals),
        map(alt((tag("equals"), tag("is"))), |_| Token::Equals),
        map(alt((tag("not equals"), tag("is not"), tag("is not equals"))), |_| Token::NotEquals),
    ))(input)
}

fn parse_and(input: &str) -> IResult<&str, Token> {
    map(tag("and"), |_| Token::And)(input)
}

fn parse_or(input: &str) -> IResult<&str, Token> {
    map(tag("or"), |_| Token::Or)(input)
}

fn parse_none(input: &str) -> IResult<&str, Token> {
    map(tag("none"), |_| Token::NoneType)(input)
}

fn parse_self_operation(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("increase"), |_| Token::KeywordIncrement),
        map(tag("decrease"), |_| Token::KeywordDecrement),
        map(tag("multiply"), |_| Token::KeywordMultiply),
        map(tag("divide"), |_| Token::KeywordDivide),
    ))(input)
}

fn parse_list(input: &str) -> IResult<&str, Token> {
    let list_keywords = alt((tag("a list of"), tag("a group of"), tag("list of"), tag("group of"), tag("contains"), tag("containing")));

    let parse_elements = separated_list0(
        parse_comma,
        alt((
            parse_number,    // Parse as number
            parse_text,      // Parse as string
            parse_boolean,   // Parse as boolean
            parse_list,      // Recursively parse nested lists
            parse_map,       // Recursively parse nested maps
            parse_none,
            parse_calculation,
            parse_comparison
        )),
    );

    preceded(
        list_keywords,
        map(parse_elements, Token::List),
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Token> {
    let map_keywords = alt((tag("a map of"), tag("a dict of"), tag("map of"), tag("dict of")));

    let parse_pair = separated_pair(
        alt((parse_identifier, parse_text)),
        alt((parse_equals, map(tag("as"), |_| Token::Equals))),
        alt((
            parse_number,    // Parse as number
            parse_text,      // Parse as string
            parse_boolean,   // Parse as boolean
            parse_list,      // Recursively parse nested lists
            parse_map,       // Recursively parse nested maps
            parse_none,
            parse_calculation,
            parse_comparison
        )),
    );

    preceded(
        map_keywords,
        map(separated_list0(parse_comma, parse_pair), Token::Map),
    )(input)
}

fn parse_add_into(input: &str) -> IResult<&str, Token> {
    alt((
        map(tag("add"), |_| Token::AddInto),
        map(tag("insert"), |_| Token::AddInto),
        map(tag("put"), |_| Token::AddInto),
    ))(input)
}


pub(crate) fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        alt((
            parse_keyword_set,
            parse_keyword_display,
            parse_keyword_loop,
            parse_keyword_function,
            parse_keyword_class,
            parse_keyword_try,
            parse_keyword_catch,
            parse_keyword_import,
            parse_conditional_start,
            parse_conditional_alternative,
        )),
        alt((
            parse_comparison,
            parse_number,
            parse_text,
            parse_boolean,
            parse_comma,
            parse_period,
            parse_comment,
            parse_calculation,
            parse_self_operation
        )),
        alt((
            parse_and,
            parse_or,
            parse_none,
            parse_list,
            parse_map,
            parse_equals,

            parse_add_into,

            parse_skip_words,
            parse_identifier,
        )),
    ))(input)
}