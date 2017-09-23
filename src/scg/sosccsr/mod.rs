#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOSCCSR {
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
#[doc = "Possible values of the field `SOSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCENR {
    #[doc = "System OSC is disabled"] _0,
    #[doc = "System OSC is enabled"] _1,
}
impl SOSCENR {
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
            SOSCENR::_0 => false,
            SOSCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCENR {
        match value {
            false => SOSCENR::_0,
            true => SOSCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOSCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOSCENR::_1
    }
}
#[doc = "Possible values of the field `SOSCCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCMR {
    #[doc = "System OSC Clock Monitor is disabled"] _0,
    #[doc = "System OSC Clock Monitor is enabled"] _1,
}
impl SOSCCMR {
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
            SOSCCMR::_0 => false,
            SOSCCMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCCMR {
        match value {
            false => SOSCCMR::_0,
            true => SOSCCMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOSCCMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOSCCMR::_1
    }
}
#[doc = "Possible values of the field `SOSCCMRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCCMRER {
    #[doc = "Clock Monitor generates interrupt when error detected"] _0,
    #[doc = "Clock Monitor generates reset when error detected"] _1,
}
impl SOSCCMRER {
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
            SOSCCMRER::_0 => false,
            SOSCCMRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCCMRER {
        match value {
            false => SOSCCMRER::_0,
            true => SOSCCMRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOSCCMRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOSCCMRER::_1
    }
}
#[doc = "Possible values of the field `LK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKR {
    #[doc = "This Control Status Register can be written."] _0,
    #[doc = "This Control Status Register cannot be written."] _1,
}
impl LKR {
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
            LKR::_0 => false,
            LKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LKR {
        match value {
            false => LKR::_0,
            true => LKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LKR::_1
    }
}
#[doc = "Possible values of the field `SOSCVLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCVLDR {
    #[doc = "System OSC is not enabled or clock is not valid"] _0,
    #[doc = "System OSC is enabled and output clock is valid"] _1,
}
impl SOSCVLDR {
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
            SOSCVLDR::_0 => false,
            SOSCVLDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCVLDR {
        match value {
            false => SOSCVLDR::_0,
            true => SOSCVLDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOSCVLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOSCVLDR::_1
    }
}
#[doc = "Possible values of the field `SOSCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCSELR {
    #[doc = "System OSC is not the system clock source"] _0,
    #[doc = "System OSC is the system clock source"] _1,
}
impl SOSCSELR {
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
            SOSCSELR::_0 => false,
            SOSCSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCSELR {
        match value {
            false => SOSCSELR::_0,
            true => SOSCSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOSCSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOSCSELR::_1
    }
}
#[doc = "Possible values of the field `SOSCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCERRR {
    #[doc = "System OSC Clock Monitor is disabled or has not detected an error"] _0,
    #[doc = "System OSC Clock Monitor is enabled and detected an error"] _1,
}
impl SOSCERRR {
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
            SOSCERRR::_0 => false,
            SOSCERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOSCERRR {
        match value {
            false => SOSCERRR::_0,
            true => SOSCERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOSCERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOSCERRR::_1
    }
}
#[doc = "Values that can be written to the field `SOSCEN`"]
pub enum SOSCENW {
    #[doc = "System OSC is disabled"] _0,
    #[doc = "System OSC is enabled"] _1,
}
impl SOSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOSCENW::_0 => false,
            SOSCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System OSC is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCENW::_0)
    }
    #[doc = "System OSC is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCENW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOSCCM`"]
pub enum SOSCCMW {
    #[doc = "System OSC Clock Monitor is disabled"] _0,
    #[doc = "System OSC Clock Monitor is enabled"] _1,
}
impl SOSCCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOSCCMW::_0 => false,
            SOSCCMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCCMW<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System OSC Clock Monitor is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCCMW::_0)
    }
    #[doc = "System OSC Clock Monitor is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCCMW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOSCCMRE`"]
pub enum SOSCCMREW {
    #[doc = "Clock Monitor generates interrupt when error detected"] _0,
    #[doc = "Clock Monitor generates reset when error detected"] _1,
}
impl SOSCCMREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOSCCMREW::_0 => false,
            SOSCCMREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCCMREW<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCCMREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCCMREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock Monitor generates interrupt when error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCCMREW::_0)
    }
    #[doc = "Clock Monitor generates reset when error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCCMREW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LK`"]
pub enum LKW {
    #[doc = "This Control Status Register can be written."] _0,
    #[doc = "This Control Status Register cannot be written."] _1,
}
impl LKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LKW::_0 => false,
            LKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LKW<'a> {
    w: &'a mut W,
}
impl<'a> _LKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This Control Status Register can be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LKW::_0)
    }
    #[doc = "This Control Status Register cannot be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LKW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOSCERR`"]
pub enum SOSCERRW {
    #[doc = "System OSC Clock Monitor is disabled or has not detected an error"] _0,
    #[doc = "System OSC Clock Monitor is enabled and detected an error"] _1,
}
impl SOSCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOSCERRW::_0 => false,
            SOSCERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System OSC Clock Monitor is disabled or has not detected an error"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOSCERRW::_0)
    }
    #[doc = "System OSC Clock Monitor is enabled and detected an error"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOSCERRW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - System OSC Enable"]
    #[inline]
    pub fn soscen(&self) -> SOSCENR {
        SOSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - System OSC Clock Monitor"]
    #[inline]
    pub fn sosccm(&self) -> SOSCCMR {
        SOSCCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
    #[inline]
    pub fn sosccmre(&self) -> SOSCCMRER {
        SOSCCMRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&self) -> LKR {
        LKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - System OSC Valid"]
    #[inline]
    pub fn soscvld(&self) -> SOSCVLDR {
        SOSCVLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - System OSC Selected"]
    #[inline]
    pub fn soscsel(&self) -> SOSCSELR {
        SOSCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - System OSC Clock Error"]
    #[inline]
    pub fn soscerr(&self) -> SOSCERRR {
        SOSCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - System OSC Enable"]
    #[inline]
    pub fn soscen(&mut self) -> _SOSCENW {
        _SOSCENW { w: self }
    }
    #[doc = "Bit 16 - System OSC Clock Monitor"]
    #[inline]
    pub fn sosccm(&mut self) -> _SOSCCMW {
        _SOSCCMW { w: self }
    }
    #[doc = "Bit 17 - System OSC Clock Monitor Reset Enable"]
    #[inline]
    pub fn sosccmre(&mut self) -> _SOSCCMREW {
        _SOSCCMREW { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&mut self) -> _LKW {
        _LKW { w: self }
    }
    #[doc = "Bit 26 - System OSC Clock Error"]
    #[inline]
    pub fn soscerr(&mut self) -> _SOSCERRW {
        _SOSCERRW { w: self }
    }
}
