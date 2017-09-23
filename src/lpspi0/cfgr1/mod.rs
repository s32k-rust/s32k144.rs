#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR1 {
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
#[doc = "Possible values of the field `MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERR {
    #[doc = "Slave mode."] _0,
    #[doc = "Master mode."] _1,
}
impl MASTERR {
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
            MASTERR::_0 => false,
            MASTERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASTERR {
        match value {
            false => MASTERR::_0,
            true => MASTERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MASTERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MASTERR::_1
    }
}
#[doc = "Possible values of the field `SAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLER {
    #[doc = "Input data sampled on SCK edge."] _0,
    #[doc = "Input data sampled on delayed SCK edge."] _1,
}
impl SAMPLER {
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
            SAMPLER::_0 => false,
            SAMPLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAMPLER {
        match value {
            false => SAMPLER::_0,
            true => SAMPLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAMPLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAMPLER::_1
    }
}
#[doc = "Possible values of the field `AUTOPCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOPCSR {
    #[doc = "Automatic PCS generation disabled."] _0,
    #[doc = "Automatic PCS generation enabled."] _1,
}
impl AUTOPCSR {
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
            AUTOPCSR::_0 => false,
            AUTOPCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOPCSR {
        match value {
            false => AUTOPCSR::_0,
            true => AUTOPCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AUTOPCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AUTOPCSR::_1
    }
}
#[doc = "Possible values of the field `NOSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTALLR {
    #[doc = "Transfers will stall when transmit FIFO is empty or receive FIFO is full."] _0,
    #[doc = "Transfers will not stall, allowing transmit FIFO underrun or receive FIFO overrun to occur."]
    _1,
}
impl NOSTALLR {
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
            NOSTALLR::_0 => false,
            NOSTALLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOSTALLR {
        match value {
            false => NOSTALLR::_0,
            true => NOSTALLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NOSTALLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NOSTALLR::_1
    }
}
#[doc = "Possible values of the field `PCSPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSPOLR {
    #[doc = "The PCSx is active low."] _0000,
    #[doc = "The PCSx is active high."] _0001,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl PCSPOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSPOLR::_0000 => 0,
            PCSPOLR::_0001 => 1,
            PCSPOLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSPOLR {
        match value {
            0 => PCSPOLR::_0000,
            1 => PCSPOLR::_0001,
            i => PCSPOLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PCSPOLR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PCSPOLR::_0001
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Match is disabled."] _000,
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    _010,
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    _011,
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., [(1st data word = MATCH0) * (2nd data word = MATCH1)]"]
    _100,
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., [(any data word = MATCH0) * (next data word = MATCH1)]"]
    _101,
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(1st data word * MATCH1) = (MATCH0 * MATCH1)]"]
    _110,
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(any data word * MATCH1) = (MATCH0 * MATCH1)]"]
    _111,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::_000 => 0,
            MATCFGR::_010 => 2,
            MATCFGR::_011 => 3,
            MATCFGR::_100 => 4,
            MATCFGR::_101 => 5,
            MATCFGR::_110 => 6,
            MATCFGR::_111 => 7,
            MATCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::_000,
            2 => MATCFGR::_010,
            3 => MATCFGR::_011,
            4 => MATCFGR::_100,
            5 => MATCFGR::_101,
            6 => MATCFGR::_110,
            7 => MATCFGR::_111,
            i => MATCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MATCFGR::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == MATCFGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == MATCFGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == MATCFGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == MATCFGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == MATCFGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == MATCFGR::_111
    }
}
#[doc = "Possible values of the field `PINCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCFGR {
    #[doc = "SIN is used for input data and SOUT for output data."] _00,
    #[doc = "SIN is used for both input and output data."] _01,
    #[doc = "SOUT is used for both input and output data."] _10,
    #[doc = "SOUT is used for input data and SIN for output data."] _11,
}
impl PINCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCFGR::_00 => 0,
            PINCFGR::_01 => 1,
            PINCFGR::_10 => 2,
            PINCFGR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCFGR {
        match value {
            0 => PINCFGR::_00,
            1 => PINCFGR::_01,
            2 => PINCFGR::_10,
            3 => PINCFGR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PINCFGR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PINCFGR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PINCFGR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PINCFGR::_11
    }
}
#[doc = "Possible values of the field `OUTCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTCFGR {
    #[doc = "Output data retains last value when chip select is negated."] _0,
    #[doc = "Output data is tristated when chip select is negated."] _1,
}
impl OUTCFGR {
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
            OUTCFGR::_0 => false,
            OUTCFGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTCFGR {
        match value {
            false => OUTCFGR::_0,
            true => OUTCFGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OUTCFGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OUTCFGR::_1
    }
}
#[doc = "Possible values of the field `PCSCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSCFGR {
    #[doc = "PCS[3:2] are enabled."] _0,
    #[doc = "PCS[3:2] are disabled."] _1,
}
impl PCSCFGR {
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
            PCSCFGR::_0 => false,
            PCSCFGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PCSCFGR {
        match value {
            false => PCSCFGR::_0,
            true => PCSCFGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PCSCFGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PCSCFGR::_1
    }
}
#[doc = "Values that can be written to the field `MASTER`"]
pub enum MASTERW {
    #[doc = "Slave mode."] _0,
    #[doc = "Master mode."] _1,
}
impl MASTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MASTERW::_0 => false,
            MASTERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MASTERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MASTERW::_0)
    }
    #[doc = "Master mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MASTERW::_1)
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
#[doc = "Values that can be written to the field `SAMPLE`"]
pub enum SAMPLEW {
    #[doc = "Input data sampled on SCK edge."] _0,
    #[doc = "Input data sampled on delayed SCK edge."] _1,
}
impl SAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAMPLEW::_0 => false,
            SAMPLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input data sampled on SCK edge."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAMPLEW::_0)
    }
    #[doc = "Input data sampled on delayed SCK edge."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAMPLEW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AUTOPCS`"]
