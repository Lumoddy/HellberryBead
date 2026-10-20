use core::ops;

use arduino_hal::{adc, port};

use super::{DiDoAiPin, DiDoPin, Interactable, ReservedPin, PinId};

pub struct Collection {
    pub d0: ReservedPin<port::D0>,
    pub d1: ReservedPin<port::D1>,
    pub d2: DiDoPin<port::D2>,
    pub d3: DiDoPin<port::D3>,
    pub d4: DiDoPin<port::D4>,
    pub d5: DiDoPin<port::D5>,
    pub d6: DiDoPin<port::D6>,
    pub d7: DiDoPin<port::D7>,
    pub d8: DiDoPin<port::D8>,
    pub d9: DiDoPin<port::D9>,
    pub d10: DiDoPin<port::D10>,
    pub d11: DiDoPin<port::D11>,
    pub d12: DiDoPin<port::D12>,
    pub d13: DiDoPin<port::D13>,
    pub a0: DiDoAiPin<port::A0>,
    pub a1: DiDoAiPin<port::A1>,
    pub a2: DiDoAiPin<port::A2>,
    pub a3: DiDoAiPin<port::A3>,
    pub a4: DiDoAiPin<port::A4>,
    pub a5: DiDoAiPin<port::A5>,
}

macro_rules! collect_pins {
    ($pins:ident) => {
        $crate::pin::Collection
        {
            d0: $crate::pin::ReservedPin::new(),
            d1: $crate::pin::ReservedPin::new(),
            d2: $crate::pin::DiDoPin::from($pins.d2),
            d3: $crate::pin::DiDoPin::from($pins.d3),
            d4: $crate::pin::DiDoPin::from($pins.d4),
            d5: $crate::pin::DiDoPin::from($pins.d5),
            d6: $crate::pin::DiDoPin::from($pins.d6),
            d7: $crate::pin::DiDoPin::from($pins.d7),
            d8: $crate::pin::DiDoPin::from($pins.d8),
            d9: $crate::pin::DiDoPin::from($pins.d9),
            d10: $crate::pin::DiDoPin::from($pins.d10),
            d11: $crate::pin::DiDoPin::from($pins.d11),
            d12: $crate::pin::DiDoPin::from($pins.d12),
            d13: $crate::pin::DiDoPin::from($pins.d13),
            a0: $crate::pin::DiDoAiPin::from($pins.a0),
            a1: $crate::pin::DiDoAiPin::from($pins.a1),
            a2: $crate::pin::DiDoAiPin::from($pins.a2),
            a3: $crate::pin::DiDoAiPin::from($pins.a3),
            a4: $crate::pin::DiDoAiPin::from($pins.a4),
            a5: $crate::pin::DiDoAiPin::from($pins.a5),
        }
    };
}

pub(crate) use collect_pins;

impl ops::Index<PinId> for Collection {
    type Output = dyn Interactable<adc::Adc>;

    fn index(&self, index: PinId) -> &Self::Output
    {
        match index {
            PinId::D0 => &self.d0,
            PinId::D1 => &self.d1,
            PinId::D2 => &self.d2,
            PinId::D3 => &self.d3,
            PinId::D4 => &self.d4,
            PinId::D5 => &self.d5,
            PinId::D6 => &self.d6,
            PinId::D7 => &self.d7,
            PinId::D8 => &self.d8,
            PinId::D9 => &self.d9,
            PinId::D10 => &self.d10,
            PinId::D11 => &self.d11,
            PinId::D12 => &self.d12,
            PinId::D13 => &self.d13,
            PinId::A0 => &self.a0,
            PinId::A1 => &self.a1,
            PinId::A2 => &self.a2,
            PinId::A3 => &self.a3,
            PinId::A4 => &self.a4,
            PinId::A5 => &self.a5,
        }
    }
}

