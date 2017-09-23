#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHIPCTL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `ADC_INTERLEAVE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_INTERLEAVE_ENR {
    #[doc = "Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    _0000,
    #[doc = "PTB14 to ADC1_SE9 and ADC0_SE9"] _1XXX,
    #[doc = "PTB13 to ADC1_SE8 and ADC0_SE8"] X1XX,
    #[doc = "PTB1 to ADC0_SE5 and ADC1_SE15"] XX1X,
    #[doc = "PTB0 to ADC0_SE4 and ADC1_SE14"] XXX1,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl ADC_INTERLEAVE_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_INTERLEAVE_ENR::_0000 => 0,
            ADC_INTERLEAVE_ENR::_1XXX => 8,
            ADC_INTERLEAVE_ENR::X1XX => 4,
            ADC_INTERLEAVE_ENR::XX1X => 2,
            ADC_INTERLEAVE_ENR::XXX1 => 1,
            ADC_INTERLEAVE_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC_INTERLEAVE_ENR {
        match value {
            0 => ADC_INTERLEAVE_ENR::_0000,
            8 => ADC_INTERLEAVE_ENR::_1XXX,
            4 => ADC_INTERLEAVE_ENR::X1XX,
            2 => ADC_INTERLEAVE_ENR::XX1X,
            1 => ADC_INTERLEAVE_ENR::XXX1,
            i => ADC_INTERLEAVE_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == ADC_INTERLEAVE_ENR::_0000
    }
    #[doc = "Checks if the value of the field is `_1XXX`"]
    #[inline]
    pub fn is_1xxx(&self) -> bool {
        *self == ADC_INTERLEAVE_ENR::_1XXX
    }
    #[doc = "Checks if the value of the field is `X1XX`"]
    #[inline]
    pub fn is_x1xx(&self) -> bool {
        *self == ADC_INTERLEAVE_ENR::X1XX
    }
    #[doc = "Checks if the value of the field is `XX1X`"]
    #[inline]
    pub fn is_xx1x(&self) -> bool {
        *self == ADC_INTERLEAVE_ENR::XX1X
    }
    #[doc = "Checks if the value of the field is `XXX1`"]
    #[inline]
    pub fn is_xxx1(&self) -> bool {
        *self == ADC_INTERLEAVE_ENR::XXX1
    }
}
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "SCG CLKOUT"] _0000,
    #[doc = "SOSC DIV2 CLK"] _0010,
    #[doc = "SIRC DIV2 CLK"] _0100,
    #[doc = "For S32K148: QSPI SFIF_CLK_HYP: Divide by 2 clock (configured through SCLKCONFIG[5]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    _0101,
    #[doc = "FIRC DIV2 CLK"] _0110,
    #[doc = "HCLK"] _0111,
    #[doc = "SPLL DIV2 CLK"] _1000,
    #[doc = "BUS_CLK"] _1001,
    #[doc = "LPO128K_CLK"] _1010,
    #[doc = "For S32K148: QSPI IPG_CLK; For others: Reserved"] _1011,
    #[doc = "LPO_CLK as selected by SIM_LPOCLKS[LPOCLKSEL]"] _1100,
    #[doc = "For S32K148: QSPI IPG_CLK_SFIF; For others: Reserved"] _1101,
    #[doc = "RTC_CLK as selected by SIM_LPOCLKS[RTCCLKSEL]"] _1110,
    #[doc = "For S32K148: QSPI IPG_CLK_2XSFIF; For others: Reserved"] _1111,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::_0000 => 0,
            CLKOUTSELR::_0010 => 2,
            CLKOUTSELR::_0100 => 4,
            CLKOUTSELR::_0101 => 5,
            CLKOUTSELR::_0110 => 6,
            CLKOUTSELR::_0111 => 7,
            CLKOUTSELR::_1000 => 8,
            CLKOUTSELR::_1001 => 9,
            CLKOUTSELR::_1010 => 10,
            CLKOUTSELR::_1011 => 11,
            CLKOUTSELR::_1100 => 12,
            CLKOUTSELR::_1101 => 13,
            CLKOUTSELR::_1110 => 14,
            CLKOUTSELR::_1111 => 15,
            CLKOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            0 => CLKOUTSELR::_0000,
            2 => CLKOUTSELR::_0010,
            4 => CLKOUTSELR::_0100,
            5 => CLKOUTSELR::_0101,
            6 => CLKOUTSELR::_0110,
            7 => CLKOUTSELR::_0111,
            8 => CLKOUTSELR::_1000,
            9 => CLKOUTSELR::_1001,
            10 => CLKOUTSELR::_1010,
            11 => CLKOUTSELR::_1011,
            12 => CLKOUTSELR::_1100,
            13 => CLKOUTSELR::_1101,
            14 => CLKOUTSELR::_1110,
            15 => CLKOUTSELR::_1111,
            i => CLKOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == CLKOUTSELR::_0000
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == CLKOUTSELR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == CLKOUTSELR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == CLKOUTSELR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == CLKOUTSELR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == CLKOUTSELR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == CLKOUTSELR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == CLKOUTSELR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == CLKOUTSELR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == CLKOUTSELR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == CLKOUTSELR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == CLKOUTSELR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == CLKOUTSELR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == CLKOUTSELR::_1111
    }
}
#[doc = "Possible values of the field `CLKOUTDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTDIVR {
    #[doc = "Divide by 1"] _000,
    #[doc = "Divide by 2"] _001,
    #[doc = "Divide by 3"] _010,
    #[doc = "Divide by 4"] _011,
    #[doc = "Divide by 5"] _100,
    #[doc = "Divide by 6"] _101,
    #[doc = "Divide by 7"] _110,
    #[doc = "Divide by 8"] _111,
}
impl CLKOUTDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTDIVR::_000 => 0,
            CLKOUTDIVR::_001 => 1,
            CLKOUTDIVR::_010 => 2,
            CLKOUTDIVR::_011 => 3,
            CLKOUTDIVR::_100 => 4,
            CLKOUTDIVR::_101 => 5,
            CLKOUTDIVR::_110 => 6,
            CLKOUTDIVR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTDIVR {
        match value {
            0 => CLKOUTDIVR::_000,
            1 => CLKOUTDIVR::_001,
            2 => CLKOUTDIVR::_010,
            3 => CLKOUTDIVR::_011,
            4 => CLKOUTDIVR::_100,
            5 => CLKOUTDIVR::_101,
            6 => CLKOUTDIVR::_110,
            7 => CLKOUTDIVR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTDIVR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == CLKOUTDIVR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTDIVR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTDIVR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTDIVR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTDIVR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTDIVR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTDIVR::_111
    }
}
#[doc = "Possible values of the field `CLKOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTENR {
    #[doc = "Clockout disable"] _0,
    #[doc = "Clockout enable"] _1,
}
impl CLKOUTENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLKOUTENR::_0 => false,
            CLKOUTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKOUTENR {
        match value {
            false => CLKOUTENR::_0,
            true => CLKOUTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKOUTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKOUTENR::_1
    }
}
#[doc = "Possible values of the field `TRACECLK_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLK_SELR {
    #[doc = "Core clock"] _0,
    #[doc = "Platform clock"] _1,
}
impl TRACECLK_SELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRACECLK_SELR::_0 => false,
            TRACECLK_SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRACECLK_SELR {
        match value {
            false => TRACECLK_SELR::_0,
            true => TRACECLK_SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRACECLK_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRACECLK_SELR::_1
    }
}
#[doc = "Possible values of the field `PDB_BB_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDB_BB_SELR {
    #[doc = "PDB0 channel 0 back-to-back operation with ADC0 COCO[7:0] and PDB1 channel 0 back-to-back operation with ADC1 COCO[7:0]"]
    _0,
    #[doc = "Channel 0 of PDB0 and PDB1 back-to-back operation with COCO[7:0] of ADC0 and ADC1."] _1,
}
impl PDB_BB_SELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PDB_BB_SELR::_0 => false,
            PDB_BB_SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDB_BB_SELR {
        match value {
            false => PDB_BB_SELR::_0,
            true => PDB_BB_SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDB_BB_SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDB_BB_SELR::_1
    }
}
#[doc = "Possible values of the field `ADC_SUPPLY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SUPPLYR {
    #[doc = "5 V input VDD supply (VDD)"] _000,
    #[doc = "5 V input analog supply (VDDA)"] _001,
    #[doc = "ADC Reference Supply (VREFH)"] _010,
    #[doc = "3.3 V Oscillator Regulator Output (VDD_3V)"] _011,
    #[doc = "3.3 V flash regulator output (VDD_flash_3V)"] _100,
    #[doc = "1.2 V core regulator output (VDD_LV)"] _101,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl ADC_SUPPLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_SUPPLYR::_000 => 0,
            ADC_SUPPLYR::_001 => 1,
            ADC_SUPPLYR::_010 => 2,
            ADC_SUPPLYR::_011 => 3,
            ADC_SUPPLYR::_100 => 4,
            ADC_SUPPLYR::_101 => 5,
            ADC_SUPPLYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC_SUPPLYR {
        match value {
            0 => ADC_SUPPLYR::_000,
            1 => ADC_SUPPLYR::_001,
            2 => ADC_SUPPLYR::_010,
            3 => ADC_SUPPLYR::_011,
            4 => ADC_SUPPLYR::_100,
            5 => ADC_SUPPLYR::_101,
            i => ADC_SUPPLYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == ADC_SUPPLYR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == ADC_SUPPLYR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == ADC_SUPPLYR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == ADC_SUPPLYR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == ADC_SUPPLYR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == ADC_SUPPLYR::_101
    }
}
#[doc = "Possible values of the field `ADC_SUPPLYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SUPPLYENR {
    #[doc = "Disable internal supply monitoring"] _0,
    #[doc = "Enable internal supply monitoring"] _1,
}
impl ADC_SUPPLYENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADC_SUPPLYENR::_0 => false,
            ADC_SUPPLYENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_SUPPLYENR {
        match value {
            false => ADC_SUPPLYENR::_0,
            true => ADC_SUPPLYENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC_SUPPLYENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC_SUPPLYENR::_1
    }
}
#[doc = "Possible values of the field `SRAMU_RETEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMU_RETENR {
    #[doc = "SRAMU contents are retained across resets"] _0,
    #[doc = "No SRAMU retention"] _1,
}
impl SRAMU_RETENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRAMU_RETENR::_0 => false,
            SRAMU_RETENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAMU_RETENR {
        match value {
            false => SRAMU_RETENR::_0,
            true => SRAMU_RETENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRAMU_RETENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRAMU_RETENR::_1
    }
}
#[doc = "Possible values of the field `SRAML_RETEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAML_RETENR {
    #[doc = "SRAML contents are retained across resets"] _0,
    #[doc = "No SRAML retention"] _1,
}
impl SRAML_RETENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRAML_RETENR::_0 => false,
            SRAML_RETENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAML_RETENR {
        match value {
            false => SRAML_RETENR::_0,
            true => SRAML_RETENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRAML_RETENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRAML_RETENR::_1
    }
}
#[doc = "Values that can be written to the field `ADC_INTERLEAVE_EN`"]
pub enum ADC_INTERLEAVE_ENW {
    #[doc = "Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    _0000,
    #[doc = "PTB14 to ADC1_SE9 and ADC0_SE9"] _1XXX,
    #[doc = "PTB13 to ADC1_SE8 and ADC0_SE8"] X1XX,
    #[doc = "PTB1 to ADC0_SE5 and ADC1_SE15"] XX1X,
    #[doc = "PTB0 to ADC0_SE4 and ADC1_SE14"] XXX1,
}
impl ADC_INTERLEAVE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_INTERLEAVE_ENW::_0000 => 0,
            ADC_INTERLEAVE_ENW::_1XXX => 8,
            ADC_INTERLEAVE_ENW::X1XX => 4,
            ADC_INTERLEAVE_ENW::XX1X => 2,
            ADC_INTERLEAVE_ENW::XXX1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_INTERLEAVE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_INTERLEAVE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_INTERLEAVE_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_ENW::_0000)
    }
    #[doc = "PTB14 to ADC1_SE9 and ADC0_SE9"]
    #[inline]
    pub fn _1xxx(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_ENW::_1XXX)
    }
    #[doc = "PTB13 to ADC1_SE8 and ADC0_SE8"]
    #[inline]
    pub fn x1xx(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_ENW::X1XX)
    }
    #[doc = "PTB1 to ADC0_SE5 and ADC1_SE15"]
    #[inline]
    pub fn xx1x(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_ENW::XX1X)
    }
    #[doc = "PTB0 to ADC0_SE4 and ADC1_SE14"]
    #[inline]
    pub fn xxx1(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_ENW::XXX1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "SCG CLKOUT"] _0000,
    #[doc = "SOSC DIV2 CLK"] _0010,
    #[doc = "SIRC DIV2 CLK"] _0100,
    #[doc = "For S32K148: QSPI SFIF_CLK_HYP: Divide by 2 clock (configured through SCLKCONFIG[5]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    _0101,
    #[doc = "FIRC DIV2 CLK"] _0110,
    #[doc = "HCLK"] _0111,
    #[doc = "SPLL DIV2 CLK"] _1000,
    #[doc = "BUS_CLK"] _1001,
    #[doc = "LPO128K_CLK"] _1010,
    #[doc = "For S32K148: QSPI IPG_CLK; For others: Reserved"] _1011,
    #[doc = "LPO_CLK as selected by SIM_LPOCLKS[LPOCLKSEL]"] _1100,
    #[doc = "For S32K148: QSPI IPG_CLK_SFIF; For others: Reserved"] _1101,
    #[doc = "RTC_CLK as selected by SIM_LPOCLKS[RTCCLKSEL]"] _1110,
    #[doc = "For S32K148: QSPI IPG_CLK_2XSFIF; For others: Reserved"] _1111,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::_0000 => 0,
            CLKOUTSELW::_0010 => 2,
            CLKOUTSELW::_0100 => 4,
            CLKOUTSELW::_0101 => 5,
            CLKOUTSELW::_0110 => 6,
            CLKOUTSELW::_0111 => 7,
            CLKOUTSELW::_1000 => 8,
            CLKOUTSELW::_1001 => 9,
            CLKOUTSELW::_1010 => 10,
            CLKOUTSELW::_1011 => 11,
            CLKOUTSELW::_1100 => 12,
            CLKOUTSELW::_1101 => 13,
            CLKOUTSELW::_1110 => 14,
            CLKOUTSELW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SCG CLKOUT"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0000)
    }
    #[doc = "SOSC DIV2 CLK"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0010)
    }
    #[doc = "SIRC DIV2 CLK"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0100)
    }
    #[doc = "For S32K148: QSPI SFIF_CLK_HYP: Divide by 2 clock (configured through SCLKCONFIG[5]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0101)
    }
    #[doc = "FIRC DIV2 CLK"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0110)
    }
    #[doc = "HCLK"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0111)
    }
    #[doc = "SPLL DIV2 CLK"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1000)
    }
    #[doc = "BUS_CLK"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1001)
    }
    #[doc = "LPO128K_CLK"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1010)
    }
    #[doc = "For S32K148: QSPI IPG_CLK; For others: Reserved"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1011)
    }
    #[doc = "LPO_CLK as selected by SIM_LPOCLKS[LPOCLKSEL]"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1100)
    }
    #[doc = "For S32K148: QSPI IPG_CLK_SFIF; For others: Reserved"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1101)
    }
    #[doc = "RTC_CLK as selected by SIM_LPOCLKS[RTCCLKSEL]"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1110)
    }
    #[doc = "For S32K148: QSPI IPG_CLK_2XSFIF; For others: Reserved"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTDIV`"]
