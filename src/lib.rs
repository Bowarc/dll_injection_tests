use detour::GenericDetour;

use std::net::TcpStream;
use std::sync::Mutex;
use tracing::{error, info};
use winapi::shared::basetsd::ULONG_PTR;

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
        error!("Could not create hooks: {e:?}");
    }
    info!("Good");

    if let Err(e) = unsafe { test_memory() } {
        error!("Stuff went south: {e:?}");
    }
}

unsafe fn do_faillible_stuff() -> color_eyre::Result<()> {
    // hook::end_paint::create_hook();
    // hook::finish_command_list::create_hook();
    // hook::update_window::create_hook()?;

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
