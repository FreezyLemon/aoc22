use aoc2022::get_input;

fn main() {
    let content = get_input().unwrap();
    let mut elements: Vec<Element> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().expect("can parse as packet"))
        .collect();

    elements.push("[2]]".parse().unwrap());
    elements.push("[6]]".parse().unwrap());

    elements.sort_unstable();

    let div1: Element = "[2]]".parse().unwrap();
    let div2: Element = "[6]]".parse().unwrap();

    let decoder_key = elements.into_iter()
        .enumerate()
        .filter(|(_, p)| p == &div1 || p == &div2)
        .map(|(idx, _)| idx + 1)
        .reduce(|acc, elem| acc * elem)
        .expect("has elements");
    
    println!("result: {decoder_key}");
}

use std::{str::FromStr, convert::Infallible, cmp::Ordering, fmt, collections::VecDeque};

#[derive(Debug)]
pub enum Element {
    Integer(u32),
    List(Vec<Element>),
}

const LIST_START: char= '[';
const LIST_END: char = ']';
const DELIM: char = ',';

impl Element {
    /// Parses an Element::List from a char iterator
    fn from_chars<T: Iterator<Item = char>>(chars: &mut T) -> Self {
        let mut vec: Vec<Element> = vec![];
        // input never uses more than 2 chars per integer
        let mut buf: VecDeque<char> = VecDeque::with_capacity(2);

        while let Some(c) = chars.next() {
            match c {
                DELIM | LIST_END => {
                    if !buf.is_empty() {
                        let s: String = buf.drain(..).collect();
                        let i = s.parse().expect("can parse");
                        vec.push(Element::Integer(i));
                    }

                    if c == LIST_END {
                        break;
                    } else {
                        continue;
                    }
                },
                LIST_START => vec.push(Self::from_chars(chars)),
                d => buf.push_back(d) ,
            }
        }

        Element::List(vec)
    }
}

impl FromStr for Element {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_chars(&mut s.chars().skip(1)))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Element::Integer(left) => match other {
                Element::Integer(right) => {
                    if left < right {
                        Ordering::Less
                    } else if left == right {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                },
                Element::List(_) => {
                    let left = Element::List(vec![Element::Integer(*left)]);
                    left.cmp(other)
                },
            },
            Element::List(left) => match other {
                Element::Integer(right) => {
                    let right = Element::List(vec![Element::Integer(*right)]);
                    self.cmp(&right)
                }
                Element::List(right) => {
                    for (l, r) in left.into_iter().zip(right) {
                        let res = l.cmp(r);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }

                    let len_left = left.len();
                    let len_right = right.len();
                    if len_left < len_right {
                        Ordering::Less
                    } else if len_left == len_right {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                },
            },
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::Integer(val) => write!(f, "{val}"),
            Element::List(l) => {
                write!(f, "[")?;
                for idx in 0..l.len() {
                    let elem = &l[idx];
                    elem.fmt(f)?;

                    if idx < l.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            },
        }
    }
}
