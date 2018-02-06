#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(macro_reexport))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for S32K144 microcontrollers (generated using svd2rust v0.12.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.12.0/svd2rust/#peripheral-api"]
#![allow(private_no_mangle_statics)]
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
use core::marker::PhantomData;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
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
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CSE_PRAM: CSE_PRAM {
                _marker: PhantomData,
            },
            AIPS: AIPS {
                _marker: PhantomData,
            },
            MSCM: MSCM {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            ERM: ERM {
                _marker: PhantomData,
            },
            EIM: EIM {
                _marker: PhantomData,
            },
            FTFC: FTFC {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            FTM0: FTM0 {
                _marker: PhantomData,
            },
            FTM1: FTM1 {
                _marker: PhantomData,
            },
            FTM2: FTM2 {
                _marker: PhantomData,
            },
            FTM3: FTM3 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            LPSPI0: LPSPI0 {
                _marker: PhantomData,
            },
            LPSPI1: LPSPI1 {
                _marker: PhantomData,
            },
            LPSPI2: LPSPI2 {
                _marker: PhantomData,
            },
            PDB0: PDB0 {
                _marker: PhantomData,
            },
            PDB1: PDB1 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            LPIT0: LPIT0 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            PORTD: PORTD {
                _marker: PhantomData,
            },
            PORTE: PORTE {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            FLEXIO: FLEXIO {
                _marker: PhantomData,
            },
            EWM: EWM {
                _marker: PhantomData,
            },
            TRGMUX: TRGMUX {
                _marker: PhantomData,
            },
            SCG: SCG {
                _marker: PhantomData,
            },
            PCC: PCC {
                _marker: PhantomData,
            },
            LPI2C0: LPI2C0 {
                _marker: PhantomData,
            },
            LPUART0: LPUART0 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            LPUART2: LPUART2 {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            PTA: PTA {
                _marker: PhantomData,
            },
            PTB: PTB {
                _marker: PhantomData,
            },
            PTC: PTC {
                _marker: PhantomData,
            },
            PTD: PTD {
                _marker: PhantomData,
            },
            PTE: PTE {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            LMEM: LMEM {
                _marker: PhantomData,
            },
        }
    }
}
