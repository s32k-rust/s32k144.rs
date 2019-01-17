#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: TIMER,
    _reserved0: [u8; 4usize],
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
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: IMASK1,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: IFLAG1,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: CTRL2,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: ESR2,
    _reserved3: [u8; 8usize],
    #[doc = "0x44 - CRC Register"]
    pub crcr: CRCR,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: RXFGMASK,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: RXFIR,
    #[doc = "0x50 - CAN Bit Timing Register"]
    pub cbt: CBT,
    _reserved4: [u8; 44usize],
    #[doc = "0x80 - Embedded RAM"]
    pub embedded_ram: [EMBEDDEDRAM; 128],
    _reserved5: [u8; 1536usize],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr: [RXIMR; 16],
    _reserved6: [u8; 576usize],
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
    _reserved7: [u8; 24usize],
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
    _reserved8: [u8; 128usize],
    #[doc = "0xc00 - CAN FD Control Register"]
    pub fdctrl: FDCTRL,
    #[doc = "0xc04 - CAN FD Bit Timing Register"]
    pub fdcbt: FDCBT,
    #[doc = "0xc08 - CAN FD CRC Register"]
    pub fdcrc: FDCRC,
}
#[doc = "Module Configuration Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Control 1 register"]
pub struct CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "Free Running Timer"]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "Rx Mailboxes Global Mask Register"]
pub struct RXMGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "Rx 14 Mask register"]
pub struct RX14MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "Rx 15 Mask register"]
pub struct RX15MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "Error Counter"]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "Error and Status 1 register"]
pub struct ESR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "Interrupt Masks 1 register"]
pub struct IMASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "Interrupt Flags 1 register"]
pub struct IFLAG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "Control 2 register"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "Error and Status 2 register"]
pub struct ESR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRC Register"]
pub struct CRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "Rx FIFO Global Mask register"]
pub struct RXFGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "Rx FIFO Information Register"]
pub struct RXFIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "CAN Bit Timing Register"]
pub struct CBT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Bit Timing Register"]
pub mod cbt;
#[doc = "Embedded RAM"]
pub struct EMBEDDEDRAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Embedded RAM"]
pub mod embedded_ram;
#[doc = "Rx Individual Mask Registers"]
pub struct RXIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Individual Mask Registers"]
pub mod rximr;
#[doc = "Pretended Networking Control 1 Register"]
pub struct CTRL1_PN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Control 1 Register"]
pub mod ctrl1_pn;
#[doc = "Pretended Networking Control 2 Register"]
pub struct CTRL2_PN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Control 2 Register"]
pub mod ctrl2_pn;
#[doc = "Pretended Networking Wake Up Match Register"]
pub struct WU_MTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Wake Up Match Register"]
pub mod wu_mtc;
#[doc = "Pretended Networking ID Filter 1 Register"]
pub struct FLT_ID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking ID Filter 1 Register"]
pub mod flt_id1;
#[doc = "Pretended Networking DLC Filter Register"]
pub struct FLT_DLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking DLC Filter Register"]
pub mod flt_dlc;
#[doc = "Pretended Networking Payload Low Filter 1 Register"]
pub struct PL1_LO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Payload Low Filter 1 Register"]
pub mod pl1_lo;
#[doc = "Pretended Networking Payload High Filter 1 Register"]
pub struct PL1_HI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Payload High Filter 1 Register"]
pub mod pl1_hi;
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register"]
pub struct FLT_ID2_IDMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register"]
pub mod flt_id2_idmask;
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
pub struct PL2_PLMASK_LO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
pub mod pl2_plmask_lo;
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
pub struct PL2_PLMASK_HI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
pub mod pl2_plmask_hi;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub struct WMB0_CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb0_cs;
#[doc = "Wake Up Message Buffer Register for ID"]
pub struct WMB0_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb0_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub struct WMB0_D03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb0_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub struct WMB0_D47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb0_d47;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub struct WMB1_CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb1_cs;
#[doc = "Wake Up Message Buffer Register for ID"]
pub struct WMB1_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb1_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub struct WMB1_D03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb1_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub struct WMB1_D47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb1_d47;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub struct WMB2_CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb2_cs;
#[doc = "Wake Up Message Buffer Register for ID"]
pub struct WMB2_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb2_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub struct WMB2_D03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb2_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub struct WMB2_D47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb2_d47;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub struct WMB3_CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb3_cs;
#[doc = "Wake Up Message Buffer Register for ID"]
pub struct WMB3_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb3_id;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub struct WMB3_D03 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb3_d03;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub struct WMB3_D47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb3_d47;
#[doc = "CAN FD Control Register"]
pub struct FDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN FD Control Register"]
pub mod fdctrl;
#[doc = "CAN FD Bit Timing Register"]
pub struct FDCBT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN FD Bit Timing Register"]
pub mod fdcbt;
#[doc = "CAN FD CRC Register"]
pub struct FDCRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN FD CRC Register"]
pub mod fdcrc;
