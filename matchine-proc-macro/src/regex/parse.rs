use core::num::IntErrorKind;
use std::str;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassPart
{
    Char(char),
    Range { min: char, max: char },
    Custom(usize),
    Dot,
    Class(Vec<ClassPart>),
    InvertClass(Vec<ClassPart>),
    NegateClass(Vec<ClassPart>),
    NegateInvertClass(Vec<ClassPart>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegexPart
{
    Char(char),
    Custom(usize),
    Dot,
    GreedyMultiply { part: Box<RegexPart>, min: usize, max: Option<usize> },
    LazyMultiply { part: Box<RegexPart>, min: usize, max: Option<usize> },
    Group(Alternatives),
    AtomicGroup(Alternatives),
    Class(Vec<ClassPart>),
    InvertClass(Vec<ClassPart>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sequence(pub Vec<RegexPart>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alternatives(pub Vec<Sequence>);

fn walk_class_part<'a>(
    source: &mut str::Chars<'a>,
    get_custom: &mut impl FnMut(&'a str) -> Result<Result<usize, Vec<ClassPart>>, ()>)
    -> Result<ClassPart, String>
{
    let first = match source.next().unwrap()
    {
        '.' => ClassPart::Dot,
        '\\' => match source.next()
        {
            Some('u') =>
            {
                let before_value = source.clone();

                match source.next()
                {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') =>
                    {
                        for i in 1..4
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => continue,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..i];
                                    return Err(format!("invalid escape `'\\u{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..4];
                        match char::try_from(u16::from_str_radix(value, 16).unwrap() as u32)
                        {
                            Ok(char) => ClassPart::Char(char),
                            Err(_) => return Err(format!("char value `'\\u{value}'` is too large")),
                        }
                    },
                    Some('{') =>
                    {
                        let before_value = source.clone();

                        let mut len = 0;
                        loop
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => len += 1,
                                Some('}') => break,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..len + 1];
                                    return Err(format!("invalid escape `'\\u{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..len];
                        match char::try_from(u32::from_str_radix(value, 16).unwrap())
                        {
                            Ok(char) => ClassPart::Char(char),
                            Err(_) => return Err(format!("char value `'\\u{{{value}}}'` is too large")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\u'`")),
                }
            },
            Some('c') =>
            {
                match source.next()
                {
                    Some('{') =>
                    {
                        let before_value = source.clone();

                        let mut len = 0;
                        loop
                        {
                            match source.next()
                            {
                                Some('\\') =>
                                {
                                    let value = &before_value.as_str()[..len + 1];
                                    return Err(format!("invalid escape `'\\c{{{value}'`"));
                                },
                                Some('}') => break,
                                _ => len += 1,
                            }
                        }

                        let value = &before_value.as_str()[..len];
                        match get_custom(value)
                        {
                            Ok(Ok(x)) => ClassPart::Custom(x),
                            Ok(Err(x)) => ClassPart::Class(x),
                            Err(()) => return Err(format!("unknown custom selector `\\c{{{value}}}`")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\c'`")),
                }
            },
            Some('x') =>
            {
                let before_value = source.clone();

                match source.next()
                {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') =>
                    {
                        for i in 1..2
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => continue,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..i];
                                    return Err(format!("invalid escape `'\\x{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..2];
                        match char::try_from(u8::from_str_radix(value, 16).unwrap() as u32)
                        {
                            Ok(char) => ClassPart::Char(char),
                            Err(_) => return Err(format!("char value `'\\x{value}'` is too large")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\x'`")),
                }
            },
            Some('0') => ClassPart::Char('\0'),
            Some('a') => ClassPart::Char('\x07'),
            Some('b') => ClassPart::Char('\x08'),
            Some('n') => ClassPart::Char('\n'),
            Some('r') => ClassPart::Char('\r'),
            Some('f') => ClassPart::Char('\x0C'),
            Some('v') => ClassPart::Char('\x0B'),
            Some('s') =>
            {
                ClassPart::Class(vec!
                [
                    ClassPart::Range { min: '\x09', max: '\x0D' },
                    ClassPart::Char(' '),
                    ClassPart::Char('\u{A0}'),
                    ClassPart::Char('\u{1680}'),
                    ClassPart::Range { min: '\u{2000}', max: '\u{200A}' },
                    ClassPart::Range { min: '\u{2028}', max: '\u{202F}' },
                    ClassPart::Char('\u{202F}'),
                    ClassPart::Char('\u{205F}'),
                    ClassPart::Char('\u{3000}'),
                    ClassPart::Char('\u{FEFF}'),
                ])
            },
            Some('S') =>
            {
                ClassPart::InvertClass(vec!
                [
                    ClassPart::Range { min: '\x09', max: '\x0D' },
                    ClassPart::Char(' '),
                    ClassPart::Char('\u{A0}'),
                    ClassPart::Char('\u{1680}'),
                    ClassPart::Range { min: '\u{2000}', max: '\u{200A}' },
                    ClassPart::Range { min: '\u{2028}', max: '\u{202F}' },
                    ClassPart::Char('\u{202F}'),
                    ClassPart::Char('\u{205F}'),
                    ClassPart::Char('\u{3000}'),
                    ClassPart::Char('\u{FEFF}'),
                ])
            },
            Some(char @ ('0'..'9' | 'a'..'z' | 'A'..'Z' | '_')) =>
            {
                return Err(format!("unsupported escape `'\\{char}'`"));
            },
            Some(char) =>
            {
                ClassPart::Char(char)
            },
            None => return Err(format!("incomplete `'\\'`")),
        },
        '-' =>
        {
            let before_peek = source.clone();

            match source.next()
            {
                Some('[') =>
                {
                    let after_open = source.clone();

                    let group = match source.next()
                    {
                        Some('^') => ClassPart::NegateInvertClass,
                        _ =>
                        {
                            *source = after_open;
                            ClassPart::NegateClass
                        },
                    };

                    let mut alternatives = Vec::new();

                    loop
                    {
                        let before_peek = source.clone();
                        match source.next()
                        {
                            Some(']') =>
                            {
                                break group(alternatives);
                            },
                            None =>
                            {
                                return Err(format!("unmatched `'['`"));
                            },
                            _ =>
                            {
                                *source = before_peek;
                                alternatives.push(walk_class_part(source, get_custom)?);
                            },
                        }
                    }
                },
                _ =>
                {
                    *source = before_peek;
                    ClassPart::Char('-')
                },
            }
        },
        '[' =>
        {
            let after_open = source.clone();

            let group = match source.next()
            {
                Some('^') => ClassPart::InvertClass,
                _ =>
                {
                    *source = after_open;
                    ClassPart::Class
                },
            };

            let mut alternatives = Vec::new();

            loop
            {
                let before_peek = source.clone();
                match source.next()
                {
                    Some(']') =>
                    {
                        break group(alternatives);
                    },
                    None =>
                    {
                        return Err(format!("unmatched `'['`"));
                    },
                    _ =>
                    {
                        *source = before_peek;
                        alternatives.push(walk_class_part(source, get_custom)?);
                    },
                }
            }
        },
        ']' => return Err(format!("unmatched `']'`")),
        char => ClassPart::Char(char),
    };

    let before_peek = source.clone();

    match source.next()
    {
        Some('-') =>
        {
            let after_separator = source.clone();

            match source.next()
            {
                Some('[' | ']') =>
                {
                    *source = before_peek;
                    return Ok(first);
                },
                _ => *source = after_separator,
            }
        },
        _ =>
        {
            *source = before_peek;
            return Ok(first);
        },
    }

    let ClassPart::Char(min) = first else { return Err(format!("invalid range")) };

    let max = match source.next().unwrap()
    {
        '.' => return Err(format!("invalid range")),
        '\\' => match source.next()
        {
            Some('u') =>
            {
                let before_value = source.clone();

                match source.next()
                {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') =>
                    {
                        for i in 1..4
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => continue,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..i];
                                    return Err(format!("invalid escape `'\\u{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..4];
                        match char::try_from(u16::from_str_radix(value, 16).unwrap() as u32)
                        {
                            Ok(char) => char,
                            Err(_) => return Err(format!("char value `'\\u{value}'` is too large")),
                        }
                    },
                    Some('{') =>
                    {
                        let before_value = source.clone();

                        let mut len = 0;
                        loop
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => len += 1,
                                Some('}') => break,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..len + 1];
                                    return Err(format!("invalid escape `'\\u{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..len];
                        match char::try_from(u32::from_str_radix(value, 16).unwrap())
                        {
                            Ok(char) => char,
                            Err(_) => return Err(format!("char value `'\\u{{{value}}}'` is too large")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\u'`")),
                }
            },
            Some('x') =>
            {
                let before_value = source.clone();

                match source.next()
                {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') =>
                    {
                        for i in 1..2
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => continue,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..i];
                                    return Err(format!("invalid escape `'\\x{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..2];
                        match char::try_from(u8::from_str_radix(value, 16).unwrap() as u32)
                        {
                            Ok(char) => char,
                            Err(_) => return Err(format!("char value `'\\x{value}'` is too large")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\x'`")),
                }
            },
            Some('0') => '\0',
            Some('a') => '\x07',
            Some('b') => '\x08',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('f') => '\x0C',
            Some('v') => '\x0B',
            Some('c' | 's' | 'S') => return Err(format!("invalid range")),
            Some(char @ ('0'..'9' | 'a'..'z' | 'A'..'Z' | '_')) =>
            {
                return Err(format!("unsupported escape `'\\{char}'`"));
            },
            Some(char) =>
            {
                char
            },
            None => return Err(format!("incomplete `'\\'`")),
        },
        '-' =>
        {
            let before_peek = source.clone();

            match source.next()
            {
                Some('[') => return Err(format!("invalid range")),
                _ =>
                {
                    *source = before_peek;
                    '-'
                },
            }
        },
        '[' => return Err(format!("invalid range")),
        ']' => unreachable!(),
        char => char,
    };

    if min > max { return Err(format!("invalid range order from `{:?}` to `{:?}`", min, max)) };

    Ok(ClassPart::Range
    {
        min,
        max
    })
}

fn walk_regex_part<'a>(
    source: &mut str::Chars<'a>,
    get_custom: &mut impl FnMut(&'a str) -> Result<Result<usize, Vec<ClassPart>>, ()>)
    -> Result<RegexPart, String>
{
    let part = match source.next().unwrap()
    {
        '|' => panic!("unhandled `'|'`"),
        '^' => return Err(format!("unsupported `'^'`, the regex is anchored to the start by default")),
        '$' => return Err(format!("unsupported `'$'`")),
        '.' => RegexPart::Dot,
        '\\' => match source.next()
        {
            Some('u') =>
            {
                let before_value = source.clone();

                match source.next()
                {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') =>
                    {
                        for i in 1..4
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => continue,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..i];
                                    return Err(format!("invalid escape `'\\u{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..4];
                        match char::try_from(u16::from_str_radix(value, 16).unwrap() as u32)
                        {
                            Ok(char) => RegexPart::Char(char),
                            Err(_) => return Err(format!("char value `'\\u{value}'` is too large")),
                        }
                    },
                    Some('{') =>
                    {
                        let before_value = source.clone();

                        let mut len = 0;
                        loop
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => len += 1,
                                Some('}') => break,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..len];
                                    return Err(format!("invalid escape `'\\u{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..len];
                        match u32::from_str_radix(value, 16).map(|x| char::try_from(x))
                        {
                            Ok(Ok(char)) => RegexPart::Char(char),
                            _ => return Err(format!("char value `'\\u{{{value}}}'` is too large")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\u'`")),
                }
            },
            Some('c') =>
            {
                match source.next()
                {
                    Some('{') =>
                    {
                        let before_value = source.clone();

                        let mut len = 0;
                        loop
                        {
                            match source.next()
                            {
                                Some('\\') =>
                                {
                                    let value = &before_value.as_str()[..len + 1];
                                    return Err(format!("invalid escape `'\\c{{{value}'`"));
                                },
                                Some('}') => break,
                                _ => len += 1,
                            }
                        }

                        let value = &before_value.as_str()[..len];
                        match get_custom(value)
                        {
                            Ok(Ok(x)) => RegexPart::Custom(x),
                            Ok(Err(x)) => RegexPart::Class(x),
                            Err(()) => return Err(format!("unknown custom selector `\\c{{{value}}}`")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\c'`")),
                }
            },
            Some('x') =>
            {
                let before_value = source.clone();

                match source.next()
                {
                    Some('0'..='9' | 'a'..='f' | 'A'..='F') =>
                    {
                        for i in 1..2
                        {
                            match source.next()
                            {
                                Some('0'..='9' | 'a'..='f' | 'A'..='F') => continue,
                                _ =>
                                {
                                    let value = &before_value.as_str()[..i];
                                    return Err(format!("invalid escape `'\\x{value}'`"));
                                },
                            }
                        }

                        let value = &before_value.as_str()[..2];
                        match char::try_from(u8::from_str_radix(value, 16).unwrap() as u32)
                        {
                            Ok(char) => RegexPart::Char(char),
                            Err(_) => return Err(format!("char value `'\\x{value}'` is too large")),
                        }
                    },
                    _ => return Err(format!("invalid escape `'\\x'`")),
                }
            },
            Some('0') => RegexPart::Char('\0'),
            Some('a') => RegexPart::Char('\x07'),
            Some('n') => RegexPart::Char('\n'),
            Some('r') => RegexPart::Char('\r'),
            Some('f') => RegexPart::Char('\x0C'),
            Some('v') => RegexPart::Char('\x0B'),
            Some('s') =>
            {
                RegexPart::Class(vec!
                [
                    ClassPart::Range { min: '\x09', max: '\x0D' },
                    ClassPart::Char(' '),
                    ClassPart::Char('\u{A0}'),
                    ClassPart::Char('\u{1680}'),
                    ClassPart::Range { min: '\u{2000}', max: '\u{200A}' },
                    ClassPart::Range { min: '\u{2028}', max: '\u{202F}' },
                    ClassPart::Char('\u{202F}'),
                    ClassPart::Char('\u{205F}'),
                    ClassPart::Char('\u{3000}'),
                    ClassPart::Char('\u{FEFF}'),
                ])
            },
            Some('S') =>
            {
                RegexPart::InvertClass(vec!
                [
                    ClassPart::Range { min: '\x09', max: '\x0D' },
                    ClassPart::Char(' '),
                    ClassPart::Char('\u{A0}'),
                    ClassPart::Char('\u{1680}'),
                    ClassPart::Range { min: '\u{2000}', max: '\u{200A}' },
                    ClassPart::Range { min: '\u{2028}', max: '\u{202F}' },
                    ClassPart::Char('\u{202F}'),
                    ClassPart::Char('\u{205F}'),
                    ClassPart::Char('\u{3000}'),
                    ClassPart::Char('\u{FEFF}'),
                ])
            },
            Some(char @ ('0'..'9' | 'a'..'z' | 'A'..'Z' | '_')) =>
            {
                return Err(format!("unsupported escape `'\\{char}'`"));
            },
            Some(char) =>
            {
                RegexPart::Char(char)
            },
            None => return Err(format!("incomplete `'\\'`")),
        },
        '*' => return Err(format!("cannot multiply nothing with `'*'`")),
        '+' => return Err(format!("cannot multiply nothing with `'+'`")),
        '?' => return Err(format!("cannot multiply nothing with `'?'`")),
        '(' =>
        {
            let after_open = source.clone();

            let group = match source.next()
            {
                Some('?') => match source.next()
                {
                    Some('>') => RegexPart::AtomicGroup,
                    Some(':') => RegexPart::Group,
                    Some('=') => return Err(format!("unsupported `'(?='`")),
                    Some('!') => return Err(format!("unsupported `'(?!'`")),
                    Some('<') => match source.next()
                    {
                        Some('=') => return Err(format!("unsupported `'(?<='`")),
                        Some('!') => return Err(format!("unsupported `'(?<!'`")),
                        _ => return Err(format!("incomplete `'(?'`")),
                    },
                    _ => return Err(format!("incomplete `'(?'`")),
                },
                _ =>
                {
                    *source = after_open;
                    RegexPart::Group
                },
            };

            let mut alternatives = Alternatives(Vec::new());

            loop
            {
                let sequence = match alternatives.0.last_mut()
                {
                    Some(x) => x,
                    None => alternatives.0.push_mut(Sequence(Vec::new())),
                };

                let before_peek = source.clone();
                match source.next()
                {
                    Some('|') =>
                    {
                        alternatives.0.push(Sequence(Vec::new()));
                    },
                    Some(')') =>
                    {
                        break group(alternatives);
                    },
                    None =>
                    {
                        return Err(format!("unmatched `'('`"));
                    },
                    _ =>
                    {
                        *source = before_peek;
                        sequence.0.push(walk_regex_part(source, get_custom)?);
                    },
                }
            }
        },
        ')' => return Err(format!("unmatched `')'`")),
        '[' =>
        {
            let after_open = source.clone();

            let group = match source.next()
            {
                Some('^') => RegexPart::InvertClass,
                _ =>
                {
                    *source = after_open;
                    RegexPart::Class
                },
            };

            let mut alternatives = Vec::new();

            loop
            {
                let before_peek = source.clone();
                match source.next()
                {
                    Some(']') =>
                    {
                        break group(alternatives);
                    },
                    None =>
                    {
                        return Err(format!("unmatched `'['`"));
                    },
                    _ =>
                    {
                        *source = before_peek;
                        alternatives.push(walk_class_part(source, get_custom)?);
                    },
                }
            }
        },
        char => RegexPart::Char(char),
    };

    let before_peek = source.clone();

    Ok(match source.next()
    {
        Some('*') =>
        {
            let before_peek = source.clone();

            match source.next()
            {
                Some('?') =>
                {
                    if matches!(source.clone().next(), Some('*' | '+' | '?'))
                    {
                        return Err(format!("cannot multiply a multiplier"));
                    }

                    RegexPart::LazyMultiply { part: Box::new(part), min: 0, max: None }
                },
                Some('*' | '+') =>
                {
                    return Err(format!("cannot multiply a multiplier"));
                },
                _ =>
                {
                    *source = before_peek;

                    RegexPart::GreedyMultiply { part: Box::new(part), min: 0, max: None }
                },
            }
        },
        Some('+') =>
        {
            let before_peek = source.clone();

            match source.next()
            {
                Some('?') =>
                {
                    if matches!(source.clone().next(), Some('*' | '+' | '?'))
                    {
                        return Err(format!("cannot multiply a multiplier"));
                    }

                    RegexPart::LazyMultiply { part: Box::new(part), min: 1, max: None }
                },
                Some('*' | '+') =>
                {
                    return Err(format!("cannot multiply a multiplier"));
                },
                _ =>
                {
                    *source = before_peek;

                    RegexPart::GreedyMultiply { part: Box::new(part), min: 1, max: None }
                },
            }
        },
        Some('?') =>
        {
            let before_peek = source.clone();

            match source.next()
            {
                Some('?') =>
                {
                    if matches!(source.clone().next(), Some('*' | '+' | '?'))
                    {
                        return Err(format!("cannot multiply a multiplier"));
                    }

                    RegexPart::LazyMultiply { part: Box::new(part), min: 0, max: Some(1) }
                },
                Some('*' | '+') =>
                {
                    return Err(format!("cannot multiply a multiplier"));
                },
                _ =>
                {
                    *source = before_peek;

                    RegexPart::GreedyMultiply { part: Box::new(part), min: 0, max: Some(1) }
                },
            }
        },
        Some('{') =>
        {
            let before_value = source.clone();

            let mut min_len = 0;
            loop
            {
                match source.next()
                {
                    Some('0'..='9') => min_len += 1,
                    Some(',') => break,
                    Some('}') =>
                    {
                        let before_peek = source.clone();

                        let min_value = &before_value.as_str()[..min_len];
                        let min = match usize::from_str_radix(min_value, 10)
                        {
                            Ok(x) => x,
                            Err(_) => return Err(format!("multiplier count `'{min_value}'` is too large")),
                        };

                        match source.next()
                        {
                            Some('?') =>
                            {
                                if matches!(source.clone().next(), Some('*' | '+' | '?'))
                                {
                                    return Err(format!("cannot multiply a multiplier"));
                                }

                                return Ok(RegexPart::LazyMultiply { part: Box::new(part), min, max: Some(min) });
                            },
                            Some('*' | '+') =>
                            {
                                return Err(format!("cannot multiply a multiplier"));
                            },
                            _ =>
                            {
                                *source = before_peek;

                                return Ok(RegexPart::GreedyMultiply { part: Box::new(part), min, max: Some(min) });
                            },
                        }
                    },
                    _ =>
                    {
                        *source = before_peek;

                        return Ok(part);
                    },
                }
            }

            let min_value = &before_value.as_str()[..min_len];
            let min = match usize::from_str_radix(min_value, 10)
            {
                Ok(x) => x,
                Err(_) => return Err(format!("multiplier count `'{min_value}'` is too large")),
            };

            let before_value = source.clone();

            let mut max_len = 0;
            loop
            {
                match source.next()
                {
                    Some('0'..='9') => max_len += 1,
                    Some('}') =>
                    {
                        let before_peek = source.clone();

                        let max_value = &before_value.as_str()[..max_len];
                        let max = match usize::from_str_radix(max_value, 10)
                        {
                            Ok(x) => x,
                            Err(x) if *x.kind() == IntErrorKind::Empty =>
                            {
                                match source.next()
                                {
                                    Some('?') =>
                                    {
                                        if matches!(source.clone().next(), Some('*' | '+' | '?'))
                                        {
                                            return Err(format!("cannot multiply a multiplier"));
                                        }

                                        return Ok(RegexPart::LazyMultiply { part: Box::new(part), min, max: None });
                                    },
                                    Some('*' | '+') =>
                                    {
                                        return Err(format!("cannot multiply a multiplier"));
                                    },
                                    _ =>
                                    {
                                        *source = before_peek;

                                        return Ok(RegexPart::GreedyMultiply { part: Box::new(part), min, max: None });
                                    },
                                }
                            },
                            Err(_) => return Err(format!("multiplier count `'{max_value}'` is too large")),
                        };

                        match source.next()
                        {
                            Some('?') =>
                            {
                                if matches!(source.clone().next(), Some('*' | '+' | '?'))
                                {
                                    return Err(format!("cannot multiply a multiplier"));
                                }

                                return Ok(RegexPart::LazyMultiply { part: Box::new(part), min, max: Some(max) });
                            },
                            Some('*' | '+') =>
                            {
                                return Err(format!("cannot multiply a multiplier"));
                            },
                            _ =>
                            {
                                *source = before_peek;

                                return Ok(RegexPart::GreedyMultiply { part: Box::new(part), min, max: Some(max) });
                            },
                        }
                    },
                    _ =>
                    {
                        *source = before_peek;

                        return Ok(part);
                    },
                };
            }
        },
        _ =>
        {
            *source = before_peek;
            part
        },
    })
}

pub fn parse_class<'a>(
    source: &mut str::Chars<'a>,
    get_custom: &mut impl FnMut(&'a str) -> Result<Result<usize, Vec<ClassPart>>, ()>)
    -> Result<Vec<ClassPart>, String>
{
    let mut alternatives = Vec::new();

    loop
    {
        let before_peek = source.clone();
        match source.next()
        {
            None => break,
            _ =>
            {
                *source = before_peek;
                alternatives.push(walk_class_part(source, get_custom)?);
            },
        }
    }

    Ok(alternatives)
}

pub fn parse_regex<'a>(
    source: &mut str::Chars<'a>,
    get_custom: &mut impl FnMut(&'a str) -> Result<Result<usize, Vec<ClassPart>>, ()>)
    -> Result<Alternatives, String>
{
    let mut alternatives = Alternatives(Vec::new());

    loop
    {
        let sequence = match alternatives.0.last_mut()
        {
            Some(x) => x,
            None => alternatives.0.push_mut(Sequence(Vec::new())),
        };

        let before_peek = source.clone();
        match source.next()
        {
            Some('|') =>
            {
                alternatives.0.push(Sequence(Vec::new()));
            },
            None => break,
            _ =>
            {
                *source = before_peek;
                sequence.0.push(walk_regex_part(source, get_custom)?);
            },
        }
    }

    Ok(alternatives)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn print()
    {
        dbg!(parse_regex(&mut r#"\s*{\s*"pin"\s*:\s*0\s*}"#.chars(), &mut |_| Err(())));
    }
}