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

    // if let Err(e) = unsafe { do_faillible_stuff() } {
    //     error!("Could not create hooks: {e:?}");
    // }
    info!("Good");

    if let Err(e) = unsafe { test_memory() } {
        error!("Stuff went south: {e:?}");
    }
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

    // hook::create_generic_hook::<extern "system" fn(INT) -> SHORT>(
    //     "User32.dll",
    //     "GetKeyState",
    //     hook::get_key_state::function_hooked,
    //     &mut hook::get_key_state::DETOUR,
    //     hook::get_key_state::setup,
    // )?;

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
    let addr = 0x1bfec586ac8;
    read_memory::<u32>(addr).unwrap();

    write_memory::<u32>(addr, 101).unwrap();

    Ok(())
}

pub fn read_memory<T>(address: usize) -> color_eyre::Result<T>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    use process_memory::Memory;

    let lm: process_memory::LocalMember<T> = process_memory::LocalMember::new_offset(vec![address]);

    // let res: &T = unsafe { &*(address as *const T) };
    let res = unsafe { lm.read()? };
    info!("Read: {:?} at address: 0x{:x}", res, address);

    Ok(res)
}

pub fn write_memory<T>(address: usize, value: T) -> color_eyre::Result<()>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    use process_memory::Memory;

    info!("Writing {:?} at address: 0x{:x}", value, address);
    let lm: process_memory::LocalMember<T> = process_memory::LocalMember::new_offset(vec![address]);

    lm.write(&value).unwrap();

    Ok(())
}
