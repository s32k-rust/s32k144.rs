#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1_PN {
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
#[doc = "Possible values of the field `FCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCSR {
    #[doc = "Message ID filtering only"]
    _00,
    #[doc = "Message ID filtering and payload filtering"]
    _01,
    #[doc = "Message ID filtering occurring a specified number of times."]
    _10,
    #[doc = "Message ID filtering and payload filtering a specified number of times"]
    _11,
}
impl FCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCSR::_00 => 0,
            FCSR::_01 => 1,
            FCSR::_10 => 2,
            FCSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCSR {
        match value {
            0 => FCSR::_00,
            1 => FCSR::_01,
            2 => FCSR::_10,
            3 => FCSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FCSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FCSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FCSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FCSR::_11
    }
}
#[doc = "Possible values of the field `IDFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDFSR {
    #[doc = "Match upon a ID contents against an exact target value"]
    _00,
    #[doc = "Match upon a ID value greater than or equal to a specified target value"]
    _01,
    #[doc = "Match upon a ID value smaller than or equal to a specified target value"]
    _10,
    #[doc = "Match upon a ID value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    _11,
}
impl IDFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDFSR::_00 => 0,
            IDFSR::_01 => 1,
            IDFSR::_10 => 2,
            IDFSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDFSR {
        match value {
            0 => IDFSR::_00,
            1 => IDFSR::_01,
            2 => IDFSR::_10,
            3 => IDFSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == IDFSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == IDFSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == IDFSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == IDFSR::_11
    }
}
#[doc = "Possible values of the field `PLFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLFSR {
    #[doc = "Match upon a payload contents against an exact target value"]
    _00,
    #[doc = "Match upon a payload value greater than or equal to a specified target value"]
    _01,
    #[doc = "Match upon a payload value smaller than or equal to a specified target value"]
    _10,
    #[doc = "Match upon a payload value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    _11,
}
impl PLFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLFSR::_00 => 0,
            PLFSR::_01 => 1,
            PLFSR::_10 => 2,
            PLFSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLFSR {
        match value {
            0 => PLFSR::_00,
            1 => PLFSR::_01,
            2 => PLFSR::_10,
            3 => PLFSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PLFSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PLFSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PLFSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PLFSR::_11
    }
}
#[doc = "Possible values of the field `NMATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMATCHR {
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL once before generating a wake up event."]
    _00000001,
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wake up event."]
    _00000010,
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wake up event."]
    _11111111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NMATCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NMATCHR::_00000001 => 1,
            NMATCHR::_00000010 => 2,
            NMATCHR::_11111111 => 255,
            NMATCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NMATCHR {
        match value {
            1 => NMATCHR::_00000001,
            2 => NMATCHR::_00000010,
            255 => NMATCHR::_11111111,
            i => NMATCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline]
    pub fn is_00000001(&self) -> bool {
        *self == NMATCHR::_00000001
    }
    #[doc = "Checks if the value of the field is `_00000010`"]
    #[inline]
    pub fn is_00000010(&self) -> bool {
        *self == NMATCHR::_00000010
    }
    #[doc = "Checks if the value of the field is `_11111111`"]
    #[inline]
    pub fn is_11111111(&self) -> bool {
        *self == NMATCHR::_11111111
    }
}
#[doc = "Possible values of the field `WUMF_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUMF_MSKR {
    #[doc = "Wake up match event is disabled"]
    _0,
    #[doc = "Wake up match event is enabled"]
    _1,
}
impl WUMF_MSKR {
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
            WUMF_MSKR::_0 => false,
            WUMF_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUMF_MSKR {
        match value {
            false => WUMF_MSKR::_0,
            true => WUMF_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUMF_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUMF_MSKR::_1
    }
}
#[doc = "Possible values of the field `WTOF_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTOF_MSKR {
    #[doc = "Timeout wake up event is disabled"]
    _0,
    #[doc = "Timeout wake up event is enabled"]
    _1,
}
impl WTOF_MSKR {
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
            WTOF_MSKR::_0 => false,
            WTOF_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTOF_MSKR {
        match value {
            false => WTOF_MSKR::_0,
            true => WTOF_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WTOF_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WTOF_MSKR::_1
    }
}
#[doc = "Values that can be written to the field `FCS`"]
pub enum FCSW {
    #[doc = "Message ID filtering only"]
    _00,
    #[doc = "Message ID filtering and payload filtering"]
    _01,
    #[doc = "Message ID filtering occurring a specified number of times."]
    _10,
    #[doc = "Message ID filtering and payload filtering a specified number of times"]
    _11,
}
impl FCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCSW::_00 => 0,
            FCSW::_01 => 1,
            FCSW::_10 => 2,
            FCSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCSW<'a> {
    w: &'a mut W,
}
impl<'a> _FCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Message ID filtering only"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCSW::_00)
    }
    #[doc = "Message ID filtering and payload filtering"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCSW::_01)
    }
    #[doc = "Message ID filtering occurring a specified number of times."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCSW::_10)
    }
    #[doc = "Message ID filtering and payload filtering a specified number of times"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDFS`"]
