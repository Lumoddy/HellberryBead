use core::ops::{self, AddAssign};
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use std::{fmt, range};

pub trait MaskElement: Ord + Clone
{
    const MIN: Self;
    const MAX: Self;

    fn checked_after(self) -> Option<Self>;

    fn checked_before(self) -> Option<Self>;
}

impl MaskElement for char
{
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;

    fn checked_after(self) -> Option<Self>
    {
        (self as u32).checked_add(1).and_then(char::from_u32)
    }

    fn checked_before(self) -> Option<Self>
    {
        (self as u32).checked_sub(1).and_then(char::from_u32)
    }
}

macro_rules! int_impl
{
    ( $( $ty:ident )* ) =>
    {
        $(
            impl MaskElement for $ty
            {
                const MIN: Self = Self::MIN;
                const MAX: Self = Self::MAX;

                fn checked_after(self) -> Option<Self> { self.checked_add(1) }

                fn checked_before(self) -> Option<Self> { self.checked_sub(1) }
            }
        )*
    };
}

int_impl!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize);

macro_rules! tuple_impl
{
    ( $( ( $( $field:tt : $field_ty:ident ),*$(,)? ) ),*$(,)? ) =>
    {
        $(
            impl< $( $field_ty : MaskElement , )* > MaskElement for ( $( $field_ty , )* )
            {
                const MIN: Self = ( $( $field_ty::MIN , )* );
                const MAX: Self = ( $( $field_ty::MAX , )* );

                fn checked_after(self) -> Option<Self>
                {
                    Some(( $( self. $field .checked_after()? , )* ))
                }

                fn checked_before(self) -> Option<Self>
                {
                    Some(( $( self. $field .checked_before()? , )* ))
                }
            }
        )*
    };
}

tuple_impl!
{
    (),
    (0: T0),
    (0: T0, 1: T1),
    (0: T0, 1: T1, 2: T2),
    (0: T0, 1: T1, 2: T2, 3: T3),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6),
    (0: T0, 1: T1, 2: T2, 3: T3, 4: T4, 5: T5, 6: T6, 7: T7),
}

pub trait IntoMaskRange<T: MaskElement>
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>;
}

impl<T: MaskElement> IntoMaskRange<T> for T
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        Some(range::RangeInclusive { start: self.clone(), last: self })
    }
}

impl<T: MaskElement> IntoMaskRange<T> for Range<T>
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        self.end.checked_before().map(|last| range::RangeInclusive { start: self.start, last })
    }
}

impl<T: MaskElement> IntoMaskRange<T> for RangeInclusive<T>
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        Some(range::RangeInclusive::from(self))
    }
}

impl<T: MaskElement> IntoMaskRange<T> for RangeTo<T>
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        self.end.checked_before().map(|last| range::RangeInclusive { start: T::MIN, last })
    }
}

impl<T: MaskElement> IntoMaskRange<T> for RangeToInclusive<T>
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        Some(range::RangeInclusive { start: T::MIN, last: self.end })
    }
}

impl<T: MaskElement> IntoMaskRange<T> for RangeFrom<T>
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        Some(range::RangeInclusive { start: self.start, last: T::MAX })
    }
}

impl<T: MaskElement> IntoMaskRange<T> for RangeFull
{
    fn into_mask_range(self) -> Option<range::RangeInclusive<T>>
    {
        Some(range::RangeInclusive { start: T::MIN, last: T::MAX })
    }
}

#[derive(Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mask<T: MaskElement> { fences: Vec<T> }

impl<T: MaskElement> Mask<T>
{
    pub fn empty() -> Self { Self { fences: vec![] } }

    pub fn full() -> Self { Self { fences: vec![T::MIN] } }

    pub fn get(&self, value: &T) -> bool
    {
        self.fences.partition_point(|x| x <= value) % 2 == 1
    }

    #[inline]
    pub fn set(&mut self, range: impl IntoMaskRange<T>, state: bool)
    {
        let Some(range) = range.into_mask_range() else { return };
        self.set_impl(range.start, range.last, state);
    }

