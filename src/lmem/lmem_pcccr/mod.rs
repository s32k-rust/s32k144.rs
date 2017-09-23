#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LMEM_PCCCR {
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
#[doc = "Possible values of the field `ENCACHE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCACHER {
    #[doc = "Cache disabled"] _0,
    #[doc = "Cache enabled"] _1,
}
impl ENCACHER {
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
            ENCACHER::_0 => false,
            ENCACHER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCACHER {
        match value {
            false => ENCACHER::_0,
            true => ENCACHER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENCACHER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENCACHER::_1
    }
}
#[doc = r" Value of the field"]
pub struct PCCR2R {
    bits: bool,
}
impl PCCR2R {
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
pub struct PCCR3R {
    bits: bool,
}
impl PCCR3R {
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
#[doc = "Possible values of the field `INVW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW0R {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, invalidate all lines in way 0."] _1,
}
impl INVW0R {
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
            INVW0R::_0 => false,
            INVW0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVW0R {
        match value {
            false => INVW0R::_0,
            true => INVW0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INVW0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INVW0R::_1
    }
}
#[doc = "Possible values of the field `PUSHW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW0R {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, push all modified lines in way 0"] _1,
}
impl PUSHW0R {
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
            PUSHW0R::_0 => false,
            PUSHW0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUSHW0R {
        match value {
            false => PUSHW0R::_0,
            true => PUSHW0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PUSHW0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PUSHW0R::_1
    }
}
#[doc = "Possible values of the field `INVW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW1R {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, invalidate all lines in way 1"] _1,
}
impl INVW1R {
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
            INVW1R::_0 => false,
            INVW1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVW1R {
        match value {
            false => INVW1R::_0,
            true => INVW1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INVW1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INVW1R::_1
    }
}
#[doc = "Possible values of the field `PUSHW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHW1R {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, push all modified lines in way 1"] _1,
}
impl PUSHW1R {
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
            PUSHW1R::_0 => false,
            PUSHW1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUSHW1R {
        match value {
            false => PUSHW1R::_0,
            true => PUSHW1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PUSHW1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PUSHW1R::_1
    }
}
#[doc = "Possible values of the field `GO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GOR {
    #[doc = "Write: no effect. Read: no cache command active."] _0,
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."] _1,
}
impl GOR {
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
            GOR::_0 => false,
            GOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GOR {
        match value {
            false => GOR::_0,
            true => GOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GOR::_1
    }
}
#[doc = "Values that can be written to the field `ENCACHE`"]
pub enum ENCACHEW {
    #[doc = "Cache disabled"] _0,
    #[doc = "Cache enabled"] _1,
}
impl ENCACHEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCACHEW::_0 => false,
            ENCACHEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCACHEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENCACHEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCACHEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cache disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCACHEW::_0)
    }
    #[doc = "Cache enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCACHEW::_1)
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
#[doc = r" Proxy"]
pub struct _PCCR2W<'a> {
    w: &'a mut W,
}
impl<'a> _PCCR2W<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCCR3W<'a> {
    w: &'a mut W,
}
impl<'a> _PCCR3W<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INVW0`"]
pub enum INVW0W {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, invalidate all lines in way 0."] _1,
}
impl INVW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW0W::_0 => false,
            INVW0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW0W<'a> {
    w: &'a mut W,
}
impl<'a> _INVW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVW0W::_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVW0W::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PUSHW0`"]
pub enum PUSHW0W {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, push all modified lines in way 0"] _1,
}
impl PUSHW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUSHW0W::_0 => false,
            PUSHW0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUSHW0W<'a> {
    w: &'a mut W,
}
impl<'a> _PUSHW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUSHW0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUSHW0W::_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUSHW0W::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INVW1`"]
pub enum INVW1W {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, invalidate all lines in way 1"] _1,
}
impl INVW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW1W::_0 => false,
            INVW1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVW1W<'a> {
    w: &'a mut W,
}
impl<'a> _INVW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVW1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVW1W::_0)
    }
    #[doc = "When setting the GO bit, invalidate all lines in way 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVW1W::_1)
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
#[doc = "Values that can be written to the field `PUSHW1`"]
pub enum PUSHW1W {
    #[doc = "No operation"] _0,
    #[doc = "When setting the GO bit, push all modified lines in way 1"] _1,
}
impl PUSHW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUSHW1W::_0 => false,
            PUSHW1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUSHW1W<'a> {
    w: &'a mut W,
}
impl<'a> _PUSHW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUSHW1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PUSHW1W::_0)
    }
    #[doc = "When setting the GO bit, push all modified lines in way 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PUSHW1W::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GO`"]
pub enum GOW {
    #[doc = "Write: no effect. Read: no cache command active."] _0,
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."] _1,
}
impl GOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GOW::_0 => false,
            GOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GOW<'a> {
    w: &'a mut W,
}
impl<'a> _GOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect. Read: no cache command active."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GOW::_0)
    }
    #[doc = "Write: initiate command indicated by bits 27-24. Read: cache command active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GOW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Cache enable"]
    #[inline]
    pub fn encache(&self) -> ENCACHER {
        ENCACHER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Forces all cacheable spaces to write through"]
    #[inline]
    pub fn pccr2(&self) -> PCCR2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCCR2R { bits }
    }
    #[doc = "Bit 3 - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
    #[inline]
    pub fn pccr3(&self) -> PCCR3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCCR3R { bits }
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline]
    pub fn invw0(&self) -> INVW0R {
        INVW0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline]
    pub fn pushw0(&self) -> PUSHW0R {
        PUSHW0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline]
    pub fn invw1(&self) -> INVW1R {
        INVW1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline]
    pub fn pushw1(&self) -> PUSHW1R {
        PUSHW1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline]
    pub fn go(&self) -> GOR {
        GOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Cache enable"]
    #[inline]
    pub fn encache(&mut self) -> _ENCACHEW {
        _ENCACHEW { w: self }
    }
    #[doc = "Bit 2 - Forces all cacheable spaces to write through"]
    #[inline]
    pub fn pccr2(&mut self) -> _PCCR2W {
        _PCCR2W { w: self }
    }
    #[doc = "Bit 3 - Forces no allocation on cache misses (must also have PCCR2 asserted)"]
    #[inline]
    pub fn pccr3(&mut self) -> _PCCR3W {
        _PCCR3W { w: self }
    }
    #[doc = "Bit 24 - Invalidate Way 0"]
    #[inline]
    pub fn invw0(&mut self) -> _INVW0W {
        _INVW0W { w: self }
    }
    #[doc = "Bit 25 - Push Way 0"]
    #[inline]
    pub fn pushw0(&mut self) -> _PUSHW0W {
        _PUSHW0W { w: self }
    }
    #[doc = "Bit 26 - Invalidate Way 1"]
    #[inline]
    pub fn invw1(&mut self) -> _INVW1W {
        _INVW1W { w: self }
    }
    #[doc = "Bit 27 - Push Way 1"]
    #[inline]
    pub fn pushw1(&mut self) -> _PUSHW1W {
        _PUSHW1W { w: self }
    }
    #[doc = "Bit 31 - Initiate Cache Command"]
    #[inline]
    pub fn go(&mut self) -> _GOW {
        _GOW { w: self }
    }
}
