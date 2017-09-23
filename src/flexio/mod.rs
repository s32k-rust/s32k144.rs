use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"] pub verid: VERID,
    #[doc = "0x04 - Parameter Register"] pub param: PARAM,
    #[doc = "0x08 - FlexIO Control Register"] pub ctrl: CTRL,
    #[doc = "0x0c - Pin State Register"] pub pin: PIN,
    #[doc = "0x10 - Shifter Status Register"] pub shiftstat: SHIFTSTAT,
    #[doc = "0x14 - Shifter Error Register"] pub shifterr: SHIFTERR,
    #[doc = "0x18 - Timer Status Register"] pub timstat: TIMSTAT,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Shifter Status Interrupt Enable"] pub shiftsien: SHIFTSIEN,
    #[doc = "0x24 - Shifter Error Interrupt Enable"] pub shifteien: SHIFTEIEN,
    #[doc = "0x28 - Timer Interrupt Enable Register"] pub timien: TIMIEN,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Shifter Status DMA Enable"] pub shiftsden: SHIFTSDEN,
    _reserved2: [u8; 76usize],
    #[doc = "0x80 - Shifter Control N Register"] pub shiftctl0: SHIFTCTL0,
    #[doc = "0x84 - Shifter Control N Register"] pub shiftctl1: SHIFTCTL1,
    #[doc = "0x88 - Shifter Control N Register"] pub shiftctl2: SHIFTCTL2,
    #[doc = "0x8c - Shifter Control N Register"] pub shiftctl3: SHIFTCTL3,
    _reserved3: [u8; 112usize],
    #[doc = "0x100 - Shifter Configuration N Register"] pub shiftcfg0: SHIFTCFG0,
    #[doc = "0x104 - Shifter Configuration N Register"] pub shiftcfg1: SHIFTCFG1,
    #[doc = "0x108 - Shifter Configuration N Register"] pub shiftcfg2: SHIFTCFG2,
    #[doc = "0x10c - Shifter Configuration N Register"] pub shiftcfg3: SHIFTCFG3,
    _reserved4: [u8; 240usize],
    #[doc = "0x200 - Shifter Buffer N Register"] pub shiftbuf0: SHIFTBUF0,
    #[doc = "0x204 - Shifter Buffer N Register"] pub shiftbuf1: SHIFTBUF1,
    #[doc = "0x208 - Shifter Buffer N Register"] pub shiftbuf2: SHIFTBUF2,
    #[doc = "0x20c - Shifter Buffer N Register"] pub shiftbuf3: SHIFTBUF3,
    _reserved5: [u8; 112usize],
    #[doc = "0x280 - Shifter Buffer N Bit Swapped Register"] pub shiftbufbis0: SHIFTBUFBIS0,
    #[doc = "0x284 - Shifter Buffer N Bit Swapped Register"] pub shiftbufbis1: SHIFTBUFBIS1,
    #[doc = "0x288 - Shifter Buffer N Bit Swapped Register"] pub shiftbufbis2: SHIFTBUFBIS2,
    #[doc = "0x28c - Shifter Buffer N Bit Swapped Register"] pub shiftbufbis3: SHIFTBUFBIS3,
    _reserved6: [u8; 112usize],
    #[doc = "0x300 - Shifter Buffer N Byte Swapped Register"] pub shiftbufbys0: SHIFTBUFBYS0,
    #[doc = "0x304 - Shifter Buffer N Byte Swapped Register"] pub shiftbufbys1: SHIFTBUFBYS1,
    #[doc = "0x308 - Shifter Buffer N Byte Swapped Register"] pub shiftbufbys2: SHIFTBUFBYS2,
    #[doc = "0x30c - Shifter Buffer N Byte Swapped Register"] pub shiftbufbys3: SHIFTBUFBYS3,
    _reserved7: [u8; 112usize],
    #[doc = "0x380 - Shifter Buffer N Bit Byte Swapped Register"] pub shiftbufbbs0: SHIFTBUFBBS0,
    #[doc = "0x384 - Shifter Buffer N Bit Byte Swapped Register"] pub shiftbufbbs1: SHIFTBUFBBS1,
    #[doc = "0x388 - Shifter Buffer N Bit Byte Swapped Register"] pub shiftbufbbs2: SHIFTBUFBBS2,
    #[doc = "0x38c - Shifter Buffer N Bit Byte Swapped Register"] pub shiftbufbbs3: SHIFTBUFBBS3,
    _reserved8: [u8; 112usize],
    #[doc = "0x400 - Timer Control N Register"] pub timctl0: TIMCTL0,
    #[doc = "0x404 - Timer Control N Register"] pub timctl1: TIMCTL1,
    #[doc = "0x408 - Timer Control N Register"] pub timctl2: TIMCTL2,
    #[doc = "0x40c - Timer Control N Register"] pub timctl3: TIMCTL3,
    _reserved9: [u8; 112usize],
    #[doc = "0x480 - Timer Configuration N Register"] pub timcfg0: TIMCFG0,
    #[doc = "0x484 - Timer Configuration N Register"] pub timcfg1: TIMCFG1,
    #[doc = "0x488 - Timer Configuration N Register"] pub timcfg2: TIMCFG2,
    #[doc = "0x48c - Timer Configuration N Register"] pub timcfg3: TIMCFG3,
    _reserved10: [u8; 112usize],
    #[doc = "0x500 - Timer Compare N Register"] pub timcmp0: TIMCMP0,
    #[doc = "0x504 - Timer Compare N Register"] pub timcmp1: TIMCMP1,
    #[doc = "0x508 - Timer Compare N Register"] pub timcmp2: TIMCMP2,
    #[doc = "0x50c - Timer Compare N Register"] pub timcmp3: TIMCMP3,
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "FlexIO Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "Pin State Register"]
pub struct PIN {
    register: VolatileCell<u32>,
}
#[doc = "Pin State Register"]
pub mod pin;
#[doc = "Shifter Status Register"]
pub struct SHIFTSTAT {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "Shifter Error Register"]
pub struct SHIFTERR {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "Timer Status Register"]
pub struct TIMSTAT {
    register: VolatileCell<u32>,
}
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "Shifter Status Interrupt Enable"]
pub struct SHIFTSIEN {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "Shifter Error Interrupt Enable"]
pub struct SHIFTEIEN {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "Timer Interrupt Enable Register"]
pub struct TIMIEN {
    register: VolatileCell<u32>,
}
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "Shifter Status DMA Enable"]
pub struct SHIFTSDEN {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "Shifter Control N Register"]
pub struct SHIFTCTL0 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Control N Register"]
pub mod shiftctl0;
#[doc = "Shifter Control N Register"]
pub struct SHIFTCTL1 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Control N Register"]
pub mod shiftctl1;
#[doc = "Shifter Control N Register"]
pub struct SHIFTCTL2 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Control N Register"]
pub mod shiftctl2;
#[doc = "Shifter Control N Register"]
pub struct SHIFTCTL3 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Control N Register"]
pub mod shiftctl3;
#[doc = "Shifter Configuration N Register"]
pub struct SHIFTCFG0 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg0;
#[doc = "Shifter Configuration N Register"]
pub struct SHIFTCFG1 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg1;
#[doc = "Shifter Configuration N Register"]
pub struct SHIFTCFG2 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg2;
#[doc = "Shifter Configuration N Register"]
pub struct SHIFTCFG3 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg3;
#[doc = "Shifter Buffer N Register"]
pub struct SHIFTBUF0 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf0;
#[doc = "Shifter Buffer N Register"]
pub struct SHIFTBUF1 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf1;
#[doc = "Shifter Buffer N Register"]
pub struct SHIFTBUF2 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf2;
#[doc = "Shifter Buffer N Register"]
pub struct SHIFTBUF3 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf3;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub struct SHIFTBUFBIS0 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis0;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub struct SHIFTBUFBIS1 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis1;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub struct SHIFTBUFBIS2 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis2;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub struct SHIFTBUFBIS3 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis3;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub struct SHIFTBUFBYS0 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys0;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub struct SHIFTBUFBYS1 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys1;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub struct SHIFTBUFBYS2 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys2;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub struct SHIFTBUFBYS3 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys3;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub struct SHIFTBUFBBS0 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs0;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub struct SHIFTBUFBBS1 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs1;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub struct SHIFTBUFBBS2 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs2;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub struct SHIFTBUFBBS3 {
    register: VolatileCell<u32>,
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs3;
#[doc = "Timer Control N Register"]
pub struct TIMCTL0 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Control N Register"]
pub mod timctl0;
#[doc = "Timer Control N Register"]
pub struct TIMCTL1 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Control N Register"]
pub mod timctl1;
#[doc = "Timer Control N Register"]
pub struct TIMCTL2 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Control N Register"]
pub mod timctl2;
#[doc = "Timer Control N Register"]
pub struct TIMCTL3 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Control N Register"]
pub mod timctl3;
#[doc = "Timer Configuration N Register"]
pub struct TIMCFG0 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Configuration N Register"]
pub mod timcfg0;
#[doc = "Timer Configuration N Register"]
pub struct TIMCFG1 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Configuration N Register"]
pub mod timcfg1;
#[doc = "Timer Configuration N Register"]
pub struct TIMCFG2 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Configuration N Register"]
pub mod timcfg2;
#[doc = "Timer Configuration N Register"]
pub struct TIMCFG3 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Configuration N Register"]
pub mod timcfg3;
#[doc = "Timer Compare N Register"]
pub struct TIMCMP0 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Compare N Register"]
pub mod timcmp0;
#[doc = "Timer Compare N Register"]
pub struct TIMCMP1 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Compare N Register"]
pub mod timcmp1;
#[doc = "Timer Compare N Register"]
pub struct TIMCMP2 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Compare N Register"]
pub mod timcmp2;
#[doc = "Timer Compare N Register"]
pub struct TIMCMP3 {
    register: VolatileCell<u32>,
}
#[doc = "Timer Compare N Register"]
pub mod timcmp3;
