#![no_std]

use core::ops;
use core::task::Poll;

pub trait Machine<T>
{
    type Buffer: ops::DerefMut<Target = [T]> + IntoIterator<IntoIter: ExactSizeIterator, Item = T>;

    type Output;

    fn step(&mut self, value: T) -> Poll<Result<(Self::Output, Self::Buffer), ()>>;
}