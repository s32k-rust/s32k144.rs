#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FTMOPT0 {
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
#[doc = "Possible values of the field `FTM0FLTxSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLTXSELR {
    #[doc = "FTM0_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM0 out"]
    _001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM0FLTXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM0FLTXSELR::_000 => 0,
            FTM0FLTXSELR::_001 => 1,
            FTM0FLTXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM0FLTXSELR {
        match value {
            0 => FTM0FLTXSELR::_000,
            1 => FTM0FLTXSELR::_001,
            i => FTM0FLTXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FTM0FLTXSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FTM0FLTXSELR::_001
    }
}
#[doc = "Possible values of the field `FTM1FLTxSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1FLTXSELR {
    #[doc = "FTM1_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM1 out"]
    _001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM1FLTXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM1FLTXSELR::_000 => 0,
            FTM1FLTXSELR::_001 => 1,
            FTM1FLTXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM1FLTXSELR {
        match value {
            0 => FTM1FLTXSELR::_000,
            1 => FTM1FLTXSELR::_001,
            i => FTM1FLTXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FTM1FLTXSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FTM1FLTXSELR::_001
    }
}
#[doc = "Possible values of the field `FTM2FLTxSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2FLTXSELR {
    #[doc = "FTM2_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM2 out"]
    _001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM2FLTXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM2FLTXSELR::_000 => 0,
            FTM2FLTXSELR::_001 => 1,
            FTM2FLTXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM2FLTXSELR {
        match value {
            0 => FTM2FLTXSELR::_000,
            1 => FTM2FLTXSELR::_001,
            i => FTM2FLTXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FTM2FLTXSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FTM2FLTXSELR::_001
    }
}
#[doc = "Possible values of the field `FTM3FLTxSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3FLTXSELR {
    #[doc = "FTM3_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM3 out"]
    _001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM3FLTXSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM3FLTXSELR::_000 => 0,
            FTM3FLTXSELR::_001 => 1,
            FTM3FLTXSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM3FLTXSELR {
        match value {
            0 => FTM3FLTXSELR::_000,
            1 => FTM3FLTXSELR::_001,
            i => FTM3FLTXSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FTM3FLTXSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FTM3FLTXSELR::_001
    }
}
#[doc = "Possible values of the field `FTM0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0CLKSELR {
    #[doc = "FTM0 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM0 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM0 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM0CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM0CLKSELR::_00 => 0,
            FTM0CLKSELR::_01 => 1,
            FTM0CLKSELR::_10 => 2,
            FTM0CLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM0CLKSELR {
        match value {
            0 => FTM0CLKSELR::_00,
            1 => FTM0CLKSELR::_01,
            2 => FTM0CLKSELR::_10,
            3 => FTM0CLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM0CLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM0CLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FTM0CLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FTM0CLKSELR::_11
    }
}
#[doc = "Possible values of the field `FTM1CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1CLKSELR {
    #[doc = "FTM1 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM1 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM1 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM1CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM1CLKSELR::_00 => 0,
            FTM1CLKSELR::_01 => 1,
            FTM1CLKSELR::_10 => 2,
            FTM1CLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM1CLKSELR {
        match value {
            0 => FTM1CLKSELR::_00,
            1 => FTM1CLKSELR::_01,
            2 => FTM1CLKSELR::_10,
            3 => FTM1CLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM1CLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM1CLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FTM1CLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FTM1CLKSELR::_11
    }
}
#[doc = "Possible values of the field `FTM2CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CLKSELR {
    #[doc = "FTM2 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM2 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM2 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM2CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM2CLKSELR::_00 => 0,
            FTM2CLKSELR::_01 => 1,
            FTM2CLKSELR::_10 => 2,
            FTM2CLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM2CLKSELR {
        match value {
            0 => FTM2CLKSELR::_00,
            1 => FTM2CLKSELR::_01,
            2 => FTM2CLKSELR::_10,
            3 => FTM2CLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM2CLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM2CLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FTM2CLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FTM2CLKSELR::_11
    }
}
#[doc = "Possible values of the field `FTM3CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3CLKSELR {
    #[doc = "FTM3 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM3 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM3 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM3CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM3CLKSELR::_00 => 0,
            FTM3CLKSELR::_01 => 1,
            FTM3CLKSELR::_10 => 2,
            FTM3CLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM3CLKSELR {
        match value {
            0 => FTM3CLKSELR::_00,
            1 => FTM3CLKSELR::_01,
            2 => FTM3CLKSELR::_10,
            3 => FTM3CLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM3CLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM3CLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FTM3CLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FTM3CLKSELR::_11
    }
}
#[doc = "Values that can be written to the field `FTM0FLTxSEL`"]
pub enum FTM0FLTXSELW {
    #[doc = "FTM0_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM0 out"]
    _001,
}
impl FTM0FLTXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM0FLTXSELW::_000 => 0,
            FTM0FLTXSELW::_001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0FLTXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0FLTXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0FLTXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM0_FLTx pin"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM0FLTXSELW::_000)
    }
    #[doc = "TRGMUX_FTM0 out"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM0FLTXSELW::_001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM1FLTxSEL`"]
