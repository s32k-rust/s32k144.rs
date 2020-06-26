#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data: [u8; 4usize],
    #[doc = "0x04 - CRC Polynomial register"]
    pub gpoly: GPOLY,
    #[doc = "0x08 - CRC Control register"]
    pub ctrl: CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub fn datall(&self) -> &DATALL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATALL) }
    }
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub fn datall_mut(&self) -> &mut DATALL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATALL) }
    }
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub fn datal(&self) -> &DATAL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATAL) }
    }
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub fn datal_mut(&self) -> &mut DATAL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATAL) }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn data(&self) -> &DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATA) }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn data_mut(&self) -> &mut DATA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATA) }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub fn datalu(&self) -> &DATALU {
        unsafe { &*(((self as *const Self) as *const u8).add(1usize) as *const DATALU) }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub fn datalu_mut(&self) -> &mut DATALU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1usize) as *mut DATALU) }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub fn datahl(&self) -> &DATAHL {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const DATAHL) }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub fn datahl_mut(&self) -> &mut DATAHL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut DATAHL) }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub fn datah(&self) -> &DATAH {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const DATAH) }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub fn datah_mut(&self) -> &mut DATAH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut DATAH) }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub fn datahu(&self) -> &DATAHU {
        unsafe { &*(((self as *const Self) as *const u8).add(3usize) as *const DATAHU) }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub fn datahu_mut(&self) -> &mut DATAHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3usize) as *mut DATAHU) }
    }
}
#[doc = "CRC Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "CRC Data register"]
pub mod data;
#[doc = "CRC_DATAL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datal](datal) module"]
pub type DATAL = crate::Reg<u16, _DATAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAL;
#[doc = "`read()` method returns [datal::R](datal::R) reader structure"]
impl crate::Readable for DATAL {}
#[doc = "`write(|w| ..)` method takes [datal::W](datal::W) writer structure"]
impl crate::Writable for DATAL {}
#[doc = "CRC_DATAL register."]
pub mod datal;
#[doc = "CRC_DATALL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datall](datall) module"]
pub type DATALL = crate::Reg<u8, _DATALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALL;
#[doc = "`read()` method returns [datall::R](datall::R) reader structure"]
impl crate::Readable for DATALL {}
#[doc = "`write(|w| ..)` method takes [datall::W](datall::W) writer structure"]
impl crate::Writable for DATALL {}
#[doc = "CRC_DATALL register."]
pub mod datall;
#[doc = "CRC_DATALU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalu](datalu) module"]
pub type DATALU = crate::Reg<u8, _DATALU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALU;
#[doc = "`read()` method returns [datalu::R](datalu::R) reader structure"]
impl crate::Readable for DATALU {}
#[doc = "`write(|w| ..)` method takes [datalu::W](datalu::W) writer structure"]
impl crate::Writable for DATALU {}
#[doc = "CRC_DATALU register."]
pub mod datalu;
#[doc = "CRC_DATAH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datah](datah) module"]
pub type DATAH = crate::Reg<u16, _DATAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAH;
#[doc = "`read()` method returns [datah::R](datah::R) reader structure"]
impl crate::Readable for DATAH {}
#[doc = "`write(|w| ..)` method takes [datah::W](datah::W) writer structure"]
impl crate::Writable for DATAH {}
#[doc = "CRC_DATAH register."]
pub mod datah;
#[doc = "CRC_DATAHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datahl](datahl) module"]
pub type DATAHL = crate::Reg<u8, _DATAHL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAHL;
#[doc = "`read()` method returns [datahl::R](datahl::R) reader structure"]
impl crate::Readable for DATAHL {}
#[doc = "`write(|w| ..)` method takes [datahl::W](datahl::W) writer structure"]
impl crate::Writable for DATAHL {}
#[doc = "CRC_DATAHL register."]
pub mod datahl;
#[doc = "CRC_DATAHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datahu](datahu) module"]
pub type DATAHU = crate::Reg<u8, _DATAHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAHU;
#[doc = "`read()` method returns [datahu::R](datahu::R) reader structure"]
impl crate::Readable for DATAHU {}
#[doc = "`write(|w| ..)` method takes [datahu::W](datahu::W) writer structure"]
impl crate::Writable for DATAHU {}
#[doc = "CRC_DATAHU register."]
pub mod datahu;
#[doc = "CRC Polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpoly](gpoly) module"]
pub type GPOLY = crate::Reg<u32, _GPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLY;
#[doc = "`read()` method returns [gpoly::R](gpoly::R) reader structure"]
impl crate::Readable for GPOLY {}
#[doc = "`write(|w| ..)` method takes [gpoly::W](gpoly::W) writer structure"]
impl crate::Writable for GPOLY {}
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CRC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CRC Control register"]
pub mod ctrl;
