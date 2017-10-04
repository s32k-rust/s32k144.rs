#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(macro_reexport))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for S32K144 microcontrollers (generated using svd2rust v0.11.4)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.11.4/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[macro_reexport(default_handler, exception)]
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::ops::Deref;
use bare_metal::Peripheral;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::FPB;
pub use cortex_m::peripheral::FPU;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
pub use cortex_m::peripheral::TPIU;
#[doc = "CSE_PRAM"]
pub const CSE_PRAM: Peripheral<CSE_PRAM> = unsafe { Peripheral::new(335548416) };
#[doc = "CSE_PRAM"]
pub mod cse_pram;
#[doc = "CSE_PRAM"]
pub struct CSE_PRAM {
    register_block: cse_pram::RegisterBlock,
}
impl Deref for CSE_PRAM {
    type Target = cse_pram::RegisterBlock;
    fn deref(&self) -> &cse_pram::RegisterBlock {
        &self.register_block
    }
}
#[doc = "AIPS-Lite Bridge"]
pub const AIPS: Peripheral<AIPS> = unsafe { Peripheral::new(1073741824) };
#[doc = "AIPS-Lite Bridge"]
pub mod aips;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS {
    register_block: aips::RegisterBlock,
}
impl Deref for AIPS {
    type Target = aips::RegisterBlock;
    fn deref(&self) -> &aips::RegisterBlock {
        &self.register_block
    }
}
#[doc = "MSCM"]
pub const MSCM: Peripheral<MSCM> = unsafe { Peripheral::new(1073745920) };
#[doc = "MSCM"]
pub mod mscm;
#[doc = "MSCM"]
pub struct MSCM {
    register_block: mscm::RegisterBlock,
}
impl Deref for MSCM {
    type Target = mscm::RegisterBlock;
    fn deref(&self) -> &mscm::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Enhanced Direct Memory Access"]
pub const DMA: Peripheral<DMA> = unsafe { Peripheral::new(1073774592) };
#[doc = "Enhanced Direct Memory Access"]
pub mod dma;
#[doc = "Enhanced Direct Memory Access"]
pub struct DMA {
    register_block: dma::RegisterBlock,
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &dma::RegisterBlock {
        &self.register_block
    }
}
#[doc = "ERM"]
pub const ERM: Peripheral<ERM> = unsafe { Peripheral::new(1073840128) };
#[doc = "ERM"]
pub mod erm;
#[doc = "ERM"]
pub struct ERM {
    register_block: erm::RegisterBlock,
}
impl Deref for ERM {
    type Target = erm::RegisterBlock;
    fn deref(&self) -> &erm::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Error Injection Module"]
pub const EIM: Peripheral<EIM> = unsafe { Peripheral::new(1073844224) };
#[doc = "Error Injection Module"]
pub mod eim;
#[doc = "Error Injection Module"]
pub struct EIM {
    register_block: eim::RegisterBlock,
}
impl Deref for EIM {
    type Target = eim::RegisterBlock;
    fn deref(&self) -> &eim::RegisterBlock {
        &self.register_block
    }
}
#[doc = "FTFC"]
pub const FTFC: Peripheral<FTFC> = unsafe { Peripheral::new(1073872896) };
#[doc = "FTFC"]
pub mod ftfc;
#[doc = "FTFC"]
pub struct FTFC {
    register_block: ftfc::RegisterBlock,
}
impl Deref for FTFC {
    type Target = ftfc::RegisterBlock;
    fn deref(&self) -> &ftfc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "DMA channel multiplexor"]
pub const DMAMUX: Peripheral<DMAMUX> = unsafe { Peripheral::new(1073876992) };
#[doc = "DMA channel multiplexor"]
pub mod dmamux;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX {
    register_block: dmamux::RegisterBlock,
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    fn deref(&self) -> &dmamux::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flex Controller Area Network module"]
pub const CAN0: Peripheral<CAN0> = unsafe { Peripheral::new(1073889280) };
#[doc = "Flex Controller Area Network module"]
pub mod can0;
#[doc = "Flex Controller Area Network module"]
pub struct CAN0 {
    register_block: can0::RegisterBlock,
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flex Controller Area Network module"]
pub const CAN1: Peripheral<CAN1> = unsafe { Peripheral::new(1073893376) };
#[doc = r" Register block"]
pub struct CAN1 {
    register_block: can0::RegisterBlock,
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flex Controller Area Network module"]
pub const CAN2: Peripheral<CAN2> = unsafe { Peripheral::new(1073917952) };
#[doc = r" Register block"]
pub struct CAN2 {
    register_block: can0::RegisterBlock,
}
impl Deref for CAN2 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "FlexTimer Module"]
pub const FTM0: Peripheral<FTM0> = unsafe { Peripheral::new(1073971200) };
#[doc = "FlexTimer Module"]
pub mod ftm0;
#[doc = "FlexTimer Module"]
pub struct FTM0 {
    register_block: ftm0::RegisterBlock,
}
impl Deref for FTM0 {
    type Target = ftm0::RegisterBlock;
    fn deref(&self) -> &ftm0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "FlexTimer Module"]
pub const FTM1: Peripheral<FTM1> = unsafe { Peripheral::new(1073975296) };
#[doc = "FlexTimer Module"]
pub mod ftm1;
#[doc = "FlexTimer Module"]
pub struct FTM1 {
    register_block: ftm1::RegisterBlock,
}
impl Deref for FTM1 {
    type Target = ftm1::RegisterBlock;
    fn deref(&self) -> &ftm1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "FlexTimer Module"]
pub const FTM2: Peripheral<FTM2> = unsafe { Peripheral::new(1073979392) };
#[doc = "FlexTimer Module"]
pub mod ftm2;
#[doc = "FlexTimer Module"]
pub struct FTM2 {
    register_block: ftm2::RegisterBlock,
}
impl Deref for FTM2 {
    type Target = ftm2::RegisterBlock;
    fn deref(&self) -> &ftm2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "FlexTimer Module"]
pub const FTM3: Peripheral<FTM3> = unsafe { Peripheral::new(1073897472) };
#[doc = "FlexTimer Module"]
pub mod ftm3;
#[doc = "FlexTimer Module"]
pub struct FTM3 {
    register_block: ftm3::RegisterBlock,
}
impl Deref for FTM3 {
    type Target = ftm3::RegisterBlock;
    fn deref(&self) -> &ftm3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog-to-Digital Converter"]
pub const ADC0: Peripheral<ADC0> = unsafe { Peripheral::new(1073983488) };
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    register_block: adc0::RegisterBlock,
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &adc0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog-to-Digital Converter"]
pub const ADC1: Peripheral<ADC1> = unsafe { Peripheral::new(1073901568) };
#[doc = "Analog-to-Digital Converter"]
pub mod adc1;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1 {
    register_block: adc1::RegisterBlock,
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub const LPSPI0: Peripheral<LPSPI0> = unsafe { Peripheral::new(1073922048) };
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub mod lpspi0;
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub struct LPSPI0 {
    register_block: lpspi0::RegisterBlock,
}
impl Deref for LPSPI0 {
    type Target = lpspi0::RegisterBlock;
    fn deref(&self) -> &lpspi0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub const LPSPI1: Peripheral<LPSPI1> = unsafe { Peripheral::new(1073926144) };
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub mod lpspi1;
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub struct LPSPI1 {
    register_block: lpspi1::RegisterBlock,
}
impl Deref for LPSPI1 {
    type Target = lpspi1::RegisterBlock;
    fn deref(&self) -> &lpspi1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub const LPSPI2: Peripheral<LPSPI2> = unsafe { Peripheral::new(1073930240) };
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub mod lpspi2;
#[doc = "The LPSPI Memory Map/Register Definition can be found here."]
pub struct LPSPI2 {
    register_block: lpspi2::RegisterBlock,
}
impl Deref for LPSPI2 {
    type Target = lpspi2::RegisterBlock;
    fn deref(&self) -> &lpspi2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Programmable Delay Block"]
pub const PDB0: Peripheral<PDB0> = unsafe { Peripheral::new(1073963008) };
#[doc = "Programmable Delay Block"]
pub mod pdb0;
#[doc = "Programmable Delay Block"]
pub struct PDB0 {
    register_block: pdb0::RegisterBlock,
}
impl Deref for PDB0 {
    type Target = pdb0::RegisterBlock;
    fn deref(&self) -> &pdb0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Programmable Delay Block"]
pub const PDB1: Peripheral<PDB1> = unsafe { Peripheral::new(1073942528) };
#[doc = "Programmable Delay Block"]
pub mod pdb1;
#[doc = "Programmable Delay Block"]
pub struct PDB1 {
    register_block: pdb1::RegisterBlock,
}
impl Deref for PDB1 {
    type Target = pdb1::RegisterBlock;
    fn deref(&self) -> &pdb1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Cyclic Redundancy Check"]
pub const CRC: Peripheral<CRC> = unsafe { Peripheral::new(1073946624) };
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    register_block: crc::RegisterBlock,
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Low Power Periodic Interrupt Timer (LPIT)"]
pub const LPIT0: Peripheral<LPIT0> = unsafe { Peripheral::new(1073967104) };
#[doc = "Low Power Periodic Interrupt Timer (LPIT)"]
pub mod lpit0;
#[doc = "Low Power Periodic Interrupt Timer (LPIT)"]
pub struct LPIT0 {
    register_block: lpit0::RegisterBlock,
}
impl Deref for LPIT0 {
    type Target = lpit0::RegisterBlock;
    fn deref(&self) -> &lpit0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Secure Real Time Clock"]
pub const RTC: Peripheral<RTC> = unsafe { Peripheral::new(1073991680) };
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    register_block: rtc::RegisterBlock,
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Low Power Timer"]
pub const LPTMR0: Peripheral<LPTMR0> = unsafe { Peripheral::new(1074003968) };
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    register_block: lptmr0::RegisterBlock,
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &lptmr0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Integration Module"]
pub const SIM: Peripheral<SIM> = unsafe { Peripheral::new(1074036736) };
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "System Integration Module"]
pub struct SIM {
    register_block: sim::RegisterBlock,
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &sim::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Pin Control and Interrupts"]
pub const PORTA: Peripheral<PORTA> = unsafe { Peripheral::new(1074040832) };
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    register_block: porta::RegisterBlock,
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &porta::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Pin Control and Interrupts"]
pub const PORTB: Peripheral<PORTB> = unsafe { Peripheral::new(1074044928) };
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    register_block: portb::RegisterBlock,
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &portb::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Pin Control and Interrupts"]
pub const PORTC: Peripheral<PORTC> = unsafe { Peripheral::new(1074049024) };
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    register_block: portc::RegisterBlock,
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &portc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Pin Control and Interrupts"]
pub const PORTD: Peripheral<PORTD> = unsafe { Peripheral::new(1074053120) };
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    register_block: portd::RegisterBlock,
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &portd::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Pin Control and Interrupts"]
pub const PORTE: Peripheral<PORTE> = unsafe { Peripheral::new(1074057216) };
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    register_block: porte::RegisterBlock,
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &porte::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Watchdog timer"]
pub const WDOG: Peripheral<WDOG> = unsafe { Peripheral::new(1074077696) };
#[doc = "Watchdog timer"]
pub mod wdog;
#[doc = "Watchdog timer"]
pub struct WDOG {
    register_block: wdog::RegisterBlock,
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &wdog::RegisterBlock {
        &self.register_block
    }
}
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub const FLEXIO: Peripheral<FLEXIO> = unsafe { Peripheral::new(1074110464) };
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub mod flexio;
#[doc = "The FLEXIO Memory Map/Register Definition can be found here."]
pub struct FLEXIO {
    register_block: flexio::RegisterBlock,
}
impl Deref for FLEXIO {
    type Target = flexio::RegisterBlock;
    fn deref(&self) -> &flexio::RegisterBlock {
        &self.register_block
    }
}
#[doc = "External Watchdog Monitor"]
pub const EWM: Peripheral<EWM> = unsafe { Peripheral::new(1074139136) };
#[doc = "External Watchdog Monitor"]
pub mod ewm;
#[doc = "External Watchdog Monitor"]
pub struct EWM {
    register_block: ewm::RegisterBlock,
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    fn deref(&self) -> &ewm::RegisterBlock {
        &self.register_block
    }
}
#[doc = "TRGMUX"]
pub const TRGMUX: Peripheral<TRGMUX> = unsafe { Peripheral::new(1074147328) };
#[doc = "TRGMUX"]
pub mod trgmux;
#[doc = "TRGMUX"]
pub struct TRGMUX {
    register_block: trgmux::RegisterBlock,
}
impl Deref for TRGMUX {
    type Target = trgmux::RegisterBlock;
    fn deref(&self) -> &trgmux::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Clock Generator"]
pub const SCG: Peripheral<SCG> = unsafe { Peripheral::new(1074151424) };
#[doc = "System Clock Generator"]
pub mod scg;
#[doc = "System Clock Generator"]
pub struct SCG {
    register_block: scg::RegisterBlock,
}
impl Deref for SCG {
    type Target = scg::RegisterBlock;
    fn deref(&self) -> &scg::RegisterBlock {
        &self.register_block
    }
}
#[doc = "PCC"]
pub const PCC: Peripheral<PCC> = unsafe { Peripheral::new(1074155520) };
#[doc = "PCC"]
pub mod pcc;
#[doc = "PCC"]
pub struct PCC {
    register_block: pcc::RegisterBlock,
}
impl Deref for PCC {
    type Target = pcc::RegisterBlock;
    fn deref(&self) -> &pcc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "The LPI2C Memory Map/Register Definition can be found here."]
pub const LPI2C0: Peripheral<LPI2C0> = unsafe { Peripheral::new(1074159616) };
#[doc = "The LPI2C Memory Map/Register Definition can be found here."]
pub mod lpi2c0;
#[doc = "The LPI2C Memory Map/Register Definition can be found here."]
pub struct LPI2C0 {
    register_block: lpi2c0::RegisterBlock,
}
impl Deref for LPI2C0 {
    type Target = lpi2c0::RegisterBlock;
    fn deref(&self) -> &lpi2c0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const LPUART0: Peripheral<LPUART0> = unsafe { Peripheral::new(1074176000) };
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart0;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART0 {
    register_block: lpuart0::RegisterBlock,
}
impl Deref for LPUART0 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const LPUART1: Peripheral<LPUART1> = unsafe { Peripheral::new(1074180096) };
#[doc = r" Register block"]
pub struct LPUART1 {
    register_block: lpuart0::RegisterBlock,
}
impl Deref for LPUART1 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const LPUART2: Peripheral<LPUART2> = unsafe { Peripheral::new(1074184192) };
#[doc = r" Register block"]
pub struct LPUART2 {
    register_block: lpuart0::RegisterBlock,
}
impl Deref for LPUART2 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &lpuart0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub const CMP0: Peripheral<CMP0> = unsafe { Peripheral::new(1074212864) };
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    register_block: cmp0::RegisterBlock,
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &cmp0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "PMC"]
pub const PMC: Peripheral<PMC> = unsafe { Peripheral::new(1074253824) };
#[doc = "PMC"]
pub mod pmc;
#[doc = "PMC"]
pub struct PMC {
    register_block: pmc::RegisterBlock,
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Mode Controller"]
pub const SMC: Peripheral<SMC> = unsafe { Peripheral::new(1074257920) };
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "System Mode Controller"]
pub struct SMC {
    register_block: smc::RegisterBlock,
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Reset Control Module"]
pub const RCM: Peripheral<RCM> = unsafe { Peripheral::new(1074262016) };
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "Reset Control Module"]
pub struct RCM {
    register_block: rcm::RegisterBlock,
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &rcm::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose Input/Output"]
pub const PTA: Peripheral<PTA> = unsafe { Peripheral::new(1074786304) };
#[doc = "General Purpose Input/Output"]
pub mod pta;
#[doc = "General Purpose Input/Output"]
pub struct PTA {
    register_block: pta::RegisterBlock,
}
impl Deref for PTA {
    type Target = pta::RegisterBlock;
    fn deref(&self) -> &pta::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose Input/Output"]
pub const PTB: Peripheral<PTB> = unsafe { Peripheral::new(1074786368) };
#[doc = "General Purpose Input/Output"]
pub mod ptb;
#[doc = "General Purpose Input/Output"]
pub struct PTB {
    register_block: ptb::RegisterBlock,
}
impl Deref for PTB {
    type Target = ptb::RegisterBlock;
    fn deref(&self) -> &ptb::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose Input/Output"]
pub const PTC: Peripheral<PTC> = unsafe { Peripheral::new(1074786432) };
#[doc = "General Purpose Input/Output"]
pub mod ptc;
#[doc = "General Purpose Input/Output"]
pub struct PTC {
    register_block: ptc::RegisterBlock,
}
impl Deref for PTC {
    type Target = ptc::RegisterBlock;
    fn deref(&self) -> &ptc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose Input/Output"]
pub const PTD: Peripheral<PTD> = unsafe { Peripheral::new(1074786496) };
#[doc = "General Purpose Input/Output"]
pub mod ptd;
#[doc = "General Purpose Input/Output"]
pub struct PTD {
    register_block: ptd::RegisterBlock,
}
impl Deref for PTD {
    type Target = ptd::RegisterBlock;
    fn deref(&self) -> &ptd::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose Input/Output"]
pub const PTE: Peripheral<PTE> = unsafe { Peripheral::new(1074786560) };
#[doc = "General Purpose Input/Output"]
pub mod pte;
#[doc = "General Purpose Input/Output"]
pub struct PTE {
    register_block: pte::RegisterBlock,
}
impl Deref for PTE {
    type Target = pte::RegisterBlock;
    fn deref(&self) -> &pte::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub const MCM: Peripheral<MCM> = unsafe { Peripheral::new(3758620672) };
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    register_block: mcm::RegisterBlock,
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &mcm::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Local Memory Controller"]
pub const LMEM: Peripheral<LMEM> = unsafe { Peripheral::new(3758628864) };
#[doc = "Local Memory Controller"]
pub mod lmem;
#[doc = "Local Memory Controller"]
pub struct LMEM {
    register_block: lmem::RegisterBlock,
}
impl Deref for LMEM {
    type Target = lmem::RegisterBlock;
    fn deref(&self) -> &lmem::RegisterBlock {
        &self.register_block
    }
}
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals<'a> {
    #[doc = "CPUID"] pub CPUID: &'a CPUID,
    #[doc = "DCB"] pub DCB: &'a DCB,
    #[doc = "DWT"] pub DWT: &'a DWT,
    #[doc = "FPB"] pub FPB: &'a FPB,
    #[doc = "FPU"] pub FPU: &'a FPU,
    #[doc = "ITM"] pub ITM: &'a ITM,
    #[doc = "MPU"] pub MPU: &'a MPU,
    #[doc = "NVIC"] pub NVIC: &'a NVIC,
    #[doc = "SCB"] pub SCB: &'a SCB,
    #[doc = "SYST"] pub SYST: &'a SYST,
    #[doc = "TPIU"] pub TPIU: &'a TPIU,
    #[doc = "CSE_PRAM"] pub CSE_PRAM: &'a CSE_PRAM,
    #[doc = "AIPS"] pub AIPS: &'a AIPS,
    #[doc = "MSCM"] pub MSCM: &'a MSCM,
    #[doc = "DMA"] pub DMA: &'a DMA,
    #[doc = "ERM"] pub ERM: &'a ERM,
    #[doc = "EIM"] pub EIM: &'a EIM,
    #[doc = "FTFC"] pub FTFC: &'a FTFC,
    #[doc = "DMAMUX"] pub DMAMUX: &'a DMAMUX,
    #[doc = "CAN0"] pub CAN0: &'a CAN0,
    #[doc = "CAN1"] pub CAN1: &'a CAN1,
    #[doc = "CAN2"] pub CAN2: &'a CAN2,
    #[doc = "FTM0"] pub FTM0: &'a FTM0,
    #[doc = "FTM1"] pub FTM1: &'a FTM1,
    #[doc = "FTM2"] pub FTM2: &'a FTM2,
    #[doc = "FTM3"] pub FTM3: &'a FTM3,
    #[doc = "ADC0"] pub ADC0: &'a ADC0,
    #[doc = "ADC1"] pub ADC1: &'a ADC1,
    #[doc = "LPSPI0"] pub LPSPI0: &'a LPSPI0,
    #[doc = "LPSPI1"] pub LPSPI1: &'a LPSPI1,
    #[doc = "LPSPI2"] pub LPSPI2: &'a LPSPI2,
    #[doc = "PDB0"] pub PDB0: &'a PDB0,
    #[doc = "PDB1"] pub PDB1: &'a PDB1,
    #[doc = "CRC"] pub CRC: &'a CRC,
    #[doc = "LPIT0"] pub LPIT0: &'a LPIT0,
    #[doc = "RTC"] pub RTC: &'a RTC,
    #[doc = "LPTMR0"] pub LPTMR0: &'a LPTMR0,
    #[doc = "SIM"] pub SIM: &'a SIM,
    #[doc = "PORTA"] pub PORTA: &'a PORTA,
    #[doc = "PORTB"] pub PORTB: &'a PORTB,
    #[doc = "PORTC"] pub PORTC: &'a PORTC,
    #[doc = "PORTD"] pub PORTD: &'a PORTD,
    #[doc = "PORTE"] pub PORTE: &'a PORTE,
    #[doc = "WDOG"] pub WDOG: &'a WDOG,
    #[doc = "FLEXIO"] pub FLEXIO: &'a FLEXIO,
    #[doc = "EWM"] pub EWM: &'a EWM,
    #[doc = "TRGMUX"] pub TRGMUX: &'a TRGMUX,
    #[doc = "SCG"] pub SCG: &'a SCG,
    #[doc = "PCC"] pub PCC: &'a PCC,
    #[doc = "LPI2C0"] pub LPI2C0: &'a LPI2C0,
    #[doc = "LPUART0"] pub LPUART0: &'a LPUART0,
    #[doc = "LPUART1"] pub LPUART1: &'a LPUART1,
    #[doc = "LPUART2"] pub LPUART2: &'a LPUART2,
    #[doc = "CMP0"] pub CMP0: &'a CMP0,
    #[doc = "PMC"] pub PMC: &'a PMC,
    #[doc = "SMC"] pub SMC: &'a SMC,
    #[doc = "RCM"] pub RCM: &'a RCM,
    #[doc = "PTA"] pub PTA: &'a PTA,
    #[doc = "PTB"] pub PTB: &'a PTB,
    #[doc = "PTC"] pub PTC: &'a PTC,
    #[doc = "PTD"] pub PTD: &'a PTD,
    #[doc = "PTE"] pub PTE: &'a PTE,
    #[doc = "MCM"] pub MCM: &'a MCM,
    #[doc = "LMEM"] pub LMEM: &'a LMEM,
}
impl<'a> Peripherals<'a> {
    #[doc = r" Grants access to all the peripherals"]
    pub unsafe fn all() -> Self {
        Peripherals {
            CPUID: &*CPUID.get(),
            DCB: &*DCB.get(),
            DWT: &*DWT.get(),
            FPB: &*FPB.get(),
            FPU: &*FPU.get(),
            ITM: &*ITM.get(),
            MPU: &*MPU.get(),
            NVIC: &*NVIC.get(),
            SCB: &*SCB.get(),
            SYST: &*SYST.get(),
            TPIU: &*TPIU.get(),
            CSE_PRAM: &*CSE_PRAM.get(),
            AIPS: &*AIPS.get(),
            MSCM: &*MSCM.get(),
            DMA: &*DMA.get(),
            ERM: &*ERM.get(),
            EIM: &*EIM.get(),
            FTFC: &*FTFC.get(),
            DMAMUX: &*DMAMUX.get(),
            CAN0: &*CAN0.get(),
            CAN1: &*CAN1.get(),
            CAN2: &*CAN2.get(),
            FTM0: &*FTM0.get(),
            FTM1: &*FTM1.get(),
            FTM2: &*FTM2.get(),
            FTM3: &*FTM3.get(),
            ADC0: &*ADC0.get(),
            ADC1: &*ADC1.get(),
            LPSPI0: &*LPSPI0.get(),
            LPSPI1: &*LPSPI1.get(),
            LPSPI2: &*LPSPI2.get(),
            PDB0: &*PDB0.get(),
            PDB1: &*PDB1.get(),
            CRC: &*CRC.get(),
            LPIT0: &*LPIT0.get(),
            RTC: &*RTC.get(),
            LPTMR0: &*LPTMR0.get(),
            SIM: &*SIM.get(),
            PORTA: &*PORTA.get(),
            PORTB: &*PORTB.get(),
            PORTC: &*PORTC.get(),
            PORTD: &*PORTD.get(),
            PORTE: &*PORTE.get(),
            WDOG: &*WDOG.get(),
            FLEXIO: &*FLEXIO.get(),
            EWM: &*EWM.get(),
            TRGMUX: &*TRGMUX.get(),
            SCG: &*SCG.get(),
            PCC: &*PCC.get(),
            LPI2C0: &*LPI2C0.get(),
            LPUART0: &*LPUART0.get(),
            LPUART1: &*LPUART1.get(),
            LPUART2: &*LPUART2.get(),
            CMP0: &*CMP0.get(),
            PMC: &*PMC.get(),
            SMC: &*SMC.get(),
            RCM: &*RCM.get(),
            PTA: &*PTA.get(),
            PTB: &*PTB.get(),
            PTC: &*PTC.get(),
            PTD: &*PTD.get(),
            PTE: &*PTE.get(),
            MCM: &*MCM.get(),
            LMEM: &*LMEM.get(),
        }
    }
}
