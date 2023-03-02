use detour::GenericDetour;
use process_memory::Memory;
use process_memory::TryIntoProcessHandle;
use winapi::shared::minwindef::HINSTANCE__;
use winapi::shared::ntdef::SHORT;

use tracing::info;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::INT;
use winapi::shared::minwindef::PBYTE;

use winapi::shared::ntdef::HRESULT;

// use winapi::um::d3d11::ID3D11CommandList;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

static mut DETOUR: Option<GenericDetour<extern "system" fn(INT) -> SHORT>> = None;

pub unsafe fn create_hook() -> color_eyre::Result<()> {
    let dll = GetModuleHandleA("User32.dll\0".as_ptr() as *const i8);

    let function = GetProcAddress(
        dll as *mut HINSTANCE__,
        "GetKeyState\0".as_ptr() as *const i8,
    );
    // info!("We found the GetKeyboardState  function !");
    let function: extern "system" fn(INT) -> SHORT = std::mem::transmute(function);
    let hook = GenericDetour::new(function, function_hooked)?;
    hook.enable()?;
    DETOUR = Some(hook);

    info!("GetKeyState hook created");

    Ok(())
}

extern "system" fn function_hooked(n_virt_key: INT) -> SHORT {
    unsafe {
        let res = DETOUR.as_mut().unwrap().call(n_virt_key);

        info!(
            "GetKeyState function has been called with param: {n_virt_key:?} and returned: {res:?}"
        );

        res
    }
}