pub enum IDFSW {
    #[doc = "Match upon a ID contents against an exact target value"]
    _00,
    #[doc = "Match upon a ID value greater than or equal to a specified target value"]
    _01,
    #[doc = "Match upon a ID value smaller than or equal to a specified target value"]
    _10,
    #[doc = "Match upon a ID value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    _11,
}
impl IDFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDFSW::_00 => 0,
            IDFSW::_01 => 1,
            IDFSW::_10 => 2,
            IDFSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDFSW<'a> {
    w: &'a mut W,
}
impl<'a> _IDFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDFSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Match upon a ID contents against an exact target value"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDFSW::_00)
    }
    #[doc = "Match upon a ID value greater than or equal to a specified target value"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDFSW::_01)
    }
    #[doc = "Match upon a ID value smaller than or equal to a specified target value"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDFSW::_10)
    }
    #[doc = "Match upon a ID value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDFSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLFS`"]
pub enum PLFSW {
    #[doc = "Match upon a payload contents against an exact target value"]
    _00,
    #[doc = "Match upon a payload value greater than or equal to a specified target value"]
    _01,
    #[doc = "Match upon a payload value smaller than or equal to a specified target value"]
    _10,
    #[doc = "Match upon a payload value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    _11,
}
impl PLFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLFSW::_00 => 0,
            PLFSW::_01 => 1,
            PLFSW::_10 => 2,
            PLFSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLFSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLFSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Match upon a payload contents against an exact target value"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLFSW::_00)
    }
    #[doc = "Match upon a payload value greater than or equal to a specified target value"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLFSW::_01)
    }
    #[doc = "Match upon a payload value smaller than or equal to a specified target value"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PLFSW::_10)
    }
    #[doc = "Match upon a payload value inside a range, greater than or equal to a specified lower limit and smaller than or equal a specified upper limit"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLFSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NMATCH`"]
pub enum NMATCHW {
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL once before generating a wake up event."]
    _00000001,
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wake up event."]
    _00000010,
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wake up event."]
    _11111111,
}
impl NMATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NMATCHW::_00000001 => 1,
            NMATCHW::_00000010 => 2,
            NMATCHW::_11111111 => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _NMATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NMATCHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL once before generating a wake up event."]
    #[inline]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(NMATCHW::_00000001)
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wake up event."]
    #[inline]
    pub fn _00000010(self) -> &'a mut W {
        self.variant(NMATCHW::_00000010)
    }
    #[doc = "Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wake up event."]
    #[inline]
    pub fn _11111111(self) -> &'a mut W {
        self.variant(NMATCHW::_11111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUMF_MSK`"]
pub enum WUMF_MSKW {
    #[doc = "Wake up match event is disabled"]
    _0,
    #[doc = "Wake up match event is enabled"]
    _1,
}
impl WUMF_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUMF_MSKW::_0 => false,
            WUMF_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUMF_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _WUMF_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUMF_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake up match event is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUMF_MSKW::_0)
    }
    #[doc = "Wake up match event is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUMF_MSKW::_1)
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
#[doc = "Values that can be written to the field `WTOF_MSK`"]
pub enum WTOF_MSKW {
    #[doc = "Timeout wake up event is disabled"]
    _0,
    #[doc = "Timeout wake up event is enabled"]
    _1,
}
impl WTOF_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTOF_MSKW::_0 => false,
            WTOF_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTOF_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _WTOF_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTOF_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timeout wake up event is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTOF_MSKW::_0)
    }
    #[doc = "Timeout wake up event is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTOF_MSKW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Filtering Combination Selection"]
    #[inline]
    pub fn fcs(&self) -> FCSR {
        FCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - ID Filtering Selection"]
    #[inline]
    pub fn idfs(&self) -> IDFSR {
        IDFSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Payload Filtering Selection"]
    #[inline]
    pub fn plfs(&self) -> PLFSR {
        PLFSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Number of Messages Matching the Same Filtering Criteria"]
    #[inline]
    pub fn nmatch(&self) -> NMATCHR {
        NMATCHR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Mask Bit"]
    #[inline]
    pub fn wumf_msk(&self) -> WUMF_MSKR {
        WUMF_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Mask Bit"]
    #[inline]
    pub fn wtof_msk(&self) -> WTOF_MSKR {
        WTOF_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Filtering Combination Selection"]
    #[inline]
    pub fn fcs(&mut self) -> _FCSW {
        _FCSW { w: self }
    }
    #[doc = "Bits 2:3 - ID Filtering Selection"]
    #[inline]
    pub fn idfs(&mut self) -> _IDFSW {
        _IDFSW { w: self }
    }
    #[doc = "Bits 4:5 - Payload Filtering Selection"]
    #[inline]
    pub fn plfs(&mut self) -> _PLFSW {
        _PLFSW { w: self }
    }
    #[doc = "Bits 8:15 - Number of Messages Matching the Same Filtering Criteria"]
    #[inline]
    pub fn nmatch(&mut self) -> _NMATCHW {
        _NMATCHW { w: self }
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Mask Bit"]
    #[inline]
    pub fn wumf_msk(&mut self) -> _WUMF_MSKW {
        _WUMF_MSKW { w: self }
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Mask Bit"]
    #[inline]
    pub fn wtof_msk(&mut self) -> _WTOF_MSKW {
        _WTOF_MSKW { w: self }
    }
}
