#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2 {
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
#[doc = "Possible values of the field `EDFLTDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFLTDISR {
    #[doc = "Edge Filter is enabled."]
    _0,
    #[doc = "Edge Filter is disabled."]
    _1,
}
impl EDFLTDISR {
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
            EDFLTDISR::_0 => false,
            EDFLTDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDFLTDISR {
        match value {
            false => EDFLTDISR::_0,
            true => EDFLTDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDFLTDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDFLTDISR::_1
    }
}
#[doc = "Possible values of the field `ISOCANFDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOCANFDENR {
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    _0,
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    _1,
}
impl ISOCANFDENR {
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
            ISOCANFDENR::_0 => false,
            ISOCANFDENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISOCANFDENR {
        match value {
            false => ISOCANFDENR::_0,
            true => ISOCANFDENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ISOCANFDENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ISOCANFDENR::_1
    }
}
#[doc = "Possible values of the field `PREXCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREXCENR {
    #[doc = "Protocol Exception is disabled."]
    _0,
    #[doc = "Protocol Exception is enabled."]
    _1,
}
impl PREXCENR {
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
            PREXCENR::_0 => false,
            PREXCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PREXCENR {
        match value {
            false => PREXCENR::_0,
            true => PREXCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PREXCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PREXCENR::_1
    }
}
#[doc = "Possible values of the field `TIMER_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER_SRCR {
    #[doc = "The Free Running Timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    _0,
    #[doc = "The Free Running Timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device specific section for details about the external time tick."]
    _1,
}
impl TIMER_SRCR {
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
            TIMER_SRCR::_0 => false,
            TIMER_SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER_SRCR {
        match value {
            false => TIMER_SRCR::_0,
            true => TIMER_SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMER_SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMER_SRCR::_1
    }
}
#[doc = "Possible values of the field `EACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EACENR {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1,
}
impl EACENR {
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
            EACENR::_0 => false,
            EACENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EACENR {
        match value {
            false => EACENR::_0,
            true => EACENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EACENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EACENR::_1
    }
}
#[doc = "Possible values of the field `RRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRSR {
    #[doc = "Remote Response Frame is generated."]
    _0,
    #[doc = "Remote Request Frame is stored."]
    _1,
}
impl RRSR {
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
            RRSR::_0 => false,
            RRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRSR {
        match value {
            false => RRSR::_0,
            true => RRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RRSR::_1
    }
}
#[doc = "Possible values of the field `MRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRPR {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    _0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    _1,
}
impl MRPR {
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
            MRPR::_0 => false,
            MRPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRPR {
        match value {
            false => MRPR::_0,
            true => MRPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MRPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MRPR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TASDR {
    bits: u8,
}
impl TASDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFFNR {
    bits: u8,
}
impl RFFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BOFFDONEMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFDONEMSKR {
    #[doc = "Bus Off Done interrupt disabled."]
    _0,
    #[doc = "Bus Off Done interrupt enabled."]
    _1,
}
impl BOFFDONEMSKR {
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
            BOFFDONEMSKR::_0 => false,
            BOFFDONEMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFDONEMSKR {
        match value {
            false => BOFFDONEMSKR::_0,
            true => BOFFDONEMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOFFDONEMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOFFDONEMSKR::_1
    }
}
#[doc = "Possible values of the field `ERRMSK_FAST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSK_FASTR {
    #[doc = "ERRINT_FAST Error interrupt disabled."]
    _0,
    #[doc = "ERRINT_FAST Error interrupt enabled."]
    _1,
}
impl ERRMSK_FASTR {
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
            ERRMSK_FASTR::_0 => false,
            ERRMSK_FASTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRMSK_FASTR {
        match value {
            false => ERRMSK_FASTR::_0,
            true => ERRMSK_FASTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERRMSK_FASTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERRMSK_FASTR::_1
    }
}
#[doc = "Values that can be written to the field `EDFLTDIS`"]
pub enum EDFLTDISW {
    #[doc = "Edge Filter is enabled."]
    _0,
    #[doc = "Edge Filter is disabled."]
    _1,
}
impl EDFLTDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDFLTDISW::_0 => false,
            EDFLTDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDFLTDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EDFLTDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDFLTDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge Filter is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDFLTDISW::_0)
    }
    #[doc = "Edge Filter is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDFLTDISW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ISOCANFDEN`"]
pub enum ISOCANFDENW {
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    _0,
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    _1,
}
impl ISOCANFDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISOCANFDENW::_0 => false,
            ISOCANFDENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISOCANFDENW<'a> {
    w: &'a mut W,
}
impl<'a> _ISOCANFDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISOCANFDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISOCANFDENW::_0)
    }
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISOCANFDENW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PREXCEN`"]
pub enum PREXCENW {
    #[doc = "Protocol Exception is disabled."]
    _0,
    #[doc = "Protocol Exception is enabled."]
    _1,
}
impl PREXCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PREXCENW::_0 => false,
            PREXCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PREXCENW<'a> {
    w: &'a mut W,
}
impl<'a> _PREXCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREXCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Protocol Exception is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PREXCENW::_0)
    }
    #[doc = "Protocol Exception is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PREXCENW::_1)
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
#[doc = "Values that can be written to the field `TIMER_SRC`"]
pub enum TIMER_SRCW {
    #[doc = "The Free Running Timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    _0,
    #[doc = "The Free Running Timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device specific section for details about the external time tick."]
    _1,
}
impl TIMER_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER_SRCW::_0 => false,
            TIMER_SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Free Running Timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMER_SRCW::_0)
    }
    #[doc = "The Free Running Timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device specific section for details about the external time tick."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMER_SRCW::_1)
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
#[doc = "Values that can be written to the field `EACEN`"]
pub enum EACENW {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1,
}
impl EACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EACENW::_0 => false,
            EACENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EACENW<'a> {
    w: &'a mut W,
}
impl<'a> _EACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EACENW::_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EACENW::_1)
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
#[doc = "Values that can be written to the field `RRS`"]
pub enum RRSW {
    #[doc = "Remote Response Frame is generated."]
    _0,
    #[doc = "Remote Request Frame is stored."]
    _1,
}
impl RRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRSW::_0 => false,
            RRSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRSW<'a> {
    w: &'a mut W,
}
impl<'a> _RRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Remote Response Frame is generated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRSW::_0)
    }
    #[doc = "Remote Request Frame is stored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRSW::_1)
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
#[doc = "Values that can be written to the field `MRP`"]
pub enum MRPW {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    _0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    _1,
}
impl MRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRPW::_0 => false,
            MRPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRPW<'a> {
    w: &'a mut W,
}
impl<'a> _MRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRPW::_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRPW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TASDW<'a> {
    w: &'a mut W,
}
impl<'a> _TASDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFFNW<'a> {
    w: &'a mut W,
}
impl<'a> _RFFNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOFFDONEMSK`"]
pub enum BOFFDONEMSKW {
    #[doc = "Bus Off Done interrupt disabled."]
    _0,
    #[doc = "Bus Off Done interrupt enabled."]
    _1,
}
impl BOFFDONEMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFDONEMSKW::_0 => false,
            BOFFDONEMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFDONEMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFDONEMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFDONEMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus Off Done interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFDONEMSKW::_0)
    }
    #[doc = "Bus Off Done interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFDONEMSKW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERRMSK_FAST`"]
