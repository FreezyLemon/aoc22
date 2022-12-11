use core::fmt;
use std::{collections::VecDeque, str::FromStr};

type MonkeyIndex = usize;
type WorryLevel = usize;

pub struct Monkey {
    inspected_count: usize,
    items: VecDeque<WorryLevel>,
    on_inspect: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    div_by: usize,
    if_true: MonkeyIndex,
    if_false: MonkeyIndex,
}

impl Monkey {
    pub fn throw_items(&mut self, reduce_worries: usize) -> Vec<(MonkeyIndex, WorryLevel)> {
        self.inspected_count += self.items.len();
        let mut result = Vec::with_capacity(self.items.len());

        while let Some(item) = self.items.pop_back() {
            let item = (self.on_inspect)(item) % reduce_worries;
            let target = self.choose_target(item);
            result.push((target, item));
        }

        result
    }

    fn choose_target(&self, item: WorryLevel) -> MonkeyIndex {
        if item % self.div_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }

    pub fn catch_item(&mut self, item: WorryLevel) {
        self.items.push_back(item)
    }

    pub fn get_count(&self) -> usize {
        self.inspected_count
    }

    pub fn get_div(&self) -> usize {
        self.div_by
    }
}

// Parsing code below

macro_rules! parse_error {
    ($type:ident) => {
        Err(Self::Err::from(ParseMonkeyError::$type))
    };
    ($type:ident, $line:ident) => {
        Err(Self::Err::from(ParseMonkeyError::$type(Line::$line)))
    };
}

macro_rules! ok {
    ($on:expr, $err_type:ident, $err_line:ident) => {
        $on.ok_or(ParseMonkeyError::$err_type(Line::$err_line))?
    };
}

impl FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = &mut s.split('\n');

        let idx = take_line_validated(lines, Line::Id, "Monkey ")?;
        let _idx: MonkeyIndex = idx.trim_end_matches(':').parse()?; // unused

        let items = take_line_validated(lines, Line::Items, "Starting items: ")?;
        let items: VecDeque<WorryLevel> = items
            .split(',')
            .map(|t| t.trim().parse().expect("can be parsed"))
            .collect();

        let op = take_line_validated(lines, Line::Operation, "Operation: ")?;
        assert!(op.starts_with("new = "));
        let (_, op) = ok!(op.split_once('='), InvalidLine, Operation);

        let mut tokens = op.split_whitespace();
        let lhs = ok!(tokens.next(), InvalidLine, Operation);

        if lhs != "old" {
            return parse_error!(InvalidLine, Operation);
        }

        let op = ok!(tokens.next(), InvalidLine, Operation).to_string(); // needed to be moved into a closure

        let rhs = ok!(tokens.next(), InvalidLine, Operation).parse();

        let op: Box<dyn Fn(WorryLevel) -> WorryLevel> = Box::new(move |x: WorryLevel| {
            let rhs: usize = match rhs {
                Ok(parsed) => parsed,
                Err(_) => x, // can't be parsed -> "old"
            };

            match op.as_str() {
                "+" => x + rhs,
                "*" => x * rhs,
                _ => unimplemented!(),
            }
        });

        if tokens.next().is_some() {
            return parse_error!(InvalidLine, Operation);
        }

        let div_by = take_line_validated(lines, Line::Test, "Test: divisible by ")?;
        let div_by: usize = div_by.parse()?;

        let if_true = take_line_validated(lines, Line::IfTrue, "If true: throw to monkey ")?;
        let if_true: MonkeyIndex = if_true.parse()?;

        let if_false = take_line_validated(lines, Line::IfFalse, "If false: throw to monkey ")?;
        let if_false: MonkeyIndex = if_false.parse()?;

        if lines.next().is_some() {
            return parse_error!(TooManyLines);
        }

        Ok(Monkey {
            inspected_count: 0,
            items,
            on_inspect: op,
            div_by,
            if_true,
            if_false,
        })
    }
}

fn take_line_validated<'a, T>(
    lines: &mut T,
    // line: Option<&'a str>,
    line_id: Line,
    starts_with: &str,
) -> Result<&'a str, ParseMonkeyError>
where T: Iterator<Item = &'a str> {
    let (empty, line) = lines
        .next()
        .ok_or(ParseMonkeyError::MissingLine(line_id))?
        .split_once(starts_with)
        .ok_or(ParseMonkeyError::InvalidLine(line_id))?;

    if empty.chars().all(|c| c.is_whitespace()) {
        Ok(line)
    } else {
        Err(ParseMonkeyError::InvalidLine(line_id))
    }
}

#[derive(Debug)]
enum ParseMonkeyError {
    MissingLine(Line),
    InvalidLine(Line),
    TooManyLines,
}

impl fmt::Display for ParseMonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseMonkeyError::MissingLine(l) => write!(f, "Line `{l}` is missing"),
            ParseMonkeyError::InvalidLine(l) => write!(f, "Line `{l}` is invalid"),
            ParseMonkeyError::TooManyLines => write!(f, "Lines left after parsing"),
        }
    }
}

impl std::error::Error for ParseMonkeyError {}

// Each monkey needs all of these
#[derive(Debug, Clone, Copy)]
enum Line {
    Id,
    Items,
    Operation,
    Test,
    IfTrue,
    IfFalse,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Line::Id => write!(f, "ID"),
            Line::Items => write!(f, "Items"),
            Line::Operation => write!(f, "Operation"),
            Line::Test => write!(f, "Test"),
            Line::IfTrue => write!(f, "If true"),
            Line::IfFalse => write!(f, "If false"),
        }
    }
}
