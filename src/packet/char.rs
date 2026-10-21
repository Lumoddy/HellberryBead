use arduino_hal::prelude::{_embedded_hal_serial_Read as Read, _embedded_hal_serial_Write as Write};

use super::{PacketDeserialize, PacketSerialize};

pub trait ConstChar: Clone + Copy + Default {
    const SLICE: &'static [char];
    const UTF8: &'static str;
    const UTF16: &'static [u16];
}

macro_rules! char_impl
{
    ( @ignore $( $tt:tt )* ) => { };

    (
        $dollar:tt
        $($name:ident<$($c:ident $(,)?)*>),*$(,)?
    ) => {
        macro_rules! Char {
            $(
                ( $( $dollar $c :expr ),* $dollar (,)? ) => {
                    $crate ::packet:: $name < $( $dollar $c ),* >
                };
            )*
        }

        $(
            /// A statically known str represented as a zero-sized-type.
            #[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
            pub struct $name<$(const $c: char,)*>(());

            impl<$(const $c: char,)*> $name<$($c,)*> {
                $(pub const $c: char = $c;)*

                const ARRAY: [char; 0 $( + { char_impl!(@ignore $c); 1 } )* ] = [$($c,)*];

                #[allow(unused_mut)]
                const UTF8_BUFFER: ([u8; 0 $( + { char_impl!(@ignore $c); 4 } )* ], usize) = {
                    let mut buffer = [0; _];
                    let mut i = 0;
                    let mut cursor = 0;
                    while i < Self::ARRAY.len() {
                        // SAFETY: Mimics normal slicing.
                        let slice = unsafe {
                            assert!(i < buffer.len());
                            core::slice::from_raw_parts_mut(buffer.as_mut_ptr().add(cursor), buffer.len() - cursor)
                        };
                        cursor += char::encode_utf8(Self::ARRAY[i], slice).len();
                        i += 1;
                    }

                    (buffer, cursor)
                };

                #[allow(unused_mut)]
                const UTF16_BUFFER: ([u16; 0 $( + { char_impl!(@ignore $c ); 2 } )* ], usize) = {
                    let mut buffer = [0; _];
                    let mut i = 0;
                    let mut cursor = 0;
                    while i < Self::ARRAY.len() {
                        // SAFETY: Mimics normal slicing.
                        let slice = unsafe {
                            assert!(i < buffer.len());
                            core::slice::from_raw_parts_mut(buffer.as_mut_ptr().add(cursor), buffer.len() - cursor)
                        };
                        cursor += char::encode_utf16(Self::ARRAY[i], slice).len();
                        i += 1;
                    }

                    (buffer, cursor)
                };

                const IS_WORD: bool = 'to_result: {
                    let mut i = 0;
                    while i < Self::ARRAY.len() {
                        if !matches!(Self::ARRAY[i], 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                            break 'to_result false;
                        }
                        i += 1;
                    }
                    i > 0
                };

                #[must_use]
                pub const fn new() -> Self { Self(()) }

                #[must_use]
                pub const fn as_str(&self) -> &'static str { Self::UTF8 }

                #[must_use]
                pub const fn as_slice(&self) -> &'static [char] { &Self::ARRAY }

                #[must_use]
                pub const fn as_utf16(&self) -> &'static [u16] { Self::UTF16 }

                #[must_use]
                pub const fn into_array(self) -> [char; 0 $( + { char_impl!(@ignore $c); 1 } )* ] { Self::ARRAY }
            }

            impl<$(const $c: char,)*> ConstChar for $name<$($c,)*> {
                const SLICE: &'static [char] = Self::ARRAY.as_slice();

                const UTF8: &'static str = {
                    // SAFETY: The buffer is filled with valid UTF-8 bytes, and
                    // the length can never be larger than the buffer's.
                    unsafe {
                        let slice = core::slice::from_raw_parts(Self::UTF8_BUFFER.0.as_ptr(), Self::UTF8_BUFFER.1);
                        str::from_utf8_unchecked(slice)
                    }
                };

                const UTF16: &'static [u16] = {
                    // SAFETY: The length can never be larger than the buffer's.
                    unsafe {
                        core::slice::from_raw_parts(Self::UTF16_BUFFER.0.as_ptr(), Self::UTF16_BUFFER.1)
                    }
                };
            }

            impl<$(const $c: char,)*> From<$name<$($c,)*>>
                for [char; 0 $( + { char_impl!(@ignore $c ); 1 } )*] {
                fn from(_: $name < $( $c ,)* >) -> Self { $name::<$($c,)*>::ARRAY }
            }

            impl<$(const $c: char,)*> From<$name<$($c,)*>> for &[char] {
                fn from(_: $name < $( $c ,)* >) -> Self { $name::<$($c,)*>::SLICE }
            }

            impl<$(const $c: char,)*> AsRef<[char]> for $name<$($c,)*> {
                fn as_ref(&self) -> &[char] { Self::SLICE }
            }

            impl<$(const $c: char,)*> From<$name<$($c,)*>> for &str {
                fn from(_: $name < $( $c ,)* >) -> Self { $name ::< $( $c ,)* >::UTF8 }
            }

            impl<$(const $c: char,)*> AsRef<str> for $name <$($c,)*> {
                fn as_ref(&self) -> &str { Self::UTF8 }
            }

            impl<$(const $c: char,)*> ufmt::uDisplay for $name<$($c,)*> {
                fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
                where
                    W: ufmt::uWrite + ?Sized, {
                    ufmt::uDisplay::fmt(self.as_str(), f)
                }
            }

            impl<$(const $c: char,)*>
                TryFrom<[char; 0 $( + { char_impl!(@ignore $c ); 1 } )* ]>
                for $name < $( $c ,)* > {
                type Error = ();

                fn try_from(value: [char; 0 $( + { char_impl!(@ignore $c ); 1 } )* ]) -> Result<Self, Self::Error>
                {
                    if value == Self::ARRAY { Ok(Self(())) }
                    else { Err(()) }
                }
            }

            impl<$(const $c: char,)*> TryFrom<&[char]> for $name<$($c,)*> {
                type Error = ();

                fn try_from(value: &[char]) -> Result<Self, Self::Error>
                {
                    if value == Self::SLICE { Ok(Self(())) }
                    else { Err(()) }
                }
            }

            impl<$(const $c: char,)*> TryFrom<&str> for $name<$($c,)*> {
                type Error = ();

                fn try_from(value: &str) -> Result<Self, Self::Error>
                {
                    if value == Self::UTF8 { Ok(Self(())) }
                    else { Err(()) }
                }
            }

            impl<$(const $c: char,)*> PacketSerialize for $name<$($c,)*> {
                fn serialize<W: Write<u8>>(&self, writer: &mut W, _: bool) -> Result<(), W::Error> {
                    for byte in self.as_str().bytes() {
                        nb::block!(writer.write(byte))?;
                    }
                    Ok(())
                }
            }

            impl<$(const $c: char,)*> PacketDeserialize for $name<$($c,)*> {
                type Error = ();

                fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
                    -> Result<Result<Self, Self::Error>, R::Error> {
                    let mut first = match peek.take() {
                        Some(x) => x,
                        None => nb::block!(reader.read())?,
                    };

                    if whitespace {
                        while let b' ' | b'\r' | b'\n' = first {
                            first = nb::block!(reader.read())?;
                        }
                    }

                    'to_complete: {
                        let mut bytes = Self::UTF8.bytes();

                        match bytes.next() {
                            None => break 'to_complete,
                            Some(x) if x == first => { },
                            Some(byte) => {
                                *peek = Some(byte);
                                return Ok(Err(()));
                            },
                        }

                        for byte in bytes {
                            if nb::block!(reader.read())? != byte {
                                *peek = Some(byte);
                                return Ok(Err(()));
                            }
                        }
                    }

                    if !Self::IS_WORD {
                        return Ok(Ok(Self::new()));
                    }
                    match nb::block!(reader.read())? {
                        b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' => return Ok(Err(())),
                        byte => {
                            *peek = Some(byte);
                            return Ok(Ok(Self::new()));
                        },
                    }
                }
            }

            impl<$(const $c: char,)*> PacketSerialize for Option<$name<$($c,)*>> {
                fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
                    match self {
                        Some(x) => x.serialize(writer, whitespace),
                        None => Ok(()),
                    }
                }
            }

            impl<$(const $c: char,)*> PacketDeserialize for Option<$name<$($c,)*>> {
                type Error = ();

                fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
                    -> Result<Result<Self, Self::Error>, R::Error> {
                    let mut first = match peek.take() {
                        Some(x) => x,
                        None => nb::block!(reader.read())?,
                    };

                    if whitespace {
                        while let b' ' | b'\r' | b'\n' = first {
                            first = nb::block!(reader.read())?;
                        }
                    }

                    'to_complete: {
                        let mut bytes = $name::<$($c,)*>::UTF8.bytes();

                        match bytes.next() {
                            None => break 'to_complete,
                            Some(x) if x == first => { },
                            Some(byte) => {
                                *peek = Some(byte);
                                return Ok(Ok(None));
                            },
                        }

                        for byte in bytes {
                            if nb::block!(reader.read())? != byte {
                                *peek = Some(byte);
                                return Ok(Err(()));
                            }
                        }
                    }

                    if !$name::<$($c,)*>::IS_WORD {
                        return Ok(Ok(Some($name::<$($c,)*>::new())));
                    }
                    match nb::block!(reader.read())? {
                        b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' => return Ok(Err(())),
                        byte => {
                            *peek = Some(byte);
                            return Ok(Ok(Some($name::<$($c,)*>::new())));
                        },
                    }
                }
            }
        )*
    };
}