pub enum FTM1FLTXSELW {
    #[doc = "FTM1_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM1 out"]
    _001,
}
impl FTM1FLTXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM1FLTXSELW::_000 => 0,
            FTM1FLTXSELW::_001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1FLTXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1FLTXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1FLTXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM1_FLTx pin"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM1FLTXSELW::_000)
    }
    #[doc = "TRGMUX_FTM1 out"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM1FLTXSELW::_001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2FLTxSEL`"]
pub enum FTM2FLTXSELW {
    #[doc = "FTM2_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM2 out"]
    _001,
}
impl FTM2FLTXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM2FLTXSELW::_000 => 0,
            FTM2FLTXSELW::_001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2FLTXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2FLTXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2FLTXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM2_FLTx pin"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM2FLTXSELW::_000)
    }
    #[doc = "TRGMUX_FTM2 out"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM2FLTXSELW::_001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM3FLTxSEL`"]
pub enum FTM3FLTXSELW {
    #[doc = "FTM3_FLTx pin"]
    _000,
    #[doc = "TRGMUX_FTM3 out"]
    _001,
}
impl FTM3FLTXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM3FLTXSELW::_000 => 0,
            FTM3FLTXSELW::_001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3FLTXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3FLTXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3FLTXSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM3_FLTx pin"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM3FLTXSELW::_000)
    }
    #[doc = "TRGMUX_FTM3 out"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM3FLTXSELW::_001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM0CLKSEL`"]
pub enum FTM0CLKSELW {
    #[doc = "FTM0 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM0 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM0 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM0CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM0CLKSELW::_00 => 0,
            FTM0CLKSELW::_01 => 1,
            FTM0CLKSELW::_10 => 2,
            FTM0CLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FTM0 external clock driven by TCLK0 pin."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM0CLKSELW::_00)
    }
    #[doc = "FTM0 external clock driven by TCLK1 pin."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM0CLKSELW::_01)
    }
    #[doc = "FTM0 external clock driven by TCLK2 pin."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM0CLKSELW::_10)
    }
    #[doc = "No clock input"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM0CLKSELW::_11)
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
#[doc = "Values that can be written to the field `FTM1CLKSEL`"]
pub enum FTM1CLKSELW {
    #[doc = "FTM1 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM1 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM1 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM1CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM1CLKSELW::_00 => 0,
            FTM1CLKSELW::_01 => 1,
            FTM1CLKSELW::_10 => 2,
            FTM1CLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FTM1 external clock driven by TCLK0 pin."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CLKSELW::_00)
    }
    #[doc = "FTM1 external clock driven by TCLK1 pin."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CLKSELW::_01)
    }
    #[doc = "FTM1 external clock driven by TCLK2 pin."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CLKSELW::_10)
    }
    #[doc = "No clock input"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM1CLKSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2CLKSEL`"]
pub enum FTM2CLKSELW {
    #[doc = "FTM2 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM2 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM2 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM2CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM2CLKSELW::_00 => 0,
            FTM2CLKSELW::_01 => 1,
            FTM2CLKSELW::_10 => 2,
            FTM2CLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FTM2 external clock driven by TCLK0 pin."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CLKSELW::_00)
    }
    #[doc = "FTM2 external clock driven by TCLK1 pin."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CLKSELW::_01)
    }
    #[doc = "FTM2 external clock driven by TCLK2 pin."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CLKSELW::_10)
    }
    #[doc = "No clock input"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM2CLKSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM3CLKSEL`"]
