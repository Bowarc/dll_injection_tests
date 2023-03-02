use detour::GenericDetour;

use tracing::info;

use winapi::shared::minwindef::HINSTANCE__;

use winapi::shared::ntdef::HRESULT;

// use winapi::um::d3d11::ID3D11CommandList;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

static mut DETOUR: Option<GenericDetour<extern "system" fn(bool, Vec<String>) -> HRESULT>> = None;

pub unsafe fn create_hook() -> color_eyre::Result<()> {
    let dll = GetModuleHandleA("D3d11.dll\0".as_ptr() as *const i8);

    let function = GetProcAddress(
        dll as *mut HINSTANCE__,
        "FinishCommandList\0".as_ptr() as *const i8,
    );
    info!("We found the FinishCommandList function !");
    let function: extern "system" fn(bool, Vec<String>) -> HRESULT = std::mem::transmute(function);
    let hook = GenericDetour::new(function, function_hooked)?;
    hook.enable()?;
    DETOUR = Some(hook);

    info!("FinishCommandList hook created");

    Ok(())
}

extern "system" fn function_hooked(
    restore_deferred_context_state: bool,
    pp_command_list: Vec<String>,
) -> HRESULT {
    // info!("Hooked function has been called with param: {nVirtKey:?}");
    // call the original

    unsafe {
        let res = DETOUR
            .as_mut()
            .unwrap()
            .call(restore_deferred_context_state, pp_command_list.clone());

        info!("FinishCommandList function has been called with param: {restore_deferred_context_state:?}, {pp_command_list:?} and returned: {res:?}");

        res
    }
}
