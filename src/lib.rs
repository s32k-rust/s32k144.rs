#![doc = "Peripheral access API for S32K144 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0();
    fn DMA1();
    fn DMA2();
    fn DMA3();
    fn DMA4();
    fn DMA5();
    fn DMA6();
    fn DMA7();
    fn DMA8();
    fn DMA9();
    fn DMA10();
    fn DMA11();
    fn DMA12();
    fn DMA13();
    fn DMA14();
    fn DMA15();
    fn DMA_ERROR();
    fn MCM();
    fn FTFC();
    fn READ_COLLISION();
    fn LVD_LVW();
    fn FTFC_FAULT();
    fn WDOG_EWM();
    fn RCM();
    fn LPI2C0_MASTER();
    fn LPI2C0_SLAVE();
    fn LPSPI0();
    fn LPSPI1();
    fn LPSPI2();
    fn LPUART0_RXTX();
    fn LPUART1_RXTX();
    fn LPUART2_RXTX();
    fn ADC0();
    fn ADC1();
    fn CMP0();
    fn ERM_SINGLE_FAULT();
    fn ERM_DOUBLE_FAULT();
    fn RTC();
    fn RTC_SECONDS();
    fn LPIT0_CH0();
    fn LPIT0_CH1();
    fn LPIT0_CH2();
    fn LPIT0_CH3();
    fn PDB0();
    fn SCG();
    fn LPTMR0();
    fn PORTA();
    fn PORTB();
    fn PORTC();
    fn PORTD();
    fn PORTE();
    fn PDB1();
    fn FLEXIO();
    fn CAN0_ORED();
    fn CAN0_ERROR();
    fn CAN0_WAKE_UP();
    fn CAN0_ORED_0_15_MB();
    fn CAN0_ORED_16_31_MB();
    fn CAN1_ORED();
    fn CAN1_ERROR();
    fn CAN1_ORED_0_15_MB();
    fn CAN2_ORED();
    fn CAN2_ERROR();
    fn CAN2_ORED_0_15_MB();
    fn FTM0_CH0_CH1();
    fn FTM0_CH2_CH3();
    fn FTM0_CH4_CH5();
    fn FTM0_CH6_CH7();
    fn FTM0_FAULT();
    fn FTM0_OVF_RELOAD();
    fn FTM1_CH0_CH1();
    fn FTM1_CH2_CH3();
    fn FTM1_CH4_CH5();
    fn FTM1_CH6_CH7();
    fn FTM1_FAULT();
    fn FTM1_OVF_RELOAD();
    fn FTM2_CH0_CH1();
    fn FTM2_CH2_CH3();
    fn FTM2_CH4_CH5();
    fn FTM2_CH6_CH7();
    fn FTM2_FAULT();
    fn FTM2_OVF_RELOAD();
    fn FTM3_CH0_CH1();
    fn FTM3_CH2_CH3();
    fn FTM3_CH4_CH5();
    fn FTM3_CH6_CH7();
    fn FTM3_FAULT();
    fn FTM3_OVF_RELOAD();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 123] = [
    Vector { _handler: DMA0 },
    Vector { _handler: DMA1 },
    Vector { _handler: DMA2 },
    Vector { _handler: DMA3 },
    Vector { _handler: DMA4 },
    Vector { _handler: DMA5 },
    Vector { _handler: DMA6 },
    Vector { _handler: DMA7 },
    Vector { _handler: DMA8 },
    Vector { _handler: DMA9 },
    Vector { _handler: DMA10 },
    Vector { _handler: DMA11 },
    Vector { _handler: DMA12 },
    Vector { _handler: DMA13 },
    Vector { _handler: DMA14 },
    Vector { _handler: DMA15 },
    Vector { _handler: DMA_ERROR },
    Vector { _handler: MCM },
    Vector { _handler: FTFC },
    Vector { _handler: READ_COLLISION },
    Vector { _handler: LVD_LVW },
    Vector { _handler: FTFC_FAULT },
    Vector { _handler: WDOG_EWM },
    Vector { _handler: RCM },
    Vector { _handler: LPI2C0_MASTER },
    Vector { _handler: LPI2C0_SLAVE },
    Vector { _handler: LPSPI0 },
    Vector { _handler: LPSPI1 },
    Vector { _handler: LPSPI2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPUART0_RXTX },
    Vector { _reserved: 0 },
    Vector { _handler: LPUART1_RXTX },
    Vector { _reserved: 0 },
    Vector { _handler: LPUART2_RXTX },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC0 },
    Vector { _handler: ADC1 },
    Vector { _handler: CMP0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ERM_SINGLE_FAULT },
    Vector { _handler: ERM_DOUBLE_FAULT },
    Vector { _handler: RTC },
    Vector { _handler: RTC_SECONDS },
    Vector { _handler: LPIT0_CH0 },
    Vector { _handler: LPIT0_CH1 },
    Vector { _handler: LPIT0_CH2 },
    Vector { _handler: LPIT0_CH3 },
    Vector { _handler: PDB0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SCG },
    Vector { _handler: LPTMR0 },
    Vector { _handler: PORTA },
    Vector { _handler: PORTB },
    Vector { _handler: PORTC },
    Vector { _handler: PORTD },
    Vector { _handler: PORTE },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PDB1 },
    Vector { _handler: FLEXIO },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CAN0_ORED },
    Vector { _handler: CAN0_ERROR },
    Vector { _handler: CAN0_WAKE_UP },
    Vector { _handler: CAN0_ORED_0_15_MB },
    Vector { _handler: CAN0_ORED_16_31_MB },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CAN1_ORED },
    Vector { _handler: CAN1_ERROR },
    Vector { _reserved: 0 },
    Vector { _handler: CAN1_ORED_0_15_MB },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CAN2_ORED },
    Vector { _handler: CAN2_ERROR },
    Vector { _reserved: 0 },
    Vector { _handler: CAN2_ORED_0_15_MB },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FTM0_CH0_CH1 },
    Vector { _handler: FTM0_CH2_CH3 },
    Vector { _handler: FTM0_CH4_CH5 },
    Vector { _handler: FTM0_CH6_CH7 },
    Vector { _handler: FTM0_FAULT },
    Vector { _handler: FTM0_OVF_RELOAD },
    Vector { _handler: FTM1_CH0_CH1 },
    Vector { _handler: FTM1_CH2_CH3 },
    Vector { _handler: FTM1_CH4_CH5 },
    Vector { _handler: FTM1_CH6_CH7 },
    Vector { _handler: FTM1_FAULT },
    Vector { _handler: FTM1_OVF_RELOAD },
    Vector { _handler: FTM2_CH0_CH1 },
    Vector { _handler: FTM2_CH2_CH3 },
    Vector { _handler: FTM2_CH4_CH5 },
    Vector { _handler: FTM2_CH6_CH7 },
    Vector { _handler: FTM2_FAULT },
    Vector { _handler: FTM2_OVF_RELOAD },
    Vector { _handler: FTM3_CH0_CH1 },
    Vector { _handler: FTM3_CH2_CH3 },
    Vector { _handler: FTM3_CH4_CH5 },
    Vector { _handler: FTM3_CH6_CH7 },
    Vector { _handler: FTM3_FAULT },
    Vector { _handler: FTM3_OVF_RELOAD },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - DMA0"]
    DMA0,
    #[doc = "1 - DMA1"]
    DMA1,
    #[doc = "2 - DMA2"]
    DMA2,
    #[doc = "3 - DMA3"]
    DMA3,
    #[doc = "4 - DMA4"]
    DMA4,
    #[doc = "5 - DMA5"]
    DMA5,
    #[doc = "6 - DMA6"]
    DMA6,
    #[doc = "7 - DMA7"]
    DMA7,
    #[doc = "8 - DMA8"]
    DMA8,
    #[doc = "9 - DMA9"]
    DMA9,
    #[doc = "10 - DMA10"]
    DMA10,
    #[doc = "11 - DMA11"]
    DMA11,
    #[doc = "12 - DMA12"]
    DMA12,
    #[doc = "13 - DMA13"]
    DMA13,
    #[doc = "14 - DMA14"]
    DMA14,
    #[doc = "15 - DMA15"]
    DMA15,
    #[doc = "16 - DMA_Error"]
    DMA_ERROR,
    #[doc = "17 - MCM"]
    MCM,
    #[doc = "18 - FTFC"]
    FTFC,
    #[doc = "19 - Read_Collision"]
    READ_COLLISION,
    #[doc = "20 - LVD_LVW"]
    LVD_LVW,
    #[doc = "21 - FTFC_Fault"]
    FTFC_FAULT,
    #[doc = "22 - WDOG_EWM"]
    WDOG_EWM,
    #[doc = "23 - RCM"]
    RCM,
    #[doc = "24 - LPI2C0_Master"]
    LPI2C0_MASTER,
    #[doc = "25 - LPI2C0_Slave"]
    LPI2C0_SLAVE,
    #[doc = "26 - LPSPI0"]
    LPSPI0,
    #[doc = "27 - LPSPI1"]
    LPSPI1,
    #[doc = "28 - LPSPI2"]
    LPSPI2,
    #[doc = "31 - LPUART0_RxTx"]
    LPUART0_RXTX,
    #[doc = "33 - LPUART1_RxTx"]
    LPUART1_RXTX,
    #[doc = "35 - LPUART2_RxTx"]
    LPUART2_RXTX,
    #[doc = "39 - ADC0"]
    ADC0,
    #[doc = "40 - ADC1"]
    ADC1,
    #[doc = "41 - CMP0"]
    CMP0,
    #[doc = "44 - ERM_single_fault"]
    ERM_SINGLE_FAULT,
    #[doc = "45 - ERM_double_fault"]
    ERM_DOUBLE_FAULT,
    #[doc = "46 - RTC"]
    RTC,
    #[doc = "47 - RTC_Seconds"]
    RTC_SECONDS,
    #[doc = "48 - LPIT0_Ch0"]
    LPIT0_CH0,
    #[doc = "49 - LPIT0_Ch1"]
    LPIT0_CH1,
    #[doc = "50 - LPIT0_Ch2"]
    LPIT0_CH2,
    #[doc = "51 - LPIT0_Ch3"]
    LPIT0_CH3,
    #[doc = "52 - PDB0"]
    PDB0,
    #[doc = "57 - SCG"]
    SCG,
    #[doc = "58 - LPTMR0"]
    LPTMR0,
    #[doc = "59 - PORTA"]
    PORTA,
    #[doc = "60 - PORTB"]
    PORTB,
    #[doc = "61 - PORTC"]
    PORTC,
    #[doc = "62 - PORTD"]
    PORTD,
    #[doc = "63 - PORTE"]
    PORTE,
    #[doc = "68 - PDB1"]
    PDB1,
    #[doc = "69 - FLEXIO"]
    FLEXIO,
    #[doc = "78 - CAN0_ORed"]
    CAN0_ORED,
    #[doc = "79 - CAN0_Error"]
    CAN0_ERROR,
    #[doc = "80 - CAN0_Wake_Up"]
    CAN0_WAKE_UP,
    #[doc = "81 - CAN0_ORed_0_15_MB"]
    CAN0_ORED_0_15_MB,
    #[doc = "82 - CAN0_ORed_16_31_MB"]
    CAN0_ORED_16_31_MB,
    #[doc = "85 - CAN1_ORed"]
    CAN1_ORED,
    #[doc = "86 - CAN1_Error"]
    CAN1_ERROR,
    #[doc = "88 - CAN1_ORed_0_15_MB"]
    CAN1_ORED_0_15_MB,
    #[doc = "92 - CAN2_ORed"]
    CAN2_ORED,
    #[doc = "93 - CAN2_Error"]
    CAN2_ERROR,
    #[doc = "95 - CAN2_ORed_0_15_MB"]
    CAN2_ORED_0_15_MB,
    #[doc = "99 - FTM0_Ch0_Ch1"]
    FTM0_CH0_CH1,
    #[doc = "100 - FTM0_Ch2_Ch3"]
    FTM0_CH2_CH3,
    #[doc = "101 - FTM0_Ch4_Ch5"]
    FTM0_CH4_CH5,
    #[doc = "102 - FTM0_Ch6_Ch7"]
    FTM0_CH6_CH7,
    #[doc = "103 - FTM0_Fault"]
    FTM0_FAULT,
    #[doc = "104 - FTM0_Ovf_Reload"]
    FTM0_OVF_RELOAD,
    #[doc = "105 - FTM1_Ch0_Ch1"]
    FTM1_CH0_CH1,
    #[doc = "106 - FTM1_Ch2_Ch3"]
    FTM1_CH2_CH3,
    #[doc = "107 - FTM1_Ch4_Ch5"]
    FTM1_CH4_CH5,
    #[doc = "108 - FTM1_Ch6_Ch7"]
    FTM1_CH6_CH7,
    #[doc = "109 - FTM1_Fault"]
    FTM1_FAULT,
    #[doc = "110 - FTM1_Ovf_Reload"]
    FTM1_OVF_RELOAD,
    #[doc = "111 - FTM2_Ch0_Ch1"]
    FTM2_CH0_CH1,
    #[doc = "112 - FTM2_Ch2_Ch3"]
    FTM2_CH2_CH3,
    #[doc = "113 - FTM2_Ch4_Ch5"]
    FTM2_CH4_CH5,
    #[doc = "114 - FTM2_Ch6_Ch7"]
    FTM2_CH6_CH7,
    #[doc = "115 - FTM2_Fault"]
    FTM2_FAULT,
    #[doc = "116 - FTM2_Ovf_Reload"]
    FTM2_OVF_RELOAD,
    #[doc = "117 - FTM3_Ch0_Ch1"]
    FTM3_CH0_CH1,
    #[doc = "118 - FTM3_Ch2_Ch3"]
    FTM3_CH2_CH3,
    #[doc = "119 - FTM3_Ch4_Ch5"]
    FTM3_CH4_CH5,
    #[doc = "120 - FTM3_Ch6_Ch7"]
    FTM3_CH6_CH7,
    #[doc = "121 - FTM3_Fault"]
    FTM3_FAULT,
    #[doc = "122 - FTM3_Ovf_Reload"]
    FTM3_OVF_RELOAD,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0 => 0,
            Interrupt::DMA1 => 1,
            Interrupt::DMA2 => 2,
            Interrupt::DMA3 => 3,
            Interrupt::DMA4 => 4,
            Interrupt::DMA5 => 5,
            Interrupt::DMA6 => 6,
            Interrupt::DMA7 => 7,
            Interrupt::DMA8 => 8,
            Interrupt::DMA9 => 9,
            Interrupt::DMA10 => 10,
            Interrupt::DMA11 => 11,
            Interrupt::DMA12 => 12,
            Interrupt::DMA13 => 13,
            Interrupt::DMA14 => 14,
            Interrupt::DMA15 => 15,
            Interrupt::DMA_ERROR => 16,
            Interrupt::MCM => 17,
            Interrupt::FTFC => 18,
            Interrupt::READ_COLLISION => 19,
            Interrupt::LVD_LVW => 20,
            Interrupt::FTFC_FAULT => 21,
            Interrupt::WDOG_EWM => 22,
            Interrupt::RCM => 23,
            Interrupt::LPI2C0_MASTER => 24,
            Interrupt::LPI2C0_SLAVE => 25,
            Interrupt::LPSPI0 => 26,
            Interrupt::LPSPI1 => 27,
            Interrupt::LPSPI2 => 28,
            Interrupt::LPUART0_RXTX => 31,
            Interrupt::LPUART1_RXTX => 33,
            Interrupt::LPUART2_RXTX => 35,
            Interrupt::ADC0 => 39,
            Interrupt::ADC1 => 40,
            Interrupt::CMP0 => 41,
            Interrupt::ERM_SINGLE_FAULT => 44,
            Interrupt::ERM_DOUBLE_FAULT => 45,
            Interrupt::RTC => 46,
            Interrupt::RTC_SECONDS => 47,
            Interrupt::LPIT0_CH0 => 48,
            Interrupt::LPIT0_CH1 => 49,
            Interrupt::LPIT0_CH2 => 50,
            Interrupt::LPIT0_CH3 => 51,
            Interrupt::PDB0 => 52,
            Interrupt::SCG => 57,
            Interrupt::LPTMR0 => 58,
            Interrupt::PORTA => 59,
            Interrupt::PORTB => 60,
            Interrupt::PORTC => 61,
            Interrupt::PORTD => 62,
            Interrupt::PORTE => 63,
            Interrupt::PDB1 => 68,
            Interrupt::FLEXIO => 69,
            Interrupt::CAN0_ORED => 78,
            Interrupt::CAN0_ERROR => 79,
            Interrupt::CAN0_WAKE_UP => 80,
            Interrupt::CAN0_ORED_0_15_MB => 81,
            Interrupt::CAN0_ORED_16_31_MB => 82,
            Interrupt::CAN1_ORED => 85,
            Interrupt::CAN1_ERROR => 86,
            Interrupt::CAN1_ORED_0_15_MB => 88,
            Interrupt::CAN2_ORED => 92,
            Interrupt::CAN2_ERROR => 93,
            Interrupt::CAN2_ORED_0_15_MB => 95,
            Interrupt::FTM0_CH0_CH1 => 99,
            Interrupt::FTM0_CH2_CH3 => 100,
            Interrupt::FTM0_CH4_CH5 => 101,
            Interrupt::FTM0_CH6_CH7 => 102,
            Interrupt::FTM0_FAULT => 103,
            Interrupt::FTM0_OVF_RELOAD => 104,
            Interrupt::FTM1_CH0_CH1 => 105,
            Interrupt::FTM1_CH2_CH3 => 106,
            Interrupt::FTM1_CH4_CH5 => 107,
            Interrupt::FTM1_CH6_CH7 => 108,
            Interrupt::FTM1_FAULT => 109,
            Interrupt::FTM1_OVF_RELOAD => 110,
            Interrupt::FTM2_CH0_CH1 => 111,
            Interrupt::FTM2_CH2_CH3 => 112,
            Interrupt::FTM2_CH4_CH5 => 113,
            Interrupt::FTM2_CH6_CH7 => 114,
            Interrupt::FTM2_FAULT => 115,
            Interrupt::FTM2_OVF_RELOAD => 116,
            Interrupt::FTM3_CH0_CH1 => 117,
            Interrupt::FTM3_CH2_CH3 => 118,
            Interrupt::FTM3_CH4_CH5 => 119,
            Interrupt::FTM3_CH6_CH7 => 120,
            Interrupt::FTM3_FAULT => 121,
            Interrupt::FTM3_OVF_RELOAD => 122,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "CSE_PRAM"]
