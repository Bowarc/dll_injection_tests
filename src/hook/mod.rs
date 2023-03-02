pub mod end_paint;
pub mod finish_command_list;
pub mod get_key_state;
pub mod get_keyboard_state;
pub mod update_window;

use detour::GenericDetour;

use tracing::warn;

use winapi::shared::minwindef::HINSTANCE__;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

pub unsafe fn create_generic_hook_1arg<I: 'static, O: 'static>(
    dll_name: &str,
    function_name: &str,
    hooked_function: extern "system" fn(I) -> O,
    detour_const: &mut Option<GenericDetour<extern "system" fn(I) -> O>>,
    setup_function: impl Fn(),
) -> color_eyre::Result<()> {
    let dll = GetModuleHandleA(format!("{}\0", dll_name).as_ptr() as *const i8);

    let function = GetProcAddress(
        dll as *mut HINSTANCE__,
        format!("{}\0", function_name).as_ptr() as *const i8,
    );

    let function: extern "system" fn(I) -> O = std::mem::transmute(function);

    let hook = match GenericDetour::new(function, hooked_function) {
        Ok(hook) => hook,
        Err(e) => {
            warn!(
                "Could not create the hook for function {}::{}. {e:?}",
                dll_name, function_name
            );
            return Err(e.into());
        }
    };
    match hook.enable() {
        Ok(()) => (),
        Err(e) => {
            warn!(
                "Could not enable hook of {}::{}. {e:?}",
                dll_name, function_name
            );
            return Err(e.into());
        }
    }
    *detour_const = Some(hook);

    setup_function();

    Ok(())
}

pub unsafe fn create_generic_hook_2arg<I: 'static, J: 'static, O: 'static>(
    dll_name: &str,
    function_name: &str,
    hooked_function: extern "system" fn(I, J) -> O,
    detour_const: &mut Option<GenericDetour<extern "system" fn(I, J) -> O>>,
    setup_function: impl Fn(),
) -> color_eyre::Result<()> {
    let dll = GetModuleHandleA(format!("{}\0", dll_name).as_ptr() as *const i8);

    let function = GetProcAddress(
        dll as *mut HINSTANCE__,
        format!("{}\0", function_name).as_ptr() as *const i8,
    );

    let function: extern "system" fn(I, J) -> O = std::mem::transmute(function);

    let hook = match GenericDetour::new(function, hooked_function) {
        Ok(hook) => hook,
        Err(e) => {
            warn!(
                "Could not create the hook for function {}::{}. {e:?}",
                dll_name, function_name
            );
            return Err(e.into());
        }
    };
    match hook.enable() {
        Ok(()) => (),
        Err(e) => {
            warn!(
                "Could not enable hook of {}::{}. {e:?}",
                dll_name, function_name
            );
            return Err(e.into());
        }
    }
    *detour_const = Some(hook);

    setup_function();

    Ok(())
}
