#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LMEM_PCCLCR {
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
#[doc = "Possible values of the field `LGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LGOR {
    #[doc = "Write: no effect. Read: no line command active."] _0,
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."] _1,
}
impl LGOR {
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
            LGOR::_0 => false,
            LGOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LGOR {
        match value {
            false => LGOR::_0,
            true => LGOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LGOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LGOR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CACHEADDRR {
    bits: u16,
}
impl CACHEADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `WSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSELR {
    #[doc = "Way 0"] _0,
    #[doc = "Way 1"] _1,
}
impl WSELR {
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
            WSELR::_0 => false,
            WSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSELR {
        match value {
            false => WSELR::_0,
            true => WSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WSELR::_1
    }
}
#[doc = "Possible values of the field `TDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDSELR {
    #[doc = "Data"] _0,
    #[doc = "Tag"] _1,
}
impl TDSELR {
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
            TDSELR::_0 => false,
            TDSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDSELR {
        match value {
            false => TDSELR::_0,
            true => TDSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDSELR::_1
    }
}
#[doc = r" Value of the field"]
pub struct LCIVBR {
    bits: bool,
}
impl LCIVBR {
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
pub struct LCIMBR {
    bits: bool,
}
impl LCIMBR {
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
pub struct LCWAYR {
    bits: bool,
}
impl LCWAYR {
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
#[doc = "Possible values of the field `LCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCMDR {
    #[doc = "Search and read or write"] _00,
    #[doc = "Invalidate"] _01,
    #[doc = "Push"] _10,
    #[doc = "Clear"] _11,
}
impl LCMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LCMDR::_00 => 0,
            LCMDR::_01 => 1,
            LCMDR::_10 => 2,
            LCMDR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LCMDR {
        match value {
            0 => LCMDR::_00,
            1 => LCMDR::_01,
            2 => LCMDR::_10,
            3 => LCMDR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LCMDR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LCMDR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LCMDR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LCMDR::_11
    }
}
#[doc = "Possible values of the field `LADSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADSELR {
    #[doc = "Cache address"] _0,
    #[doc = "Physical address"] _1,
}
impl LADSELR {
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
            LADSELR::_0 => false,
            LADSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LADSELR {
        match value {
            false => LADSELR::_0,
            true => LADSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LADSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LADSELR::_1
    }
}
#[doc = "Possible values of the field `LACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LACCR {
    #[doc = "Read"] _0,
    #[doc = "Write"] _1,
}
impl LACCR {
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
            LACCR::_0 => false,
            LACCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LACCR {
        match value {
            false => LACCR::_0,
            true => LACCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LACCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LACCR::_1
    }
}
#[doc = "Values that can be written to the field `LGO`"]
pub enum LGOW {
    #[doc = "Write: no effect. Read: no line command active."] _0,
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."] _1,
}
impl LGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LGOW::_0 => false,
            LGOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LGOW<'a> {
    w: &'a mut W,
}
impl<'a> _LGOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LGOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGOW::_0)
    }
    #[doc = "Write: initiate line command indicated by bits 27-24. Read: line command active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGOW::_1)
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
pub struct _CACHEADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _CACHEADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WSEL`"]
pub enum WSELW {
    #[doc = "Way 0"] _0,
    #[doc = "Way 1"] _1,
}
impl WSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSELW::_0 => false,
            WSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Way 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSELW::_0)
    }
    #[doc = "Way 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSELW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TDSEL`"]
pub enum TDSELW {
    #[doc = "Data"] _0,
    #[doc = "Tag"] _1,
}
impl TDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDSELW::_0 => false,
            TDSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDSELW::_0)
    }
    #[doc = "Tag"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDSELW::_1)
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
#[doc = r" Proxy"]
pub struct _LCIVBW<'a> {
    w: &'a mut W,
}
impl<'a> _LCIVBW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LCIMBW<'a> {
    w: &'a mut W,
}
impl<'a> _LCIMBW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LCWAYW<'a> {
    w: &'a mut W,
}
impl<'a> _LCWAYW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LCMD`"]
pub enum LCMDW {
    #[doc = "Search and read or write"] _00,
    #[doc = "Invalidate"] _01,
    #[doc = "Push"] _10,
    #[doc = "Clear"] _11,
}
impl LCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LCMDW::_00 => 0,
            LCMDW::_01 => 1,
            LCMDW::_10 => 2,
            LCMDW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _LCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LCMDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Search and read or write"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LCMDW::_00)
    }
    #[doc = "Invalidate"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LCMDW::_01)
    }
    #[doc = "Push"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LCMDW::_10)
    }
    #[doc = "Clear"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LCMDW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LADSEL`"]
