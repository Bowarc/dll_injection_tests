use detour::GenericDetour;

use tracing::{info, warn};
use winapi::shared::minwindef::HINSTANCE__;
use winapi::shared::minwindef::INT;
use winapi::shared::ntdef::SHORT;

// use winapi::um::d3d11::ID3D11CommandList;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;

pub static mut DETOUR: Option<GenericDetour<extern "system" fn(INT) -> SHORT>> = None;

pub fn setup() {}

pub extern "system" fn function_hooked(n_virt_key: INT) -> SHORT {
    unsafe {
        let res = DETOUR.as_mut().unwrap().call(n_virt_key);

        info!(
            "GetKeyState function has been called with param: {n_virt_key:?} and returned: {res:?}"
        );

        res
    }
}
