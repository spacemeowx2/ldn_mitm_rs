#![no_std]
#![no_main]

#[macro_use]
extern crate nx;

#[macro_use]
extern crate alloc;

extern crate paste;

use nx::diag::assert;
use nx::diag::log;
use nx::fs;
use nx::ipc::server;
use nx::ipc::sf;
use nx::mem;
use nx::result::*;
use nx::service::sm;
use nx::thread;
use nx::util;

use core::panic;

mod fsext;
mod logger;
mod types;

pub trait ILdnServer {
    ipc_cmif_interface_define_command!(create_user_local_communication_service: () => (out: mem::Shared<dyn sf::IObject>));
}

pub struct LdnServer {
    session: sf::Session,
}

impl sf::IObject for LdnServer {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec![ipc_cmif_interface_make_command_meta!(create_user_local_communication_service: 0)]
    }
}

impl ILdnServer for LdnServer {
    fn create_user_local_communication_service(&mut self) -> Result<mem::Shared<dyn sf::IObject>> {
        logger::log_line("create_user_local_communication_service");
        Err(ResultCode::new(0x1234))
    }
    // fn get_network_info(&mut self) -> Result<types::NetworkInfo> {
    //     todo!()
    // }
    // fn get_ipv4_address(&mut self) -> Result<(types::Ipv4Address, u32)> {
    //     todo!()
    // }
    // fn get_disconnect_reason(&mut self) -> Result<u32> {
    //     todo!()
    // }
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

const POINTER_BUF_SIZE: usize = 0x1000;
type Manager = server::ServerManager<POINTER_BUF_SIZE>;

#[no_mangle]
pub fn main() -> Result<()> {
    thread::get_current_thread().name.set_str("ldn_mitm.Main")?;

    fs::initialize()?;
    fs::mount_sd_card("sdmc")?;
    fsext::ensure_directories();
    logger::initialize();

    logger::log_line("start");

    let mut manager = Manager::new()?;
    manager.register_mitm_service_server::<LdnServer>()?;

    logger::log_line("loop_process");
    manager.loop_process()?;

    Ok(())
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<log::LmLogger>(info, assert::AssertLevel::FatalThrow())
}
