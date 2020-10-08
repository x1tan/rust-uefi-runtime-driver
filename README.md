# Rust UEFI Runtime Driver

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/x1tan/rust-uefi-runtime-driver/build)
![GitHub](https://img.shields.io/github/license/x1tan/rust-uefi-runtime-driver)
![Twitter Follow](https://img.shields.io/twitter/follow/x1tan)

> Template for UEFI runtime drivers written in Rust with serial logging and debugging support.

## Overview

This projects serves as a foundation for UEFI runtime driver development in Rust. Its equipped with various features to help you get started:

* Up-to-date Cargo configuration to built a `x86_64-unknown-uefi` target on Rust nightly and link it as a runtime driver
* Logging to serial output
* Debugging support (wait loop until debugger is attached as well as a Python GDB script to locate the PE file in memory)
* Continuous integration with GitHub Actions, minimal dependencies, custom panic handler and more

For more details regarding the implementation and debugging the driver have a look at my [blog post](https://xitan.me/posts/rust-uefi-runtime-driver/). If you are interested in creating a Rust UEFI application e.g. as your bootloader instead of a runtime driver, I recommend reading ["An EFI App a bit rusty"](https://gil0mendes.io/blog/an-efi-app-a-bit-rusty/) for more information about [uefi-rs](https://github.com/rust-osdev/uefi-rs).

## References

- [UEFI Specification](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_8_final.pdf): Unified Extensible Firmware Interface Specification (v2.8).
- [r-efi](https://github.com/r-efi/r-efis): UEFI Reference Specification Protocol Constants and Definitions.
- [uefi-rs](https://github.com/rust-osdev/uefi-rs): Rust wrapper for UEFI.

## License

[MIT](https://github.com/x1tan/rust-uefi-runtime-driver/blob/master/LICENSE) [@x1tan](https://twitter.com/x1tan)