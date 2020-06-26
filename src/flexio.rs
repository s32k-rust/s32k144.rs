#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - FlexIO Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Pin State Register"]
    pub pin: PIN,
    #[doc = "0x10 - Shifter Status Register"]
    pub shiftstat: SHIFTSTAT,
    #[doc = "0x14 - Shifter Error Register"]
    pub shifterr: SHIFTERR,
    #[doc = "0x18 - Timer Status Register"]
    pub timstat: TIMSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    pub shiftsien: SHIFTSIEN,
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    pub shifteien: SHIFTEIEN,
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    pub timien: TIMIEN,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Shifter Status DMA Enable"]
    pub shiftsden: SHIFTSDEN,
    _reserved11: [u8; 76usize],
    #[doc = "0x80 - Shifter Control N Register"]
    pub shiftctl0: SHIFTCTL0,
    #[doc = "0x84 - Shifter Control N Register"]
    pub shiftctl1: SHIFTCTL1,
    #[doc = "0x88 - Shifter Control N Register"]
    pub shiftctl2: SHIFTCTL2,
    #[doc = "0x8c - Shifter Control N Register"]
    pub shiftctl3: SHIFTCTL3,
    _reserved15: [u8; 112usize],
    #[doc = "0x100 - Shifter Configuration N Register"]
    pub shiftcfg0: SHIFTCFG0,
    #[doc = "0x104 - Shifter Configuration N Register"]
    pub shiftcfg1: SHIFTCFG1,
    #[doc = "0x108 - Shifter Configuration N Register"]
    pub shiftcfg2: SHIFTCFG2,
    #[doc = "0x10c - Shifter Configuration N Register"]
    pub shiftcfg3: SHIFTCFG3,
    _reserved19: [u8; 240usize],
    #[doc = "0x200 - Shifter Buffer N Register"]
    pub shiftbuf0: SHIFTBUF0,
    #[doc = "0x204 - Shifter Buffer N Register"]
    pub shiftbuf1: SHIFTBUF1,
    #[doc = "0x208 - Shifter Buffer N Register"]
    pub shiftbuf2: SHIFTBUF2,
    #[doc = "0x20c - Shifter Buffer N Register"]
    pub shiftbuf3: SHIFTBUF3,
    _reserved23: [u8; 112usize],
    #[doc = "0x280 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis0: SHIFTBUFBIS0,
    #[doc = "0x284 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis1: SHIFTBUFBIS1,
    #[doc = "0x288 - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis2: SHIFTBUFBIS2,
    #[doc = "0x28c - Shifter Buffer N Bit Swapped Register"]
    pub shiftbufbis3: SHIFTBUFBIS3,
    _reserved27: [u8; 112usize],
    #[doc = "0x300 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys0: SHIFTBUFBYS0,
    #[doc = "0x304 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys1: SHIFTBUFBYS1,
    #[doc = "0x308 - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys2: SHIFTBUFBYS2,
    #[doc = "0x30c - Shifter Buffer N Byte Swapped Register"]
    pub shiftbufbys3: SHIFTBUFBYS3,
    _reserved31: [u8; 112usize],
    #[doc = "0x380 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs0: SHIFTBUFBBS0,
    #[doc = "0x384 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs1: SHIFTBUFBBS1,
    #[doc = "0x388 - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs2: SHIFTBUFBBS2,
    #[doc = "0x38c - Shifter Buffer N Bit Byte Swapped Register"]
    pub shiftbufbbs3: SHIFTBUFBBS3,
    _reserved35: [u8; 112usize],
    #[doc = "0x400 - Timer Control N Register"]
    pub timctl0: TIMCTL0,
    #[doc = "0x404 - Timer Control N Register"]
    pub timctl1: TIMCTL1,
    #[doc = "0x408 - Timer Control N Register"]
    pub timctl2: TIMCTL2,
    #[doc = "0x40c - Timer Control N Register"]
    pub timctl3: TIMCTL3,
    _reserved39: [u8; 112usize],
    #[doc = "0x480 - Timer Configuration N Register"]
    pub timcfg0: TIMCFG0,
    #[doc = "0x484 - Timer Configuration N Register"]
    pub timcfg1: TIMCFG1,
    #[doc = "0x488 - Timer Configuration N Register"]
    pub timcfg2: TIMCFG2,
    #[doc = "0x48c - Timer Configuration N Register"]
    pub timcfg3: TIMCFG3,
    _reserved43: [u8; 112usize],
    #[doc = "0x500 - Timer Compare N Register"]
    pub timcmp0: TIMCMP0,
    #[doc = "0x504 - Timer Compare N Register"]
    pub timcmp1: TIMCMP1,
    #[doc = "0x508 - Timer Compare N Register"]
    pub timcmp2: TIMCMP2,
    #[doc = "0x50c - Timer Compare N Register"]
    pub timcmp3: TIMCMP3,
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "FlexIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "Pin State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "Shifter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstat](shiftstat) module"]
pub type SHIFTSTAT = crate::Reg<u32, _SHIFTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSTAT;
#[doc = "`read()` method returns [shiftstat::R](shiftstat::R) reader structure"]
impl crate::Readable for SHIFTSTAT {}
#[doc = "`write(|w| ..)` method takes [shiftstat::W](shiftstat::W) writer structure"]
impl crate::Writable for SHIFTSTAT {}
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "Shifter Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifterr](shifterr) module"]
pub type SHIFTERR = crate::Reg<u32, _SHIFTERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTERR;
#[doc = "`read()` method returns [shifterr::R](shifterr::R) reader structure"]
impl crate::Readable for SHIFTERR {}
#[doc = "`write(|w| ..)` method takes [shifterr::W](shifterr::W) writer structure"]
impl crate::Writable for SHIFTERR {}
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timstat](timstat) module"]
pub type TIMSTAT = crate::Reg<u32, _TIMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMSTAT;
#[doc = "`read()` method returns [timstat::R](timstat::R) reader structure"]
impl crate::Readable for TIMSTAT {}
#[doc = "`write(|w| ..)` method takes [timstat::W](timstat::W) writer structure"]
impl crate::Writable for TIMSTAT {}
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "Shifter Status Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsien](shiftsien) module"]
pub type SHIFTSIEN = crate::Reg<u32, _SHIFTSIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSIEN;
#[doc = "`read()` method returns [shiftsien::R](shiftsien::R) reader structure"]
impl crate::Readable for SHIFTSIEN {}
#[doc = "`write(|w| ..)` method takes [shiftsien::W](shiftsien::W) writer structure"]
impl crate::Writable for SHIFTSIEN {}
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "Shifter Error Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifteien](shifteien) module"]
pub type SHIFTEIEN = crate::Reg<u32, _SHIFTEIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTEIEN;
#[doc = "`read()` method returns [shifteien::R](shifteien::R) reader structure"]
impl crate::Readable for SHIFTEIEN {}
#[doc = "`write(|w| ..)` method takes [shifteien::W](shifteien::W) writer structure"]
impl crate::Writable for SHIFTEIEN {}
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "Timer Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timien](timien) module"]
pub type TIMIEN = crate::Reg<u32, _TIMIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMIEN;
#[doc = "`read()` method returns [timien::R](timien::R) reader structure"]
impl crate::Readable for TIMIEN {}
#[doc = "`write(|w| ..)` method takes [timien::W](timien::W) writer structure"]
impl crate::Writable for TIMIEN {}
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "Shifter Status DMA Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftsden](shiftsden) module"]
pub type SHIFTSDEN = crate::Reg<u32, _SHIFTSDEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTSDEN;
#[doc = "`read()` method returns [shiftsden::R](shiftsden::R) reader structure"]
impl crate::Readable for SHIFTSDEN {}
#[doc = "`write(|w| ..)` method takes [shiftsden::W](shiftsden::W) writer structure"]
impl crate::Writable for SHIFTSDEN {}
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl0](shiftctl0) module"]
pub type SHIFTCTL0 = crate::Reg<u32, _SHIFTCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCTL0;
#[doc = "`read()` method returns [shiftctl0::R](shiftctl0::R) reader structure"]
impl crate::Readable for SHIFTCTL0 {}
#[doc = "`write(|w| ..)` method takes [shiftctl0::W](shiftctl0::W) writer structure"]
impl crate::Writable for SHIFTCTL0 {}
#[doc = "Shifter Control N Register"]
pub mod shiftctl0;
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl1](shiftctl1) module"]
pub type SHIFTCTL1 = crate::Reg<u32, _SHIFTCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCTL1;
#[doc = "`read()` method returns [shiftctl1::R](shiftctl1::R) reader structure"]
impl crate::Readable for SHIFTCTL1 {}
#[doc = "`write(|w| ..)` method takes [shiftctl1::W](shiftctl1::W) writer structure"]
impl crate::Writable for SHIFTCTL1 {}
#[doc = "Shifter Control N Register"]
pub mod shiftctl1;
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl2](shiftctl2) module"]
pub type SHIFTCTL2 = crate::Reg<u32, _SHIFTCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCTL2;
#[doc = "`read()` method returns [shiftctl2::R](shiftctl2::R) reader structure"]
impl crate::Readable for SHIFTCTL2 {}
#[doc = "`write(|w| ..)` method takes [shiftctl2::W](shiftctl2::W) writer structure"]
impl crate::Writable for SHIFTCTL2 {}
#[doc = "Shifter Control N Register"]
pub mod shiftctl2;
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl3](shiftctl3) module"]
pub type SHIFTCTL3 = crate::Reg<u32, _SHIFTCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCTL3;
#[doc = "`read()` method returns [shiftctl3::R](shiftctl3::R) reader structure"]
impl crate::Readable for SHIFTCTL3 {}
#[doc = "`write(|w| ..)` method takes [shiftctl3::W](shiftctl3::W) writer structure"]
impl crate::Writable for SHIFTCTL3 {}
#[doc = "Shifter Control N Register"]
pub mod shiftctl3;
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg0](shiftcfg0) module"]
pub type SHIFTCFG0 = crate::Reg<u32, _SHIFTCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCFG0;
#[doc = "`read()` method returns [shiftcfg0::R](shiftcfg0::R) reader structure"]
impl crate::Readable for SHIFTCFG0 {}
#[doc = "`write(|w| ..)` method takes [shiftcfg0::W](shiftcfg0::W) writer structure"]
impl crate::Writable for SHIFTCFG0 {}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg0;
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg1](shiftcfg1) module"]
pub type SHIFTCFG1 = crate::Reg<u32, _SHIFTCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCFG1;
#[doc = "`read()` method returns [shiftcfg1::R](shiftcfg1::R) reader structure"]
impl crate::Readable for SHIFTCFG1 {}
#[doc = "`write(|w| ..)` method takes [shiftcfg1::W](shiftcfg1::W) writer structure"]
impl crate::Writable for SHIFTCFG1 {}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg1;
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg2](shiftcfg2) module"]
pub type SHIFTCFG2 = crate::Reg<u32, _SHIFTCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCFG2;
#[doc = "`read()` method returns [shiftcfg2::R](shiftcfg2::R) reader structure"]
impl crate::Readable for SHIFTCFG2 {}
#[doc = "`write(|w| ..)` method takes [shiftcfg2::W](shiftcfg2::W) writer structure"]
impl crate::Writable for SHIFTCFG2 {}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg2;
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg3](shiftcfg3) module"]
pub type SHIFTCFG3 = crate::Reg<u32, _SHIFTCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTCFG3;
#[doc = "`read()` method returns [shiftcfg3::R](shiftcfg3::R) reader structure"]
impl crate::Readable for SHIFTCFG3 {}
#[doc = "`write(|w| ..)` method takes [shiftcfg3::W](shiftcfg3::W) writer structure"]
impl crate::Writable for SHIFTCFG3 {}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg3;
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf0](shiftbuf0) module"]
pub type SHIFTBUF0 = crate::Reg<u32, _SHIFTBUF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUF0;
#[doc = "`read()` method returns [shiftbuf0::R](shiftbuf0::R) reader structure"]
impl crate::Readable for SHIFTBUF0 {}
#[doc = "`write(|w| ..)` method takes [shiftbuf0::W](shiftbuf0::W) writer structure"]
impl crate::Writable for SHIFTBUF0 {}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf0;
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf1](shiftbuf1) module"]
pub type SHIFTBUF1 = crate::Reg<u32, _SHIFTBUF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUF1;
#[doc = "`read()` method returns [shiftbuf1::R](shiftbuf1::R) reader structure"]
impl crate::Readable for SHIFTBUF1 {}
#[doc = "`write(|w| ..)` method takes [shiftbuf1::W](shiftbuf1::W) writer structure"]
impl crate::Writable for SHIFTBUF1 {}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf1;
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf2](shiftbuf2) module"]
pub type SHIFTBUF2 = crate::Reg<u32, _SHIFTBUF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUF2;
#[doc = "`read()` method returns [shiftbuf2::R](shiftbuf2::R) reader structure"]
impl crate::Readable for SHIFTBUF2 {}
#[doc = "`write(|w| ..)` method takes [shiftbuf2::W](shiftbuf2::W) writer structure"]
impl crate::Writable for SHIFTBUF2 {}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf2;
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf3](shiftbuf3) module"]
pub type SHIFTBUF3 = crate::Reg<u32, _SHIFTBUF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUF3;
#[doc = "`read()` method returns [shiftbuf3::R](shiftbuf3::R) reader structure"]
impl crate::Readable for SHIFTBUF3 {}
#[doc = "`write(|w| ..)` method takes [shiftbuf3::W](shiftbuf3::W) writer structure"]
impl crate::Writable for SHIFTBUF3 {}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf3;
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis0](shiftbufbis0) module"]
pub type SHIFTBUFBIS0 = crate::Reg<u32, _SHIFTBUFBIS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBIS0;
#[doc = "`read()` method returns [shiftbufbis0::R](shiftbufbis0::R) reader structure"]
impl crate::Readable for SHIFTBUFBIS0 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbis0::W](shiftbufbis0::W) writer structure"]
impl crate::Writable for SHIFTBUFBIS0 {}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis0;
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis1](shiftbufbis1) module"]
pub type SHIFTBUFBIS1 = crate::Reg<u32, _SHIFTBUFBIS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBIS1;
#[doc = "`read()` method returns [shiftbufbis1::R](shiftbufbis1::R) reader structure"]
impl crate::Readable for SHIFTBUFBIS1 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbis1::W](shiftbufbis1::W) writer structure"]
impl crate::Writable for SHIFTBUFBIS1 {}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis1;
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis2](shiftbufbis2) module"]
pub type SHIFTBUFBIS2 = crate::Reg<u32, _SHIFTBUFBIS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBIS2;
#[doc = "`read()` method returns [shiftbufbis2::R](shiftbufbis2::R) reader structure"]
impl crate::Readable for SHIFTBUFBIS2 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbis2::W](shiftbufbis2::W) writer structure"]
impl crate::Writable for SHIFTBUFBIS2 {}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis2;
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis3](shiftbufbis3) module"]
pub type SHIFTBUFBIS3 = crate::Reg<u32, _SHIFTBUFBIS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBIS3;
#[doc = "`read()` method returns [shiftbufbis3::R](shiftbufbis3::R) reader structure"]
impl crate::Readable for SHIFTBUFBIS3 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbis3::W](shiftbufbis3::W) writer structure"]
impl crate::Writable for SHIFTBUFBIS3 {}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis3;
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys0](shiftbufbys0) module"]
pub type SHIFTBUFBYS0 = crate::Reg<u32, _SHIFTBUFBYS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBYS0;
#[doc = "`read()` method returns [shiftbufbys0::R](shiftbufbys0::R) reader structure"]
impl crate::Readable for SHIFTBUFBYS0 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbys0::W](shiftbufbys0::W) writer structure"]
impl crate::Writable for SHIFTBUFBYS0 {}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys0;
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys1](shiftbufbys1) module"]
pub type SHIFTBUFBYS1 = crate::Reg<u32, _SHIFTBUFBYS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBYS1;
#[doc = "`read()` method returns [shiftbufbys1::R](shiftbufbys1::R) reader structure"]
impl crate::Readable for SHIFTBUFBYS1 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbys1::W](shiftbufbys1::W) writer structure"]
impl crate::Writable for SHIFTBUFBYS1 {}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys1;
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys2](shiftbufbys2) module"]
pub type SHIFTBUFBYS2 = crate::Reg<u32, _SHIFTBUFBYS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBYS2;
#[doc = "`read()` method returns [shiftbufbys2::R](shiftbufbys2::R) reader structure"]
impl crate::Readable for SHIFTBUFBYS2 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbys2::W](shiftbufbys2::W) writer structure"]
impl crate::Writable for SHIFTBUFBYS2 {}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys2;
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys3](shiftbufbys3) module"]
pub type SHIFTBUFBYS3 = crate::Reg<u32, _SHIFTBUFBYS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBYS3;
#[doc = "`read()` method returns [shiftbufbys3::R](shiftbufbys3::R) reader structure"]
impl crate::Readable for SHIFTBUFBYS3 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbys3::W](shiftbufbys3::W) writer structure"]
impl crate::Writable for SHIFTBUFBYS3 {}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys3;
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbbs0](shiftbufbbs0) module"]
pub type SHIFTBUFBBS0 = crate::Reg<u32, _SHIFTBUFBBS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBBS0;
#[doc = "`read()` method returns [shiftbufbbs0::R](shiftbufbbs0::R) reader structure"]
impl crate::Readable for SHIFTBUFBBS0 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbbs0::W](shiftbufbbs0::W) writer structure"]
impl crate::Writable for SHIFTBUFBBS0 {}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs0;
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbbs1](shiftbufbbs1) module"]
pub type SHIFTBUFBBS1 = crate::Reg<u32, _SHIFTBUFBBS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBBS1;
#[doc = "`read()` method returns [shiftbufbbs1::R](shiftbufbbs1::R) reader structure"]
impl crate::Readable for SHIFTBUFBBS1 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbbs1::W](shiftbufbbs1::W) writer structure"]
impl crate::Writable for SHIFTBUFBBS1 {}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs1;
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbbs2](shiftbufbbs2) module"]
pub type SHIFTBUFBBS2 = crate::Reg<u32, _SHIFTBUFBBS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBBS2;
#[doc = "`read()` method returns [shiftbufbbs2::R](shiftbufbbs2::R) reader structure"]
impl crate::Readable for SHIFTBUFBBS2 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbbs2::W](shiftbufbbs2::W) writer structure"]
impl crate::Writable for SHIFTBUFBBS2 {}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs2;
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbbs3](shiftbufbbs3) module"]
pub type SHIFTBUFBBS3 = crate::Reg<u32, _SHIFTBUFBBS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHIFTBUFBBS3;
#[doc = "`read()` method returns [shiftbufbbs3::R](shiftbufbbs3::R) reader structure"]
impl crate::Readable for SHIFTBUFBBS3 {}
#[doc = "`write(|w| ..)` method takes [shiftbufbbs3::W](shiftbufbbs3::W) writer structure"]
impl crate::Writable for SHIFTBUFBBS3 {}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs3;
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl0](timctl0) module"]
pub type TIMCTL0 = crate::Reg<u32, _TIMCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTL0;
#[doc = "`read()` method returns [timctl0::R](timctl0::R) reader structure"]
impl crate::Readable for TIMCTL0 {}
#[doc = "`write(|w| ..)` method takes [timctl0::W](timctl0::W) writer structure"]
impl crate::Writable for TIMCTL0 {}
#[doc = "Timer Control N Register"]
pub mod timctl0;
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl1](timctl1) module"]
pub type TIMCTL1 = crate::Reg<u32, _TIMCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTL1;
#[doc = "`read()` method returns [timctl1::R](timctl1::R) reader structure"]
impl crate::Readable for TIMCTL1 {}
#[doc = "`write(|w| ..)` method takes [timctl1::W](timctl1::W) writer structure"]
impl crate::Writable for TIMCTL1 {}
#[doc = "Timer Control N Register"]
pub mod timctl1;
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl2](timctl2) module"]
pub type TIMCTL2 = crate::Reg<u32, _TIMCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTL2;
#[doc = "`read()` method returns [timctl2::R](timctl2::R) reader structure"]
impl crate::Readable for TIMCTL2 {}
#[doc = "`write(|w| ..)` method takes [timctl2::W](timctl2::W) writer structure"]
impl crate::Writable for TIMCTL2 {}
#[doc = "Timer Control N Register"]
pub mod timctl2;
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl3](timctl3) module"]
pub type TIMCTL3 = crate::Reg<u32, _TIMCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCTL3;
#[doc = "`read()` method returns [timctl3::R](timctl3::R) reader structure"]
impl crate::Readable for TIMCTL3 {}
#[doc = "`write(|w| ..)` method takes [timctl3::W](timctl3::W) writer structure"]
impl crate::Writable for TIMCTL3 {}
#[doc = "Timer Control N Register"]
pub mod timctl3;
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg0](timcfg0) module"]
pub type TIMCFG0 = crate::Reg<u32, _TIMCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG0;
#[doc = "`read()` method returns [timcfg0::R](timcfg0::R) reader structure"]
impl crate::Readable for TIMCFG0 {}
#[doc = "`write(|w| ..)` method takes [timcfg0::W](timcfg0::W) writer structure"]
impl crate::Writable for TIMCFG0 {}
#[doc = "Timer Configuration N Register"]
pub mod timcfg0;
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg1](timcfg1) module"]
pub type TIMCFG1 = crate::Reg<u32, _TIMCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG1;
#[doc = "`read()` method returns [timcfg1::R](timcfg1::R) reader structure"]
impl crate::Readable for TIMCFG1 {}
#[doc = "`write(|w| ..)` method takes [timcfg1::W](timcfg1::W) writer structure"]
impl crate::Writable for TIMCFG1 {}
#[doc = "Timer Configuration N Register"]
pub mod timcfg1;
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg2](timcfg2) module"]
pub type TIMCFG2 = crate::Reg<u32, _TIMCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG2;
#[doc = "`read()` method returns [timcfg2::R](timcfg2::R) reader structure"]
impl crate::Readable for TIMCFG2 {}
#[doc = "`write(|w| ..)` method takes [timcfg2::W](timcfg2::W) writer structure"]
impl crate::Writable for TIMCFG2 {}
#[doc = "Timer Configuration N Register"]
pub mod timcfg2;
#[doc = "Timer Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcfg3](timcfg3) module"]
pub type TIMCFG3 = crate::Reg<u32, _TIMCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCFG3;
#[doc = "`read()` method returns [timcfg3::R](timcfg3::R) reader structure"]
impl crate::Readable for TIMCFG3 {}
#[doc = "`write(|w| ..)` method takes [timcfg3::W](timcfg3::W) writer structure"]
impl crate::Writable for TIMCFG3 {}
#[doc = "Timer Configuration N Register"]
pub mod timcfg3;
#[doc = "Timer Compare N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcmp0](timcmp0) module"]
pub type TIMCMP0 = crate::Reg<u32, _TIMCMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCMP0;
#[doc = "`read()` method returns [timcmp0::R](timcmp0::R) reader structure"]
impl crate::Readable for TIMCMP0 {}
#[doc = "`write(|w| ..)` method takes [timcmp0::W](timcmp0::W) writer structure"]
impl crate::Writable for TIMCMP0 {}
#[doc = "Timer Compare N Register"]
pub mod timcmp0;
#[doc = "Timer Compare N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcmp1](timcmp1) module"]
pub type TIMCMP1 = crate::Reg<u32, _TIMCMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCMP1;
#[doc = "`read()` method returns [timcmp1::R](timcmp1::R) reader structure"]
impl crate::Readable for TIMCMP1 {}
#[doc = "`write(|w| ..)` method takes [timcmp1::W](timcmp1::W) writer structure"]
impl crate::Writable for TIMCMP1 {}
#[doc = "Timer Compare N Register"]
pub mod timcmp1;
#[doc = "Timer Compare N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcmp2](timcmp2) module"]
pub type TIMCMP2 = crate::Reg<u32, _TIMCMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCMP2;
#[doc = "`read()` method returns [timcmp2::R](timcmp2::R) reader structure"]
impl crate::Readable for TIMCMP2 {}
#[doc = "`write(|w| ..)` method takes [timcmp2::W](timcmp2::W) writer structure"]
impl crate::Writable for TIMCMP2 {}
#[doc = "Timer Compare N Register"]
pub mod timcmp2;
#[doc = "Timer Compare N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcmp3](timcmp3) module"]
pub type TIMCMP3 = crate::Reg<u32, _TIMCMP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCMP3;
#[doc = "`read()` method returns [timcmp3::R](timcmp3::R) reader structure"]
impl crate::Readable for TIMCMP3 {}
#[doc = "`write(|w| ..)` method takes [timcmp3::W](timcmp3::W) writer structure"]
impl crate::Writable for TIMCMP3 {}
#[doc = "Timer Compare N Register"]
pub mod timcmp3;
