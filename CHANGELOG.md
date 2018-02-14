# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
### Changed
### Removed
## [0.9.0] - 2018-02-14
### Removed
 - Debug assertion in Peripherals::steal() is removed so peripherals can be stolen in panic_fmt.
## [0.8.0] - 2018-02-13
### Removed
 - PCC dummy register was removed. These register should not be used as there are no hardware corresponding to them.