    pub fn invert(&mut self)
    {
        match self.fences.first()
        {
            Some(x) if *x == T::MIN => drop(self.fences.remove(0)),
            _ => self.fences.insert(0, T::MIN),
        }
    }

    pub fn count(self) -> usize
    where
        T: ops::Sub,
        usize: AddAssign<T::Output>,
    {
        let mut counter = 0;

        let mut i = 0;

        while i < self.fences.len()
        {
            counter += self.fences[i].clone()
                - if i + 1 == self.fences.len() { T::MAX } else { self.fences[i + 1].clone() };

            AddAssign::<usize>::add_assign(&mut i, 2);
        }

        counter
    }

    fn set_impl(&mut self, start: T, last: T, state: bool)
    {
        assert!(start <= last);

        match last.checked_after()
        {
            Some(end) =>
            {
                let start_index = self.fences.partition_point(|x| *x <= start);
                let end_index = self.fences.partition_point(|x| *x <= end);

                match ((start_index % 2 == 1) ^ state, (end_index % 2 == 1) ^ state)
                {
                    (false, false) => drop(self.fences.drain(start_index..end_index)),
                    (false, true) => drop(self.fences.splice(start_index..end_index, [end])),
                    (true, false) => drop(self.fences.splice(start_index..end_index, [start])),
                    (true, true) => drop(self.fences.splice(start_index..end_index, [start, end])),
                }
            },
            None =>
            {
                let start_index = self.fences.partition_point(|x| *x <= start);

                match (start_index % 2 == 1) ^ state
                {
                    false => drop(self.fences.drain(start_index..)),
                    true => drop(self.fences.splice(start_index.., [start])),
                }
            },
        }

        debug_assert!(self.fences.array_windows().all(|[a, b]| a < b))
    }
}

impl<T: MaskElement, A: IntoMaskRange<T>> Extend<A> for Mask<T>
{
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I)
    {
        for range in iter { self.set(range, true) }
    }
}

impl<T: MaskElement, A: IntoMaskRange<T>> FromIterator<A> for Mask<T>
{
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self
    {
        let mut x = Self::empty();
        x.extend(iter);
        x
    }
}

impl<T: MaskElement + fmt::Debug> fmt::Debug for Mask<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut f = f.debug_list();
        let mut iter = self.fences.iter();
        while let Some(start) = iter.next()
        {
            if *start == T::MIN
            {
                if let Some(end) = iter.next()
                {
                    if *start == (*end).clone().checked_before().unwrap()
                    {
                        f.entry(&start);
                    }
                    else
                    {
                        f.entry(&format_args!("..{:?}", end));
                    }
                }
                else
                {
                    f.entry(&"..");
                }
            }
            else
            {
                if let Some(end) = iter.next()
                {
                    if *start == (*end).clone().checked_before().unwrap()
                    {
                        f.entry(&start);
                    }
                    else
                    {
                        f.entry(&format_args!("{:?}..{:?}", start, end));
                    }
                }
                else
                {
                    f.entry(&format_args!("{:?}..", start));
                }
            }
        }

        f.finish()
    }
}

#[derive(Clone)]
pub struct RangeIter<'a, T: MaskElement>
{
    inner: std::slice::Iter<'a, T>,
}

impl<T: MaskElement> Iterator for RangeIter<'_, T>
{
    type Item = range::RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.inner.next()
        {
            None => None,
            Some(start) => match self.inner.next()
            {
                None => Some(range::RangeInclusive
                {
                    start: start.clone(),
                    last: T::MAX,
                }),
                Some(end) => Some(range::RangeInclusive
                {
                    start: start.clone(),
                    last: end.clone().checked_before().unwrap(),
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn print()
    {
        for x in 240u8..=252u8
        {
            let mut mask = Mask { fences: vec![1, 6, 9, 10] };
            let before = mask.clone();
            let range = x..=x + 3;
            mask.set(range.clone(), true);
            println!("{:?} U {:?} = {:?}", before, range, mask);
        }
    }
}