pub enum AUTOPCSW {
    #[doc = "Automatic PCS generation disabled."] _0,
    #[doc = "Automatic PCS generation enabled."] _1,
}
impl AUTOPCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOPCSW::_0 => false,
            AUTOPCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOPCSW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOPCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOPCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic PCS generation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOPCSW::_0)
    }
    #[doc = "Automatic PCS generation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOPCSW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NOSTALL`"]
pub enum NOSTALLW {
    #[doc = "Transfers will stall when transmit FIFO is empty or receive FIFO is full."] _0,
    #[doc = "Transfers will not stall, allowing transmit FIFO underrun or receive FIFO overrun to occur."]
    _1,
}
impl NOSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOSTALLW::_0 => false,
            NOSTALLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _NOSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfers will stall when transmit FIFO is empty or receive FIFO is full."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOSTALLW::_0)
    }
    #[doc = "Transfers will not stall, allowing transmit FIFO underrun or receive FIFO overrun to occur."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOSTALLW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCSPOL`"]
pub enum PCSPOLW {
    #[doc = "The PCSx is active low."] _0000,
    #[doc = "The PCSx is active high."] _0001,
}
impl PCSPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSPOLW::_0000 => 0,
            PCSPOLW::_0001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSPOLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The PCSx is active low."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PCSPOLW::_0000)
    }
    #[doc = "The PCSx is active high."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PCSPOLW::_0001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Match is disabled."] _000,
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    _010,
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    _011,
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., [(1st data word = MATCH0) * (2nd data word = MATCH1)]"]
    _100,
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., [(any data word = MATCH0) * (next data word = MATCH1)]"]
    _101,
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(1st data word * MATCH1) = (MATCH0 * MATCH1)]"]
    _110,
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(any data word * MATCH1) = (MATCH0 * MATCH1)]"]
    _111,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::_000 => 0,
            MATCFGW::_010 => 2,
            MATCFGW::_011 => 3,
            MATCFGW::_100 => 4,
            MATCFGW::_101 => 5,
            MATCFGW::_110 => 6,
            MATCFGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Match is disabled."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(MATCFGW::_000)
    }
    #[doc = "010b - Match is enabled, if 1st data word equals MATCH0 OR MATCH1, i.e., (1st data word = MATCH0 + MATCH1)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(MATCFGW::_010)
    }
    #[doc = "011b - Match is enabled, if any data word equals MATCH0 OR MATCH1, i.e., (any data word = MATCH0 + MATCH1)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(MATCFGW::_011)
    }
    #[doc = "100b - Match is enabled, if 1st data word equals MATCH0 AND 2nd data word equals MATCH1, i.e., [(1st data word = MATCH0) * (2nd data word = MATCH1)]"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(MATCFGW::_100)
    }
    #[doc = "101b - Match is enabled, if any data word equals MATCH0 AND the next data word equals MATCH1, i.e., [(any data word = MATCH0) * (next data word = MATCH1)]"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(MATCFGW::_101)
    }
    #[doc = "110b - Match is enabled, if (1st data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(1st data word * MATCH1) = (MATCH0 * MATCH1)]"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(MATCFGW::_110)
    }
    #[doc = "111b - Match is enabled, if (any data word AND MATCH1) equals (MATCH0 AND MATCH1), i.e., [(any data word * MATCH1) = (MATCH0 * MATCH1)]"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(MATCFGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCFG`"]