pub enum CLKOUTDIVW {
    #[doc = "Divide by 1"] _000,
    #[doc = "Divide by 2"] _001,
    #[doc = "Divide by 3"] _010,
    #[doc = "Divide by 4"] _011,
    #[doc = "Divide by 5"] _100,
    #[doc = "Divide by 6"] _101,
    #[doc = "Divide by 7"] _110,
    #[doc = "Divide by 8"] _111,
}
impl CLKOUTDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTDIVW::_000 => 0,
            CLKOUTDIVW::_001 => 1,
            CLKOUTDIVW::_010 => 2,
            CLKOUTDIVW::_011 => 3,
            CLKOUTDIVW::_100 => 4,
            CLKOUTDIVW::_101 => 5,
            CLKOUTDIVW::_110 => 6,
            CLKOUTDIVW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_000)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_001)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_011)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_100)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_101)
    }
    #[doc = "Divide by 7"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_110)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTDIVW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKOUTEN`"]
pub enum CLKOUTENW {
    #[doc = "Clockout disable"] _0,
    #[doc = "Clockout enable"] _1,
}
impl CLKOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKOUTENW::_0 => false,
            CLKOUTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clockout disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOUTENW::_0)
    }
    #[doc = "Clockout enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOUTENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRACECLK_SEL`"]