pub enum FTM3CLKSELW {
    #[doc = "FTM3 external clock driven by TCLK0 pin."]
    _00,
    #[doc = "FTM3 external clock driven by TCLK1 pin."]
    _01,
    #[doc = "FTM3 external clock driven by TCLK2 pin."]
    _10,
    #[doc = "No clock input"]
    _11,
}
impl FTM3CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM3CLKSELW::_00 => 0,
            FTM3CLKSELW::_01 => 1,
            FTM3CLKSELW::_10 => 2,
            FTM3CLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FTM3 external clock driven by TCLK0 pin."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM3CLKSELW::_00)
    }
    #[doc = "FTM3 external clock driven by TCLK1 pin."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM3CLKSELW::_01)
    }
    #[doc = "FTM3 external clock driven by TCLK2 pin."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM3CLKSELW::_10)
    }
    #[doc = "No clock input"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM3CLKSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - FTM0 Fault X Select"]
    #[inline]
    pub fn ftm0fltx_sel(&self) -> FTM0FLTXSELR {
        FTM0FLTXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - FTM1 Fault X Select"]
    #[inline]
    pub fn ftm1fltx_sel(&self) -> FTM1FLTXSELR {
        FTM1FLTXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - FTM2 Fault X Select"]
    #[inline]
    pub fn ftm2fltx_sel(&self) -> FTM2FLTXSELR {
        FTM2FLTXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - FTM3 Fault X Select"]
    #[inline]
    pub fn ftm3fltx_sel(&self) -> FTM3FLTXSELR {
        FTM3FLTXSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - FTM0 External Clock Pin Select"]
    #[inline]
    pub fn ftm0clksel(&self) -> FTM0CLKSELR {
        FTM0CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - FTM1 External Clock Pin Select"]
    #[inline]
    pub fn ftm1clksel(&self) -> FTM1CLKSELR {
        FTM1CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - FTM2 External Clock Pin Select"]
    #[inline]
    pub fn ftm2clksel(&self) -> FTM2CLKSELR {
        FTM2CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - FTM3 External Clock Pin Select"]
    #[inline]
    pub fn ftm3clksel(&self) -> FTM3CLKSELR {
        FTM3CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - FTM0 Fault X Select"]
    #[inline]
    pub fn ftm0fltx_sel(&mut self) -> _FTM0FLTXSELW {
        _FTM0FLTXSELW { w: self }
    }
    #[doc = "Bits 4:6 - FTM1 Fault X Select"]
    #[inline]
    pub fn ftm1fltx_sel(&mut self) -> _FTM1FLTXSELW {
        _FTM1FLTXSELW { w: self }
    }
    #[doc = "Bits 8:10 - FTM2 Fault X Select"]
    #[inline]
    pub fn ftm2fltx_sel(&mut self) -> _FTM2FLTXSELW {
        _FTM2FLTXSELW { w: self }
    }
    #[doc = "Bits 12:14 - FTM3 Fault X Select"]
    #[inline]
    pub fn ftm3fltx_sel(&mut self) -> _FTM3FLTXSELW {
        _FTM3FLTXSELW { w: self }
    }
    #[doc = "Bits 24:25 - FTM0 External Clock Pin Select"]
    #[inline]
    pub fn ftm0clksel(&mut self) -> _FTM0CLKSELW {
        _FTM0CLKSELW { w: self }
    }
    #[doc = "Bits 26:27 - FTM1 External Clock Pin Select"]
    #[inline]
    pub fn ftm1clksel(&mut self) -> _FTM1CLKSELW {
        _FTM1CLKSELW { w: self }
    }
    #[doc = "Bits 28:29 - FTM2 External Clock Pin Select"]
    #[inline]
    pub fn ftm2clksel(&mut self) -> _FTM2CLKSELW {
        _FTM2CLKSELW { w: self }
    }
    #[doc = "Bits 30:31 - FTM3 External Clock Pin Select"]
    #[inline]
    pub fn ftm3clksel(&mut self) -> _FTM3CLKSELW {
        _FTM3CLKSELW { w: self }
    }
}
