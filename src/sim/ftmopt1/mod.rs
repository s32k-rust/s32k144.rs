#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FTMOPT1 {
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
        R { bits: self.register.get() }
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
pub struct FTM0SYNCBITR {
    bits: bool,
}
impl FTM0SYNCBITR {
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
pub struct FTM1SYNCBITR {
    bits: bool,
}
impl FTM1SYNCBITR {
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
pub struct FTM2SYNCBITR {
    bits: bool,
}
impl FTM2SYNCBITR {
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
pub struct FTM3SYNCBITR {
    bits: bool,
}
impl FTM3SYNCBITR {
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
#[doc = "Possible values of the field `FTM1CH0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1CH0SELR {
    #[doc = "FTM1_CH0 input"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM1CH0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM1CH0SELR::_00 => 0,
            FTM1CH0SELR::_01 => 1,
            FTM1CH0SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM1CH0SELR {
        match value {
            0 => FTM1CH0SELR::_00,
            1 => FTM1CH0SELR::_01,
            i => FTM1CH0SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM1CH0SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM1CH0SELR::_01
    }
}
#[doc = "Possible values of the field `FTM2CH0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH0SELR {
    #[doc = "FTM2_CH0 input"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM2CH0SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM2CH0SELR::_00 => 0,
            FTM2CH0SELR::_01 => 1,
            FTM2CH0SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM2CH0SELR {
        match value {
            0 => FTM2CH0SELR::_00,
            1 => FTM2CH0SELR::_01,
            i => FTM2CH0SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM2CH0SELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM2CH0SELR::_01
    }
}
#[doc = "Possible values of the field `FTM2CH1SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH1SELR {
    #[doc = "FTM2_CH1 input"]
    _0,
    #[doc = "exclusive OR of FTM2_CH0,FTM2_CH1,and FTM1_CH1"]
    _1,
}
impl FTM2CH1SELR {
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
            FTM2CH1SELR::_0 => false,
            FTM2CH1SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2CH1SELR {
        match value {
            false => FTM2CH1SELR::_0,
            true => FTM2CH1SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2CH1SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2CH1SELR::_1
    }
}
#[doc = "Possible values of the field `FTMGLDOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTMGLDOKR {
    #[doc = "FTM Global load mechanism disabled."]
    _0,
    #[doc = "FTM Global load mechanism enabled"]
    _1,
}
impl FTMGLDOKR {
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
            FTMGLDOKR::_0 => false,
            FTMGLDOKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTMGLDOKR {
        match value {
            false => FTMGLDOKR::_0,
            true => FTMGLDOKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTMGLDOKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTMGLDOKR::_1
    }
}
#[doc = "Possible values of the field `FTM0_OUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_OUTSELR {
    #[doc = "No modulation with FTM1_CH1"]
    _00000000,
    #[doc = "Modulation with FTM1_CH1"]
    _00000001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM0_OUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM0_OUTSELR::_00000000 => 0,
            FTM0_OUTSELR::_00000001 => 1,
            FTM0_OUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM0_OUTSELR {
        match value {
            0 => FTM0_OUTSELR::_00000000,
            1 => FTM0_OUTSELR::_00000001,
            i => FTM0_OUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline]
    pub fn is_00000000(&self) -> bool {
        *self == FTM0_OUTSELR::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline]
    pub fn is_00000001(&self) -> bool {
        *self == FTM0_OUTSELR::_00000001
    }
}
#[doc = "Possible values of the field `FTM3_OUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3_OUTSELR {
    #[doc = "No modulation with FTM2_CH1"]
    _00000000,
    #[doc = "Modulation with FTM2_CH1"]
    _00000001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM3_OUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM3_OUTSELR::_00000000 => 0,
            FTM3_OUTSELR::_00000001 => 1,
            FTM3_OUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM3_OUTSELR {
        match value {
            0 => FTM3_OUTSELR::_00000000,
            1 => FTM3_OUTSELR::_00000001,
            i => FTM3_OUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline]
    pub fn is_00000000(&self) -> bool {
        *self == FTM3_OUTSELR::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline]
    pub fn is_00000001(&self) -> bool {
        *self == FTM3_OUTSELR::_00000001
    }
}
#[doc = r" Proxy"]
pub struct _FTM0SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0SYNCBITW<'a> {
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
pub struct _FTM1SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1SYNCBITW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FTM2SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2SYNCBITW<'a> {
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
pub struct _FTM3SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3SYNCBITW<'a> {
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
#[doc = "Values that can be written to the field `FTM1CH0SEL`"]
pub enum FTM1CH0SELW {
    #[doc = "FTM1_CH0 input"]
    _00,
    #[doc = "CMP0 output"]
    _01,
}
impl FTM1CH0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM1CH0SELW::_00 => 0,
            FTM1CH0SELW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1CH0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1CH0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1CH0SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM1_CH0 input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CH0SELW::_00)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CH0SELW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2CH0SEL`"]
pub enum FTM2CH0SELW {
    #[doc = "FTM2_CH0 input"]
    _00,
    #[doc = "CMP0 output"]
    _01,
}
impl FTM2CH0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM2CH0SELW::_00 => 0,
            FTM2CH0SELW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2CH0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2CH0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2CH0SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM2_CH0 input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CH0SELW::_00)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CH0SELW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2CH1SEL`"]
pub enum FTM2CH1SELW {
    #[doc = "FTM2_CH1 input"]
    _0,
    #[doc = "exclusive OR of FTM2_CH0,FTM2_CH1,and FTM1_CH1"]
    _1,
}
impl FTM2CH1SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2CH1SELW::_0 => false,
            FTM2CH1SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2CH1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2CH1SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2CH1SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM2_CH1 input"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CH1SELW::_0)
    }
    #[doc = "exclusive OR of FTM2_CH0,FTM2_CH1,and FTM1_CH1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CH1SELW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTMGLDOK`"]
pub enum FTMGLDOKW {
    #[doc = "FTM Global load mechanism disabled."]
    _0,
    #[doc = "FTM Global load mechanism enabled"]
    _1,
}
impl FTMGLDOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTMGLDOKW::_0 => false,
            FTMGLDOKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTMGLDOKW<'a> {
    w: &'a mut W,
}
impl<'a> _FTMGLDOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTMGLDOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM Global load mechanism disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTMGLDOKW::_0)
    }
    #[doc = "FTM Global load mechanism enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTMGLDOKW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM0_OUTSEL`"]
