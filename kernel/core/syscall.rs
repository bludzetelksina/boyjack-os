#![no_std]

use crate::core::{process::exit_process, ipc::Message};
use crate::scheduler::yield_now;
use crate::userland::syscall_id::SyscallId;

/// Тип значения, возвращаемого системным вызовом
pub type SyscallResult = u64;

/// Главный обработчик системных вызовов
pub fn handle_syscall(syscall_id: usize, arg1: usize, arg2: usize, arg3: usize) -> SyscallResult {
    match syscall_id {
        x if x == SyscallId::Exit as usize => {
            exit_process(arg1 as i32);
            0
        }

        x if x == SyscallId::Yield as usize => {
            yield_now();
            0
        }

        x if x == SyscallId::Write as usize => {
            let ptr = arg1 as *const u8;
            let len = arg2;
            unsafe {
                for i in 0..len {
                    let byte = *ptr.add(i);
                    crate::drivers::tty::print_char(byte as char);
                }
            }
            len as u64
        }

        x if x == SyscallId::SendMessage as usize => {
            let msg = Message::Data(arg1 as u64);
            crate::ipc::send_to_pid(arg2 as u32, msg);
            0
        }

        x if x == SyscallId::ReceiveMessage as usize => {
            match crate::ipc::recv_from_pid(arg1 as u32) {
                Some(msg) => match msg {
                    Message::Data(data) => data,
                    _ => 0,
                },
                None => u64::MAX,
            }
        }

        _ => {
            crate::drivers::tty::print_str("Unknown syscall\n");
            u64::MAX
        }
    }
}
