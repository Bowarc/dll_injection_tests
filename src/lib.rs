use address::*;
use std::net::TcpStream;
use std::sync::Mutex;
use tracing::{error, info, warn};
mod address;
mod hook;

#[ctor::ctor]
fn ctor() {
    println!("Hi i have access to your console ;)");

    let stream = TcpStream::connect("127.0.0.1:7331").unwrap();

    tracing_subscriber::fmt()
        .with_writer(Mutex::new(stream))
        .init();

    info!("Hi from the lib");

    if let Err(e) = unsafe { do_faillible_stuff() } {
        warn!("Could not create hooks: {e:?}");
    } else {
        info!("Hooks created");
    }

    if let Err(e) = unsafe { test_memory() } {
        warn!("Stuff went south: {e:?}");
    }
}

unsafe fn do_faillible_stuff() -> color_eyre::Result<()> {
    use winapi::{
        shared::{
            basetsd::{PSIZE_T, SIZE_T},
            minwindef::{BOOL, INT, PBYTE},
            ntdef::{HANDLE, NTSTATUS, PVOID, SHORT},
            windef::HWND,
        },
        um::{memoryapi::WIN32_MEMORY_INFORMATION_CLASS, winuser::PAINTSTRUCT},
    };

    // This might have potential, diddn't digged for now
    // hook::create_generic_hook::<extern "system" fn(HWND, PAINTSTRUCT) -> BOOL>(
    //     "User32.dll",
    //     "EndPaint",
    //     hook::end_paint::function_hooked,
    //     &mut hook::end_paint::DETOUR,
    //     hook::end_paint::setup,
    // )?;

    // This has potential, not digged for now tho
    // hook::create_generic_hook::<extern "system" fn(HWND) -> BOOL>(
    //     "User32.dll",
    //     "UpdateWindow",
    //     hook::update_window::function_hooked,
    //     hook::update_window::setup,
    // )?;

    // This is kinda buggy (might be the fact that i not understand it clearly)
    // hook::create_generic_hook::<extern "system" fn(INT) -> SHORT>(
    //     "User32.dll",
    //     "GetKeyState",
    //     hook::get_key_state::function_hooked,
    //     &mut hook::get_key_state::DETOUR,
    //     hook::get_key_state::setup,
    // )?;

    // This is cool and usefull, well, nvm, modifying the array doesn't do sht
    hook::create_generic_hook::<extern "system" fn(PBYTE) -> BOOL>(
        "User32.dll",
        "GetKeyboardState",
        hook::get_keyboard_state::function_hooked,
        &mut hook::get_keyboard_state::DETOUR, /* setup_function */
        hook::get_keyboard_state::setup,
    )?;

    // This has insane potential
    // hook::create_generic_hook::<
    //     extern "system" fn(
    //         HANDLE,
    //         PVOID,
    //         WIN32_MEMORY_INFORMATION_CLASS,
    //         PVOID,
    //         SIZE_T,
    //         PSIZE_T,
    //     ) -> NTSTATUS,
    // >(
    //     "ntdll.dll",
    //     "ZwQueryVirtualMemory",
    //     hook::zw_query_virtual_memory::function_hooked,
    //     &mut hook::zw_query_virtual_memory::DETOUR,
    //     hook::zw_query_virtual_memory::setup,
    // )?;

    Ok(())
}

unsafe fn test_memory() -> color_eyre::Result<()> {
    let addr: Address = 0x27e230791d8.into();

    read_memory::<u32>(addr).unwrap();

    write_memory::<u32>(addr, 745_000).unwrap();

    Ok(())
}

pub fn read_memory<T>(address: Address) -> color_eyre::Result<T>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    use process_memory::Memory;

    let lm: process_memory::LocalMember<T> =
        process_memory::LocalMember::new_offset(vec![*address]);

    // let res: &T = unsafe { &*(address as *const T) };
    let res = unsafe { lm.read()? };
    info!("Reading {:?} at address: {}", res, address);

    Ok(res)
}

pub fn write_memory<T>(address: Address, value: T) -> color_eyre::Result<()>
where
    T: std::marker::Copy + std::fmt::Debug,
{
    use process_memory::Memory;

    info!("Writing {:?} at address: {}", value, address);
    let lm: process_memory::LocalMember<T> =
        process_memory::LocalMember::new_offset(vec![*address]);

    lm.write(&value).unwrap();

    Ok(())
}
