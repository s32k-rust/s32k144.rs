use bare_metal::Nr;
#[cfg(all(target_arch = "arm", feature = "rt"))]
global_asm!("\n                    .thumb_func\n                    DH_TRAMPOLINE:\n                        b DEFAULT_HANDLER\n                    ");
#[doc = r" Hack to compile on x86"]
#[cfg(all(target_arch = "x86_64", feature = "rt"))]
global_asm!("\n                    DH_TRAMPOLINE:\n                        jmp DEFAULT_HANDLER\n                    ");
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak DMA0\nDMA0 = DH_TRAMPOLINE\n.weak DMA1\nDMA1 = DH_TRAMPOLINE\n.weak DMA2\nDMA2 = DH_TRAMPOLINE\n.weak DMA3\nDMA3 = DH_TRAMPOLINE\n.weak DMA4\nDMA4 = DH_TRAMPOLINE\n.weak DMA5\nDMA5 = DH_TRAMPOLINE\n.weak DMA6\nDMA6 = DH_TRAMPOLINE\n.weak DMA7\nDMA7 = DH_TRAMPOLINE\n.weak DMA8\nDMA8 = DH_TRAMPOLINE\n.weak DMA9\nDMA9 = DH_TRAMPOLINE\n.weak DMA10\nDMA10 = DH_TRAMPOLINE\n.weak DMA11\nDMA11 = DH_TRAMPOLINE\n.weak DMA12\nDMA12 = DH_TRAMPOLINE\n.weak DMA13\nDMA13 = DH_TRAMPOLINE\n.weak DMA14\nDMA14 = DH_TRAMPOLINE\n.weak DMA15\nDMA15 = DH_TRAMPOLINE\n.weak DMA_ERROR\nDMA_ERROR = DH_TRAMPOLINE\n.weak MCM\nMCM = DH_TRAMPOLINE\n.weak FTFC\nFTFC = DH_TRAMPOLINE\n.weak READ_COLLISION\nREAD_COLLISION = DH_TRAMPOLINE\n.weak LVD_LVW\nLVD_LVW = DH_TRAMPOLINE\n.weak FTFC_FAULT\nFTFC_FAULT = DH_TRAMPOLINE\n.weak WDOG_EWM\nWDOG_EWM = DH_TRAMPOLINE\n.weak RCM\nRCM = DH_TRAMPOLINE\n.weak LPI2C0_MASTER\nLPI2C0_MASTER = DH_TRAMPOLINE\n.weak LPI2C0_SLAVE\nLPI2C0_SLAVE = DH_TRAMPOLINE\n.weak LPSPI0\nLPSPI0 = DH_TRAMPOLINE\n.weak LPSPI1\nLPSPI1 = DH_TRAMPOLINE\n.weak LPSPI2\nLPSPI2 = DH_TRAMPOLINE\n.weak LPUART0_RXTX\nLPUART0_RXTX = DH_TRAMPOLINE\n.weak LPUART1_RXTX\nLPUART1_RXTX = DH_TRAMPOLINE\n.weak LPUART2_RXTX\nLPUART2_RXTX = DH_TRAMPOLINE\n.weak ADC0\nADC0 = DH_TRAMPOLINE\n.weak ADC1\nADC1 = DH_TRAMPOLINE\n.weak CMP0\nCMP0 = DH_TRAMPOLINE\n.weak ERM_SINGLE_FAULT\nERM_SINGLE_FAULT = DH_TRAMPOLINE\n.weak ERM_DOUBLE_FAULT\nERM_DOUBLE_FAULT = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak RTC_SECONDS\nRTC_SECONDS = DH_TRAMPOLINE\n.weak LPIT0_CH0\nLPIT0_CH0 = DH_TRAMPOLINE\n.weak LPIT0_CH1\nLPIT0_CH1 = DH_TRAMPOLINE\n.weak LPIT0_CH2\nLPIT0_CH2 = DH_TRAMPOLINE\n.weak LPIT0_CH3\nLPIT0_CH3 = DH_TRAMPOLINE\n.weak PDB0\nPDB0 = DH_TRAMPOLINE\n.weak SCG\nSCG = DH_TRAMPOLINE\n.weak LPTMR0\nLPTMR0 = DH_TRAMPOLINE\n.weak PORTA\nPORTA = DH_TRAMPOLINE\n.weak PORTB\nPORTB = DH_TRAMPOLINE\n.weak PORTC\nPORTC = DH_TRAMPOLINE\n.weak PORTD\nPORTD = DH_TRAMPOLINE\n.weak PORTE\nPORTE = DH_TRAMPOLINE\n.weak PDB1\nPDB1 = DH_TRAMPOLINE\n.weak FLEXIO\nFLEXIO = DH_TRAMPOLINE\n.weak CAN0_ORED\nCAN0_ORED = DH_TRAMPOLINE\n.weak CAN0_ERROR\nCAN0_ERROR = DH_TRAMPOLINE\n.weak CAN0_WAKE_UP\nCAN0_WAKE_UP = DH_TRAMPOLINE\n.weak CAN0_ORED_0_15_MB\nCAN0_ORED_0_15_MB = DH_TRAMPOLINE\n.weak CAN0_ORED_16_31_MB\nCAN0_ORED_16_31_MB = DH_TRAMPOLINE\n.weak CAN1_ORED\nCAN1_ORED = DH_TRAMPOLINE\n.weak CAN1_ERROR\nCAN1_ERROR = DH_TRAMPOLINE\n.weak CAN1_ORED_0_15_MB\nCAN1_ORED_0_15_MB = DH_TRAMPOLINE\n.weak CAN2_ORED\nCAN2_ORED = DH_TRAMPOLINE\n.weak CAN2_ERROR\nCAN2_ERROR = DH_TRAMPOLINE\n.weak CAN2_ORED_0_15_MB\nCAN2_ORED_0_15_MB = DH_TRAMPOLINE\n.weak FTM0_CH0_CH1\nFTM0_CH0_CH1 = DH_TRAMPOLINE\n.weak FTM0_CH2_CH3\nFTM0_CH2_CH3 = DH_TRAMPOLINE\n.weak FTM0_CH4_CH5\nFTM0_CH4_CH5 = DH_TRAMPOLINE\n.weak FTM0_CH6_CH7\nFTM0_CH6_CH7 = DH_TRAMPOLINE\n.weak FTM0_FAULT\nFTM0_FAULT = DH_TRAMPOLINE\n.weak FTM0_OVF_RELOAD\nFTM0_OVF_RELOAD = DH_TRAMPOLINE\n.weak FTM1_CH0_CH1\nFTM1_CH0_CH1 = DH_TRAMPOLINE\n.weak FTM1_CH2_CH3\nFTM1_CH2_CH3 = DH_TRAMPOLINE\n.weak FTM1_CH4_CH5\nFTM1_CH4_CH5 = DH_TRAMPOLINE\n.weak FTM1_CH6_CH7\nFTM1_CH6_CH7 = DH_TRAMPOLINE\n.weak FTM1_FAULT\nFTM1_FAULT = DH_TRAMPOLINE\n.weak FTM1_OVF_RELOAD\nFTM1_OVF_RELOAD = DH_TRAMPOLINE\n.weak FTM2_CH0_CH1\nFTM2_CH0_CH1 = DH_TRAMPOLINE\n.weak FTM2_CH2_CH3\nFTM2_CH2_CH3 = DH_TRAMPOLINE\n.weak FTM2_CH4_CH5\nFTM2_CH4_CH5 = DH_TRAMPOLINE\n.weak FTM2_CH6_CH7\nFTM2_CH6_CH7 = DH_TRAMPOLINE\n.weak FTM2_FAULT\nFTM2_FAULT = DH_TRAMPOLINE\n.weak FTM2_OVF_RELOAD\nFTM2_OVF_RELOAD = DH_TRAMPOLINE\n.weak FTM3_CH0_CH1\nFTM3_CH0_CH1 = DH_TRAMPOLINE\n.weak FTM3_CH2_CH3\nFTM3_CH2_CH3 = DH_TRAMPOLINE\n.weak FTM3_CH4_CH5\nFTM3_CH4_CH5 = DH_TRAMPOLINE\n.weak FTM3_CH6_CH7\nFTM3_CH6_CH7 = DH_TRAMPOLINE\n.weak FTM3_FAULT\nFTM3_FAULT = DH_TRAMPOLINE\n.weak FTM3_OVF_RELOAD\nFTM3_OVF_RELOAD = DH_TRAMPOLINE" ) ;
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
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 123] = [
    Some(DMA0),
    Some(DMA1),
    Some(DMA2),
    Some(DMA3),
    Some(DMA4),
    Some(DMA5),
    Some(DMA6),
    Some(DMA7),
    Some(DMA8),
    Some(DMA9),
    Some(DMA10),
    Some(DMA11),
    Some(DMA12),
    Some(DMA13),
    Some(DMA14),
    Some(DMA15),
    Some(DMA_ERROR),
    Some(MCM),
    Some(FTFC),
    Some(READ_COLLISION),
    Some(LVD_LVW),
    Some(FTFC_FAULT),
    Some(WDOG_EWM),
    Some(RCM),
    Some(LPI2C0_MASTER),
    Some(LPI2C0_SLAVE),
    Some(LPSPI0),
    Some(LPSPI1),
    Some(LPSPI2),
    None,
    None,
    Some(LPUART0_RXTX),
    None,
    Some(LPUART1_RXTX),
    None,
    Some(LPUART2_RXTX),
    None,
    None,
    None,
    Some(ADC0),
    Some(ADC1),
    Some(CMP0),
    None,
    None,
    Some(ERM_SINGLE_FAULT),
    Some(ERM_DOUBLE_FAULT),
    Some(RTC),
    Some(RTC_SECONDS),
    Some(LPIT0_CH0),
    Some(LPIT0_CH1),
    Some(LPIT0_CH2),
    Some(LPIT0_CH3),
    Some(PDB0),
    None,
    None,
    None,
    None,
    Some(SCG),
    Some(LPTMR0),
    Some(PORTA),
    Some(PORTB),
    Some(PORTC),
    Some(PORTD),
    Some(PORTE),
    None,
    None,
    None,
    None,
    Some(PDB1),
    Some(FLEXIO),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(CAN0_ORED),
    Some(CAN0_ERROR),
    Some(CAN0_WAKE_UP),
    Some(CAN0_ORED_0_15_MB),
    Some(CAN0_ORED_16_31_MB),
    None,
    None,
    Some(CAN1_ORED),
    Some(CAN1_ERROR),
    None,
    Some(CAN1_ORED_0_15_MB),
    None,
    None,
    None,
    Some(CAN2_ORED),
    Some(CAN2_ERROR),
    None,
    Some(CAN2_ORED_0_15_MB),
    None,
    None,
    None,
    Some(FTM0_CH0_CH1),
    Some(FTM0_CH2_CH3),
    Some(FTM0_CH4_CH5),
    Some(FTM0_CH6_CH7),
    Some(FTM0_FAULT),
    Some(FTM0_OVF_RELOAD),
    Some(FTM1_CH0_CH1),
    Some(FTM1_CH2_CH3),
    Some(FTM1_CH4_CH5),
    Some(FTM1_CH6_CH7),
    Some(FTM1_FAULT),
    Some(FTM1_OVF_RELOAD),
    Some(FTM2_CH0_CH1),
    Some(FTM2_CH2_CH3),
    Some(FTM2_CH4_CH5),
    Some(FTM2_CH6_CH7),
    Some(FTM2_FAULT),
    Some(FTM2_OVF_RELOAD),
    Some(FTM3_CH0_CH1),
    Some(FTM3_CH2_CH3),
    Some(FTM3_CH4_CH5),
    Some(FTM3_CH6_CH7),
    Some(FTM3_FAULT),
    Some(FTM3_OVF_RELOAD),
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
unsafe impl Nr for Interrupt {
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
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
