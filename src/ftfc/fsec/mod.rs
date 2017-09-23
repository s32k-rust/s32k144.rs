#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FSEC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECR {
    #[doc = "MCU security status is unsecure (The standard shipping condition of the FTFC is unsecure.)"]
    _10,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl SECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECR::_10 => 2,
            SECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECR {
        match value {
            2 => SECR::_10,
            i => SECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SECR::_10
    }
}
#[doc = "Possible values of the field `FSLACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLACCR {
    #[doc = "Factory access granted"] _00,
    #[doc = "Factory access granted"] _11,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl FSLACCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSLACCR::_00 => 0,
            FSLACCR::_11 => 3,
            FSLACCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSLACCR {
        match value {
            0 => FSLACCR::_00,
            3 => FSLACCR::_11,
            i => FSLACCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FSLACCR::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FSLACCR::_11
    }
}
#[doc = "Possible values of the field `MEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEENR {
    #[doc = "Mass erase is enabled"] _00,
    #[doc = "Mass erase is enabled"] _01,
    #[doc = "Mass erase is enabled"] _11,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl MEENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MEENR::_00 => 0,
            MEENR::_01 => 1,
            MEENR::_11 => 3,
            MEENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MEENR {
        match value {
            0 => MEENR::_00,
            1 => MEENR::_01,
            3 => MEENR::_11,
            i => MEENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MEENR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MEENR::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MEENR::_11
    }
}
#[doc = "Possible values of the field `KEYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYENR {
    #[doc = "Backdoor key access disabled"] _00,
    #[doc = "Backdoor key access disabled (preferred KEYEN state to disable backdoor key access)"]
    _01,
    #[doc = "Backdoor key access enabled"] _10,
    #[doc = "Backdoor key access disabled"] _11,
}
impl KEYENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYENR::_00 => 0,
            KEYENR::_01 => 1,
            KEYENR::_10 => 2,
            KEYENR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYENR {
        match value {
            0 => KEYENR::_00,
            1 => KEYENR::_01,
            2 => KEYENR::_10,
            3 => KEYENR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == KEYENR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == KEYENR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == KEYENR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == KEYENR::_11
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline]
    pub fn sec(&self) -> SECR {
        SECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Factory Failure Analysis Access Code"]
    #[inline]
    pub fn fslacc(&self) -> FSLACCR {
        FSLACCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Mass Erase Enable Bits"]
    #[inline]
    pub fn meen(&self) -> MEENR {
        MEENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline]
    pub fn keyen(&self) -> KEYENR {
        KEYENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
