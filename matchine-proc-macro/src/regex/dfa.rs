use std::collections::{HashMap, HashSet};

use super::{Alternatives, Mask, MaskElement, RegexPart, Sequence};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Condition
{
    Char(char),
    CustomMatrix(usize),
}

impl MaskElement for Condition
{
    const MIN: Self = Self::Char(char::MIN);
    const MAX: Self = Self::CustomMatrix(usize::MAX);

    fn checked_after(self) -> Option<Self>
    {
        match self
        {
            Self::Char(x) =>
            {
                x.checked_after().map(Self::Char).or(Some(Self::CustomMatrix(usize::MIN)))
            },
            Self::CustomMatrix(x) =>
            {
                x.checked_after().map(Self::CustomMatrix)
            },
        }
    }

    fn checked_before(self) -> Option<Self>
    {
        match self
        {
            Self::Char(x) =>
            {
                x.checked_before().map(Self::Char)
            },
            Self::CustomMatrix(x) =>
            {
                x.checked_before().map(Self::CustomMatrix).or(Some(Self::Char(char::MAX)))
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Destination
{
    target_state: Vec<usize>,
    allow_backtracking: bool,
}

struct DFA
{
    states: HashSet<Box<[usize]>>,
    transitions: Vec<HashMap<Mask<Condition>, (Box<[usize]>, bool)>>,
}

struct NFAState
{
    
}

fn process_alternatives(
    source: &Alternatives,
    stack: &mut Vec<usize>,
    states: &mut Vec<HashMap<Mask<Condition>, (usize, bool)>>)
{
    stack.push(0);

    for (i, source) in source.0.iter().enumerate()
    {
        *stack.last_mut().unwrap() = i;

        
    }
}