pub enum TRACECLK_SELW {
    #[doc = "Core clock"] _0,
    #[doc = "Platform clock"] _1,
}
impl TRACECLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACECLK_SELW::_0 => false,
            TRACECLK_SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACECLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACECLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACECLK_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Core clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLK_SELW::_0)
    }
    #[doc = "Platform clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLK_SELW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDB_BB_SEL`"]
pub enum PDB_BB_SELW {
    #[doc = "PDB0 channel 0 back-to-back operation with ADC0 COCO[7:0] and PDB1 channel 0 back-to-back operation with ADC1 COCO[7:0]"]
    _0,
    #[doc = "Channel 0 of PDB0 and PDB1 back-to-back operation with COCO[7:0] of ADC0 and ADC1."] _1,
}
impl PDB_BB_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDB_BB_SELW::_0 => false,
            PDB_BB_SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDB_BB_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PDB_BB_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDB_BB_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB0 channel 0 back-to-back operation with ADC0 COCO[7:0] and PDB1 channel 0 back-to-back operation with ADC1 COCO[7:0]"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_BB_SELW::_0)
    }
    #[doc = "Channel 0 of PDB0 and PDB1 back-to-back operation with COCO[7:0] of ADC0 and ADC1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_BB_SELW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_SUPPLY`"]
