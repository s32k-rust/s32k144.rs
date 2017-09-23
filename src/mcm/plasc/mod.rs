#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
impl super::PLASC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ASC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASCR {
    #[doc = "A bus slave connection to AXBS input port n is absent"] _0,
    #[doc = "A bus slave connection to AXBS input port n is present"] _1,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl ASCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASCR::_0 => 0,
            ASCR::_1 => 1,
            ASCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASCR {
        match value {
            0 => ASCR::_0,
            1 => ASCR::_1,
            i => ASCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASCR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:7 - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
    #[inline]
    pub fn asc(&self) -> ASCR {
        ASCR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
