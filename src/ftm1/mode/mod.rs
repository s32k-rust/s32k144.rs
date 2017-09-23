#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
#[doc = "Possible values of the field `FTMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTMENR {
    #[doc = "TPM compatibility. Free running counter and synchronization compatible with TPM."] _0,
    #[doc = "Free running counter and synchronization are different from TPM behavior."] _1,
}
impl FTMENR {
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
            FTMENR::_0 => false,
            FTMENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTMENR {
        match value {
            false => FTMENR::_0,
            true => FTMENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTMENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTMENR::_1
    }
}
#[doc = "Possible values of the field `WPDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPDISR {
    #[doc = "Write protection is enabled."] _0,
    #[doc = "Write protection is disabled."] _1,
}
impl WPDISR {
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
            WPDISR::_0 => false,
            WPDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPDISR {
        match value {
            false => WPDISR::_0,
            true => WPDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPDISR::_1
    }
}
#[doc = "Possible values of the field `PWMSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSYNCR {
    #[doc = "No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    _0,
    #[doc = "Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    _1,
}
impl PWMSYNCR {
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
            PWMSYNCR::_0 => false,
            PWMSYNCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMSYNCR {
        match value {
            false => PWMSYNCR::_0,
            true => PWMSYNCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMSYNCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMSYNCR::_1
    }
}
#[doc = "Possible values of the field `CAPTEST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTESTR {
    #[doc = "Capture test mode is disabled."] _0,
    #[doc = "Capture test mode is enabled."] _1,
}
impl CAPTESTR {
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
            CAPTESTR::_0 => false,
            CAPTESTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTESTR {
        match value {
            false => CAPTESTR::_0,
            true => CAPTESTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CAPTESTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CAPTESTR::_1
    }
}
#[doc = "Possible values of the field `FAULTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTMR {
    #[doc = "Fault control is disabled for all channels."] _00,
    #[doc = "Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    _01,
    #[doc = "Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    _10,
    #[doc = "Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    _11,
}
impl FAULTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FAULTMR::_00 => 0,
            FAULTMR::_01 => 1,
            FAULTMR::_10 => 2,
            FAULTMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FAULTMR {
        match value {
            0 => FAULTMR::_00,
            1 => FAULTMR::_01,
            2 => FAULTMR::_10,
            3 => FAULTMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FAULTMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FAULTMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FAULTMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FAULTMR::_11
    }
}
#[doc = "Possible values of the field `FAULTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTIER {
    #[doc = "Fault control interrupt is disabled."] _0,
    #[doc = "Fault control interrupt is enabled."] _1,
}
impl FAULTIER {
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
            FAULTIER::_0 => false,
            FAULTIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTIER {
        match value {
            false => FAULTIER::_0,
            true => FAULTIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTIER::_1
    }
}
#[doc = "Values that can be written to the field `FTMEN`"]
pub enum FTMENW {
    #[doc = "TPM compatibility. Free running counter and synchronization compatible with TPM."] _0,
    #[doc = "Free running counter and synchronization are different from TPM behavior."] _1,
}
impl FTMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTMENW::_0 => false,
            FTMENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTMENW<'a> {
    w: &'a mut W,
}
impl<'a> _FTMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM compatibility. Free running counter and synchronization compatible with TPM."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTMENW::_0)
    }
    #[doc = "Free running counter and synchronization are different from TPM behavior."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTMENW::_1)
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
pub struct _INITW<'a> {
    w: &'a mut W,
}
impl<'a> _INITW<'a> {
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
#[doc = "Values that can be written to the field `WPDIS`"]
pub enum WPDISW {
    #[doc = "Write protection is enabled."] _0,
    #[doc = "Write protection is disabled."] _1,
}
impl WPDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPDISW::_0 => false,
            WPDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPDISW<'a> {
    w: &'a mut W,
}
impl<'a> _WPDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write protection is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPDISW::_0)
    }
    #[doc = "Write protection is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPDISW::_1)
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
#[doc = "Values that can be written to the field `PWMSYNC`"]
pub enum PWMSYNCW {
    #[doc = "No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    _0,
    #[doc = "Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    _1,
}
impl PWMSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMSYNCW::_0 => false,
            PWMSYNCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMSYNCW::_0)
    }
    #[doc = "Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMSYNCW::_1)
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
#[doc = "Values that can be written to the field `CAPTEST`"]
pub enum CAPTESTW {
    #[doc = "Capture test mode is disabled."] _0,
    #[doc = "Capture test mode is enabled."] _1,
}
impl CAPTESTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTESTW::_0 => false,
            CAPTESTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTESTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTESTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTESTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture test mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPTESTW::_0)
    }
    #[doc = "Capture test mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPTESTW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FAULTM`"]
