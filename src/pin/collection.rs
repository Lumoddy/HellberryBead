use core::ops;

use arduino_hal::adc;

use super::{Interactable, PinId};

use ufmt::{uDisplay, uWrite, uwrite};

macro_rules! collection_impl {
    (
        $(
            $field:ident: $pin:ident::<$port:ident>::$($path:ident)::+ $(($($param:tt)*))?
        ),*$(,)?
    ) => {
        pub struct Collection {
            $(pub $field: $crate::pin::$pin::<::arduino_hal::port::$port>,)*
        }

        macro_rules! collect_pins {
            ($pins:ident) => {{
                macro_rules! pins { () => { $pins }; }

                $crate::pin::Collection
                {
                    $(
                        $field: $crate::pin::$pin::<::arduino_hal::port::$port>
                            ::$($path)::+ $(($($param)*))?,
                    )*
                }
            }};
        }

        impl ops::Index<PinId> for Collection {
            type Output = dyn Interactable<adc::Adc>;

            fn index(&self, index: PinId) -> &Self::Output
            {
                match index {
                    $($crate::pin::PinId::$port => &self.$field,)*
                }
            }
        }

        impl ops::IndexMut<PinId> for Collection {
            fn index_mut(&mut self, index: PinId) -> &mut Self::Output
            {
                match index {
                    $($crate::pin::PinId::$port => &mut self.$field,)*
                }
            }
        }

        pub struct CollectionIter<'a> {
            source: &'a Collection,
            index: Option<PinId>,
        }

        impl<'a> Iterator for CollectionIter<'a> {
            type Item = &'a dyn Interactable<adc::Adc>;

            fn next(&mut self) -> Option<Self::Item> {
                unsafe fn transmute_lifetime<'a, 'b, T: ?Sized>(value: &'a T) -> &'b T {
                    unsafe { &*(value as *const T) }
                }

                let index = self.index?;
                self.index = index.next();

                // SAFETY: None of the borrows overlap like `core::slice::IterMut`.
                unsafe {
                    match index {
                        $($crate::pin::PinId::$port => Some(transmute_lifetime(&self.source.$field)),)*
                    }
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

                let index = self.index?;
                self.index = index.next();

                // SAFETY: None of the borrows overlap like `core::slice::IterMut`.
                unsafe {
                    match index {
                        $($crate::pin::PinId::$port => Some(transmute_lifetime(&mut self.source.$field)),)*
                    }
                }
            }
        }

        impl uDisplay for Collection {
            fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized {
                uwrite!(f, "{{ model: \"Arduino Uno\", pins: [")?;
                let mut first = true;
                for pin in self {
                    if first {
                        first = false;
                    } else {
                        uwrite!(f, ", ")?;
                    }
                    uwrite!(f, "new Pin(\"{}\", {})", pin.id(), pin.flags())?;
                }
                uwrite!(f, "] }}")?;
                Ok(())
            }
        }
    };
}

collection_impl! {
    d0: ReservedPin::<D0>::new(),
    d1: ReservedPin::<D1>::new(),
    d2: DiDoPin::<D2>::from(pins!().d2),
    d3: DiDoPin::<D3>::from(pins!().d3),
    d4: DiDoPin::<D4>::from(pins!().d4),
    d5: DiDoPin::<D5>::from(pins!().d5),
    d6: DiDoPin::<D6>::from(pins!().d6),
    d7: DiDoPin::<D7>::from(pins!().d7),
    d8: DiDoPin::<D8>::from(pins!().d8),
    d9: DiDoPin::<D9>::from(pins!().d9),
    d10: DiDoPin::<D10>::from(pins!().d10),
    d11: DiDoPin::<D11>::from(pins!().d11),
    d12: DiDoPin::<D12>::from(pins!().d12),
    d13: DiDoPin::<D13>::from(pins!().d13),
    a0: DiDoAiPin::<A0>::from(pins!().a0),
    a1: DiDoAiPin::<A1>::from(pins!().a1),
    a2: DiDoAiPin::<A2>::from(pins!().a2),
    a3: DiDoAiPin::<A3>::from(pins!().a3),
    a4: DiDoAiPin::<A4>::from(pins!().a4),
    a5: DiDoAiPin::<A5>::from(pins!().a5),
}

pub(crate) use collect_pins;

impl Collection {
    pub const fn iter(&self) -> CollectionIter<'_> {
        CollectionIter {
            source: self,
            index: Some(PinId::first()),
        }
    }

    pub const fn iter_mut(&mut self) -> CollectionIterMut<'_> {
        CollectionIterMut {
            source: self,
            index: Some(PinId::first()),
        }
    }
}

impl<'a> IntoIterator for &'a Collection {
    type Item = &'a dyn Interactable<adc::Adc>;

    type IntoIter = CollectionIter<'a>;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a> IntoIterator for &'a mut Collection {
    type Item = &'a mut dyn Interactable<adc::Adc>;

    type IntoIter = CollectionIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}