#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ESR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `IMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMBR {
    #[doc = "If ESR2[VPS] is asserted, the ESR2[LPTM] is not an inactive Mailbox."] _0,
    #[doc = "If ESR2[VPS] is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    _1,
}
impl IMBR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IMBR::_0 => false,
            IMBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IMBR {
        match value {
            false => IMBR::_0,
            true => IMBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IMBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IMBR::_1
    }
}
#[doc = "Possible values of the field `VPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPSR {
    #[doc = "Contents of IMB and LPTM are invalid."] _0,
    #[doc = "Contents of IMB and LPTM are valid."] _1,
}
impl VPSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            VPSR::_0 => false,
            VPSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VPSR {
        match value {
            false => VPSR::_0,
            true => VPSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VPSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VPSR::_1
    }
}
#[doc = r" Value of the field"]
pub struct LPTMR {
    bits: u8,
}
impl LPTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 13 - Inactive Mailbox"]
    #[inline]
    pub fn imb(&self) -> IMBR {
        IMBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Valid Priority Status"]
    #[inline]
    pub fn vps(&self) -> VPSR {
        VPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:22 - Lowest Priority Tx Mailbox"]
    #[inline]
    pub fn lptm(&self) -> LPTMR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPTMR { bits }
    }
}
