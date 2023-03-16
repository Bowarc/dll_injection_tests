use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

use dll_syringe::{process::OwnedProcess, Syringe};
use tracing::{error, info, metadata::LevelFilter};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    info!("Injector start");

    let target_process_name = "basic_template";

    let target_process = OwnedProcess::find_first_by_name(target_process_name).unwrap();

    info!("Found the {target_process_name} process");

    let syringe = Syringe::for_process(target_process);

    info!("Created syringe");

    let listener = TcpListener::bind("127.0.0.1:7331")?;

    let dll_path = "target/debug/alfred.dll";

    let injected_payload = syringe.inject(dll_path).unwrap();
    info!("Injected in {target_process_name}");

    let (mut stream, addr) = listener.accept()?;
    info!("{addr} accepted");

    let mut buffer = vec![0u8; 1024];
    let mut stdout = std::io::stdout();

    while let Ok(n) = stream.read(&mut buffer[..]) {
        stdout.write_all(&buffer[..n])?;
        if std::str::from_utf8(&buffer).unwrap().contains("ERROR") {
            error!("A lib crash has been detected, uninjecting and exiting");
            syringe.eject(injected_payload).unwrap();
            break;
        }
    }

    Ok(())
}
