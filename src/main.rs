#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(panic_info_message)]

#[macro_use]
mod logger;
mod utils;

use core::borrow::BorrowMut;
use core::mem::MaybeUninit;

use log::{error, info};
use r_efi::*;

static mut SYSTEM_TABLE: MaybeUninit<efi::SystemTable> = MaybeUninit::uninit();

pub fn system_table() -> &'static efi::SystemTable {
    unsafe { &*SYSTEM_TABLE.as_ptr() }
}

pub fn runtime_services() -> &'static efi::RuntimeServices {
    unsafe { &*system_table().runtime_services }
}

pub fn boot_services() -> &'static efi::BootServices {
    unsafe { &*system_table().boot_services }
}

extern "win64" fn handle_exit_boot_services(_event: base::Event, _context: *mut core::ffi::c_void) {
    info!("[~] ExitBootServices() has been called.");
}

extern "win64" fn handle_set_virtual_address_map(
    _event: base::Event,
    _context: *mut core::ffi::c_void,
) {
    info!("[~] SetVirtualAddressMap() has been called.");
}

#[no_mangle]
fn efi_main(_image_handle: efi::Handle, raw_system_table: *mut efi::SystemTable) -> efi::Status {
    #[cfg(debug_assertions)]
    {
        utils::wait_for_debugger();
    }

    unsafe { SYSTEM_TABLE = MaybeUninit::new(raw_system_table.read()) };

    // Setup the serial port logger.
    logger::Logger::initialize();

    // Register to events relevant for runtime drivers.
    let mut event: base::Event = core::ptr::null_mut();

    let mut status = (boot_services().create_event_ex)(
        efi::EVT_NOTIFY_SIGNAL,
        efi::TPL_CALLBACK,
        handle_set_virtual_address_map,
        runtime_services().borrow_mut() as *mut _ as *mut core::ffi::c_void,
        &efi::EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE,
        event.borrow_mut(),
    );

    if status.is_error() {
        error!(
            "[-] Creating VIRTUAL_ADDRESS_CHANGE event failed: {:#x}",
            status.as_usize()
        );
        return status;
    }

    status = (boot_services().create_event_ex)(
        efi::EVT_NOTIFY_SIGNAL,
        efi::TPL_CALLBACK,
        handle_exit_boot_services,
        runtime_services().borrow_mut() as *mut _ as *mut core::ffi::c_void,
        &efi::EVENT_GROUP_EXIT_BOOT_SERVICES,
        event.borrow_mut(),
    );

    if status.is_error() {
        error!(
            "[-] Creating EXIT_BOOT_SERVICES event failed: {:#x}",
            status.as_usize()
        );
        return status;
    }

    info!("[~] EFI runtime driver has been loaded and initialized.");

    efi::Status::SUCCESS
}
