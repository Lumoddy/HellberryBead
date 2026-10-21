use arduino_hal::prelude::{_embedded_hal_serial_Read as Read, _embedded_hal_serial_Write as Write};
use nodyn::nodyn;

use super::{Char, PacketDeserialize, PacketSerialize};

macro_rules! impl_deserialize_literal {
    (
        @ $reader:ident, $peek:ident, read(),
        $($tt:tt)*
    ) => {
        {
            let byte = nb::block!($reader.read())?;
            impl_deserialize_literal!
            {
                @ $reader, $peek, byte,
                $($tt)*
            }
        }
    };
    (
        @ $reader:ident, $peek:ident, $byte:ident,
        { $($pat:pat => $($val:tt)::+ $(($($round:tt)*))? $({ $($curly:tt)* })?),*$(,)? }
    ) => {
        match $byte {
            $(
                $pat => impl_deserialize_literal!
                {
                    @ $reader, $peek, read(),
                    $($val)::+ $(($($round)*))? $({ $($curly)* })?
                },
            )*
            #[allow(unreachable_patterns)]
            byte => {
                *$peek = Some(byte);
                Err(())
            },
        }
    };
    (
        @ $reader:ident, $peek:ident, $byte:ident $(())?,
        $($tt:tt)*
    ) => {
        match $byte {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' => Err(()),
            byte => {
                *$peek = Some(byte);
                Ok($($tt)*)
            },
        }
    };
    (
        for $ty:ty { $($tt:tt)* }
    ) => {
        impl PacketDeserialize for $ty {
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

                Ok(impl_deserialize_literal! { @ reader, peek, first, { $($tt)* } })
            }
        }
    };
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralDot {
        Dot(Char!['.']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
    impl PacketDeserialize {
        type Error = ();
        fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
            -> Result<Result<Self, Self::Error>, R::Error> {
            Ok(<_>::deserialize(reader, peek, whitespace)?.map(Self::Dot))
        }
    }
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralOpen {
        Open(Char!['(']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
    impl PacketDeserialize {
        type Error = ();
        fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
            -> Result<Result<Self, Self::Error>, R::Error> {
            Ok(<_>::deserialize(reader, peek, whitespace)?.map(Self::Open))
        }
    }
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralClose {
        Close(Char![')']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
    impl PacketDeserialize {
        type Error = ();
        fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
            -> Result<Result<Self, Self::Error>, R::Error> {
            Ok(<_>::deserialize(reader, peek, whitespace)?.map(Self::Close))
        }
    }
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralComma {
        Comma(Char![',']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
    impl PacketDeserialize {
        type Error = ();
        fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
            -> Result<Result<Self, Self::Error>, R::Error> {
            Ok(<_>::deserialize(reader, peek, whitespace)?.map(Self::Comma))
        }
    }
}

impl PacketSerialize for Option<LiteralComma> {
    fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error> {
        self.map(|LiteralComma::Comma(x)| x).serialize(writer, whitespace)
    }
}
impl PacketDeserialize for Option<LiteralComma> {
    type Error = ();
    fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
        -> Result<Result<Self, Self::Error>, R::Error> {
        Ok(Option::deserialize(reader, peek, whitespace)?.map(|x| x.map(LiteralComma::Comma)))
    }
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralSemiColon {
        SemiColon(Char![';']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
    impl PacketDeserialize {
        type Error = ();
        fn deserialize<R: Read<u8>>(reader: &mut R, peek: &mut Option<u8>, whitespace: bool)
            -> Result<Result<Self, Self::Error>, R::Error> {
            Ok(<_>::deserialize(reader, peek, whitespace)?.map(Self::SemiColon))
        }
    }
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralPingOrPins {
        Ping(Char!['p', 'i', 'n', 'g']),
        Pins(Char!['p', 'i', 'n', 's']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
}
impl_deserialize_literal! {
    for LiteralPingOrPins {
        b'p' => {
            b'i' => {
                b'n' => {
                    b'g' => Self::Ping(<_>::default()),
                    b's' => Self::Pins(<_>::default()),
                },
            },
        },
    }
}

nodyn! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum LiteralPinFunctionName {
        GetPower(Char!['g', 'e', 't', 'P', 'o', 'w', 'e', 'r']),
        GetMode(Char!['g', 'e', 't', 'M', 'o', 'd', 'e']),
        Flags(Char!['f', 'l', 'a', 'g', 's']),
        Supports(Char!['s', 'u', 'p', 'p', 'o', 'r', 't', 's']),
        SetPower(Char!['s', 'e', 't', 'P', 'o', 'w', 'e', 'r']),
        SetMode(Char!['s', 'e', 't', 'M', 'o', 'd', 'e']),
    }
    impl PacketSerialize {
        fn serialize<W: Write<u8>>(&self, writer: &mut W, whitespace: bool) -> Result<(), W::Error>;
    }
}
impl_deserialize_literal! {
    for LiteralPinFunctionName {
        b'g' => {
            b'e' => {
                b't' => {
                    b'P' => {
                        b'o' => {
                            b'w' => {
                                b'e' => {
                                    b'r' => Self::GetPower(<_>::default()),
                                },
                            },
                        },
                    },
                    b'M' => {
                        b'o' => {
                            b'd' => {
                                b'e' => Self::GetMode(<_>::default()),
                            },
                        },
                    },
                },
            },
        },
        b'f' => {
            b'l' => {
                b'a' => {
                    b'g' => {
                        b's' => Self::Flags(<_>::default()),
                    },
                },
            },
        },
        b's' => {
            b'e' => {
                b't' => {
                    b'P' => {
                        b'o' => {
                            b'w' => {
                                b'e' => {
                                    b'r' => Self::SetPower(<_>::default()),
                                },
                            },
                        },
                    },
                    b'M' => {
                        b'o' => {
                            b'd' => {
                                b'e' => Self::SetMode(<_>::default()),
                            },
                        },
                    },
                },
            },
            b'u' => {
                b'p' => {
                    b'p' => {
                        b'o' => {
                            b'r' => {
                                b't' => {
                                    b's' => Self::Supports(<_>::default()),
                                },
                            },
                        },
                    },
                },
            },
        },
    }
}