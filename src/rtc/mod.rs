use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Time Seconds Register"] pub tsr: TSR,
    #[doc = "0x04 - RTC Time Prescaler Register"] pub tpr: TPR,
    #[doc = "0x08 - RTC Time Alarm Register"] pub tar: TAR,
    #[doc = "0x0c - RTC Time Compensation Register"] pub tcr: TCR,
    #[doc = "0x10 - RTC Control Register"] pub cr: CR,
    #[doc = "0x14 - RTC Status Register"] pub sr: SR,
    #[doc = "0x18 - RTC Lock Register"] pub lr: LR,
    #[doc = "0x1c - RTC Interrupt Enable Register"] pub ier: IER,
}
#[doc = "RTC Time Seconds Register"]
pub struct TSR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Time Seconds Register"]
pub mod tsr;
#[doc = "RTC Time Prescaler Register"]
pub struct TPR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Time Prescaler Register"]
pub mod tpr;
#[doc = "RTC Time Alarm Register"]
pub struct TAR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Time Alarm Register"]
pub mod tar;
#[doc = "RTC Time Compensation Register"]
pub struct TCR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Time Compensation Register"]
pub mod tcr;
#[doc = "RTC Control Register"]
pub struct CR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Control Register"]
pub mod cr;
#[doc = "RTC Status Register"]
pub struct SR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Status Register"]
pub mod sr;
#[doc = "RTC Lock Register"]
pub struct LR {
    register: VolatileCell<u32>,
}
#[doc = "RTC Lock Register"]
pub mod lr;
#[doc = "RTC Interrupt Enable Register"]
pub struct IER {
    register: VolatileCell<u32>,
}
#[doc = "RTC Interrupt Enable Register"]
pub mod ier;
