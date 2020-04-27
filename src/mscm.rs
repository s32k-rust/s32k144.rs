#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor X Type Register"]
    pub cpx_type: CPXTYPE,
    #[doc = "0x04 - Processor X Number Register"]
    pub cpx_num: CPXNUM,
    #[doc = "0x08 - Processor X Master Register"]
    pub cpx_master: CPXMASTER,
    #[doc = "0x0c - Processor X Count Register"]
    pub cpx_count: CPXCOUNT,
    #[doc = "0x10 - Processor X Configuration Register 0"]
    pub cpx_cfg0: CPXCFG0,
    #[doc = "0x14 - Processor X Configuration Register 1"]
    pub cpx_cfg1: CPXCFG1,
    #[doc = "0x18 - Processor X Configuration Register 2"]
    pub cpx_cfg2: CPXCFG2,
    #[doc = "0x1c - Processor X Configuration Register 3"]
    pub cpx_cfg3: CPXCFG3,
    #[doc = "0x20 - Processor 0 Type Register"]
    pub cp0type: CP0TYPE,
    #[doc = "0x24 - Processor 0 Number Register"]
    pub cp0num: CP0NUM,
    #[doc = "0x28 - Processor 0 Master Register"]
    pub cp0master: CP0MASTER,
    #[doc = "0x2c - Processor 0 Count Register"]
    pub cp0count: CP0COUNT,
    #[doc = "0x30 - Processor 0 Configuration Register 0"]
    pub cp0cfg0: CP0CFG0,
    #[doc = "0x34 - Processor 0 Configuration Register 1"]
    pub cp0cfg1: CP0CFG1,
    #[doc = "0x38 - Processor 0 Configuration Register 2"]
    pub cp0cfg2: CP0CFG2,
    #[doc = "0x3c - Processor 0 Configuration Register 3"]
    pub cp0cfg3: CP0CFG3,
    _reserved16: [u8; 960usize],
    #[doc = "0x400 - On-Chip Memory Descriptor Register"]
    pub ocmdr0: OCMDR0,
    #[doc = "0x404 - On-Chip Memory Descriptor Register"]
    pub ocmdr1: OCMDR1,
    #[doc = "0x408 - On-Chip Memory Descriptor Register"]
    pub ocmdr2: OCMDR2,
    #[doc = "0x40c - On-Chip Memory Descriptor Register"]
    pub ocmdr3: OCMDR3,
}
#[doc = "Processor X Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_type](cpx_type) module"]
pub type CPXTYPE = crate::Reg<u32, _CPXTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXTYPE;
#[doc = "`read()` method returns [cpx_type::R](cpx_type::R) reader structure"]
impl crate::Readable for CPXTYPE {}
#[doc = "Processor X Type Register"]
pub mod cpx_type;
#[doc = "Processor X Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_num](cpx_num) module"]
pub type CPXNUM = crate::Reg<u32, _CPXNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXNUM;
#[doc = "`read()` method returns [cpx_num::R](cpx_num::R) reader structure"]
impl crate::Readable for CPXNUM {}
#[doc = "Processor X Number Register"]
pub mod cpx_num;
#[doc = "Processor X Master Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_master](cpx_master) module"]
pub type CPXMASTER = crate::Reg<u32, _CPXMASTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXMASTER;
#[doc = "`read()` method returns [cpx_master::R](cpx_master::R) reader structure"]
impl crate::Readable for CPXMASTER {}
#[doc = "Processor X Master Register"]
pub mod cpx_master;
#[doc = "Processor X Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_count](cpx_count) module"]
pub type CPXCOUNT = crate::Reg<u32, _CPXCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXCOUNT;
#[doc = "`read()` method returns [cpx_count::R](cpx_count::R) reader structure"]
impl crate::Readable for CPXCOUNT {}
#[doc = "Processor X Count Register"]
pub mod cpx_count;
#[doc = "Processor X Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_cfg0](cpx_cfg0) module"]
pub type CPXCFG0 = crate::Reg<u32, _CPXCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXCFG0;
#[doc = "`read()` method returns [cpx_cfg0::R](cpx_cfg0::R) reader structure"]
impl crate::Readable for CPXCFG0 {}
#[doc = "Processor X Configuration Register 0"]
pub mod cpx_cfg0;
#[doc = "Processor X Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_cfg1](cpx_cfg1) module"]
pub type CPXCFG1 = crate::Reg<u32, _CPXCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXCFG1;
#[doc = "`read()` method returns [cpx_cfg1::R](cpx_cfg1::R) reader structure"]
impl crate::Readable for CPXCFG1 {}
#[doc = "Processor X Configuration Register 1"]
pub mod cpx_cfg1;
#[doc = "Processor X Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_cfg2](cpx_cfg2) module"]
pub type CPXCFG2 = crate::Reg<u32, _CPXCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXCFG2;
#[doc = "`read()` method returns [cpx_cfg2::R](cpx_cfg2::R) reader structure"]
impl crate::Readable for CPXCFG2 {}
#[doc = "Processor X Configuration Register 2"]
pub mod cpx_cfg2;
#[doc = "Processor X Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_cfg3](cpx_cfg3) module"]
pub type CPXCFG3 = crate::Reg<u32, _CPXCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPXCFG3;
#[doc = "`read()` method returns [cpx_cfg3::R](cpx_cfg3::R) reader structure"]
impl crate::Readable for CPXCFG3 {}
#[doc = "Processor X Configuration Register 3"]
pub mod cpx_cfg3;
#[doc = "Processor 0 Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0type](cp0type) module"]
pub type CP0TYPE = crate::Reg<u32, _CP0TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0TYPE;
#[doc = "`read()` method returns [cp0type::R](cp0type::R) reader structure"]
impl crate::Readable for CP0TYPE {}
#[doc = "Processor 0 Type Register"]
pub mod cp0type;
#[doc = "Processor 0 Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0num](cp0num) module"]
pub type CP0NUM = crate::Reg<u32, _CP0NUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0NUM;
#[doc = "`read()` method returns [cp0num::R](cp0num::R) reader structure"]
impl crate::Readable for CP0NUM {}
#[doc = "Processor 0 Number Register"]
pub mod cp0num;
#[doc = "Processor 0 Master Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0master](cp0master) module"]
pub type CP0MASTER = crate::Reg<u32, _CP0MASTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0MASTER;
#[doc = "`read()` method returns [cp0master::R](cp0master::R) reader structure"]
impl crate::Readable for CP0MASTER {}
#[doc = "Processor 0 Master Register"]
pub mod cp0master;
#[doc = "Processor 0 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0count](cp0count) module"]
pub type CP0COUNT = crate::Reg<u32, _CP0COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0COUNT;
#[doc = "`read()` method returns [cp0count::R](cp0count::R) reader structure"]
impl crate::Readable for CP0COUNT {}
#[doc = "Processor 0 Count Register"]
pub mod cp0count;
#[doc = "Processor 0 Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0cfg0](cp0cfg0) module"]
pub type CP0CFG0 = crate::Reg<u32, _CP0CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0CFG0;
#[doc = "`read()` method returns [cp0cfg0::R](cp0cfg0::R) reader structure"]
impl crate::Readable for CP0CFG0 {}
#[doc = "Processor 0 Configuration Register 0"]
pub mod cp0cfg0;
#[doc = "Processor 0 Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0cfg1](cp0cfg1) module"]
pub type CP0CFG1 = crate::Reg<u32, _CP0CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0CFG1;
#[doc = "`read()` method returns [cp0cfg1::R](cp0cfg1::R) reader structure"]
impl crate::Readable for CP0CFG1 {}
#[doc = "Processor 0 Configuration Register 1"]
pub mod cp0cfg1;
#[doc = "Processor 0 Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0cfg2](cp0cfg2) module"]
pub type CP0CFG2 = crate::Reg<u32, _CP0CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0CFG2;
#[doc = "`read()` method returns [cp0cfg2::R](cp0cfg2::R) reader structure"]
impl crate::Readable for CP0CFG2 {}
#[doc = "Processor 0 Configuration Register 2"]
pub mod cp0cfg2;
#[doc = "Processor 0 Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0cfg3](cp0cfg3) module"]
pub type CP0CFG3 = crate::Reg<u32, _CP0CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP0CFG3;
#[doc = "`read()` method returns [cp0cfg3::R](cp0cfg3::R) reader structure"]
impl crate::Readable for CP0CFG3 {}
#[doc = "Processor 0 Configuration Register 3"]
pub mod cp0cfg3;
#[doc = "On-Chip Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmdr0](ocmdr0) module"]
pub type OCMDR0 = crate::Reg<u32, _OCMDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMDR0;
#[doc = "`read()` method returns [ocmdr0::R](ocmdr0::R) reader structure"]
impl crate::Readable for OCMDR0 {}
#[doc = "`write(|w| ..)` method takes [ocmdr0::W](ocmdr0::W) writer structure"]
impl crate::Writable for OCMDR0 {}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr0;
#[doc = "On-Chip Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmdr1](ocmdr1) module"]
pub type OCMDR1 = crate::Reg<u32, _OCMDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMDR1;
#[doc = "`read()` method returns [ocmdr1::R](ocmdr1::R) reader structure"]
impl crate::Readable for OCMDR1 {}
#[doc = "`write(|w| ..)` method takes [ocmdr1::W](ocmdr1::W) writer structure"]
impl crate::Writable for OCMDR1 {}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr1;
#[doc = "On-Chip Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmdr2](ocmdr2) module"]
pub type OCMDR2 = crate::Reg<u32, _OCMDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMDR2;
#[doc = "`read()` method returns [ocmdr2::R](ocmdr2::R) reader structure"]
impl crate::Readable for OCMDR2 {}
#[doc = "`write(|w| ..)` method takes [ocmdr2::W](ocmdr2::W) writer structure"]
impl crate::Writable for OCMDR2 {}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr2;
#[doc = "On-Chip Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocmdr3](ocmdr3) module"]
pub type OCMDR3 = crate::Reg<u32, _OCMDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCMDR3;
#[doc = "`read()` method returns [ocmdr3::R](ocmdr3::R) reader structure"]
impl crate::Readable for OCMDR3 {}
#[doc = "`write(|w| ..)` method takes [ocmdr3::W](ocmdr3::W) writer structure"]
impl crate::Writable for OCMDR3 {}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr3;
