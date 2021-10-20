#![no_std]
#![no_main]

#[macro_use]
extern crate nx;

#[macro_use]
extern crate alloc;

extern crate paste;

use nx::diag::assert;
use nx::diag::log;
use nx::ipc::server;
use nx::ipc::sf;
use nx::result::*;
use nx::service::sm;
use nx::util;

use core::panic;

mod types;

pub trait ILdnServer {
    ipc_cmif_interface_define_command!(get_state: () => (state: u32));
    ipc_cmif_interface_define_command!(get_network_info: () => (buffer: types::NetworkInfo));
    ipc_cmif_interface_define_command!(get_ipv4_address: () => (address: types::Ipv4Address, mask: u32));
    ipc_cmif_interface_define_command!(get_disconnect_reason: () => (reason: u32));
}

pub struct LdnServer {
    session: sf::Session,
}

impl sf::IObject for LdnServer {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec![
            ipc_cmif_interface_make_command_meta!(get_state: 0),
            ipc_cmif_interface_make_command_meta!(get_network_info: 1),
            ipc_cmif_interface_make_command_meta!(get_ipv4_address: 2),
            ipc_cmif_interface_make_command_meta!(get_disconnect_reason: 3),
        ]
    }
}

impl ILdnServer for LdnServer {
    fn get_state(&mut self) -> Result<u32> {
        let stub: u32 = 69;
        diag_log!(log::LmLogger { log::LogSeverity::Trace, true } => "Returning stubbed battery percentage as {}%...\n", stub);
        Ok(stub)
    }
    fn get_network_info(&mut self) -> Result<types::NetworkInfo> {
        todo!()
    }
    fn get_ipv4_address(&mut self) -> Result<(types::Ipv4Address, u32)> {
        todo!()
    }
    fn get_disconnect_reason(&mut self) -> Result<u32> {
        todo!()
    }
}

impl server::IMitmServerObject for LdnServer {
    fn new(_info: sm::MitmProcessInfo) -> Self {
        Self {
            session: sf::Session::new(),
        }
    }
}

impl server::IMitmService for LdnServer {
    fn get_name() -> &'static str {
        nul!("ldn:u")
    }

    fn should_mitm(_info: sm::MitmProcessInfo) -> bool {
        true
    }
}

pub const STACK_HEAP_SIZE: usize = 0x4000;
static mut STACK_HEAP: [u8; STACK_HEAP_SIZE] = [0; STACK_HEAP_SIZE];

#[no_mangle]
pub fn initialize_heap(_hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    unsafe { util::PointerAndSize::new(STACK_HEAP.as_mut_ptr(), STACK_HEAP.len()) }
}

const POINTER_BUF_SIZE: usize = 0;
type Manager = server::ServerManager<POINTER_BUF_SIZE>;

#[no_mangle]
pub fn main() -> Result<()> {
    diag_log!(log::LmLogger { log::LogSeverity::Trace, true } => "ldn_mitm start!\n");

    let mut manager = Manager::new()?;
    manager.register_mitm_service_server::<LdnServer>()?;
    manager.loop_process()?;

    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertLevel::FatalThrow())
}
