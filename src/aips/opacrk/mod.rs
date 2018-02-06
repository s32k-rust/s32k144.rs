#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACRK {
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
#[doc = "Possible values of the field `TP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP3R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP3R {
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
            TP3R::_0 => false,
            TP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP3R {
        match value {
            false => TP3R::_0,
            true => TP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP3R::_1
    }
}
#[doc = "Possible values of the field `WP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP3R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP3R {
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
            WP3R::_0 => false,
            WP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP3R {
        match value {
            false => WP3R::_0,
            true => WP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP3R::_1
    }
}
#[doc = "Possible values of the field `SP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP3R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP3R {
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
            SP3R::_0 => false,
            SP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP3R {
        match value {
            false => SP3R::_0,
            true => SP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP3R::_1
    }
}
#[doc = "Values that can be written to the field `TP3`"]
pub enum TP3W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP3W::_0 => false,
            TP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP3W<'a> {
    w: &'a mut W,
}
impl<'a> _TP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP3W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP3W::_1)
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
#[doc = "Values that can be written to the field `WP3`"]
pub enum WP3W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP3W::_0 => false,
            WP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP3W<'a> {
    w: &'a mut W,
}
impl<'a> _WP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP3W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP3W::_1)
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
#[doc = "Values that can be written to the field `SP3`"]
pub enum SP3W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP3W::_0 => false,
            SP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP3W<'a> {
    w: &'a mut W,
}
impl<'a> _SP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP3W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP3W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline]
    pub fn tp3(&self) -> TP3R {
        TP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline]
    pub fn wp3(&self) -> WP3R {
        WP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline]
    pub fn sp3(&self) -> SP3R {
        SP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 262144 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline]
    pub fn tp3(&mut self) -> _TP3W {
        _TP3W { w: self }
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline]
    pub fn wp3(&mut self) -> _WP3W {
        _WP3W { w: self }
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline]
    pub fn sp3(&mut self) -> _SP3W {
        _SP3W { w: self }
    }
}
