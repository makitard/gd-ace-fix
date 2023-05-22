#![feature(abi_thiscall)]

use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32;
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
mod hooks;

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "system" fn DllMain(
    _hModule: *const (),
    fdwReason: u32,
    _lpReserved: *const (),
) -> u32 {
    if fdwReason == 1 {
        windows::Win32::System::Threading::CreateThread(
            None,
            0,
            Some(entry),
            None,
            windows::Win32::System::Threading::THREAD_CREATION_FLAGS(0),
            None,
        )
        .unwrap();
    }

    1
}

unsafe extern "system" fn entry(_: *mut core::ffi::c_void) -> u32 {
    hooks::init().unwrap();

    0
}
