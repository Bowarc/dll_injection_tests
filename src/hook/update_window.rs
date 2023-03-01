use detour::GenericDetour;

use tracing::info;

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
    info!("We found the UpdateWindow function !");
    let function: extern "system" fn(HWND) -> bool = std::mem::transmute(function);
    let hook = GenericDetour::new(function, function_hooked)?;
    hook.enable()?;
    DETOUR = Some(hook);

    info!("UpdateWindow hook created");

    Ok(())
}

extern "system" fn function_hooked(h_wnd: HWND) -> bool {
    // info!("Hooked function has been called with param: {nVirtKey:?}");
    // call the original

    unsafe {
        let res = DETOUR.as_mut().unwrap().call(h_wnd);

        info!("UpdateWindow function has been called with param: {h_wnd:?} and returned: {res:?}");

        res
    }
}
