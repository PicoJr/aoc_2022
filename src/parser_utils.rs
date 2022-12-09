use nom::bytes::complete::take_while_m_n;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

fn number(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

pub(crate) fn positive_number(input: &str) -> IResult<&str, usize> {
    map_res(number, |out| out.parse::<usize>())(input)
}

pub(crate) fn single_space(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 1, |c: char| c == ' ')(input)
}