impl ops::IndexMut<PinId> for Collection {
    fn index_mut(&mut self, index: PinId) -> &mut Self::Output
    {
        match index {
            PinId::D0 => &mut self.d0,
            PinId::D1 => &mut self.d1,
            PinId::D2 => &mut self.d2,
            PinId::D3 => &mut self.d3,
            PinId::D4 => &mut self.d4,
            PinId::D5 => &mut self.d5,
            PinId::D6 => &mut self.d6,
            PinId::D7 => &mut self.d7,
            PinId::D8 => &mut self.d8,
            PinId::D9 => &mut self.d9,
            PinId::D10 => &mut self.d10,
            PinId::D11 => &mut self.d11,
            PinId::D12 => &mut self.d12,
            PinId::D13 => &mut self.d13,
            PinId::A0 => &mut self.a0,
            PinId::A1 => &mut self.a1,
            PinId::A2 => &mut self.a2,
            PinId::A3 => &mut self.a3,
            PinId::A4 => &mut self.a4,
            PinId::A5 => &mut self.a5,
        }
    }
}

pub struct CollectionIterMut<'a> {
    source: &'a mut Collection,
    index: Option<PinId>,
}

impl<'a> Iterator for CollectionIterMut<'a> {
    type Item = &'a mut dyn Interactable<adc::Adc>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe fn transmute_lifetime<'a, 'b, T: ?Sized>(value: &'a mut T) -> &'b mut T {
            unsafe { &mut *(value as *mut T) }
        }

        // SAFETY: None of the borrows overlap like `core::slice::IterMut`.
        unsafe {
            match self.index? {
                PinId::A0 => {
                    self.index = Some(PinId::A1);
                    Some(transmute_lifetime(&mut self.source.a0))
                },
                PinId::A1 => {
                    self.index = Some(PinId::A2);
                    Some(transmute_lifetime(&mut self.source.a1))
                },
                PinId::A2 => {
                    self.index = Some(PinId::A3);
                    Some(transmute_lifetime(&mut self.source.a2))
                },
                PinId::A3 => {
                    self.index = Some(PinId::A4);
                    Some(transmute_lifetime(&mut self.source.a3))
                },
                PinId::A4 => {
                    self.index = Some(PinId::A5);
                    Some(transmute_lifetime(&mut self.source.a4))
                },
                PinId::A5 => {
                    self.index = Some(PinId::D0);
                    Some(transmute_lifetime(&mut self.source.a5))
                },
                PinId::D0 => {
                    self.index = Some(PinId::D1);
                    Some(transmute_lifetime(&mut self.source.d0))
                },
                PinId::D1 => {
                    self.index = Some(PinId::D2);
                    Some(transmute_lifetime(&mut self.source.d1))
                },
                PinId::D2 => {
                    self.index = Some(PinId::D3);
                    Some(transmute_lifetime(&mut self.source.d2))
                },
                PinId::D3 => {
                    self.index = Some(PinId::D4);
                    Some(transmute_lifetime(&mut self.source.d3))
                },
                PinId::D4 => {
                    self.index = Some(PinId::D5);
                    Some(transmute_lifetime(&mut self.source.d4))
                },
                PinId::D5 => {
                    self.index = Some(PinId::D6);
                    Some(transmute_lifetime(&mut self.source.d5))
                },
                PinId::D6 => {
                    self.index = Some(PinId::D7);
                    Some(transmute_lifetime(&mut self.source.d6))
                },
                PinId::D7 => {
                    self.index = Some(PinId::D8);
                    Some(transmute_lifetime(&mut self.source.d7))
                },
                PinId::D8 => {
                    self.index = Some(PinId::D9);
                    Some(transmute_lifetime(&mut self.source.d8))
                },
                PinId::D9 => {
                    self.index = Some(PinId::D10);
                    Some(transmute_lifetime(&mut self.source.d9))
                },
                PinId::D10 => {
                    self.index = Some(PinId::D11);
                    Some(transmute_lifetime(&mut self.source.d10))
                },
                PinId::D11 => {
                    self.index = Some(PinId::D12);
                    Some(transmute_lifetime(&mut self.source.d11))
                },
                PinId::D12 => {
                    self.index = Some(PinId::D13);
                    Some(transmute_lifetime(&mut self.source.d12))
                },
                PinId::D13 => {
                    self.index = None;
                    Some(transmute_lifetime(&mut self.source.d13))
                },
            }
        }
    }
}