pub trait UsePartialEndianness<const N: usize, T>
{
    fn from_bytes<Endian: PartialEndianness<N, T>>(bytes: [u8; N]) -> Self;

    fn to_bytes<Endian: PartialEndianness<N, T>>(self) -> [u8; N];
}

pub trait PartialEndianness<const N: usize, T>
{
    fn from_bytes(bytes: [u8; N]) -> T;

    fn to_bytes(value: T) -> [u8; N];
}

pub trait Endianness
where
    Self: PartialEndianness<{ size_of::<u8>() }, u8>,
    Self: PartialEndianness<{ size_of::<i8>() }, i8>,
    Self: PartialEndianness<{ size_of::<u16>() }, u16>,
    Self: PartialEndianness<{ size_of::<i16>() }, i16>,
    Self: PartialEndianness<{ size_of::<u32>() }, u32>,
    Self: PartialEndianness<{ size_of::<i32>() }, i32>,
    Self: PartialEndianness<{ size_of::<u64>() }, u64>,
    Self: PartialEndianness<{ size_of::<i64>() }, i64>,
    Self: PartialEndianness<{ size_of::<u128>() }, u128>,
    Self: PartialEndianness<{ size_of::<i128>() }, i128>,
    Self: PartialEndianness<{ size_of::<usize>() }, usize>,
    Self: PartialEndianness<{ size_of::<isize>() }, isize> { }

impl<T> Endianness for T
where
    T: PartialEndianness<{ size_of::<u8>() }, u8>,
    T: PartialEndianness<{ size_of::<i8>() }, i8>,
    T: PartialEndianness<{ size_of::<u16>() }, u16>,
    T: PartialEndianness<{ size_of::<i16>() }, i16>,
    T: PartialEndianness<{ size_of::<u32>() }, u32>,
    T: PartialEndianness<{ size_of::<i32>() }, i32>,
    T: PartialEndianness<{ size_of::<u64>() }, u64>,
    T: PartialEndianness<{ size_of::<i64>() }, i64>,
    T: PartialEndianness<{ size_of::<u128>() }, u128>,
    T: PartialEndianness<{ size_of::<i128>() }, i128>,
    T: PartialEndianness<{ size_of::<usize>() }, usize>,
    T: PartialEndianness<{ size_of::<isize>() }, isize> { }

macro_rules! endianness_impl
{
    ( $($ty:ident)+ ) =>
    {
        $(
            impl PartialEndianness<{ size_of::<$ty>() }, $ty> for LittleEndianness
            {
                #[inline(always)]
                fn from_bytes(bytes: [u8; size_of::<$ty>()]) -> $ty
                {
                    $ty::from_le_bytes(bytes)
                }

                #[inline(always)]
                fn to_bytes(value: $ty) -> [u8; size_of::<$ty>()]
                {
                    value.to_le_bytes()
                }
            }

            impl PartialEndianness<{ size_of::<$ty>() }, $ty> for BigEndianness
            {
                #[inline(always)]
                fn from_bytes(bytes: [u8; size_of::<$ty>()]) -> $ty
                {
                    $ty::from_be_bytes(bytes)
                }

                #[inline(always)]
                fn to_bytes(value: $ty) -> [u8; size_of::<$ty>()]
                {
                    value.to_be_bytes()
                }
            }

            impl UsePartialEndianness<{ size_of::<$ty>() }, $ty> for $ty
            {
                #[inline(always)]
                fn from_bytes<Endian>(bytes: [u8; size_of::<$ty>()]) -> Self
                where
                    Endian: PartialEndianness<{ size_of::<$ty>() }, $ty>
                {
                    Endian::from_bytes(bytes)
                }

                #[inline(always)]
                fn to_bytes<Endian>(self) -> [u8; size_of::<$ty>()]
                where
                    Endian: PartialEndianness<{ size_of::<$ty>() }, $ty>
                {
                    Endian::to_bytes(self)
                }
            }
        )+
    };
}

endianness_impl!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize);

pub enum LittleEndianness { }

pub enum BigEndianness { }