pub enum ADC_SUPPLYW {
    #[doc = "5 V input VDD supply (VDD)"] _000,
    #[doc = "5 V input analog supply (VDDA)"] _001,
    #[doc = "ADC Reference Supply (VREFH)"] _010,
    #[doc = "3.3 V Oscillator Regulator Output (VDD_3V)"] _011,
    #[doc = "3.3 V flash regulator output (VDD_flash_3V)"] _100,
    #[doc = "1.2 V core regulator output (VDD_LV)"] _101,
}
impl ADC_SUPPLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_SUPPLYW::_000 => 0,
            ADC_SUPPLYW::_001 => 1,
            ADC_SUPPLYW::_010 => 2,
            ADC_SUPPLYW::_011 => 3,
            ADC_SUPPLYW::_100 => 4,
            ADC_SUPPLYW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_SUPPLYW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SUPPLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_SUPPLYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "5 V input VDD supply (VDD)"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADC_SUPPLYW::_000)
    }
    #[doc = "5 V input analog supply (VDDA)"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADC_SUPPLYW::_001)
    }
    #[doc = "ADC Reference Supply (VREFH)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADC_SUPPLYW::_010)
    }
    #[doc = "3.3 V Oscillator Regulator Output (VDD_3V)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADC_SUPPLYW::_011)
    }
    #[doc = "3.3 V flash regulator output (VDD_flash_3V)"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADC_SUPPLYW::_100)
    }
    #[doc = "1.2 V core regulator output (VDD_LV)"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADC_SUPPLYW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_SUPPLYEN`"]
