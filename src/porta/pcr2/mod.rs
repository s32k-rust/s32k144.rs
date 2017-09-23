#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCR2 {
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _0,
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _1,
}
impl PSR {
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
            PSR::_0 => false,
            PSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSR {
        match value {
            false => PSR::_0,
            true => PSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PSR::_1
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."] _0,
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl PER {
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
            PER::_0 => false,
            PER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::_0,
            true => PER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PER::_1
    }
}
#[doc = "Possible values of the field `MUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXR {
    #[doc = "Pin disabled (Alternative 0) (analog)."] _000,
    #[doc = "Alternative 1 (GPIO)."] _001,
    #[doc = "Alternative 2 (chip-specific)."] _010,
    #[doc = "Alternative 3 (chip-specific)."] _011,
    #[doc = "Alternative 4 (chip-specific)."] _100,
    #[doc = "Alternative 5 (chip-specific)."] _101,
    #[doc = "Alternative 6 (chip-specific)."] _110,
    #[doc = "Alternative 7 (chip-specific)."] _111,
}
impl MUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MUXR::_000 => 0,
            MUXR::_001 => 1,
            MUXR::_010 => 2,
            MUXR::_011 => 3,
            MUXR::_100 => 4,
            MUXR::_101 => 5,
            MUXR::_110 => 6,
            MUXR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MUXR {
        match value {
            0 => MUXR::_000,
            1 => MUXR::_001,
            2 => MUXR::_010,
            3 => MUXR::_011,
            4 => MUXR::_100,
            5 => MUXR::_101,
            6 => MUXR::_110,
            7 => MUXR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MUXR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == MUXR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == MUXR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == MUXR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == MUXR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == MUXR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == MUXR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == MUXR::_111
    }
}
#[doc = "Possible values of the field `LK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKR {
    #[doc = "Pin Control Register fields [15:0] are not locked."] _0,
    #[doc = "Pin Control Register fields [15:0] are locked and cannot be updated until the next system reset."]
    _1,
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
#[doc = "Possible values of the field `IRQC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQCR {
    #[doc = "Interrupt Status Flag (ISF) is disabled."] _0000,
    #[doc = "ISF flag and DMA request on rising edge."] _0001,
    #[doc = "ISF flag and DMA request on falling edge."] _0010,
    #[doc = "ISF flag and DMA request on either edge."] _0011,
    #[doc = "ISF flag and Interrupt when logic 0."] _1000,
    #[doc = "ISF flag and Interrupt on rising-edge."] _1001,
    #[doc = "ISF flag and Interrupt on falling-edge."] _1010,
    #[doc = "ISF flag and Interrupt on either edge."] _1011,
    #[doc = "ISF flag and Interrupt when logic 1."] _1100,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl IRQCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IRQCR::_0000 => 0,
            IRQCR::_0001 => 1,
            IRQCR::_0010 => 2,
            IRQCR::_0011 => 3,
            IRQCR::_1000 => 8,
            IRQCR::_1001 => 9,
            IRQCR::_1010 => 10,
            IRQCR::_1011 => 11,
            IRQCR::_1100 => 12,
            IRQCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IRQCR {
        match value {
            0 => IRQCR::_0000,
            1 => IRQCR::_0001,
            2 => IRQCR::_0010,
            3 => IRQCR::_0011,
            8 => IRQCR::_1000,
            9 => IRQCR::_1001,
            10 => IRQCR::_1010,
            11 => IRQCR::_1011,
            12 => IRQCR::_1100,
            i => IRQCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == IRQCR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == IRQCR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == IRQCR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == IRQCR::_0011
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == IRQCR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == IRQCR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == IRQCR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == IRQCR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == IRQCR::_1100
    }
}
#[doc = "Possible values of the field `ISF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISFR {
    #[doc = "Configured interrupt is not detected."] _0,
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl ISFR {
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
            ISFR::_0 => false,
            ISFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISFR {
        match value {
            false => ISFR::_0,
            true => ISFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ISFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ISFR::_1
    }
}
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _0,
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    _1,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSW::_0 => false,
            PSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSW::_0)
    }
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSW::_1)
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
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."] _0,
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::_0 => false,
            PEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEW::_0)
    }
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEW::_1)
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
#[doc = "Values that can be written to the field `MUX`"]
pub enum MUXW {
    #[doc = "Pin disabled (Alternative 0) (analog)."] _000,
    #[doc = "Alternative 1 (GPIO)."] _001,
    #[doc = "Alternative 2 (chip-specific)."] _010,
    #[doc = "Alternative 3 (chip-specific)."] _011,
    #[doc = "Alternative 4 (chip-specific)."] _100,
    #[doc = "Alternative 5 (chip-specific)."] _101,
    #[doc = "Alternative 6 (chip-specific)."] _110,
    #[doc = "Alternative 7 (chip-specific)."] _111,
}
impl MUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MUXW::_000 => 0,
            MUXW::_001 => 1,
            MUXW::_010 => 2,
            MUXW::_011 => 3,
            MUXW::_100 => 4,
            MUXW::_101 => 5,
            MUXW::_110 => 6,
            MUXW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin disabled (Alternative 0) (analog)."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(MUXW::_000)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(MUXW::_001)
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(MUXW::_010)
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(MUXW::_011)
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(MUXW::_100)
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(MUXW::_101)
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(MUXW::_110)
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(MUXW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LK`"]
pub enum LKW {
    #[doc = "Pin Control Register fields [15:0] are not locked."] _0,
    #[doc = "Pin Control Register fields [15:0] are locked and cannot be updated until the next system reset."]
    _1,
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
    #[doc = "Pin Control Register fields [15:0] are not locked."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LKW::_0)
    }
    #[doc = "Pin Control Register fields [15:0] are locked and cannot be updated until the next system reset."]
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IRQC`"]
pub enum IRQCW {
    #[doc = "Interrupt Status Flag (ISF) is disabled."] _0000,
    #[doc = "ISF flag and DMA request on rising edge."] _0001,
    #[doc = "ISF flag and DMA request on falling edge."] _0010,
    #[doc = "ISF flag and DMA request on either edge."] _0011,
    #[doc = "ISF flag and Interrupt when logic 0."] _1000,
    #[doc = "ISF flag and Interrupt on rising-edge."] _1001,
    #[doc = "ISF flag and Interrupt on falling-edge."] _1010,
    #[doc = "ISF flag and Interrupt on either edge."] _1011,
    #[doc = "ISF flag and Interrupt when logic 1."] _1100,
}
impl IRQCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IRQCW::_0000 => 0,
            IRQCW::_0001 => 1,
            IRQCW::_0010 => 2,
            IRQCW::_0011 => 3,
            IRQCW::_1000 => 8,
            IRQCW::_1001 => 9,
            IRQCW::_1010 => 10,
            IRQCW::_1011 => 11,
            IRQCW::_1100 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRQCW<'a> {
    w: &'a mut W,
}
impl<'a> _IRQCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRQCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt Status Flag (ISF) is disabled."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(IRQCW::_0000)
    }
    #[doc = "ISF flag and DMA request on rising edge."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(IRQCW::_0001)
    }
    #[doc = "ISF flag and DMA request on falling edge."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(IRQCW::_0010)
    }
    #[doc = "ISF flag and DMA request on either edge."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(IRQCW::_0011)
    }
    #[doc = "ISF flag and Interrupt when logic 0."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(IRQCW::_1000)
    }
    #[doc = "ISF flag and Interrupt on rising-edge."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(IRQCW::_1001)
    }
    #[doc = "ISF flag and Interrupt on falling-edge."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(IRQCW::_1010)
    }
    #[doc = "ISF flag and Interrupt on either edge."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(IRQCW::_1011)
    }
    #[doc = "ISF flag and Interrupt when logic 1."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(IRQCW::_1100)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ISF`"]
pub enum ISFW {
    #[doc = "Configured interrupt is not detected."] _0,
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1,
}
impl ISFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISFW::_0 => false,
            ISFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISFW<'a> {
    w: &'a mut W,
}
impl<'a> _ISFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISFW::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISFW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Pull Select"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline]
    pub fn mux(&self) -> MUXR {
        MUXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline]
    pub fn lk(&self) -> LKR {
        LKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline]
    pub fn irqc(&self) -> IRQCR {
        IRQCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline]
    pub fn isf(&self) -> ISFR {
        ISFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Pull Select"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline]
    pub fn mux(&mut self) -> _MUXW {
        _MUXW { w: self }
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline]
    pub fn lk(&mut self) -> _LKW {
        _LKW { w: self }
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline]
    pub fn irqc(&mut self) -> _IRQCW {
        _IRQCW { w: self }
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline]
    pub fn isf(&mut self) -> _ISFW {
        _ISFW { w: self }
    }
}
