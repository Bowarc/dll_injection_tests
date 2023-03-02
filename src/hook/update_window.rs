use detour::GenericDetour;
use process_memory::Memory;
use process_memory::TryIntoProcessHandle;
use winapi::shared::minwindef::BOOL;

use tracing::{info, warn};

use winapi::shared::minwindef::HINSTANCE__;

use winapi::shared::windef::HWND;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

pub static mut DETOUR: Option<GenericDetour<extern "system" fn(HWND) -> BOOL>> = None;

pub fn setup() {}

pub extern "system" fn function_hooked(hwnd: HWND) -> BOOL {
    // info!("Hooked function has been called with param: {nVirtKey:?}");
    // call the original

    unsafe {
        // let handle = (std::process::id() as process_memory::Pid)
        //     .try_into_process_handle()
        //     .unwrap();
        // let member: process_memory::DataMember<HWND> =
        //     process_memory::DataMember::new_offset(handle, vec![hwnd as *const _ as usize]);
        // info!("try read of HWND: {:?}", member.read().unwrap());

        let res = DETOUR.as_mut().unwrap().call(hwnd);

        info!("UpdateWindow function has been called with param: {hwnd:?} and returned: {res:?}");

        res
    }
}
