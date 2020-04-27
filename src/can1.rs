#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: TIMER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: RXMGMASK,
    #[doc = "0x14 - Rx 14 Mask register"]
    pub rx14mask: RX14MASK,
    #[doc = "0x18 - Rx 15 Mask register"]
    pub rx15mask: RX15MASK,
    #[doc = "0x1c - Error Counter"]
    pub ecr: ECR,
    #[doc = "0x20 - Error and Status 1 register"]
    pub esr1: ESR1,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: IMASK1,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: ESR2,
    _reserved12: [u8; 8usize],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    #[doc = "0x50 - CAN Bit Timing Register"]
    pub cbt: CBT,
    _reserved16: [u8; 44usize],
    #[doc = "0x80 - Embedded RAM"]
    pub embedded_ram: [EMBEDDEDRAM; 64],
    _reserved17: [u8; 1792usize],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 16],
    _reserved18: [u8; 576usize],
    #[doc = "0xb00 - Pretended Networking Control 1 Register"]
    pub ctrl1_pn: CTRL1_PN,
    #[doc = "0xb04 - Pretended Networking Control 2 Register"]
    pub ctrl2_pn: CTRL2_PN,
    #[doc = "0xb08 - Pretended Networking Wake Up Match Register"]
    pub wu_mtc: WU_MTC,
    #[doc = "0xb0c - Pretended Networking ID Filter 1 Register"]
    pub flt_id1: FLT_ID1,
    #[doc = "0xb10 - Pretended Networking DLC Filter Register"]
    pub flt_dlc: FLT_DLC,
    #[doc = "0xb14 - Pretended Networking Payload Low Filter 1 Register"]
    pub pl1_lo: PL1_LO,
    #[doc = "0xb18 - Pretended Networking Payload High Filter 1 Register"]
    pub pl1_hi: PL1_HI,
    #[doc = "0xb1c - Pretended Networking ID Filter 2 Register / ID Mask Register"]
    pub flt_id2_idmask: FLT_ID2_IDMASK,
    #[doc = "0xb20 - Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
    pub pl2_plmask_lo: PL2_PLMASK_LO,
    #[doc = "0xb24 - Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
    pub pl2_plmask_hi: PL2_PLMASK_HI,
    _reserved28: [u8; 24usize],
    #[doc = "0xb40 - Wake Up Message Buffer Register for C/S"]
    pub wmb0_cs: WMB0_CS,
    #[doc = "0xb44 - Wake Up Message Buffer Register for ID"]
    pub wmb0_id: WMB0_ID,
    #[doc = "0xb48 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb0_d03: WMB0_D03,
    #[doc = "0xb4c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb0_d47: WMB0_D47,
    #[doc = "0xb50 - Wake Up Message Buffer Register for C/S"]
    pub wmb1_cs: WMB1_CS,
    #[doc = "0xb54 - Wake Up Message Buffer Register for ID"]
    pub wmb1_id: WMB1_ID,
    #[doc = "0xb58 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb1_d03: WMB1_D03,
    #[doc = "0xb5c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb1_d47: WMB1_D47,
    #[doc = "0xb60 - Wake Up Message Buffer Register for C/S"]
    pub wmb2_cs: WMB2_CS,
    #[doc = "0xb64 - Wake Up Message Buffer Register for ID"]
    pub wmb2_id: WMB2_ID,
    #[doc = "0xb68 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb2_d03: WMB2_D03,
    #[doc = "0xb6c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb2_d47: WMB2_D47,
    #[doc = "0xb70 - Wake Up Message Buffer Register for C/S"]
    pub wmb3_cs: WMB3_CS,
    #[doc = "0xb74 - Wake Up Message Buffer Register for ID"]
    pub wmb3_id: WMB3_ID,
    #[doc = "0xb78 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb3_d03: WMB3_D03,
    #[doc = "0xb7c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb3_d47: WMB3_D47,
    _reserved44: [u8; 128usize],
    #[doc = "0xc00 - CAN FD Control Register"]
    pub fdctrl: FDCTRL,
    #[doc = "0xc04 - CAN FD Bit Timing Register"]
    pub fdcbt: FDCBT,
    #[doc = "0xc08 - CAN FD CRC Register"]
    pub fdcrc: FDCRC,
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "Free Running Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](timer) module"]
pub type TIMER = crate::Reg<u32, _TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER;
#[doc = "`read()` method returns [timer::R](timer::R) reader structure"]
impl crate::Readable for TIMER {}
#[doc = "`write(|w| ..)` method takes [timer::W](timer::W) writer structure"]
impl crate::Writable for TIMER {}
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "Rx Mailboxes Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmgmask](rxmgmask) module"]
pub type RXMGMASK = crate::Reg<u32, _RXMGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMGMASK;
#[doc = "`read()` method returns [rxmgmask::R](rxmgmask::R) reader structure"]
impl crate::Readable for RXMGMASK {}
#[doc = "`write(|w| ..)` method takes [rxmgmask::W](rxmgmask::W) writer structure"]
impl crate::Writable for RXMGMASK {}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "Rx 14 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx14mask](rx14mask) module"]
pub type RX14MASK = crate::Reg<u32, _RX14MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX14MASK;
#[doc = "`read()` method returns [rx14mask::R](rx14mask::R) reader structure"]
impl crate::Readable for RX14MASK {}
#[doc = "`write(|w| ..)` method takes [rx14mask::W](rx14mask::W) writer structure"]
impl crate::Writable for RX14MASK {}
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "Rx 15 Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx15mask](rx15mask) module"]
pub type RX15MASK = crate::Reg<u32, _RX15MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX15MASK;
#[doc = "`read()` method returns [rx15mask::R](rx15mask::R) reader structure"]
impl crate::Readable for RX15MASK {}
#[doc = "`write(|w| ..)` method takes [rx15mask::W](rx15mask::W) writer structure"]
impl crate::Writable for RX15MASK {}
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "Error and Status 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr1](esr1) module"]
pub type ESR1 = crate::Reg<u32, _ESR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR1;
#[doc = "`read()` method returns [esr1::R](esr1::R) reader structure"]
impl crate::Readable for ESR1 {}
#[doc = "`write(|w| ..)` method takes [esr1::W](esr1::W) writer structure"]
impl crate::Writable for ESR1 {}
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "Interrupt Masks 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imask1](imask1) module"]
pub type IMASK1 = crate::Reg<u32, _IMASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMASK1;
#[doc = "`read()` method returns [imask1::R](imask1::R) reader structure"]
impl crate::Readable for IMASK1 {}
#[doc = "`write(|w| ..)` method takes [imask1::W](imask1::W) writer structure"]
impl crate::Writable for IMASK1 {}
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "Interrupt Flags 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag1](iflag1) module"]
pub type IFLAG1 = crate::Reg<u32, _IFLAG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFLAG1;
#[doc = "`read()` method returns [iflag1::R](iflag1::R) reader structure"]
impl crate::Readable for IFLAG1 {}
#[doc = "`write(|w| ..)` method takes [iflag1::W](iflag1::W) writer structure"]
impl crate::Writable for IFLAG1 {}
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "Error and Status 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr2](esr2) module"]
pub type ESR2 = crate::Reg<u32, _ESR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR2;
#[doc = "`read()` method returns [esr2::R](esr2::R) reader structure"]
impl crate::Readable for ESR2 {}
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcr](crcr) module"]
pub type CRCR = crate::Reg<u32, _CRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCR;
#[doc = "`read()` method returns [crcr::R](crcr::R) reader structure"]
impl crate::Readable for CRCR {}
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "Rx FIFO Global Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfgmask](rxfgmask) module"]
pub type RXFGMASK = crate::Reg<u32, _RXFGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFGMASK;
#[doc = "`read()` method returns [rxfgmask::R](rxfgmask::R) reader structure"]
impl crate::Readable for RXFGMASK {}
#[doc = "`write(|w| ..)` method takes [rxfgmask::W](rxfgmask::W) writer structure"]
impl crate::Writable for RXFGMASK {}
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "Rx FIFO Information Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfir](rxfir) module"]
pub type RXFIR = crate::Reg<u32, _RXFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIR;
#[doc = "`read()` method returns [rxfir::R](rxfir::R) reader structure"]
impl crate::Readable for RXFIR {}
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "CAN Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbt](cbt) module"]
pub type CBT = crate::Reg<u32, _CBT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CBT;
#[doc = "`read()` method returns [cbt::R](cbt::R) reader structure"]
impl crate::Readable for CBT {}
#[doc = "`write(|w| ..)` method takes [cbt::W](cbt::W) writer structure"]
impl crate::Writable for CBT {}
#[doc = "CAN Bit Timing Register"]
pub mod cbt;
#[doc = "Embedded RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [embedded_ram](embedded_ram) module"]
pub type EMBEDDEDRAM = crate::Reg<u32, _EMBEDDEDRAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMBEDDEDRAM;
#[doc = "`read()` method returns [embedded_ram::R](embedded_ram::R) reader structure"]
impl crate::Readable for EMBEDDEDRAM {}
#[doc = "`write(|w| ..)` method takes [embedded_ram::W](embedded_ram::W) writer structure"]
impl crate::Writable for EMBEDDEDRAM {}
#[doc = "Embedded RAM"]
pub mod embedded_ram;
#[doc = "Rx Individual Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rximr](rximr) module"]
pub type RXIMR = crate::Reg<u32, _RXIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIMR;
#[doc = "`read()` method returns [rximr::R](rximr::R) reader structure"]
impl crate::Readable for RXIMR {}
#[doc = "`write(|w| ..)` method takes [rximr::W](rximr::W) writer structure"]
impl crate::Writable for RXIMR {}
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
#[doc = "Pretended Networking Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1_pn](ctrl1_pn) module"]
pub type CTRL1_PN = crate::Reg<u32, _CTRL1_PN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1_PN;
#[doc = "`read()` method returns [ctrl1_pn::R](ctrl1_pn::R) reader structure"]
impl crate::Readable for CTRL1_PN {}
#[doc = "`write(|w| ..)` method takes [ctrl1_pn::W](ctrl1_pn::W) writer structure"]
impl crate::Writable for CTRL1_PN {}
#[doc = "Pretended Networking Control 1 Register"]
pub mod ctrl1_pn;
#[doc = "Pretended Networking Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2_pn](ctrl2_pn) module"]
pub type CTRL2_PN = crate::Reg<u32, _CTRL2_PN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2_PN;
#[doc = "`read()` method returns [ctrl2_pn::R](ctrl2_pn::R) reader structure"]
impl crate::Readable for CTRL2_PN {}
#[doc = "`write(|w| ..)` method takes [ctrl2_pn::W](ctrl2_pn::W) writer structure"]
impl crate::Writable for CTRL2_PN {}
#[doc = "Pretended Networking Control 2 Register"]
pub mod ctrl2_pn;
#[doc = "Pretended Networking Wake Up Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wu_mtc](wu_mtc) module"]
pub type WU_MTC = crate::Reg<u32, _WU_MTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WU_MTC;
#[doc = "`read()` method returns [wu_mtc::R](wu_mtc::R) reader structure"]
impl crate::Readable for WU_MTC {}
#[doc = "`write(|w| ..)` method takes [wu_mtc::W](wu_mtc::W) writer structure"]
impl crate::Writable for WU_MTC {}
#[doc = "Pretended Networking Wake Up Match Register"]
pub mod wu_mtc;
#[doc = "Pretended Networking ID Filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_id1](flt_id1) module"]
pub type FLT_ID1 = crate::Reg<u32, _FLT_ID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT_ID1;
#[doc = "`read()` method returns [flt_id1::R](flt_id1::R) reader structure"]
impl crate::Readable for FLT_ID1 {}
#[doc = "`write(|w| ..)` method takes [flt_id1::W](flt_id1::W) writer structure"]
impl crate::Writable for FLT_ID1 {}
#[doc = "Pretended Networking ID Filter 1 Register"]
pub mod flt_id1;
#[doc = "Pretended Networking DLC Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_dlc](flt_dlc) module"]
pub type FLT_DLC = crate::Reg<u32, _FLT_DLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT_DLC;
#[doc = "`read()` method returns [flt_dlc::R](flt_dlc::R) reader structure"]
impl crate::Readable for FLT_DLC {}
#[doc = "`write(|w| ..)` method takes [flt_dlc::W](flt_dlc::W) writer structure"]
impl crate::Writable for FLT_DLC {}
#[doc = "Pretended Networking DLC Filter Register"]
pub mod flt_dlc;
#[doc = "Pretended Networking Payload Low Filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl1_lo](pl1_lo) module"]
pub type PL1_LO = crate::Reg<u32, _PL1_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL1_LO;
#[doc = "`read()` method returns [pl1_lo::R](pl1_lo::R) reader structure"]
impl crate::Readable for PL1_LO {}
#[doc = "`write(|w| ..)` method takes [pl1_lo::W](pl1_lo::W) writer structure"]
impl crate::Writable for PL1_LO {}
#[doc = "Pretended Networking Payload Low Filter 1 Register"]
pub mod pl1_lo;
#[doc = "Pretended Networking Payload High Filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl1_hi](pl1_hi) module"]
pub type PL1_HI = crate::Reg<u32, _PL1_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL1_HI;
#[doc = "`read()` method returns [pl1_hi::R](pl1_hi::R) reader structure"]
impl crate::Readable for PL1_HI {}
#[doc = "`write(|w| ..)` method takes [pl1_hi::W](pl1_hi::W) writer structure"]
impl crate::Writable for PL1_HI {}
#[doc = "Pretended Networking Payload High Filter 1 Register"]
pub mod pl1_hi;
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_id2_idmask](flt_id2_idmask) module"]
pub type FLT_ID2_IDMASK = crate::Reg<u32, _FLT_ID2_IDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT_ID2_IDMASK;
#[doc = "`read()` method returns [flt_id2_idmask::R](flt_id2_idmask::R) reader structure"]
impl crate::Readable for FLT_ID2_IDMASK {}
#[doc = "`write(|w| ..)` method takes [flt_id2_idmask::W](flt_id2_idmask::W) writer structure"]
impl crate::Writable for FLT_ID2_IDMASK {}
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register"]
pub mod flt_id2_idmask;
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl2_plmask_lo](pl2_plmask_lo) module"]
pub type PL2_PLMASK_LO = crate::Reg<u32, _PL2_PLMASK_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL2_PLMASK_LO;
#[doc = "`read()` method returns [pl2_plmask_lo::R](pl2_plmask_lo::R) reader structure"]
impl crate::Readable for PL2_PLMASK_LO {}
#[doc = "`write(|w| ..)` method takes [pl2_plmask_lo::W](pl2_plmask_lo::W) writer structure"]
impl crate::Writable for PL2_PLMASK_LO {}
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
pub mod pl2_plmask_lo;
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl2_plmask_hi](pl2_plmask_hi) module"]
pub type PL2_PLMASK_HI = crate::Reg<u32, _PL2_PLMASK_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL2_PLMASK_HI;
#[doc = "`read()` method returns [pl2_plmask_hi::R](pl2_plmask_hi::R) reader structure"]
impl crate::Readable for PL2_PLMASK_HI {}
#[doc = "`write(|w| ..)` method takes [pl2_plmask_hi::W](pl2_plmask_hi::W) writer structure"]
impl crate::Writable for PL2_PLMASK_HI {}
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
pub mod pl2_plmask_hi;
#[doc = "Wake Up Message Buffer Register for C/S\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb0_cs](wmb0_cs) module"]
pub type WMB0_CS = crate::Reg<u32, _WMB0_CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB0_CS;
#[doc = "`read()` method returns [wmb0_cs::R](wmb0_cs::R) reader structure"]
impl crate::Readable for WMB0_CS {}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb0_cs;
#[doc = "Wake Up Message Buffer Register for ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb0_id](wmb0_id) module"]
pub type WMB0_ID = crate::Reg<u32, _WMB0_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB0_ID;
#[doc = "`read()` method returns [wmb0_id::R](wmb0_id::R) reader structure"]
impl crate::Readable for WMB0_ID {}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb0_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb0_d03](wmb0_d03) module"]
pub type WMB0_D03 = crate::Reg<u32, _WMB0_D03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB0_D03;
#[doc = "`read()` method returns [wmb0_d03::R](wmb0_d03::R) reader structure"]
impl crate::Readable for WMB0_D03 {}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb0_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb0_d47](wmb0_d47) module"]
pub type WMB0_D47 = crate::Reg<u32, _WMB0_D47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB0_D47;
#[doc = "`read()` method returns [wmb0_d47::R](wmb0_d47::R) reader structure"]
impl crate::Readable for WMB0_D47 {}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb0_d47;
#[doc = "Wake Up Message Buffer Register for C/S\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb1_cs](wmb1_cs) module"]
pub type WMB1_CS = crate::Reg<u32, _WMB1_CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB1_CS;
#[doc = "`read()` method returns [wmb1_cs::R](wmb1_cs::R) reader structure"]
impl crate::Readable for WMB1_CS {}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb1_cs;
#[doc = "Wake Up Message Buffer Register for ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb1_id](wmb1_id) module"]
pub type WMB1_ID = crate::Reg<u32, _WMB1_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB1_ID;
#[doc = "`read()` method returns [wmb1_id::R](wmb1_id::R) reader structure"]
impl crate::Readable for WMB1_ID {}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb1_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb1_d03](wmb1_d03) module"]
pub type WMB1_D03 = crate::Reg<u32, _WMB1_D03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB1_D03;
#[doc = "`read()` method returns [wmb1_d03::R](wmb1_d03::R) reader structure"]
impl crate::Readable for WMB1_D03 {}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb1_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb1_d47](wmb1_d47) module"]
pub type WMB1_D47 = crate::Reg<u32, _WMB1_D47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB1_D47;
#[doc = "`read()` method returns [wmb1_d47::R](wmb1_d47::R) reader structure"]
impl crate::Readable for WMB1_D47 {}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb1_d47;
#[doc = "Wake Up Message Buffer Register for C/S\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb2_cs](wmb2_cs) module"]
pub type WMB2_CS = crate::Reg<u32, _WMB2_CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB2_CS;
#[doc = "`read()` method returns [wmb2_cs::R](wmb2_cs::R) reader structure"]
impl crate::Readable for WMB2_CS {}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb2_cs;
#[doc = "Wake Up Message Buffer Register for ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb2_id](wmb2_id) module"]
pub type WMB2_ID = crate::Reg<u32, _WMB2_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB2_ID;
#[doc = "`read()` method returns [wmb2_id::R](wmb2_id::R) reader structure"]
impl crate::Readable for WMB2_ID {}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb2_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb2_d03](wmb2_d03) module"]
pub type WMB2_D03 = crate::Reg<u32, _WMB2_D03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB2_D03;
#[doc = "`read()` method returns [wmb2_d03::R](wmb2_d03::R) reader structure"]
impl crate::Readable for WMB2_D03 {}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb2_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb2_d47](wmb2_d47) module"]
pub type WMB2_D47 = crate::Reg<u32, _WMB2_D47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB2_D47;
#[doc = "`read()` method returns [wmb2_d47::R](wmb2_d47::R) reader structure"]
impl crate::Readable for WMB2_D47 {}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb2_d47;
#[doc = "Wake Up Message Buffer Register for C/S\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb3_cs](wmb3_cs) module"]
pub type WMB3_CS = crate::Reg<u32, _WMB3_CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB3_CS;
#[doc = "`read()` method returns [wmb3_cs::R](wmb3_cs::R) reader structure"]
impl crate::Readable for WMB3_CS {}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb3_cs;
#[doc = "Wake Up Message Buffer Register for ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb3_id](wmb3_id) module"]
pub type WMB3_ID = crate::Reg<u32, _WMB3_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB3_ID;
#[doc = "`read()` method returns [wmb3_id::R](wmb3_id::R) reader structure"]
impl crate::Readable for WMB3_ID {}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb3_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb3_d03](wmb3_d03) module"]
pub type WMB3_D03 = crate::Reg<u32, _WMB3_D03>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB3_D03;
#[doc = "`read()` method returns [wmb3_d03::R](wmb3_d03::R) reader structure"]
impl crate::Readable for WMB3_D03 {}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb3_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb3_d47](wmb3_d47) module"]
pub type WMB3_D47 = crate::Reg<u32, _WMB3_D47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WMB3_D47;
#[doc = "`read()` method returns [wmb3_d47::R](wmb3_d47::R) reader structure"]
impl crate::Readable for WMB3_D47 {}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb3_d47;
#[doc = "CAN FD Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdctrl](fdctrl) module"]
pub type FDCTRL = crate::Reg<u32, _FDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCTRL;
#[doc = "`read()` method returns [fdctrl::R](fdctrl::R) reader structure"]
impl crate::Readable for FDCTRL {}
#[doc = "`write(|w| ..)` method takes [fdctrl::W](fdctrl::W) writer structure"]
impl crate::Writable for FDCTRL {}
#[doc = "CAN FD Control Register"]
pub mod fdctrl;
#[doc = "CAN FD Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcbt](fdcbt) module"]
pub type FDCBT = crate::Reg<u32, _FDCBT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCBT;
#[doc = "`read()` method returns [fdcbt::R](fdcbt::R) reader structure"]
impl crate::Readable for FDCBT {}
#[doc = "`write(|w| ..)` method takes [fdcbt::W](fdcbt::W) writer structure"]
impl crate::Writable for FDCBT {}
#[doc = "CAN FD Bit Timing Register"]
pub mod fdcbt;
#[doc = "CAN FD CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcrc](fdcrc) module"]
pub type FDCRC = crate::Reg<u32, _FDCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDCRC;
#[doc = "`read()` method returns [fdcrc::R](fdcrc::R) reader structure"]
impl crate::Readable for FDCRC {}
#[doc = "CAN FD CRC Register"]
pub mod fdcrc;