pub enum PINCFGW {
    #[doc = "SIN is used for input data and SOUT for output data."] _00,
    #[doc = "SIN is used for both input and output data."] _01,
    #[doc = "SOUT is used for both input and output data."] _10,
    #[doc = "SOUT is used for input data and SIN for output data."] _11,
}
impl PINCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINCFGW::_00 => 0,
            PINCFGW::_01 => 1,
            PINCFGW::_10 => 2,
            PINCFGW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SIN is used for input data and SOUT for output data."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PINCFGW::_00)
    }
    #[doc = "SIN is used for both input and output data."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PINCFGW::_01)
    }
    #[doc = "SOUT is used for both input and output data."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PINCFGW::_10)
    }
    #[doc = "SOUT is used for input data and SIN for output data."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PINCFGW::_11)
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
#[doc = "Values that can be written to the field `OUTCFG`"]
pub enum OUTCFGW {
    #[doc = "Output data retains last value when chip select is negated."] _0,
    #[doc = "Output data is tristated when chip select is negated."] _1,
}
impl OUTCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OUTCFGW::_0 => false,
            OUTCFGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Output data retains last value when chip select is negated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OUTCFGW::_0)
    }
    #[doc = "Output data is tristated when chip select is negated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OUTCFGW::_1)
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
#[doc = "Values that can be written to the field `PCSCFG`"]
pub enum PCSCFGW {
    #[doc = "PCS[3:2] are enabled."] _0,
    #[doc = "PCS[3:2] are disabled."] _1,
}
impl PCSCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PCSCFGW::_0 => false,
            PCSCFGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PCS[3:2] are enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSCFGW::_0)
    }
    #[doc = "PCS[3:2] are disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSCFGW::_1)
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
    #[doc = "Bit 0 - Master Mode"]
    #[inline]
    pub fn master(&self) -> MASTERR {
        MASTERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline]
    pub fn sample(&self) -> SAMPLER {
        SAMPLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline]
    pub fn autopcs(&self) -> AUTOPCSR {
        AUTOPCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline]
    pub fn nostall(&self) -> NOSTALLR {
        NOSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline]
    pub fn pcspol(&self) -> PCSPOLR {
        PCSPOLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Output Config"]
    #[inline]
    pub fn outcfg(&self) -> OUTCFGR {
        OUTCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline]
    pub fn pcscfg(&self) -> PCSCFGR {
        PCSCFGR::_from({
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
    #[doc = "Bit 0 - Master Mode"]
    #[inline]
    pub fn master(&mut self) -> _MASTERW {
        _MASTERW { w: self }
    }
    #[doc = "Bit 1 - Sample Point"]
    #[inline]
    pub fn sample(&mut self) -> _SAMPLEW {
        _SAMPLEW { w: self }
    }
    #[doc = "Bit 2 - Automatic PCS"]
    #[inline]
    pub fn autopcs(&mut self) -> _AUTOPCSW {
        _AUTOPCSW { w: self }
    }
    #[doc = "Bit 3 - No Stall"]
    #[inline]
    pub fn nostall(&mut self) -> _NOSTALLW {
        _NOSTALLW { w: self }
    }
    #[doc = "Bits 8:11 - Peripheral Chip Select Polarity"]
    #[inline]
    pub fn pcspol(&mut self) -> _PCSPOLW {
        _PCSPOLW { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bits 24:25 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
    #[doc = "Bit 26 - Output Config"]
    #[inline]
    pub fn outcfg(&mut self) -> _OUTCFGW {
        _OUTCFGW { w: self }
    }
    #[doc = "Bit 27 - Peripheral Chip Select Configuration"]
    #[inline]
    pub fn pcscfg(&mut self) -> _PCSCFGW {
        _PCSCFGW { w: self }
    }
}
