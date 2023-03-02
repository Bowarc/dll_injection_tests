use std::net::TcpStream;
use std::sync::Mutex;
use tracing::{error, info};

mod hook;

#[ctor::ctor]
fn ctor() {
    println!("Hi i have access to your console ;)");

    let stream = TcpStream::connect("127.0.0.1:7331").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    info!("Hi from the lib");

    println!("yo");

    if let Err(e) = unsafe { do_faillible_stuff() } {
        error!("Could not create hooks: {e:?}");
    }
    info!("Good");

    // if let Err(e) = unsafe { test_memory() } {
    //     error!("Stuff went south: {e:?}");
    // }
}

unsafe fn do_faillible_stuff() -> color_eyre::Result<()> {
    use winapi::{
        shared::{
            minwindef::{BOOL, INT, PBYTE},
            ntdef::SHORT,
            windef::HWND,
        },
        um::winuser::PAINTSTRUCT,
    };

    hook::create_generic_hook::<extern "system" fn(HWND, PAINTSTRUCT) -> BOOL>(
        "User32.dll",
        "EndPaint",
        hook::end_paint::function_hooked,
        &mut hook::end_paint::DETOUR,
        hook::end_paint::setup,
    )?;

    hook::create_generic_hook::<extern "system" fn(HWND) -> BOOL>(
        "User32.dll",
        "UpdateWindow",
        hook::update_window::function_hooked,
        &mut hook::update_window::DETOUR,
        hook::update_window::setup,
    )?;

    hook::create_generic_hook::<extern "system" fn(INT) -> SHORT>(
        "User32.dll",
        "GetKeyState",
        hook::get_key_state::function_hooked,
        &mut hook::get_key_state::DETOUR,
        hook::get_key_state::setup,
    )?;

    hook::create_generic_hook::<extern "system" fn(PBYTE) -> BOOL>(
        "User32.dll",
        "GetKeyboardState",
        hook::get_keyboard_state::function_hooked,
        &mut hook::get_keyboard_state::DETOUR, /* setup_function */
        hook::get_keyboard_state::setup,
    )?;

    Ok(())
}

unsafe fn test_memory() -> color_eyre::Result<()> {
    use process_memory::Memory;
    use process_memory::TryIntoProcessHandle;

    // let dm: process_memory::DataMember<u8> = process_memory::DataMember::new(
    //     (std::process::id() as process_memory::Pid).try_into_process_handle()?,
    // );

    info!("Pid: {}", std::process::id());
    info!("Backtrace: {:#?}", std::backtrace::Backtrace::capture());

    // info!("Process offset is: {:?}", dm.get_offset());

    Ok(())
}
