use detour::GenericDetour;

use winapi::shared::basetsd::PSIZE_T;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::ntdef::NTSTATUS;
use winapi::shared::ntdef::PVOID;
use winapi::um::memoryapi::WIN32_MEMORY_INFORMATION_CLASS;

use tracing::{info, warn};

use winapi::shared::ntdef::HANDLE;

pub static mut DETOUR: Option<
    GenericDetour<
        extern "system" fn(
            HANDLE,
            PVOID,
            WIN32_MEMORY_INFORMATION_CLASS,
            PVOID,
            SIZE_T,
            PSIZE_T,
        ) -> NTSTATUS,
    >,
> = None;

pub fn setup() {}

pub extern "system" fn function_hooked(
    process_handle: HANDLE,
    base_address: PVOID,
    memory_information_class: WIN32_MEMORY_INFORMATION_CLASS,
    memory_information: PVOID,
    memory_information_length: SIZE_T,
    return_length: PSIZE_T,
) -> NTSTATUS {
    // info!("Hooked function has been called with param: {nVirtKey:?}");
    // call the original

    unsafe {
        // let handle = (std::process::id() as process_memory::Pid)
        //     .try_into_process_handle()
        //     .unwrap();
        // let member: process_memory::DataMember<HWND> =
        //     process_memory::DataMember::new_offset(handle, vec![hwnd as *const _ as usize]);
        // info!("try read of HWND: {:?}", member.read().unwrap());

        let res = DETOUR.as_mut().unwrap().call(
            process_handle,
            base_address,
            memory_information_class,
            memory_information,
            memory_information_length,
            return_length,
        );

        info!("ZwQueryVirtualMemory function has been called with params: handle: {process_handle:?}, address: {base_address:?}, mem info class: {memory_information_class:?}, mem info: {memory_information:?}, mem info length: {memory_information_length:?} return length: {return_length:?} and returned: {res:?}");

        res
    }
}
