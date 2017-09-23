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
#[doc = "Possible values of the field `EWAKEUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWAKEUPR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl EWAKEUPR {
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
            EWAKEUPR::_0 => false,
            EWAKEUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWAKEUPR {
        match value {
            false => EWAKEUPR::_0,
            true => EWAKEUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EWAKEUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EWAKEUPR::_1
    }
}
#[doc = "Possible values of the field `ELVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELVDR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ELVDR {
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
            ELVDR::_0 => false,
            ELVDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELVDR {
        match value {
            false => ELVDR::_0,
            true => ELVDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ELVDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ELVDR::_1
    }
}
#[doc = "Possible values of the field `ELOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOCR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ELOCR {
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
            ELOCR::_0 => false,
            ELOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELOCR {
        match value {
            false => ELOCR::_0,
            true => ELOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ELOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ELOCR::_1
    }
}
#[doc = "Possible values of the field `ELOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOLR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ELOLR {
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
            ELOLR::_0 => false,
            ELOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELOLR {
        match value {
            false => ELOLR::_0,
            true => ELOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ELOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ELOLR::_1
    }
}
#[doc = "Possible values of the field `EWDOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWDOGR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl EWDOGR {
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
            EWDOGR::_0 => false,
            EWDOGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWDOGR {
        match value {
            false => EWDOGR::_0,
            true => EWDOGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EWDOGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EWDOGR::_1
    }
}
#[doc = "Possible values of the field `EPIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPINR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl EPINR {
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
            EPINR::_0 => false,
            EPINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPINR {
        match value {
            false => EPINR::_0,
            true => EPINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EPINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EPINR::_1
    }
}
#[doc = "Possible values of the field `EPOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPORR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl EPORR {
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
            EPORR::_0 => false,
            EPORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPORR {
        match value {
            false => EPORR::_0,
            true => EPORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EPORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EPORR::_1
    }
}
#[doc = "Possible values of the field `EJTAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EJTAGR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl EJTAGR {
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
            EJTAGR::_0 => false,
            EJTAGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EJTAGR {
        match value {
            false => EJTAGR::_0,
            true => EJTAGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EJTAGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EJTAGR::_1
    }
}
#[doc = "Possible values of the field `ELOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOCKUPR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ELOCKUPR {
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
            ELOCKUPR::_0 => false,
            ELOCKUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ELOCKUPR {
        match value {
            false => ELOCKUPR::_0,
            true => ELOCKUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ELOCKUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ELOCKUPR::_1
    }
}
#[doc = "Possible values of the field `ESW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESWR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ESWR {
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
            ESWR::_0 => false,
            ESWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESWR {
        match value {
            false => ESWR::_0,
            true => ESWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESWR::_1
    }
}
#[doc = "Possible values of the field `EMDM_AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMDM_APR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl EMDM_APR {
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
            EMDM_APR::_0 => false,
            EMDM_APR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EMDM_APR {
        match value {
            false => EMDM_APR::_0,
            true => EMDM_APR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EMDM_APR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EMDM_APR::_1
    }
}
#[doc = "Possible values of the field `ESACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESACKERRR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ESACKERRR {
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
            ESACKERRR::_0 => false,
            ESACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESACKERRR {
        match value {
            false => ESACKERRR::_0,
            true => ESACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESACKERRR::_1
    }
}
#[doc = "Possible values of the field `ETAMPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETAMPERR {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ETAMPERR {
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
            ETAMPERR::_0 => false,
            ETAMPERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETAMPERR {
        match value {
            false => ETAMPERR::_0,
            true => ETAMPERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ETAMPERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ETAMPERR::_1
    }
}
#[doc = "Possible values of the field `ECORE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECORE1R {
    #[doc = "The feature is not available."] _0,
    #[doc = "The feature is available."] _1,
}
impl ECORE1R {
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
            ECORE1R::_0 => false,
            ECORE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECORE1R {
        match value {
            false => ECORE1R::_0,
            true => ECORE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ECORE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ECORE1R::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Existence of SRS[WAKEUP] status indication feature"]
    #[inline]
    pub fn ewakeup(&self) -> EWAKEUPR {
        EWAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Existence of SRS[LVD] status indication feature"]
    #[inline]
    pub fn elvd(&self) -> ELVDR {
        ELVDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Existence of SRS[LOC] status indication feature"]
    #[inline]
    pub fn eloc(&self) -> ELOCR {
        ELOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Existence of SRS[LOL] status indication feature"]
    #[inline]
    pub fn elol(&self) -> ELOLR {
        ELOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Existence of SRS[WDOG] status indication feature"]
    #[inline]
    pub fn ewdog(&self) -> EWDOGR {
        EWDOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Existence of SRS[PIN] status indication feature"]
    #[inline]
    pub fn epin(&self) -> EPINR {
        EPINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Existence of SRS[POR] status indication feature"]
    #[inline]
    pub fn epor(&self) -> EPORR {
        EPORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Existence of SRS[JTAG] status indication feature"]
    #[inline]
    pub fn ejtag(&self) -> EJTAGR {
        EJTAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Existence of SRS[LOCKUP] status indication feature"]
    #[inline]
    pub fn elockup(&self) -> ELOCKUPR {
        ELOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Existence of SRS[SW] status indication feature"]
    #[inline]
    pub fn esw(&self) -> ESWR {
        ESWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Existence of SRS[MDM_AP] status indication feature"]
    #[inline]
    pub fn emdm_ap(&self) -> EMDM_APR {
        EMDM_APR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Existence of SRS[SACKERR] status indication feature"]
    #[inline]
    pub fn esackerr(&self) -> ESACKERRR {
        ESACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Existence of SRS[TAMPER] status indication feature"]
    #[inline]
    pub fn etamper(&self) -> ETAMPERR {
        ETAMPERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Existence of SRS[CORE1] status indication feature"]
    #[inline]
    pub fn ecore1(&self) -> ECORE1R {
        ECORE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
