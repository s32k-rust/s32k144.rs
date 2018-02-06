#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SRS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDR {
    #[doc = "Reset not caused by LVD trip, HVD trip or POR"]
    _0,
    #[doc = "Reset caused by LVD trip, HVD trip or POR"]
    _1,
}
impl LVDR {
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
            LVDR::_0 => false,
            LVDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LVDR {
        match value {
            false => LVDR::_0,
            true => LVDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LVDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LVDR::_1
    }
}
#[doc = "Possible values of the field `LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCR {
    #[doc = "Reset not caused by a loss of external clock."]
    _0,
    #[doc = "Reset caused by a loss of external clock."]
    _1,
}
impl LOCR {
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
            LOCR::_0 => false,
            LOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCR {
        match value {
            false => LOCR::_0,
            true => LOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCR::_1
    }
}
#[doc = "Possible values of the field `LOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLR {
    #[doc = "Reset not caused by a loss of lock in the PLL/FLL"]
    _0,
    #[doc = "Reset caused by a loss of lock in the PLL/FLL"]
    _1,
}
impl LOLR {
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
            LOLR::_0 => false,
            LOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOLR {
        match value {
            false => LOLR::_0,
            true => LOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOLR::_1
    }
}
#[doc = "Possible values of the field `WDOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOGR {
    #[doc = "Reset not caused by watchdog timeout"]
    _0,
    #[doc = "Reset caused by watchdog timeout"]
    _1,
}
impl WDOGR {
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
            WDOGR::_0 => false,
            WDOGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOGR {
        match value {
            false => WDOGR::_0,
            true => WDOGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WDOGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WDOGR::_1
    }
}
#[doc = "Possible values of the field `PIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINR {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl PINR {
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
            PINR::_0 => false,
            PINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINR {
        match value {
            false => PINR::_0,
            true => PINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PINR::_1
    }
}
#[doc = "Possible values of the field `POR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR {
    #[doc = "Reset not caused by POR"]
    _0,
    #[doc = "Reset caused by POR"]
    _1,
}
impl PORR {
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
            PORR::_0 => false,
            PORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORR {
        match value {
            false => PORR::_0,
            true => PORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PORR::_1
    }
}
#[doc = "Possible values of the field `JTAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAGR {
    #[doc = "Reset not caused by JTAG"]
    _0,
    #[doc = "Reset caused by JTAG"]
    _1,
}
impl JTAGR {
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
            JTAGR::_0 => false,
            JTAGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JTAGR {
        match value {
            false => JTAGR::_0,
            true => JTAGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == JTAGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == JTAGR::_1
    }
}
#[doc = "Possible values of the field `LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUPR {
    #[doc = "Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "Reset caused by core LOCKUP event"]
    _1,
}
impl LOCKUPR {
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
            LOCKUPR::_0 => false,
            LOCKUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKUPR {
        match value {
            false => LOCKUPR::_0,
            true => LOCKUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCKUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCKUPR::_1
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl SWR {
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
            SWR::_0 => false,
            SWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWR {
        match value {
            false => SWR::_0,
            true => SWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWR::_1
    }
}
#[doc = "Possible values of the field `MDM_AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_APR {
    #[doc = "Reset was not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "Reset was caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl MDM_APR {
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
            MDM_APR::_0 => false,
            MDM_APR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDM_APR {
        match value {
            false => MDM_APR::_0,
            true => MDM_APR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MDM_APR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MDM_APR::_1
    }
}
#[doc = "Possible values of the field `SACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERRR {
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl SACKERRR {
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
            SACKERRR::_0 => false,
            SACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SACKERRR {
        match value {
            false => SACKERRR::_0,
            true => SACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SACKERRR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Low-Voltage Detect Reset or High-Voltage Detect Reset"]
    #[inline]
    pub fn lvd(&self) -> LVDR {
        LVDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Loss-of-Clock Reset"]
    #[inline]
    pub fn loc(&self) -> LOCR {
        LOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Loss-of-Lock Reset"]
    #[inline]
    pub fn lol(&self) -> LOLR {
        LOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Watchdog"]
    #[inline]
    pub fn wdog(&self) -> WDOGR {
        WDOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - External Reset Pin"]
    #[inline]
    pub fn pin(&self) -> PINR {
        PINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Power-On Reset"]
    #[inline]
    pub fn por(&self) -> PORR {
        PORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline]
    pub fn jtag(&self) -> JTAGR {
        JTAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Core Lockup"]
    #[inline]
    pub fn lockup(&self) -> LOCKUPR {
        LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Software"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline]
    pub fn mdm_ap(&self) -> MDM_APR {
        MDM_APR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Stop Acknowledge Error"]
    #[inline]
    pub fn sackerr(&self) -> SACKERRR {
        SACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