pub enum ADC_SUPPLYENW {
    #[doc = "Disable internal supply monitoring"] _0,
    #[doc = "Enable internal supply monitoring"] _1,
}
impl ADC_SUPPLYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_SUPPLYENW::_0 => false,
            ADC_SUPPLYENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_SUPPLYENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SUPPLYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_SUPPLYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable internal supply monitoring"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_SUPPLYENW::_0)
    }
    #[doc = "Enable internal supply monitoring"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_SUPPLYENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAMU_RETEN`"]
pub enum SRAMU_RETENW {
    #[doc = "SRAMU contents are retained across resets"] _0,
    #[doc = "No SRAMU retention"] _1,
}
impl SRAMU_RETENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAMU_RETENW::_0 => false,
            SRAMU_RETENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMU_RETENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMU_RETENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMU_RETENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRAMU contents are retained across resets"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAMU_RETENW::_0)
    }
    #[doc = "No SRAMU retention"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAMU_RETENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAML_RETEN`"]
pub enum SRAML_RETENW {
    #[doc = "SRAML contents are retained across resets"] _0,
    #[doc = "No SRAML retention"] _1,
}
impl SRAML_RETENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRAML_RETENW::_0 => false,
            SRAML_RETENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAML_RETENW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAML_RETENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAML_RETENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SRAML contents are retained across resets"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAML_RETENW::_0)
    }
    #[doc = "No SRAML retention"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAML_RETENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - ADC interleave channel enable"]
    #[inline]
    pub fn adc_interleave_en(&self) -> ADC_INTERLEAVE_ENR {
        ADC_INTERLEAVE_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - CLKOUT Select"]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - CLKOUT Divide Ratio"]
    #[inline]
    pub fn clkoutdiv(&self) -> CLKOUTDIVR {
        CLKOUTDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - CLKOUT enable"]
    #[inline]
    pub fn clkouten(&self) -> CLKOUTENR {
        CLKOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline]
    pub fn traceclk_sel(&self) -> TRACECLK_SELR {
        TRACECLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - PDB back-to-back select"]
    #[inline]
    pub fn pdb_bb_sel(&self) -> PDB_BB_SELR {
        PDB_BB_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - ADC_SUPPLY"]
    #[inline]
    pub fn adc_supply(&self) -> ADC_SUPPLYR {
        ADC_SUPPLYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - ADC_SUPPLYEN"]
    #[inline]
    pub fn adc_supplyen(&self) -> ADC_SUPPLYENR {
        ADC_SUPPLYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SRAMU_RETEN"]
    #[inline]
    pub fn sramu_reten(&self) -> SRAMU_RETENR {
        SRAMU_RETENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SRAML_RETEN"]
    #[inline]
    pub fn sraml_reten(&self) -> SRAML_RETENR {
        SRAML_RETENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3145728 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - ADC interleave channel enable"]
    #[inline]
    pub fn adc_interleave_en(&mut self) -> _ADC_INTERLEAVE_ENW {
        _ADC_INTERLEAVE_ENW { w: self }
    }
    #[doc = "Bits 4:7 - CLKOUT Select"]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bits 8:10 - CLKOUT Divide Ratio"]
    #[inline]
    pub fn clkoutdiv(&mut self) -> _CLKOUTDIVW {
        _CLKOUTDIVW { w: self }
    }
    #[doc = "Bit 11 - CLKOUT enable"]
    #[inline]
    pub fn clkouten(&mut self) -> _CLKOUTENW {
        _CLKOUTENW { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline]
    pub fn traceclk_sel(&mut self) -> _TRACECLK_SELW {
        _TRACECLK_SELW { w: self }
    }
    #[doc = "Bit 13 - PDB back-to-back select"]
    #[inline]
    pub fn pdb_bb_sel(&mut self) -> _PDB_BB_SELW {
        _PDB_BB_SELW { w: self }
    }
    #[doc = "Bits 16:18 - ADC_SUPPLY"]
    #[inline]
    pub fn adc_supply(&mut self) -> _ADC_SUPPLYW {
        _ADC_SUPPLYW { w: self }
    }
    #[doc = "Bit 19 - ADC_SUPPLYEN"]
    #[inline]
    pub fn adc_supplyen(&mut self) -> _ADC_SUPPLYENW {
        _ADC_SUPPLYENW { w: self }
    }
    #[doc = "Bit 20 - SRAMU_RETEN"]
    #[inline]
    pub fn sramu_reten(&mut self) -> _SRAMU_RETENW {
        _SRAMU_RETENW { w: self }
    }
    #[doc = "Bit 21 - SRAML_RETEN"]
    #[inline]
    pub fn sraml_reten(&mut self) -> _SRAML_RETENW {
        _SRAML_RETENW { w: self }
    }
}
