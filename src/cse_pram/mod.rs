use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSE PRAM 0 Register"]
    pub embedded_ram0: EMBEDDEDRAM0,
    #[doc = "0x04 - CSE PRAM 1 Register"]
    pub embedded_ram1: EMBEDDEDRAM1,
    #[doc = "0x08 - CSE PRAM 2 Register"]
    pub embedded_ram2: EMBEDDEDRAM2,
    #[doc = "0x0c - CSE PRAM 3 Register"]
    pub embedded_ram3: EMBEDDEDRAM3,
    #[doc = "0x10 - CSE PRAM 4 Register"]
    pub embedded_ram4: EMBEDDEDRAM4,
    #[doc = "0x14 - CSE PRAM 5 Register"]
    pub embedded_ram5: EMBEDDEDRAM5,
    #[doc = "0x18 - CSE PRAM 6 Register"]
    pub embedded_ram6: EMBEDDEDRAM6,
    #[doc = "0x1c - CSE PRAM 7 Register"]
    pub embedded_ram7: EMBEDDEDRAM7,
    #[doc = "0x20 - CSE PRAM 8 Register"]
    pub embedded_ram8: EMBEDDEDRAM8,
    #[doc = "0x24 - CSE PRAM 9 Register"]
    pub embedded_ram9: EMBEDDEDRAM9,
    #[doc = "0x28 - CSE PRAM 10 Register"]
    pub embedded_ram10: EMBEDDEDRAM10,
    #[doc = "0x2c - CSE PRAM 11 Register"]
    pub embedded_ram11: EMBEDDEDRAM11,
    #[doc = "0x30 - CSE PRAM 12 Register"]
    pub embedded_ram12: EMBEDDEDRAM12,
    #[doc = "0x34 - CSE PRAM 13 Register"]
    pub embedded_ram13: EMBEDDEDRAM13,
    #[doc = "0x38 - CSE PRAM 14 Register"]
    pub embedded_ram14: EMBEDDEDRAM14,
    #[doc = "0x3c - CSE PRAM 15 Register"]
    pub embedded_ram15: EMBEDDEDRAM15,
    #[doc = "0x40 - CSE PRAM 16 Register"]
    pub embedded_ram16: EMBEDDEDRAM16,
    #[doc = "0x44 - CSE PRAM 17 Register"]
    pub embedded_ram17: EMBEDDEDRAM17,
    #[doc = "0x48 - CSE PRAM 18 Register"]
    pub embedded_ram18: EMBEDDEDRAM18,
    #[doc = "0x4c - CSE PRAM 19 Register"]
    pub embedded_ram19: EMBEDDEDRAM19,
    #[doc = "0x50 - CSE PRAM 20 Register"]
    pub embedded_ram20: EMBEDDEDRAM20,
    #[doc = "0x54 - CSE PRAM 21 Register"]
    pub embedded_ram21: EMBEDDEDRAM21,
    #[doc = "0x58 - CSE PRAM 22 Register"]
    pub embedded_ram22: EMBEDDEDRAM22,
    #[doc = "0x5c - CSE PRAM 23 Register"]
    pub embedded_ram23: EMBEDDEDRAM23,
    #[doc = "0x60 - CSE PRAM 24 Register"]
    pub embedded_ram24: EMBEDDEDRAM24,
    #[doc = "0x64 - CSE PRAM 25 Register"]
    pub embedded_ram25: EMBEDDEDRAM25,
    #[doc = "0x68 - CSE PRAM 26 Register"]
    pub embedded_ram26: EMBEDDEDRAM26,
    #[doc = "0x6c - CSE PRAM 27 Register"]
    pub embedded_ram27: EMBEDDEDRAM27,
    #[doc = "0x70 - CSE PRAM 28 Register"]
    pub embedded_ram28: EMBEDDEDRAM28,
    #[doc = "0x74 - CSE PRAM 29 Register"]
    pub embedded_ram29: EMBEDDEDRAM29,
    #[doc = "0x78 - CSE PRAM 30 Register"]
    pub embedded_ram30: EMBEDDEDRAM30,
    #[doc = "0x7c - CSE PRAM 31 Register"]
    pub embedded_ram31: EMBEDDEDRAM31,
}
#[doc = "CSE PRAM 0 Register"]
pub struct EMBEDDEDRAM0 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 0 Register"]
pub mod embedded_ram0;
#[doc = "CSE PRAM0LL register."]
pub struct EMBEDDEDRAM0LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM0LL register."]
pub mod embedded_ram0ll;
#[doc = "CSE PRAM0LU register."]
pub struct EMBEDDEDRAM0LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM0LU register."]
pub mod embedded_ram0lu;
#[doc = "CSE PRAM0HL register."]
pub struct EMBEDDEDRAM0HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM0HL register."]
pub mod embedded_ram0hl;
#[doc = "CSE PRAM0HU register."]
pub struct EMBEDDEDRAM0HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM0HU register."]
pub mod embedded_ram0hu;
#[doc = "CSE PRAM 1 Register"]
pub struct EMBEDDEDRAM1 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 1 Register"]
pub mod embedded_ram1;
#[doc = "CSE PRAM1LL register."]
pub struct EMBEDDEDRAM1LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM1LL register."]
pub mod embedded_ram1ll;
#[doc = "CSE PRAM1LU register."]
pub struct EMBEDDEDRAM1LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM1LU register."]
pub mod embedded_ram1lu;
#[doc = "CSE PRAM1HL register."]
pub struct EMBEDDEDRAM1HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM1HL register."]
pub mod embedded_ram1hl;
#[doc = "CSE PRAM1HU register."]
pub struct EMBEDDEDRAM1HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM1HU register."]
pub mod embedded_ram1hu;
#[doc = "CSE PRAM 2 Register"]
pub struct EMBEDDEDRAM2 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 2 Register"]
pub mod embedded_ram2;
#[doc = "CSE PRAM2LL register."]
pub struct EMBEDDEDRAM2LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM2LL register."]
pub mod embedded_ram2ll;
#[doc = "CSE PRAM2LU register."]
pub struct EMBEDDEDRAM2LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM2LU register."]
pub mod embedded_ram2lu;
#[doc = "CSE PRAM2HL register."]
pub struct EMBEDDEDRAM2HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM2HL register."]
pub mod embedded_ram2hl;
#[doc = "CSE PRAM2HU register."]
pub struct EMBEDDEDRAM2HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM2HU register."]
pub mod embedded_ram2hu;
#[doc = "CSE PRAM 3 Register"]
pub struct EMBEDDEDRAM3 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 3 Register"]
pub mod embedded_ram3;
#[doc = "CSE PRAM3LL register."]
pub struct EMBEDDEDRAM3LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM3LL register."]
pub mod embedded_ram3ll;
#[doc = "CSE PRAM3LU register."]
pub struct EMBEDDEDRAM3LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM3LU register."]
pub mod embedded_ram3lu;
#[doc = "CSE PRAM3HL register."]
pub struct EMBEDDEDRAM3HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM3HL register."]
pub mod embedded_ram3hl;
#[doc = "CSE PRAM3HU register."]
pub struct EMBEDDEDRAM3HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM3HU register."]
pub mod embedded_ram3hu;
#[doc = "CSE PRAM 4 Register"]
pub struct EMBEDDEDRAM4 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 4 Register"]
pub mod embedded_ram4;
#[doc = "CSE PRAM4LL register."]
pub struct EMBEDDEDRAM4LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM4LL register."]
pub mod embedded_ram4ll;
#[doc = "CSE PRAM4LU register."]
pub struct EMBEDDEDRAM4LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM4LU register."]
pub mod embedded_ram4lu;
#[doc = "CSE PRAM4HL register."]
pub struct EMBEDDEDRAM4HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM4HL register."]
pub mod embedded_ram4hl;
#[doc = "CSE PRAM4HU register."]
pub struct EMBEDDEDRAM4HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM4HU register."]
pub mod embedded_ram4hu;
#[doc = "CSE PRAM 5 Register"]
pub struct EMBEDDEDRAM5 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 5 Register"]
pub mod embedded_ram5;
#[doc = "CSE PRAM5LL register."]
pub struct EMBEDDEDRAM5LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM5LL register."]
pub mod embedded_ram5ll;
#[doc = "CSE PRAM5LU register."]
pub struct EMBEDDEDRAM5LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM5LU register."]
pub mod embedded_ram5lu;
#[doc = "CSE PRAM5HL register."]
pub struct EMBEDDEDRAM5HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM5HL register."]
pub mod embedded_ram5hl;
#[doc = "CSE PRAM5HU register."]
pub struct EMBEDDEDRAM5HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM5HU register."]
pub mod embedded_ram5hu;
#[doc = "CSE PRAM 6 Register"]
pub struct EMBEDDEDRAM6 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 6 Register"]
pub mod embedded_ram6;
#[doc = "CSE PRAM6LL register."]
pub struct EMBEDDEDRAM6LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM6LL register."]
pub mod embedded_ram6ll;
#[doc = "CSE PRAM6LU register."]
pub struct EMBEDDEDRAM6LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM6LU register."]
pub mod embedded_ram6lu;
#[doc = "CSE PRAM6HL register."]
pub struct EMBEDDEDRAM6HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM6HL register."]
pub mod embedded_ram6hl;
#[doc = "CSE PRAM6HU register."]
pub struct EMBEDDEDRAM6HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM6HU register."]
pub mod embedded_ram6hu;
#[doc = "CSE PRAM 7 Register"]
pub struct EMBEDDEDRAM7 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 7 Register"]
pub mod embedded_ram7;
#[doc = "CSE PRAM7LL register."]
pub struct EMBEDDEDRAM7LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM7LL register."]
pub mod embedded_ram7ll;
#[doc = "CSE PRAM7LU register."]
pub struct EMBEDDEDRAM7LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM7LU register."]
pub mod embedded_ram7lu;
#[doc = "CSE PRAM7HL register."]
pub struct EMBEDDEDRAM7HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM7HL register."]
pub mod embedded_ram7hl;
#[doc = "CSE PRAM7HU register."]
pub struct EMBEDDEDRAM7HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM7HU register."]
pub mod embedded_ram7hu;
#[doc = "CSE PRAM 8 Register"]
pub struct EMBEDDEDRAM8 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 8 Register"]
pub mod embedded_ram8;
#[doc = "CSE PRAM8LL register."]
pub struct EMBEDDEDRAM8LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM8LL register."]
pub mod embedded_ram8ll;
#[doc = "CSE PRAM8LU register."]
pub struct EMBEDDEDRAM8LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM8LU register."]
pub mod embedded_ram8lu;
#[doc = "CSE PRAM8HL register."]
pub struct EMBEDDEDRAM8HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM8HL register."]
pub mod embedded_ram8hl;
#[doc = "CSE PRAM8HU register."]
pub struct EMBEDDEDRAM8HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM8HU register."]
pub mod embedded_ram8hu;
#[doc = "CSE PRAM 9 Register"]
pub struct EMBEDDEDRAM9 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 9 Register"]
pub mod embedded_ram9;
#[doc = "CSE PRAM9LL register."]
pub struct EMBEDDEDRAM9LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM9LL register."]
pub mod embedded_ram9ll;
#[doc = "CSE PRAM9LU register."]
pub struct EMBEDDEDRAM9LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM9LU register."]
pub mod embedded_ram9lu;
#[doc = "CSE PRAM9HL register."]
pub struct EMBEDDEDRAM9HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM9HL register."]
pub mod embedded_ram9hl;
#[doc = "CSE PRAM9HU register."]
pub struct EMBEDDEDRAM9HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM9HU register."]
pub mod embedded_ram9hu;
#[doc = "CSE PRAM 10 Register"]
pub struct EMBEDDEDRAM10 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 10 Register"]
pub mod embedded_ram10;
#[doc = "CSE PRAM10LL register."]
pub struct EMBEDDEDRAM10LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM10LL register."]
pub mod embedded_ram10ll;
#[doc = "CSE PRAM10LU register."]
pub struct EMBEDDEDRAM10LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM10LU register."]
pub mod embedded_ram10lu;
#[doc = "CSE PRAM10HL register."]
pub struct EMBEDDEDRAM10HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM10HL register."]
pub mod embedded_ram10hl;
#[doc = "CSE PRAM10HU register."]
pub struct EMBEDDEDRAM10HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM10HU register."]
pub mod embedded_ram10hu;
#[doc = "CSE PRAM 11 Register"]
pub struct EMBEDDEDRAM11 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 11 Register"]
pub mod embedded_ram11;
#[doc = "CSE PRAM11LL register."]
pub struct EMBEDDEDRAM11LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM11LL register."]
pub mod embedded_ram11ll;
#[doc = "CSE PRAM11LU register."]
pub struct EMBEDDEDRAM11LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM11LU register."]
pub mod embedded_ram11lu;
#[doc = "CSE PRAM11HL register."]
pub struct EMBEDDEDRAM11HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM11HL register."]
pub mod embedded_ram11hl;
#[doc = "CSE PRAM11HU register."]
pub struct EMBEDDEDRAM11HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM11HU register."]
pub mod embedded_ram11hu;
#[doc = "CSE PRAM 12 Register"]
pub struct EMBEDDEDRAM12 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 12 Register"]
pub mod embedded_ram12;
#[doc = "CSE PRAM12LL register."]
pub struct EMBEDDEDRAM12LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM12LL register."]
pub mod embedded_ram12ll;
#[doc = "CSE PRAM12LU register."]
pub struct EMBEDDEDRAM12LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM12LU register."]
pub mod embedded_ram12lu;
#[doc = "CSE PRAM12HL register."]
pub struct EMBEDDEDRAM12HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM12HL register."]
pub mod embedded_ram12hl;
#[doc = "CSE PRAM12HU register."]
pub struct EMBEDDEDRAM12HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM12HU register."]
pub mod embedded_ram12hu;
#[doc = "CSE PRAM 13 Register"]
pub struct EMBEDDEDRAM13 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 13 Register"]
pub mod embedded_ram13;
#[doc = "CSE PRAM13LL register."]
pub struct EMBEDDEDRAM13LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM13LL register."]
pub mod embedded_ram13ll;
#[doc = "CSE PRAM13LU register."]
pub struct EMBEDDEDRAM13LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM13LU register."]
pub mod embedded_ram13lu;
#[doc = "CSE PRAM13HL register."]
pub struct EMBEDDEDRAM13HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM13HL register."]
pub mod embedded_ram13hl;
#[doc = "CSE PRAM13HU register."]
pub struct EMBEDDEDRAM13HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM13HU register."]
pub mod embedded_ram13hu;
#[doc = "CSE PRAM 14 Register"]
pub struct EMBEDDEDRAM14 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 14 Register"]
pub mod embedded_ram14;
#[doc = "CSE PRAM14LL register."]
pub struct EMBEDDEDRAM14LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM14LL register."]
pub mod embedded_ram14ll;
#[doc = "CSE PRAM14LU register."]
pub struct EMBEDDEDRAM14LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM14LU register."]
pub mod embedded_ram14lu;
#[doc = "CSE PRAM14HL register."]
pub struct EMBEDDEDRAM14HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM14HL register."]
pub mod embedded_ram14hl;
#[doc = "CSE PRAM14HU register."]
pub struct EMBEDDEDRAM14HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM14HU register."]
pub mod embedded_ram14hu;
#[doc = "CSE PRAM 15 Register"]
pub struct EMBEDDEDRAM15 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 15 Register"]
pub mod embedded_ram15;
#[doc = "CSE PRAM15LL register."]
pub struct EMBEDDEDRAM15LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM15LL register."]
pub mod embedded_ram15ll;
#[doc = "CSE PRAM15LU register."]
pub struct EMBEDDEDRAM15LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM15LU register."]
pub mod embedded_ram15lu;
#[doc = "CSE PRAM15HL register."]
pub struct EMBEDDEDRAM15HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM15HL register."]
pub mod embedded_ram15hl;
#[doc = "CSE PRAM15HU register."]
pub struct EMBEDDEDRAM15HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM15HU register."]
pub mod embedded_ram15hu;
#[doc = "CSE PRAM 16 Register"]
pub struct EMBEDDEDRAM16 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 16 Register"]
pub mod embedded_ram16;
#[doc = "CSE PRAM16LL register."]
pub struct EMBEDDEDRAM16LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM16LL register."]
pub mod embedded_ram16ll;
#[doc = "CSE PRAM16LU register."]
pub struct EMBEDDEDRAM16LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM16LU register."]
pub mod embedded_ram16lu;
#[doc = "CSE PRAM16HL register."]
pub struct EMBEDDEDRAM16HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM16HL register."]
pub mod embedded_ram16hl;
#[doc = "CSE PRAM16HU register."]
pub struct EMBEDDEDRAM16HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM16HU register."]
pub mod embedded_ram16hu;
#[doc = "CSE PRAM 17 Register"]
pub struct EMBEDDEDRAM17 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 17 Register"]
pub mod embedded_ram17;
#[doc = "CSE PRAM17LL register."]
pub struct EMBEDDEDRAM17LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM17LL register."]
pub mod embedded_ram17ll;
#[doc = "CSE PRAM17LU register."]
pub struct EMBEDDEDRAM17LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM17LU register."]
pub mod embedded_ram17lu;
#[doc = "CSE PRAM17HL register."]
pub struct EMBEDDEDRAM17HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM17HL register."]
pub mod embedded_ram17hl;
#[doc = "CSE PRAM17HU register."]
pub struct EMBEDDEDRAM17HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM17HU register."]
pub mod embedded_ram17hu;
#[doc = "CSE PRAM 18 Register"]
pub struct EMBEDDEDRAM18 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 18 Register"]
pub mod embedded_ram18;
#[doc = "CSE PRAM18LL register."]
pub struct EMBEDDEDRAM18LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM18LL register."]
pub mod embedded_ram18ll;
#[doc = "CSE PRAM18LU register."]
pub struct EMBEDDEDRAM18LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM18LU register."]
pub mod embedded_ram18lu;
#[doc = "CSE PRAM18HL register."]
pub struct EMBEDDEDRAM18HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM18HL register."]
pub mod embedded_ram18hl;
#[doc = "CSE PRAM18HU register."]
pub struct EMBEDDEDRAM18HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM18HU register."]
pub mod embedded_ram18hu;
#[doc = "CSE PRAM 19 Register"]
pub struct EMBEDDEDRAM19 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 19 Register"]
pub mod embedded_ram19;
#[doc = "CSE PRAM19LL register."]
pub struct EMBEDDEDRAM19LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM19LL register."]
pub mod embedded_ram19ll;
#[doc = "CSE PRAM19LU register."]
pub struct EMBEDDEDRAM19LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM19LU register."]
pub mod embedded_ram19lu;
#[doc = "CSE PRAM19HL register."]
pub struct EMBEDDEDRAM19HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM19HL register."]
pub mod embedded_ram19hl;
#[doc = "CSE PRAM19HU register."]
pub struct EMBEDDEDRAM19HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM19HU register."]
pub mod embedded_ram19hu;
#[doc = "CSE PRAM 20 Register"]
pub struct EMBEDDEDRAM20 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 20 Register"]
pub mod embedded_ram20;
#[doc = "CSE PRAM20LL register."]
pub struct EMBEDDEDRAM20LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM20LL register."]
pub mod embedded_ram20ll;
#[doc = "CSE PRAM20LU register."]
pub struct EMBEDDEDRAM20LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM20LU register."]
pub mod embedded_ram20lu;
#[doc = "CSE PRAM20HL register."]
pub struct EMBEDDEDRAM20HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM20HL register."]
pub mod embedded_ram20hl;
#[doc = "CSE PRAM20HU register."]
pub struct EMBEDDEDRAM20HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM20HU register."]
pub mod embedded_ram20hu;
#[doc = "CSE PRAM 21 Register"]
pub struct EMBEDDEDRAM21 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 21 Register"]
pub mod embedded_ram21;
#[doc = "CSE PRAM21LL register."]
pub struct EMBEDDEDRAM21LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM21LL register."]
pub mod embedded_ram21ll;
#[doc = "CSE PRAM21LU register."]
pub struct EMBEDDEDRAM21LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM21LU register."]
pub mod embedded_ram21lu;
#[doc = "CSE PRAM21HL register."]
pub struct EMBEDDEDRAM21HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM21HL register."]
pub mod embedded_ram21hl;
#[doc = "CSE PRAM21HU register."]
pub struct EMBEDDEDRAM21HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM21HU register."]
pub mod embedded_ram21hu;
#[doc = "CSE PRAM 22 Register"]
pub struct EMBEDDEDRAM22 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 22 Register"]
pub mod embedded_ram22;
#[doc = "CSE PRAM22LL register."]
pub struct EMBEDDEDRAM22LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM22LL register."]
pub mod embedded_ram22ll;
#[doc = "CSE PRAM22LU register."]
pub struct EMBEDDEDRAM22LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM22LU register."]
pub mod embedded_ram22lu;
#[doc = "CSE PRAM22HL register."]
pub struct EMBEDDEDRAM22HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM22HL register."]
pub mod embedded_ram22hl;
#[doc = "CSE PRAM22HU register."]
pub struct EMBEDDEDRAM22HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM22HU register."]
pub mod embedded_ram22hu;
#[doc = "CSE PRAM 23 Register"]
pub struct EMBEDDEDRAM23 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 23 Register"]
pub mod embedded_ram23;
#[doc = "CSE PRAM23LL register."]
pub struct EMBEDDEDRAM23LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM23LL register."]
pub mod embedded_ram23ll;
#[doc = "CSE PRAM23LU register."]
pub struct EMBEDDEDRAM23LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM23LU register."]
pub mod embedded_ram23lu;
#[doc = "CSE PRAM23HL register."]
pub struct EMBEDDEDRAM23HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM23HL register."]
pub mod embedded_ram23hl;
#[doc = "CSE PRAM23HU register."]
pub struct EMBEDDEDRAM23HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM23HU register."]
pub mod embedded_ram23hu;
#[doc = "CSE PRAM 24 Register"]
pub struct EMBEDDEDRAM24 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 24 Register"]
pub mod embedded_ram24;
#[doc = "CSE PRAM24LL register."]
pub struct EMBEDDEDRAM24LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM24LL register."]
pub mod embedded_ram24ll;
#[doc = "CSE PRAM24LU register."]
pub struct EMBEDDEDRAM24LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM24LU register."]
pub mod embedded_ram24lu;
#[doc = "CSE PRAM24HL register."]
pub struct EMBEDDEDRAM24HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM24HL register."]
pub mod embedded_ram24hl;
#[doc = "CSE PRAM24HU register."]
pub struct EMBEDDEDRAM24HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM24HU register."]
pub mod embedded_ram24hu;
#[doc = "CSE PRAM 25 Register"]
pub struct EMBEDDEDRAM25 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 25 Register"]
pub mod embedded_ram25;
#[doc = "CSE PRAM25LL register."]
pub struct EMBEDDEDRAM25LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM25LL register."]
pub mod embedded_ram25ll;
#[doc = "CSE PRAM25LU register."]
pub struct EMBEDDEDRAM25LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM25LU register."]
pub mod embedded_ram25lu;
#[doc = "CSE PRAM25HL register."]
pub struct EMBEDDEDRAM25HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM25HL register."]
pub mod embedded_ram25hl;
#[doc = "CSE PRAM25HU register."]
pub struct EMBEDDEDRAM25HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM25HU register."]
pub mod embedded_ram25hu;
#[doc = "CSE PRAM 26 Register"]
pub struct EMBEDDEDRAM26 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 26 Register"]
pub mod embedded_ram26;
#[doc = "CSE PRAM26LL register."]
pub struct EMBEDDEDRAM26LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM26LL register."]
pub mod embedded_ram26ll;
#[doc = "CSE PRAM26LU register."]
pub struct EMBEDDEDRAM26LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM26LU register."]
pub mod embedded_ram26lu;
#[doc = "CSE PRAM26HL register."]
pub struct EMBEDDEDRAM26HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM26HL register."]
pub mod embedded_ram26hl;
#[doc = "CSE PRAM26HU register."]
pub struct EMBEDDEDRAM26HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM26HU register."]
pub mod embedded_ram26hu;
#[doc = "CSE PRAM 27 Register"]
pub struct EMBEDDEDRAM27 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 27 Register"]
pub mod embedded_ram27;
#[doc = "CSE PRAM27LL register."]
pub struct EMBEDDEDRAM27LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM27LL register."]
pub mod embedded_ram27ll;
#[doc = "CSE PRAM27LU register."]
pub struct EMBEDDEDRAM27LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM27LU register."]
pub mod embedded_ram27lu;
#[doc = "CSE PRAM27HL register."]
pub struct EMBEDDEDRAM27HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM27HL register."]
pub mod embedded_ram27hl;
#[doc = "CSE PRAM27HU register."]
pub struct EMBEDDEDRAM27HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM27HU register."]
pub mod embedded_ram27hu;
#[doc = "CSE PRAM 28 Register"]
pub struct EMBEDDEDRAM28 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 28 Register"]
pub mod embedded_ram28;
#[doc = "CSE PRAM28LL register."]
pub struct EMBEDDEDRAM28LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM28LL register."]
pub mod embedded_ram28ll;
#[doc = "CSE PRAM28LU register."]
pub struct EMBEDDEDRAM28LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM28LU register."]
pub mod embedded_ram28lu;
#[doc = "CSE PRAM28HL register."]
pub struct EMBEDDEDRAM28HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM28HL register."]
pub mod embedded_ram28hl;
#[doc = "CSE PRAM28HU register."]
pub struct EMBEDDEDRAM28HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM28HU register."]
pub mod embedded_ram28hu;
#[doc = "CSE PRAM 29 Register"]
pub struct EMBEDDEDRAM29 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 29 Register"]
pub mod embedded_ram29;
#[doc = "CSE PRAM29LL register."]
pub struct EMBEDDEDRAM29LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM29LL register."]
pub mod embedded_ram29ll;
#[doc = "CSE PRAM29LU register."]
pub struct EMBEDDEDRAM29LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM29LU register."]
pub mod embedded_ram29lu;
#[doc = "CSE PRAM29HL register."]
pub struct EMBEDDEDRAM29HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM29HL register."]
pub mod embedded_ram29hl;
#[doc = "CSE PRAM29HU register."]
pub struct EMBEDDEDRAM29HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM29HU register."]
pub mod embedded_ram29hu;
#[doc = "CSE PRAM 30 Register"]
pub struct EMBEDDEDRAM30 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 30 Register"]
pub mod embedded_ram30;
#[doc = "CSE PRAM30LL register."]
pub struct EMBEDDEDRAM30LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM30LL register."]
pub mod embedded_ram30ll;
#[doc = "CSE PRAM30LU register."]
pub struct EMBEDDEDRAM30LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM30LU register."]
pub mod embedded_ram30lu;
#[doc = "CSE PRAM30HL register."]
pub struct EMBEDDEDRAM30HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM30HL register."]
pub mod embedded_ram30hl;
#[doc = "CSE PRAM30HU register."]
pub struct EMBEDDEDRAM30HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM30HU register."]
pub mod embedded_ram30hu;
#[doc = "CSE PRAM 31 Register"]
pub struct EMBEDDEDRAM31 {
    register: VolatileCell<u32>,
}
#[doc = "CSE PRAM 31 Register"]
pub mod embedded_ram31;
#[doc = "CSE PRAM31LL register."]
pub struct EMBEDDEDRAM31LL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM31LL register."]
pub mod embedded_ram31ll;
#[doc = "CSE PRAM31LU register."]
pub struct EMBEDDEDRAM31LU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM31LU register."]
pub mod embedded_ram31lu;
#[doc = "CSE PRAM31HL register."]
pub struct EMBEDDEDRAM31HL {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM31HL register."]
pub mod embedded_ram31hl;
#[doc = "CSE PRAM31HU register."]
pub struct EMBEDDEDRAM31HU {
    register: VolatileCell<u8>,
}
#[doc = "CSE PRAM31HU register."]
pub mod embedded_ram31hu;
