use detour::GenericDetour;
use tracing::info;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::HINSTANCE__;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::libloaderapi::GetProcAddress;
use winapi::um::winuser::PAINTSTRUCT;

static mut DETOUR: Option<GenericDetour<extern "system" fn(HWND, PAINTSTRUCT) -> BOOL>> = None;

pub unsafe fn create_hook() -> color_eyre::Result<()> {
    let dll = GetModuleHandleA("User32.dll\0".as_ptr() as *const i8);

    let function = GetProcAddress(dll as *mut HINSTANCE__, "EndPaint\0".as_ptr() as *const i8);
    info!("We found the EndPaint function !");
    let function: extern "system" fn(HWND, PAINTSTRUCT) -> BOOL = std::mem::transmute(function);
    let hook = GenericDetour::new(function, function_hooked)?;
    hook.enable()?;
    DETOUR = Some(hook);

    info!("EndPaint hook created");

    Ok(())
}

extern "system" fn function_hooked(h_wnd: HWND, lp_paint: PAINTSTRUCT) -> BOOL {
    // info!("Hooked function has been called with param: {nVirtKey:?}");
    // call the original

    unsafe {
        let res = DETOUR.as_mut().unwrap().call(h_wnd, lp_paint);

        info!(
            "EndPaint function has been called with param: {h_wnd:?}, {:?} and returned: {res:?}",
            dbg_lppaint(lp_paint)
        );

        res
    }
}

fn dbg_lppaint(lp_paint: PAINTSTRUCT) -> String {
    format!(
        "hdc: {:?}, fErase: {:?}, rcPaint: {{left: {} top: {} right: {} bottom: {}}}, fRestore: {}, fIncUpdate: {}, rgbReserved: {:?}",
        lp_paint.hdc,
        lp_paint.fErase,
        lp_paint.rcPaint.left,
        lp_paint.rcPaint.top,
        lp_paint.rcPaint.right,
        lp_paint.rcPaint.bottom,
        lp_paint.fRestore,
        lp_paint.fIncUpdate,
        lp_paint.rgbReserved
    )
}
