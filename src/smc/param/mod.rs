#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PARAM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `EHSRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EHSRUNR {
    #[doc = "The feature is not available."]
    _0,
    #[doc = "The feature is available."]
    _1,
}
impl EHSRUNR {
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
            EHSRUNR::_0 => false,
            EHSRUNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EHSRUNR {
        match value {
            false => EHSRUNR::_0,
            true => EHSRUNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EHSRUNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EHSRUNR::_1
    }
}
#[doc = "Possible values of the field `ELLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELLSR {
    #[doc = "The feature is not available."]
    _0,
    #[doc = "The feature is available."]
    _1,
}
impl ELLSR {
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
            ELLSR::_0 => false,
            ELLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELLSR {
        match value {
            false => ELLSR::_0,
            true => ELLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ELLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ELLSR::_1
    }
}
#[doc = "Possible values of the field `ELLS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELLS2R {
    #[doc = "The feature is not available."]
    _0,
    #[doc = "The feature is available."]
    _1,
}
impl ELLS2R {
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
            ELLS2R::_0 => false,
            ELLS2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELLS2R {
        match value {
            false => ELLS2R::_0,
            true => ELLS2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ELLS2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ELLS2R::_1
    }
}
#[doc = "Possible values of the field `EVLLS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVLLS0R {
    #[doc = "The feature is not available."]
    _0,
    #[doc = "The feature is available."]
    _1,
}
impl EVLLS0R {
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
            EVLLS0R::_0 => false,
            EVLLS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EVLLS0R {
        match value {
            false => EVLLS0R::_0,
            true => EVLLS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EVLLS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EVLLS0R::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Existence of HSRUN feature"]
    #[inline]
    pub fn ehsrun(&self) -> EHSRUNR {
        EHSRUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Existence of LLS feature"]
    #[inline]
    pub fn ells(&self) -> ELLSR {
        ELLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Existence of LLS2 feature"]
    #[inline]
    pub fn ells2(&self) -> ELLS2R {
        ELLS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Existence of VLLS0 feature"]
    #[inline]
    pub fn evlls0(&self) -> EVLLS0R {
        EVLLS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
