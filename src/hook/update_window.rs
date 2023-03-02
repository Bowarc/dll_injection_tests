use detour::GenericDetour;
use process_memory::Memory;
use process_memory::TryIntoProcessHandle;

use tracing::{info, warn};

use winapi::shared::minwindef::HINSTANCE__;

use winapi::shared::windef::HWND;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

static mut DETOUR: Option<GenericDetour<extern "system" fn(HWND) -> bool>> = None;

pub unsafe fn create_hook() -> color_eyre::Result<()> {
    let dll = GetModuleHandleA("User32.dll\0".as_ptr() as *const i8);

    let function = GetProcAddress(
        dll as *mut HINSTANCE__,
        "UpdateWindow\0".as_ptr() as *const i8,
    );
    let function: extern "system" fn(HWND) -> bool = std::mem::transmute(function);

    let hook = match GenericDetour::new(function, function_hooked) {
        Ok(hook) => hook,
        Err(e) => {
            warn!("Could not create the hook for function User32.dll::UpdateWindow. {e:?}");
            return Err(e.into());
        }
    };

    match hook.enable() {
        Ok(()) => (),
        Err(e) => {
            warn!("Could not enable hook of User32.dll::UpdateWindow. {e:?}");
            return Err(e.into());
        }
    }
    DETOUR = Some(hook);

    info!("User32.dll::UpdateWindow hook created");

    Ok(())
}

pub extern "system" fn function_hooked(hwnd: HWND) -> bool {
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
