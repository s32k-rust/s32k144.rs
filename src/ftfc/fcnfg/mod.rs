#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FCNFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct EEERDYR {
    bits: bool,
}
impl EEERDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct RAMRDYR {
    bits: bool,
}
impl RAMRDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ERSSUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSSUSPR {
    #[doc = "No suspend requested"] _0,
    #[doc = "Suspend the current Erase Flash Sector command execution"] _1,
}
impl ERSSUSPR {
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
            ERSSUSPR::_0 => false,
            ERSSUSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERSSUSPR {
        match value {
            false => ERSSUSPR::_0,
            true => ERSSUSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERSSUSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERSSUSPR::_1
    }
}
#[doc = "Possible values of the field `ERSAREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQR {
    #[doc = "No request or request complete"] _0,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl ERSAREQR {
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
            ERSAREQR::_0 => false,
            ERSAREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERSAREQR {
        match value {
            false => ERSAREQR::_0,
            i => ERSAREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERSAREQR::_0
    }
}
#[doc = "Possible values of the field `RDCOLLIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLLIER {
    #[doc = "Read collision error interrupt disabled"] _0,
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT[RDCOLERR])."]
    _1,
}
impl RDCOLLIER {
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
            RDCOLLIER::_0 => false,
            RDCOLLIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDCOLLIER {
        match value {
            false => RDCOLLIER::_0,
            true => RDCOLLIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDCOLLIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDCOLLIER::_1
    }
}
#[doc = "Possible values of the field `CCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIER {
    #[doc = "Command complete interrupt disabled"] _0,
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT[CCIF] flag is set."]
    _1,
}
impl CCIER {
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
            CCIER::_0 => false,
            CCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIER {
        match value {
            false => CCIER::_0,
            true => CCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCIER::_1
    }
}
#[doc = "Values that can be written to the field `ERSSUSP`"]
pub enum ERSSUSPW {
    #[doc = "No suspend requested"] _0,
    #[doc = "Suspend the current Erase Flash Sector command execution"] _1,
}
impl ERSSUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERSSUSPW::_0 => false,
            ERSSUSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERSSUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _ERSSUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERSSUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No suspend requested"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERSSUSPW::_0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERSSUSPW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDCOLLIE`"]
pub enum RDCOLLIEW {
    #[doc = "Read collision error interrupt disabled"] _0,
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT[RDCOLERR])."]
    _1,
}
impl RDCOLLIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDCOLLIEW::_0 => false,
            RDCOLLIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDCOLLIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDCOLLIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDCOLLIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read collision error interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLLIEW::_0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT[RDCOLERR])."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLLIEW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCIE`"]
pub enum CCIEW {
    #[doc = "Command complete interrupt disabled"] _0,
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT[CCIF] flag is set."]
    _1,
}
impl CCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCIEW::_0 => false,
            CCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIEW::_0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT[CCIF] flag is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIEW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - EEERDY"]
    #[inline]
    pub fn eeerdy(&self) -> EEERDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EEERDYR { bits }
    }
    #[doc = "Bit 1 - RAM Ready"]
    #[inline]
    pub fn ramrdy(&self) -> RAMRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RAMRDYR { bits }
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline]
    pub fn erssusp(&self) -> ERSSUSPR {
        ERSSUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline]
    pub fn ersareq(&self) -> ERSAREQR {
        ERSAREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline]
    pub fn rdcollie(&self) -> RDCOLLIER {
        RDCOLLIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccie(&self) -> CCIER {
        CCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline]
    pub fn erssusp(&mut self) -> _ERSSUSPW {
        _ERSSUSPW { w: self }
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline]
    pub fn rdcollie(&mut self) -> _RDCOLLIEW {
        _RDCOLLIEW { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccie(&mut self) -> _CCIEW {
        _CCIEW { w: self }
    }
}