pub enum FAULTMW {
    #[doc = "Fault control is disabled for all channels."] _00,
    #[doc = "Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    _01,
    #[doc = "Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    _10,
    #[doc = "Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    _11,
}
impl FAULTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FAULTMW::_00 => 0,
            FAULTMW::_01 => 1,
            FAULTMW::_10 => 2,
            FAULTMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTMW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Fault control is disabled for all channels."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FAULTMW::_00)
    }
    #[doc = "Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FAULTMW::_01)
    }
    #[doc = "Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FAULTMW::_10)
    }
    #[doc = "Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FAULTMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FAULTIE`"]
pub enum FAULTIEW {
    #[doc = "Fault control interrupt is disabled."] _0,
    #[doc = "Fault control interrupt is enabled."] _1,
}
impl FAULTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULTIEW::_0 => false,
            FAULTIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault control interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTIEW::_0)
    }
    #[doc = "Fault control interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTIEW::_1)
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
    #[doc = "Bit 0 - FTM Enable"]
    #[inline]
    pub fn ftmen(&self) -> FTMENR {
        FTMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write Protection Disable"]
    #[inline]
    pub fn wpdis(&self) -> WPDISR {
        WPDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - PWM Synchronization Mode"]
    #[inline]
    pub fn pwmsync(&self) -> PWMSYNCR {
        PWMSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture Test Mode Enable"]
    #[inline]
    pub fn captest(&self) -> CAPTESTR {
        CAPTESTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Fault Control Mode"]
    #[inline]
    pub fn faultm(&self) -> FAULTMR {
        FAULTMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Fault Interrupt Enable"]
    #[inline]
    pub fn faultie(&self) -> FAULTIER {
        FAULTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - FTM Enable"]
    #[inline]
    pub fn ftmen(&mut self) -> _FTMENW {
        _FTMENW { w: self }
    }
    #[doc = "Bit 1 - Initialize The Channels Output"]
    #[inline]
    pub fn init(&mut self) -> _INITW {
        _INITW { w: self }
    }
    #[doc = "Bit 2 - Write Protection Disable"]
    #[inline]
    pub fn wpdis(&mut self) -> _WPDISW {
        _WPDISW { w: self }
    }
    #[doc = "Bit 3 - PWM Synchronization Mode"]
    #[inline]
    pub fn pwmsync(&mut self) -> _PWMSYNCW {
        _PWMSYNCW { w: self }
    }
    #[doc = "Bit 4 - Capture Test Mode Enable"]
    #[inline]
    pub fn captest(&mut self) -> _CAPTESTW {
        _CAPTESTW { w: self }
    }
    #[doc = "Bits 5:6 - Fault Control Mode"]
    #[inline]
    pub fn faultm(&mut self) -> _FAULTMW {
        _FAULTMW { w: self }
    }
    #[doc = "Bit 7 - Fault Interrupt Enable"]
    #[inline]
    pub fn faultie(&mut self) -> _FAULTIEW {
        _FAULTIEW { w: self }
    }
}
