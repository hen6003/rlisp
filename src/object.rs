use crate::Lisp;

#[derive(Clone)]
pub enum Object {
    Nil,
    Pair(Box<Object>, Box<Object>),
    Symbol(String),
    Number(f32),
    Character(char),
    RustFunc(fn (&mut Lisp, Object) -> Object),
}

impl Object {
    fn parse_atom(string: &str) -> Self {
        if let Ok(i) = str::parse::<f32>(string) {
            Self::Number(i)
        } else if string.len() == 2 || string.starts_with('\\') {
            Self::Character(string.chars().nth(1).unwrap())
        } else if !string.is_empty() {
            Self::Symbol(string.to_string())
        } else {
            panic!("Unparsable atom");
        }
    }

    fn append_to_pair_list(&mut self, appende: Object) {
        let mut cur_object: &mut Self = match self {
            Self::Pair(_, b) => b,
            _ => panic!("Not a list"),
        };

        loop {
            match cur_object {
                Self::Pair(_, b) => cur_object = b,
                Self::Nil => break,
                _ => panic!("Not a list"),
            }
        }

        *cur_object = appende;
    }

    fn array_to_pair_list(array: Vec<Object>) -> Self {
        let mut ret = Self::Pair(Box::new(array[0].clone()), Box::new(Self::Nil));

        let mut iter = array.into_iter();
        iter.next();

        for i in iter {
            ret.append_to_pair_list(Self::Pair(Box::new(i), Box::new(Self::Nil)));
        }

        ret
    }

    fn iter_to_objects(strings: &mut dyn Iterator<Item = String>) -> Self {
        let mut list = Vec::new();
        let mut dot_occured = false;

        loop {
            match strings.next() {
                Some(s) => match s.as_str() {
                    "(" => {
                        if dot_occured {
                            let mut list = Self::array_to_pair_list(list);
 
                            list.append_to_pair_list(Self::iter_to_objects(strings));

                            if strings.next() != Some(")".to_string()) {
                                panic!("Unexpected object after dotted list end")
                            }

                            break list;
                        } else {
                            list.push(Self::iter_to_objects(strings))
                        }
                    },
                    ")" => {
                        break if list.is_empty() {
                            Self::Nil
                        } else {
                            Self::array_to_pair_list(list)
                        }
                    }
                    "." => {
                        if !list.is_empty() {
                            dot_occured = true
                        } else {
                            panic!("Expected object before '.'")
                        }
                    },
                    s => {
                        if dot_occured {
                            if strings.next() == Some(")".to_string()) {
                                let mut list = Self::array_to_pair_list(list);
 
                                list.append_to_pair_list(Self::parse_atom(s));

                                break list;
                            } else {
                                panic!("Unexpected object after dotted list end")
                            }
                        } else {
                            list.push(Self::parse_atom(s));
                        }
                    },
                },
                None => panic!("Unmatched '('"),
            }
        }
    }

    pub fn eval(strings: Vec<String>) -> Self {
        let mut iter = strings.into_iter();

        match iter.next() {
            Some(s) => match s.as_str() {
                "(" => Self::iter_to_objects(&mut iter),
                ")" => panic!("Unmatched ')'"),
                s => Object::parse_atom(s),
            },

            None => Object::Symbol(String::new())
        }
    }
}

use std::fmt;
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, ""), // If Nil on its own, don't display anything
            a => write!(f, "{:?}", a),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(a, b) => write!(f, "({:?} . {:?})", a, b),
            Self::Number(i) => write!(f, "{}", i),
            Self::Character(c) => write!(f, "\'{}\'", c),
            Self::Symbol(s) => write!(f, "{}", s),
            Self::RustFunc(x) => write!(f, "{:p}", x),
            Self::Nil => write!(f, "()"),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Pair(a, b) => match other {
                Self::Pair(c, d) => {
                    a == c && b == d
                },
                _ => false,
            },
            Self::Number(i) => match other {
                Self::Number(o) => {
                    i == o
                },
                _ => false,
            },
            Self::Character(c) => match other {
                Self::Character(o) => {
                    c == o
                },
                _ => false,
            },
            Self::Symbol(s) => match other {
                Self::Symbol(o) => {
                    s == o
                },
                _ => false,
            },
            Self::RustFunc(_) => false,
            Self::Nil => matches!(other, Self::Nil),
        }
    }
}