pub enum FTM0_OUTSELW {
    #[doc = "No modulation with FTM1_CH1"]
    _00000000,
    #[doc = "Modulation with FTM1_CH1"]
    _00000001,
}
impl FTM0_OUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM0_OUTSELW::_00000000 => 0,
            FTM0_OUTSELW::_00000001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0_OUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0_OUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0_OUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No modulation with FTM1_CH1"]
    #[inline]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(FTM0_OUTSELW::_00000000)
    }
    #[doc = "Modulation with FTM1_CH1"]
    #[inline]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(FTM0_OUTSELW::_00000001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM3_OUTSEL`"]
pub enum FTM3_OUTSELW {
    #[doc = "No modulation with FTM2_CH1"]
    _00000000,
    #[doc = "Modulation with FTM2_CH1"]
    _00000001,
}
impl FTM3_OUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM3_OUTSELW::_00000000 => 0,
            FTM3_OUTSELW::_00000001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3_OUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3_OUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3_OUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No modulation with FTM2_CH1"]
    #[inline]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(FTM3_OUTSELW::_00000000)
    }
    #[doc = "Modulation with FTM2_CH1"]
    #[inline]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(FTM3_OUTSELW::_00000001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - FTM0 Sync Bit"]
    #[inline]
    pub fn ftm0syncbit(&self) -> FTM0SYNCBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTM0SYNCBITR { bits }
    }
    #[doc = "Bit 1 - FTM1 Sync Bit"]
    #[inline]
    pub fn ftm1syncbit(&self) -> FTM1SYNCBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTM1SYNCBITR { bits }
    }
    #[doc = "Bit 2 - FTM2 Sync Bit"]
    #[inline]
    pub fn ftm2syncbit(&self) -> FTM2SYNCBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTM2SYNCBITR { bits }
    }
    #[doc = "Bit 3 - FTM3 Sync Bit"]
    #[inline]
    pub fn ftm3syncbit(&self) -> FTM3SYNCBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FTM3SYNCBITR { bits }
    }
    #[doc = "Bits 4:5 - FTM1 CH0 Select"]
    #[inline]
    pub fn ftm1ch0sel(&self) -> FTM1CH0SELR {
        FTM1CH0SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - FTM2 CH0 Select"]
    #[inline]
    pub fn ftm2ch0sel(&self) -> FTM2CH0SELR {
        FTM2CH0SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - FTM2 CH1 Select"]
    #[inline]
    pub fn ftm2ch1sel(&self) -> FTM2CH1SELR {
        FTM2CH1SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - FTM global load enable"]
    #[inline]
    pub fn ftmgldok(&self) -> FTMGLDOKR {
        FTMGLDOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - FTM0 channel modulation select with FTM1_CH1"]
    #[inline]
    pub fn ftm0_outsel(&self) -> FTM0_OUTSELR {
        FTM0_OUTSELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - FTM3 channel modulation select with FTM2_CH1"]
    #[inline]
    pub fn ftm3_outsel(&self) -> FTM3_OUTSELR {
        FTM3_OUTSELR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - FTM0 Sync Bit"]
    #[inline]
    pub fn ftm0syncbit(&mut self) -> _FTM0SYNCBITW {
        _FTM0SYNCBITW { w: self }
    }
    #[doc = "Bit 1 - FTM1 Sync Bit"]
    #[inline]
    pub fn ftm1syncbit(&mut self) -> _FTM1SYNCBITW {
        _FTM1SYNCBITW { w: self }
    }
    #[doc = "Bit 2 - FTM2 Sync Bit"]
    #[inline]
    pub fn ftm2syncbit(&mut self) -> _FTM2SYNCBITW {
        _FTM2SYNCBITW { w: self }
    }
    #[doc = "Bit 3 - FTM3 Sync Bit"]
    #[inline]
    pub fn ftm3syncbit(&mut self) -> _FTM3SYNCBITW {
        _FTM3SYNCBITW { w: self }
    }
    #[doc = "Bits 4:5 - FTM1 CH0 Select"]
    #[inline]
    pub fn ftm1ch0sel(&mut self) -> _FTM1CH0SELW {
        _FTM1CH0SELW { w: self }
    }
    #[doc = "Bits 6:7 - FTM2 CH0 Select"]
    #[inline]
    pub fn ftm2ch0sel(&mut self) -> _FTM2CH0SELW {
        _FTM2CH0SELW { w: self }
    }
    #[doc = "Bit 8 - FTM2 CH1 Select"]
    #[inline]
    pub fn ftm2ch1sel(&mut self) -> _FTM2CH1SELW {
        _FTM2CH1SELW { w: self }
    }
    #[doc = "Bit 15 - FTM global load enable"]
    #[inline]
    pub fn ftmgldok(&mut self) -> _FTMGLDOKW {
        _FTMGLDOKW { w: self }
    }
    #[doc = "Bits 16:23 - FTM0 channel modulation select with FTM1_CH1"]
    #[inline]
    pub fn ftm0_outsel(&mut self) -> _FTM0_OUTSELW {
        _FTM0_OUTSELW { w: self }
    }
    #[doc = "Bits 24:31 - FTM3 channel modulation select with FTM2_CH1"]
    #[inline]
    pub fn ftm3_outsel(&mut self) -> _FTM3_OUTSELW {
        _FTM3_OUTSELW { w: self }
    }
}
