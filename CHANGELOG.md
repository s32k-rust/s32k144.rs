# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.11.0] - 2020-05-01
### Added
 - Added CAN1 and CAN2 peripherals as well as LPUART1 and LPUART2 to .svd file.
### Changed
 - Regenerated crate with svd2rust v0.17.0.
 - Bumped dependencies.
 - Modified CSE_PRAM RegisterBlock. The EmbeddedRAM is now a word addressable array.
 - Modified DMAMUX Channel Configuration. The Channel Configuration registers are now elements of a Register Array and can be accessed by indexing with Channel Number.
 - Modified ADC0 and ADC1 Status and Control Register 1. SC1 is now an array, where the channels can be accessed by indexing.
 - Modified ADC0 and ADC1 Data Result Registers. The Data Result Registers are now elements of a Register Array and can be accessed by indexing.
### Removed
## [0.10.0] - 2019-01-17
### Changed
 - Regenerated crate with svd2rust v0.14.0.
 - Bumped dependencies.
## [0.9.0] - 2018-02-14
### Removed
 - Debug assertion in Peripherals::steal() is removed so peripherals can be stolen in panic_fmt.
## [0.8.0] - 2018-02-13
### Removed
 - PCC dummy register was removed. These register should not be used as there are no hardware corresponding to them.