char_impl!
{
    $
    Char1<C0>,
    Char2<C0, C1>,
    Char3<C0, C1, C2>,
    Char4<C0, C1, C2, C3>,
    Char5<C0, C1, C2, C3, C4>,
    Char6<C0, C1, C2, C3, C4, C5>,
    Char7<C0, C1, C2, C3, C4, C5, C6>,
    Char8<C0, C1, C2, C3, C4, C5, C6, C7>,
    Char9<C0, C1, C2, C3, C4, C5, C6, C7, C8>,
    Char10<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9>,
    Char11<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10>,
    Char12<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11>,
    Char13<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12>,
    Char14<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13>,
    Char15<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14>,
    Char16<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15>,
    Char17<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16>,
    Char18<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17>,
    Char19<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18>,
    Char20<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19>,
    Char21<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20>,
    Char22<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21>,
    Char23<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22>,
    Char24<C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23>,
}

pub(crate) use Char;

impl<const C: char> From<Char![C]> for char
{
    fn from(_: Char![C]) -> Self { C }
}

impl<const C: char> AsRef<char> for Char![C]
{
    fn as_ref(&self) -> &char { &C }
}

impl<const C: char> TryFrom<char> for Char![C]
{
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error>
    {
        if value == C { Ok(Self(())) }
        else { Err(()) }
    }
}