pub enum LADSELW {
    #[doc = "Cache address"] _0,
    #[doc = "Physical address"] _1,
}
impl LADSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LADSELW::_0 => false,
            LADSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LADSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LADSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LADSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Cache address"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LADSELW::_0)
    }
    #[doc = "Physical address"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LADSELW::_1)
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
#[doc = "Values that can be written to the field `LACC`"]
pub enum LACCW {
    #[doc = "Read"] _0,
    #[doc = "Write"] _1,
}
impl LACCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LACCW::_0 => false,
            LACCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LACCW<'a> {
    w: &'a mut W,
}
impl<'a> _LACCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LACCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LACCW::_0)
    }
    #[doc = "Write"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LACCW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline]
    pub fn lgo(&self) -> LGOR {
        LGOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:13 - Cache address"]
    #[inline]
    pub fn cacheaddr(&self) -> CACHEADDRR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CACHEADDRR { bits }
    }
    #[doc = "Bit 14 - Way select"]
    #[inline]
    pub fn wsel(&self) -> WSELR {
        WSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline]
    pub fn tdsel(&self) -> TDSELR {
        TDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline]
    pub fn lcivb(&self) -> LCIVBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCIVBR { bits }
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline]
    pub fn lcimb(&self) -> LCIMBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCIMBR { bits }
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline]
    pub fn lcway(&self) -> LCWAYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LCWAYR { bits }
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline]
    pub fn lcmd(&self) -> LCMDR {
        LCMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline]
    pub fn ladsel(&self) -> LADSELR {
        LADSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline]
    pub fn lacc(&self) -> LACCR {
        LACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
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
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline]
    pub fn lgo(&mut self) -> _LGOW {
        _LGOW { w: self }
    }
    #[doc = "Bits 2:13 - Cache address"]
    #[inline]
    pub fn cacheaddr(&mut self) -> _CACHEADDRW {
        _CACHEADDRW { w: self }
    }
    #[doc = "Bit 14 - Way select"]
    #[inline]
    pub fn wsel(&mut self) -> _WSELW {
        _WSELW { w: self }
    }
    #[doc = "Bit 16 - Tag/Data Select"]
    #[inline]
    pub fn tdsel(&mut self) -> _TDSELW {
        _TDSELW { w: self }
    }
    #[doc = "Bit 20 - Line Command Initial Valid Bit"]
    #[inline]
    pub fn lcivb(&mut self) -> _LCIVBW {
        _LCIVBW { w: self }
    }
    #[doc = "Bit 21 - Line Command Initial Modified Bit"]
    #[inline]
    pub fn lcimb(&mut self) -> _LCIMBW {
        _LCIMBW { w: self }
    }
    #[doc = "Bit 22 - Line Command Way"]
    #[inline]
    pub fn lcway(&mut self) -> _LCWAYW {
        _LCWAYW { w: self }
    }
    #[doc = "Bits 24:25 - Line Command"]
    #[inline]
    pub fn lcmd(&mut self) -> _LCMDW {
        _LCMDW { w: self }
    }
    #[doc = "Bit 26 - Line Address Select"]
    #[inline]
    pub fn ladsel(&mut self) -> _LADSELW {
        _LADSELW { w: self }
    }
    #[doc = "Bit 27 - Line access type"]
    #[inline]
    pub fn lacc(&mut self) -> _LACCW {
        _LACCW { w: self }
    }
}
