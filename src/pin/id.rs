use arduino_hal::hal::port;

pub(super) trait ConstPinId {
    const ID: PinId;
}

macro_rules! impl_id {
    ($($id:ident($port:ty) = $name:literal),+$(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum PinId {
            $( $id ,)+
        }

        impl PinId {
            pub const fn name(&self) -> &'static str {
                match self {
                    $(Self::$id => $name,)+
                }
            }
        }

        impl TryFrom<&[u8]> for PinId {
            type Error = ();

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                $( if value == $name.as_bytes() { Ok(Self::$id) } else )+
                { Err(()) }
            }
        }

        impl TryFrom<&str> for PinId {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.as_bytes().try_into()
            }
        }

        $(
            impl ConstPinId for $port {
                const ID: PinId = PinId::$id;
            }
        )*
    };
}

impl_id! {
    D0(port::PD0) = "d0",
    D1(port::PD1) = "d1",
    D2(port::PD2) = "d2",
    D3(port::PD3) = "d3",
    D4(port::PD4) = "d4",
    D5(port::PD5) = "d5",
    D6(port::PD6) = "d6",
    D7(port::PD7) = "d7",
    D8(port::PB0) = "d8",
    D9(port::PB1) = "d9",
    D10(port::PB2) = "d10",
    D11(port::PB3) = "d11",
    D12(port::PB4) = "d12",
    D13(port::PB5) = "d13",
    A0(port::PC0) = "a0",
    A1(port::PC1) = "a1",
    A2(port::PC2) = "a2",
    A3(port::PC3) = "a3",
    A4(port::PC4) = "a4",
    A5(port::PC5) = "a5",
}

impl PinId {
    pub const fn first() -> Self { Self::D0 }

    pub const fn next(self) -> Option<Self> {
        match self {
            Self::D0 => Some(Self::D1),
            Self::D1 => Some(Self::D2),
            Self::D2 => Some(Self::D3),
            Self::D3 => Some(Self::D4),
            Self::D4 => Some(Self::D5),
            Self::D5 => Some(Self::D6),
            Self::D6 => Some(Self::D7),
            Self::D7 => Some(Self::D8),
            Self::D8 => Some(Self::D9),
            Self::D9 => Some(Self::D10),
            Self::D10 => Some(Self::D11),
            Self::D11 => Some(Self::D12),
            Self::D12 => Some(Self::D13),
            Self::D13 => Some(Self::A0),
            Self::A0 => Some(Self::A1),
            Self::A1 => Some(Self::A2),
            Self::A2 => Some(Self::A3),
            Self::A3 => Some(Self::A4),
            Self::A4 => Some(Self::A5),
            Self::A5 => None,
        }
    }
}
