use bitflags::bitflags;
use ufmt::{uDisplay, uWrite, uwrite};

use super::PinMode;

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct PinFlags: u8 {
        const DIGITAL_INPUT = 1 << 0;
        const DIGITAL_OUTPUT = 1 << 1;
        const ANALOG_INPUT = 1 << 2;
        const ANALOG_OUTPUT = 1 << 3;
        const RESERVED = 1 << 4;
    }
}

impl uDisplay for PinFlags {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized {
        if self.is_empty() {
            uwrite!(f, "0")
        } else {
            let mut is_first = true;
            if self.contains(PinFlags::DIGITAL_INPUT) {
                if !is_first {
                    uwrite!(f, " | ")?;
                }
                uwrite!(f, "digitalInput")?;
                is_first = false;
            }
            if self.contains(PinFlags::DIGITAL_OUTPUT) {
                if !is_first {
                    uwrite!(f, " | ")?;
                }
                uwrite!(f, "digitalOutput")?;
                is_first = false;
            }
            if self.contains(PinFlags::ANALOG_INPUT) {
                if !is_first {
                    uwrite!(f, " | ")?;
                }
                uwrite!(f, "analogInput")?;
                is_first = false;
            }
            if self.contains(PinFlags::ANALOG_OUTPUT) {
                if !is_first {
                    uwrite!(f, " | ")?;
                }
                uwrite!(f, "analogOutput")?;
                is_first = false;
            }
            if self.contains(PinFlags::RESERVED) {
                if !is_first {
                    uwrite!(f, " | ")?;
                }
                uwrite!(f, "reserved")?;
            }
            Ok(())
        }
    }
}

impl From<PinMode> for PinFlags {
    fn from(value: PinMode) -> Self {
        match value {
            PinMode::Ignored => PinFlags::empty(),
            PinMode::DigitalInput => PinFlags::DIGITAL_INPUT,
            PinMode::DigitalOutput => PinFlags::DIGITAL_OUTPUT,
            PinMode::AnalogInput => PinFlags::ANALOG_INPUT,
            PinMode::AnalogOutput => PinFlags::ANALOG_OUTPUT,
        }
    }
}