pub enum ERRMSK_FASTW {
    #[doc = "ERRINT_FAST Error interrupt disabled."]
    _0,
    #[doc = "ERRINT_FAST Error interrupt enabled."]
    _1,
}
impl ERRMSK_FASTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRMSK_FASTW::_0 => false,
            ERRMSK_FASTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRMSK_FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRMSK_FASTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRMSK_FASTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ERRINT_FAST Error interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRMSK_FASTW::_0)
    }
    #[doc = "ERRINT_FAST Error interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRMSK_FASTW::_1)
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
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline]
    pub fn edfltdis(&self) -> EDFLTDISR {
        EDFLTDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline]
    pub fn isocanfden(&self) -> ISOCANFDENR {
        ISOCANFDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline]
    pub fn prexcen(&self) -> PREXCENR {
        PREXCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Timer Source"]
    #[inline]
    pub fn timer_src(&self) -> TIMER_SRCR {
        TIMER_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline]
    pub fn eacen(&self) -> EACENR {
        EACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline]
    pub fn rrs(&self) -> RRSR {
        RRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline]
    pub fn mrp(&self) -> MRPR {
        MRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline]
    pub fn tasd(&self) -> TASDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TASDR { bits }
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline]
    pub fn rffn(&self) -> RFFNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFFNR { bits }
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline]
    pub fn boffdonemsk(&self) -> BOFFDONEMSKR {
        BOFFDONEMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames"]
    #[inline]
    pub fn errmsk_fast(&self) -> ERRMSK_FASTR {
        ERRMSK_FASTR::_from({
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
        W { bits: 10485760 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline]
    pub fn edfltdis(&mut self) -> _EDFLTDISW {
        _EDFLTDISW { w: self }
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline]
    pub fn isocanfden(&mut self) -> _ISOCANFDENW {
        _ISOCANFDENW { w: self }
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline]
    pub fn prexcen(&mut self) -> _PREXCENW {
        _PREXCENW { w: self }
    }
    #[doc = "Bit 15 - Timer Source"]
    #[inline]
    pub fn timer_src(&mut self) -> _TIMER_SRCW {
        _TIMER_SRCW { w: self }
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline]
    pub fn eacen(&mut self) -> _EACENW {
        _EACENW { w: self }
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline]
    pub fn rrs(&mut self) -> _RRSW {
        _RRSW { w: self }
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline]
    pub fn mrp(&mut self) -> _MRPW {
        _MRPW { w: self }
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline]
    pub fn tasd(&mut self) -> _TASDW {
        _TASDW { w: self }
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline]
    pub fn rffn(&mut self) -> _RFFNW {
        _RFFNW { w: self }
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline]
    pub fn boffdonemsk(&mut self) -> _BOFFDONEMSKW {
        _BOFFDONEMSKW { w: self }
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the Data Phase of fast CAN FD frames"]
    #[inline]
    pub fn errmsk_fast(&mut self) -> _ERRMSK_FASTW {
        _ERRMSK_FASTW { w: self }
    }
}
