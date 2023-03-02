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
            minwindef::{BOOL, PBYTE},
            windef::HWND,
        },
        um::winuser::PAINTSTRUCT,
    };

    hook::end_paint::create_hook()?;
    // fn() -> BOOL
    hook::create_generic_hook_2arg::<HWND, PAINTSTRUCT, BOOL>(
        "User32.dll",
        "EndPaint",
        hook::end_paint::function_hooked,
        &mut hook::end_paint::DETOUR,
        hook::end_paint::setup,
    )?;

    // hook::finish_command_list::create_hook()?;
    // hook::update_window::create_hook()?;

    hook::create_generic_hook_1arg::<PBYTE, BOOL>(
        "User32.dll",
        "GetKeyboardState",
        hook::get_keyboard_state::function_hooked,
        &mut hook::get_keyboard_state::DETOUR, /* setup_function */
        hook::get_keyboard_state::setup,
    )?;
    // hook::get_keyboard_state::create_hook()?;

    // hook::get_key_state::create_hook()?;

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