pub struct CSE_PRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSE_PRAM {}
impl CSE_PRAM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cse_pram::RegisterBlock {
        335548416 as *const _
    }
}
impl Deref for CSE_PRAM {
    type Target = cse_pram::RegisterBlock;
    fn deref(&self) -> &cse_pram::RegisterBlock {
        unsafe { &*CSE_PRAM::ptr() }
    }
}
#[doc = "CSE_PRAM"]
pub mod cse_pram;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS {}
impl AIPS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aips::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for AIPS {
    type Target = aips::RegisterBlock;
    fn deref(&self) -> &aips::RegisterBlock {
        unsafe { &*AIPS::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips;
#[doc = "MSCM"]
pub struct MSCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSCM {}
impl MSCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mscm::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for MSCM {
    type Target = mscm::RegisterBlock;
    fn deref(&self) -> &mscm::RegisterBlock {
        unsafe { &*MSCM::ptr() }
    }
}
#[doc = "MSCM"]
pub mod mscm;
#[doc = "Enhanced Direct Memory Access"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "Enhanced Direct Memory Access"]
pub mod dma;
#[doc = "ERM"]
pub struct ERM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ERM {}
impl ERM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const erm::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for ERM {
    type Target = erm::RegisterBlock;
    fn deref(&self) -> &erm::RegisterBlock {
        unsafe { &*ERM::ptr() }
    }
}
#[doc = "ERM"]
pub mod erm;
#[doc = "Error Injection Module"]
pub struct EIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EIM {}
impl EIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const eim::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for EIM {
    type Target = eim::RegisterBlock;
    fn deref(&self) -> &eim::RegisterBlock {
        unsafe { &*EIM::ptr() }
    }
}
#[doc = "Error Injection Module"]
pub mod eim;
#[doc = "FTFC"]
pub struct FTFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFC {}
impl FTFC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftfc::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for FTFC {
    type Target = ftfc::RegisterBlock;
    fn deref(&self) -> &ftfc::RegisterBlock {
        unsafe { &*FTFC::ptr() }
    }
}
#[doc = "FTFC"]
pub mod ftfc;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmamux::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    fn deref(&self) -> &dmamux::RegisterBlock {
        unsafe { &*DMAMUX::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux;
#[doc = "Flex Controller Area Network module"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can0::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Flex Controller Area Network module"]
pub mod can0;
#[doc = "Flex Controller Area Network module"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can0::RegisterBlock {
        1073893376 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Flex Controller Area Network module"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can0::RegisterBlock {
        1073917952 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub struct FTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM0 {}
impl FTM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm0::RegisterBlock {
        1073971200 as *const _
    }
}
impl Deref for FTM0 {
    type Target = ftm0::RegisterBlock;
    fn deref(&self) -> &ftm0::RegisterBlock {
        unsafe { &*FTM0::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm0;
#[doc = "FlexTimer Module"]
pub struct FTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM1 {}
impl FTM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm1::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for FTM1 {
    type Target = ftm1::RegisterBlock;
    fn deref(&self) -> &ftm1::RegisterBlock {
        unsafe { &*FTM1::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm1;
#[doc = "FlexTimer Module"]
pub struct FTM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM2 {}
impl FTM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm2::RegisterBlock {
        1073979392 as *const _
    }
}
impl Deref for FTM2 {
    type Target = ftm2::RegisterBlock;
    fn deref(&self) -> &ftm2::RegisterBlock {
        unsafe { &*FTM2::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm2;
#[doc = "FlexTimer Module"]
pub struct FTM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM3 {}
impl FTM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ftm3::RegisterBlock {
        1073897472 as *const _
    }
}
impl Deref for FTM3 {
    type Target = ftm3::RegisterBlock;
    fn deref(&self) -> &ftm3::RegisterBlock {
        unsafe { &*FTM3::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm3;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc0::RegisterBlock {
        1073983488 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073901568 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc1;
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub struct LPSPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI0 {}
impl LPSPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpspi0::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for LPSPI0 {
    type Target = lpspi0::RegisterBlock;
    fn deref(&self) -> &lpspi0::RegisterBlock {
        unsafe { &*LPSPI0::ptr() }
    }
}
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub mod lpspi0;
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub struct LPSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI1 {}
impl LPSPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpspi1::RegisterBlock {
        1073926144 as *const _
    }
}
impl Deref for LPSPI1 {
    type Target = lpspi1::RegisterBlock;
    fn deref(&self) -> &lpspi1::RegisterBlock {
        unsafe { &*LPSPI1::ptr() }
    }
}
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub mod lpspi1;
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub struct LPSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPSPI2 {}
impl LPSPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpspi2::RegisterBlock {
        1073930240 as *const _
    }
}
impl Deref for LPSPI2 {
    type Target = lpspi2::RegisterBlock;
    fn deref(&self) -> &lpspi2::RegisterBlock {
        unsafe { &*LPSPI2::ptr() }
    }
}
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub mod lpspi2;
#[doc = "Programmable Delay Block"]
pub struct PDB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDB0 {}
impl PDB0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdb0::RegisterBlock {
        1073963008 as *const _
    }
}
impl Deref for PDB0 {
    type Target = pdb0::RegisterBlock;
    fn deref(&self) -> &pdb0::RegisterBlock {
        unsafe { &*PDB0::ptr() }
    }
}
#[doc = "Programmable Delay Block"]
pub mod pdb0;
#[doc = "Programmable Delay Block"]
pub struct PDB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDB1 {}
impl PDB1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdb1::RegisterBlock {
        1073942528 as *const _
    }
}
impl Deref for PDB1 {
    type Target = pdb1::RegisterBlock;
    fn deref(&self) -> &pdb1::RegisterBlock {
        unsafe { &*PDB1::ptr() }
    }
}
#[doc = "Programmable Delay Block"]
pub mod pdb1;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073946624 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "Low Power Periodic Interrupt Timer (LPIT)"]
pub struct LPIT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPIT0 {}
impl LPIT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpit0::RegisterBlock {
        1073967104 as *const _
    }
}
impl Deref for LPIT0 {
    type Target = lpit0::RegisterBlock;
    fn deref(&self) -> &lpit0::RegisterBlock {
        unsafe { &*LPIT0::ptr() }
    }
}
#[doc = "Low Power Periodic Interrupt Timer (LPIT)"]
pub mod lpit0;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073991680 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lptmr0::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &lptmr0::RegisterBlock {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sim::RegisterBlock {
        1074036736 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &sim::RegisterBlock {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porta::RegisterBlock {
        1074040832 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &porta::RegisterBlock {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portb::RegisterBlock {
        1074044928 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portc::RegisterBlock {
        1074049024 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const portd::RegisterBlock {
        1074053120 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &portd::RegisterBlock {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const porte::RegisterBlock {
        1074057216 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &porte::RegisterBlock {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "Watchdog timer"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdog::RegisterBlock {
        1074077696 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &wdog::RegisterBlock {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Watchdog timer"]
pub mod wdog;
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub struct FLEXIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXIO {}
impl FLEXIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flexio::RegisterBlock {
        1074110464 as *const _
    }
}
impl Deref for FLEXIO {
    type Target = flexio::RegisterBlock;
    fn deref(&self) -> &flexio::RegisterBlock {
        unsafe { &*FLEXIO::ptr() }
    }
}
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub mod flexio;
#[doc = "External Watchdog Monitor"]
pub struct EWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ewm::RegisterBlock {
        1074139136 as *const _
    }
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    fn deref(&self) -> &ewm::RegisterBlock {
        unsafe { &*EWM::ptr() }
    }
}
#[doc = "External Watchdog Monitor"]
pub mod ewm;
#[doc = "TRGMUX"]
pub struct TRGMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRGMUX {}
impl TRGMUX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trgmux::RegisterBlock {
        1074147328 as *const _
    }
}
impl Deref for TRGMUX {
    type Target = trgmux::RegisterBlock;
    fn deref(&self) -> &trgmux::RegisterBlock {
        unsafe { &*TRGMUX::ptr() }
    }
}
#[doc = "TRGMUX"]
pub mod trgmux;
#[doc = "System Clock Generator"]
pub struct SCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCG {}
impl SCG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scg::RegisterBlock {
        1074151424 as *const _
    }
}
impl Deref for SCG {
    type Target = scg::RegisterBlock;
    fn deref(&self) -> &scg::RegisterBlock {
        unsafe { &*SCG::ptr() }
    }
}
#[doc = "System Clock Generator"]
pub mod scg;
#[doc = "PCC"]
pub struct PCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCC {}
impl PCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pcc::RegisterBlock {
        1074155520 as *const _
    }
}
impl Deref for PCC {
    type Target = pcc::RegisterBlock;
    fn deref(&self) -> &pcc::RegisterBlock {
        unsafe { &*PCC::ptr() }
    }
}
#[doc = "PCC"]
pub mod pcc;
#[doc = "The LPI2C Memory Map/Register Definition can be found here."]
pub struct LPI2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPI2C0 {}
impl LPI2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpi2c0::RegisterBlock {
        1074159616 as *const _
    }
}
impl Deref for LPI2C0 {
    type Target = lpi2c0::RegisterBlock;
    fn deref(&self) -> &lpi2c0::RegisterBlock {
        unsafe { &*LPI2C0::ptr() }
    }
}
#[doc = "The LPI2C Memory Map/Register Definition can be found here."]
pub mod lpi2c0;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART0 {}
impl LPUART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart0::RegisterBlock {
        1074176000 as *const _
    }
}
impl Deref for LPUART0 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        unsafe { &*LPUART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart0;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart0::RegisterBlock {
        1074180096 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART2 {}
impl LPUART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpuart0::RegisterBlock {
        1074184192 as *const _
    }
}
impl Deref for LPUART2 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        unsafe { &*LPUART2::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cmp0::RegisterBlock {
        1074212864 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &cmp0::RegisterBlock {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "PMC"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074253824 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "PMC"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smc::RegisterBlock {
        1074257920 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcm::RegisterBlock {
        1074262016 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &rcm::RegisterBlock {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "General Purpose Input/Output"]
pub struct PTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTA {}
impl PTA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pta::RegisterBlock {
        1074786304 as *const _
    }
}
impl Deref for PTA {
    type Target = pta::RegisterBlock;
    fn deref(&self) -> &pta::RegisterBlock {
        unsafe { &*PTA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod pta;
#[doc = "General Purpose Input/Output"]
pub struct PTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTB {}
impl PTB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptb::RegisterBlock {
        1074786368 as *const _
    }
}
impl Deref for PTB {
    type Target = ptb::RegisterBlock;
    fn deref(&self) -> &ptb::RegisterBlock {
        unsafe { &*PTB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod ptb;
#[doc = "General Purpose Input/Output"]
pub struct PTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTC {}
impl PTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptc::RegisterBlock {
        1074786432 as *const _
    }
}
impl Deref for PTC {
    type Target = ptc::RegisterBlock;
    fn deref(&self) -> &ptc::RegisterBlock {
        unsafe { &*PTC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod ptc;
#[doc = "General Purpose Input/Output"]
pub struct PTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTD {}
impl PTD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptd::RegisterBlock {
        1074786496 as *const _
    }
}
impl Deref for PTD {
    type Target = ptd::RegisterBlock;
    fn deref(&self) -> &ptd::RegisterBlock {
        unsafe { &*PTD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod ptd;
#[doc = "General Purpose Input/Output"]
pub struct PTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTE {}
impl PTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pte::RegisterBlock {
        1074786560 as *const _
    }
}
impl Deref for PTE {
    type Target = pte::RegisterBlock;
    fn deref(&self) -> &pte::RegisterBlock {
        unsafe { &*PTE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod pte;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mcm::RegisterBlock {
        3758620672 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "Local Memory Controller"]
pub struct LMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LMEM {}
impl LMEM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lmem::RegisterBlock {
        3758628864 as *const _
    }
}
impl Deref for LMEM {
    type Target = lmem::RegisterBlock;
    fn deref(&self) -> &lmem::RegisterBlock {
        unsafe { &*LMEM::ptr() }
    }
}
#[doc = "Local Memory Controller"]
pub mod lmem;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CSE_PRAM"]
    pub CSE_PRAM: CSE_PRAM,
    #[doc = "AIPS"]
    pub AIPS: AIPS,
    #[doc = "MSCM"]
    pub MSCM: MSCM,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "ERM"]
    pub ERM: ERM,
    #[doc = "EIM"]
    pub EIM: EIM,
    #[doc = "FTFC"]
    pub FTFC: FTFC,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "FTM0"]
    pub FTM0: FTM0,
    #[doc = "FTM1"]
    pub FTM1: FTM1,
    #[doc = "FTM2"]
    pub FTM2: FTM2,
    #[doc = "FTM3"]
    pub FTM3: FTM3,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "LPSPI0"]
    pub LPSPI0: LPSPI0,
    #[doc = "LPSPI1"]
    pub LPSPI1: LPSPI1,
    #[doc = "LPSPI2"]
    pub LPSPI2: LPSPI2,
    #[doc = "PDB0"]
    pub PDB0: PDB0,
    #[doc = "PDB1"]
    pub PDB1: PDB1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "LPIT0"]
    pub LPIT0: LPIT0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "FLEXIO"]
    pub FLEXIO: FLEXIO,
    #[doc = "EWM"]
    pub EWM: EWM,
    #[doc = "TRGMUX"]
    pub TRGMUX: TRGMUX,
    #[doc = "SCG"]
    pub SCG: SCG,
    #[doc = "PCC"]
    pub PCC: PCC,
    #[doc = "LPI2C0"]
    pub LPI2C0: LPI2C0,
    #[doc = "LPUART0"]
    pub LPUART0: LPUART0,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "LPUART2"]
    pub LPUART2: LPUART2,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "PTA"]
    pub PTA: PTA,
    #[doc = "PTB"]
    pub PTB: PTB,
    #[doc = "PTC"]
    pub PTC: PTC,
    #[doc = "PTD"]
    pub PTD: PTD,
    #[doc = "PTE"]
    pub PTE: PTE,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "LMEM"]
    pub LMEM: LMEM,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CSE_PRAM: CSE_PRAM { _marker: PhantomData },
            AIPS: AIPS { _marker: PhantomData },
            MSCM: MSCM { _marker: PhantomData },
            DMA: DMA { _marker: PhantomData },
            ERM: ERM { _marker: PhantomData },
            EIM: EIM { _marker: PhantomData },
            FTFC: FTFC { _marker: PhantomData },
            DMAMUX: DMAMUX { _marker: PhantomData },
            CAN0: CAN0 { _marker: PhantomData },
            CAN1: CAN1 { _marker: PhantomData },
            CAN2: CAN2 { _marker: PhantomData },
            FTM0: FTM0 { _marker: PhantomData },
            FTM1: FTM1 { _marker: PhantomData },
            FTM2: FTM2 { _marker: PhantomData },
            FTM3: FTM3 { _marker: PhantomData },
            ADC0: ADC0 { _marker: PhantomData },
            ADC1: ADC1 { _marker: PhantomData },
            LPSPI0: LPSPI0 { _marker: PhantomData },
            LPSPI1: LPSPI1 { _marker: PhantomData },
            LPSPI2: LPSPI2 { _marker: PhantomData },
            PDB0: PDB0 { _marker: PhantomData },
            PDB1: PDB1 { _marker: PhantomData },
            CRC: CRC { _marker: PhantomData },
            LPIT0: LPIT0 { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            LPTMR0: LPTMR0 { _marker: PhantomData },
            SIM: SIM { _marker: PhantomData },
            PORTA: PORTA { _marker: PhantomData },
            PORTB: PORTB { _marker: PhantomData },
            PORTC: PORTC { _marker: PhantomData },
            PORTD: PORTD { _marker: PhantomData },
            PORTE: PORTE { _marker: PhantomData },
            WDOG: WDOG { _marker: PhantomData },
            FLEXIO: FLEXIO { _marker: PhantomData },
            EWM: EWM { _marker: PhantomData },
            TRGMUX: TRGMUX { _marker: PhantomData },
            SCG: SCG { _marker: PhantomData },
            PCC: PCC { _marker: PhantomData },
            LPI2C0: LPI2C0 { _marker: PhantomData },
            LPUART0: LPUART0 { _marker: PhantomData },
            LPUART1: LPUART1 { _marker: PhantomData },
            LPUART2: LPUART2 { _marker: PhantomData },
            CMP0: CMP0 { _marker: PhantomData },
            PMC: PMC { _marker: PhantomData },
            SMC: SMC { _marker: PhantomData },
            RCM: RCM { _marker: PhantomData },
            PTA: PTA { _marker: PhantomData },
            PTB: PTB { _marker: PhantomData },
            PTC: PTC { _marker: PhantomData },
            PTD: PTD { _marker: PhantomData },
            PTE: PTE { _marker: PhantomData },
            MCM: MCM { _marker: PhantomData },
            LMEM: LMEM { _marker: PhantomData },
        }
    }
}
