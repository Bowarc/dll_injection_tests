pub mod end_paint;
pub mod finish_command_list;
pub mod get_key_state;
pub mod get_keyboard_state;
pub mod update_window;
pub mod zw_query_virtual_memory;

use detour::GenericDetour;
use tracing::{info, warn};
use winapi::shared::minwindef::HINSTANCE__;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

pub unsafe fn create_generic_hook<F: detour::Function>(
    dll_name: &str,
    function_name: &str,
    hooked_function: F,
    detour_const: &mut Option<GenericDetour<F>>,
    setup_function: impl Fn(),
) -> color_eyre::Result<()> {
    let dll = GetModuleHandleA(format!("{}\0", dll_name).as_ptr() as *const i8);

    let function = GetProcAddress(
        dll as *mut HINSTANCE__,
        format!("{}\0", function_name).as_ptr() as *const i8,
    );

    // let function: extern "system" fn(I) -> O = ;

    let hook = match GenericDetour::new(std::mem::transmute_copy(&function), hooked_function) {
        Ok(hook) => {
            info!(
                "Successfully created a hook for {}::{}",
                dll_name, function_name
            );
            hook
        }
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
