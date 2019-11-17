[![Build Status](https://travis-ci.org/inferiorhumanorgans/sam3x8e-rs.svg?branch=master)](https://travis-ci.org/inferiorhumanorgans/sam3x8e-rs)

SAM3X8E
===
This repository contains crates to support rust on the [Atmel SAM3X8E](https://www.microchip.com/wwwproducts/en/ATSAM3X8E) and related microcontrollers in the SAM3X/SAM3A family.

What is the SAM3X8E?
---

The Atmel SAM3x8e is an ARM Cortex M3 based microcontroller.  It includes 512 kilobytes of flash memory, 100 kilobytes of SDRAM, and can be run at up to 84 MHz.  The SAM3X8E is most commonly found in the Arduino Due.

What's included in the box?
---

The included crates are designed with the interfaces and goals of the rust-embedded team in mind.  As the [Embedded Rust Book](https://rust-embedded.github.io/book/) states:

> Embedded Rust is for everyone who wants to do embedded programming while taking advantage of the higher-level concepts and safety guarantees the Rust language provides


The following crates are included:

- `sam3x8e` a [peripheral access crate](https://rust-embedded.github.io/book/start/registers.html)
- `sam3x8e-hal` a [hardware abstraction crate](https://docs.rs/embedded-hal/0.2.3/embedded_hal/)
- `sam3x8e-hal-codegen` a proc macro crate which generates code for traits that require a significant amount of boilerplate (e.g. GPIO)

Using the `sam3x8e-hal` crate should allow code to be more easily ported between different microcontrollers, for example moving from a SAM3X board to an STM32 based board.

What's the catch?
---

There's no catch.  All code is available freely under the GNU Lesser GPL version 3.0 or later.

How do I get started?
---

The [`cortex-m-quickstart`](https://github.com/rust-embedded/cortex-m-quickstart) template is a good place to start.  For a SAM3X8E microcontroller, the correct architecture is `thumbv7m-none-eabi` as this is a Cortex M3 based MCU.  The linker script at the root of this crate should work.

Deployment is typically going to involve some board specific actions.  For instance the Arduino IDE uses [`bossac`](https://github.com/shumatech/BOSSA) to deploy to the Due.

`bossac` (and likely other deployment tools) use an unstructured binary file as input, however `rustc` will generate an ELF file.   To generate the proper unstructured file, objcopy from GNU binutils can be used like so:

`arm-none-eabi-objcopy -O binary {IN_FILE} {OUT_FILE}`
