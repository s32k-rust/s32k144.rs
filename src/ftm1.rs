#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: SC,
    #[doc = "0x04 - Counter"]
    pub cnt: CNT,
    #[doc = "0x08 - Modulo"]
    pub mod_: MOD,
    #[doc = "0x0c - Channel (n) Status And Control"]
    pub c0sc: C0SC,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: C0V,
    #[doc = "0x14 - Channel (n) Status And Control"]
    pub c1sc: C1SC,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: C1V,
    #[doc = "0x1c - Channel (n) Status And Control"]
    pub c2sc: C2SC,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: C2V,
    #[doc = "0x24 - Channel (n) Status And Control"]
    pub c3sc: C3SC,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: C3V,
    #[doc = "0x2c - Channel (n) Status And Control"]
    pub c4sc: C4SC,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: C4V,
    #[doc = "0x34 - Channel (n) Status And Control"]
    pub c5sc: C5SC,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: C5V,
    #[doc = "0x3c - Channel (n) Status And Control"]
    pub c6sc: C6SC,
    #[doc = "0x40 - Channel (n) Value"]
    pub c6v: C6V,
    #[doc = "0x44 - Channel (n) Status And Control"]
    pub c7sc: C7SC,
    #[doc = "0x48 - Channel (n) Value"]
    pub c7v: C7V,
    #[doc = "0x4c - Counter Initial Value"]
    pub cntin: CNTIN,
    #[doc = "0x50 - Capture And Compare Status"]
    pub status: STATUS,
    #[doc = "0x54 - Features Mode Selection"]
    pub mode: MODE,
    #[doc = "0x58 - Synchronization"]
    pub sync: SYNC,
    #[doc = "0x5c - Initial State For Channels Output"]
    pub outinit: OUTINIT,
    #[doc = "0x60 - Output Mask"]
    pub outmask: OUTMASK,
    #[doc = "0x64 - Function For Linked Channels"]
    pub combine: COMBINE,
    #[doc = "0x68 - Deadtime Configuration"]
    pub deadtime: DEADTIME,
    #[doc = "0x6c - FTM External Trigger"]
    pub exttrig: EXTTRIG,
    #[doc = "0x70 - Channels Polarity"]
    pub pol: POL,
    #[doc = "0x74 - Fault Mode Status"]
    pub fms: FMS,
    #[doc = "0x78 - Input Capture Filter Control"]
    pub filter: FILTER,
    #[doc = "0x7c - Fault Control"]
    pub fltctrl: FLTCTRL,
    #[doc = "0x80 - Quadrature Decoder Control And Status"]
    pub qdctrl: QDCTRL,
    #[doc = "0x84 - Configuration"]
    pub conf: CONF,
    #[doc = "0x88 - FTM Fault Input Polarity"]
    pub fltpol: FLTPOL,
    #[doc = "0x8c - Synchronization Configuration"]
    pub synconf: SYNCONF,
    #[doc = "0x90 - FTM Inverting Control"]
    pub invctrl: INVCTRL,
    #[doc = "0x94 - FTM Software Output Control"]
    pub swoctrl: SWOCTRL,
    #[doc = "0x98 - FTM PWM Load"]
    pub pwmload: PWMLOAD,
    #[doc = "0x9c - Half Cycle Register"]
    pub hcr: HCR,
    #[doc = "0xa0 - Pair 0 Deadtime Configuration"]
    pub pair0deadtime: PAIR0DEADTIME,
    _reserved0: [u8; 4usize],
    #[doc = "0xa8 - Pair 1 Deadtime Configuration"]
    pub pair1deadtime: PAIR1DEADTIME,
    _reserved1: [u8; 4usize],
    #[doc = "0xb0 - Pair 2 Deadtime Configuration"]
    pub pair2deadtime: PAIR2DEADTIME,
    _reserved2: [u8; 4usize],
    #[doc = "0xb8 - Pair 3 Deadtime Configuration"]
    pub pair3deadtime: PAIR3DEADTIME,
}
#[doc = "Status And Control"]
pub struct SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status And Control"]
pub mod sc;
#[doc = "Counter"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter"]
pub mod cnt;
#[doc = "Modulo"]
pub struct MOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modulo"]
pub mod mod_;
#[doc = "Channel (n) Status And Control"]
pub struct C0SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c0sc;
#[doc = "Channel (n) Value"]
pub struct C0V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c0v;
#[doc = "Channel (n) Status And Control"]
pub struct C1SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c1sc;
#[doc = "Channel (n) Value"]
pub struct C1V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c1v;
#[doc = "Channel (n) Status And Control"]
pub struct C2SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c2sc;
#[doc = "Channel (n) Value"]
pub struct C2V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c2v;
#[doc = "Channel (n) Status And Control"]
pub struct C3SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c3sc;
#[doc = "Channel (n) Value"]
pub struct C3V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c3v;
#[doc = "Channel (n) Status And Control"]
pub struct C4SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c4sc;
#[doc = "Channel (n) Value"]
pub struct C4V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c4v;
#[doc = "Channel (n) Status And Control"]
pub struct C5SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c5sc;
#[doc = "Channel (n) Value"]
pub struct C5V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c5v;
#[doc = "Channel (n) Status And Control"]
pub struct C6SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c6sc;
#[doc = "Channel (n) Value"]
pub struct C6V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c6v;
#[doc = "Channel (n) Status And Control"]
pub struct C7SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Status And Control"]
pub mod c7sc;
#[doc = "Channel (n) Value"]
pub struct C7V {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel (n) Value"]
pub mod c7v;
#[doc = "Counter Initial Value"]
pub struct CNTIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Initial Value"]
pub mod cntin;
#[doc = "Capture And Compare Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture And Compare Status"]
pub mod status;
#[doc = "Features Mode Selection"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Features Mode Selection"]
pub mod mode;
#[doc = "Synchronization"]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization"]
pub mod sync;
#[doc = "Initial State For Channels Output"]
pub struct OUTINIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial State For Channels Output"]
pub mod outinit;
#[doc = "Output Mask"]
pub struct OUTMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Mask"]
pub mod outmask;
#[doc = "Function For Linked Channels"]
pub struct COMBINE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function For Linked Channels"]
pub mod combine;
#[doc = "Deadtime Configuration"]
pub struct DEADTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deadtime Configuration"]
pub mod deadtime;
#[doc = "FTM External Trigger"]
pub struct EXTTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTM External Trigger"]
pub mod exttrig;
#[doc = "Channels Polarity"]
pub struct POL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channels Polarity"]
pub mod pol;
#[doc = "Fault Mode Status"]
pub struct FMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault Mode Status"]
pub mod fms;
#[doc = "Input Capture Filter Control"]
pub struct FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Capture Filter Control"]
pub mod filter;
#[doc = "Fault Control"]
pub struct FLTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fault Control"]
pub mod fltctrl;
#[doc = "Quadrature Decoder Control And Status"]
pub struct QDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Decoder Control And Status"]
pub mod qdctrl;
#[doc = "Configuration"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod conf;
#[doc = "FTM Fault Input Polarity"]
pub struct FLTPOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTM Fault Input Polarity"]
pub mod fltpol;
#[doc = "Synchronization Configuration"]
pub struct SYNCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Configuration"]
pub mod synconf;
#[doc = "FTM Inverting Control"]
pub struct INVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTM Inverting Control"]
pub mod invctrl;
#[doc = "FTM Software Output Control"]
pub struct SWOCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTM Software Output Control"]
pub mod swoctrl;
#[doc = "FTM PWM Load"]
pub struct PWMLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FTM PWM Load"]
pub mod pwmload;
#[doc = "Half Cycle Register"]
pub struct HCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Half Cycle Register"]
pub mod hcr;
#[doc = "Pair 0 Deadtime Configuration"]
pub struct PAIR0DEADTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pair 0 Deadtime Configuration"]
pub mod pair0deadtime;
#[doc = "Pair 1 Deadtime Configuration"]
pub struct PAIR1DEADTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pair 1 Deadtime Configuration"]
pub mod pair1deadtime;
#[doc = "Pair 2 Deadtime Configuration"]
pub struct PAIR2DEADTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pair 2 Deadtime Configuration"]
pub mod pair2deadtime;
#[doc = "Pair 3 Deadtime Configuration"]
pub struct PAIR3DEADTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pair 3 Deadtime Configuration"]
pub mod pair3deadtime;
