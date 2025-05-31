#![no_std]

use core::ptr::null_mut;

pub type ProcessId = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

#[repr(C)]
pub struct Context {
    // Регистры, которые сохраняются при переключении процесса
    pub rip: usize,   // instruction pointer (для x86_64)
    pub rsp: usize,   // stack pointer
    pub rflags: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rbp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
}

pub struct Process {
    pub pid: ProcessId,
    pub state: ProcessState,
    pub context: Context,
    pub stack_pointer: *mut u8,
    // Можно добавить виртуальную память, приоритет, дескрипторы файлов и др.
}

impl Process {
    pub fn new(pid: ProcessId, entry_point: usize, stack_top: *mut u8) -> Self {
        let mut context = Context {
            rip: entry_point,
            rsp: stack_top as usize,
            rflags: 0x202, // типичное значение флагов (IF = 1)
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        };

        Process {
            pid,
            state: ProcessState::Ready,
            context,
            stack_pointer: stack_top,
        }
    }
}

// Список процессов (очередь готовых, заблокированных и т.п.)
pub struct ProcessTable {
    processes: [Option<Process>; 64],  // Максимум 64 процесса (пример)
    current: Option<ProcessId>,
}

impl ProcessTable {
    pub const fn new() -> Self {
        Self {
            processes: [None; 64],
            current: None,
        }
    }

    pub fn add_process(&mut self, process: Process) -> Result<(), &'static str> {
        for slot in self.processes.iter_mut() {
            if slot.is_none() {
                *slot = Some(process);
                return Ok(());
            }
        }
        Err("Process table full")
    }

    pub fn switch_to(&mut self, pid: ProcessId) {
        if let Some(current_pid) = self.current {
            if let Some(current_proc) = &mut self.processes[current_pid] {
                current_proc.state = ProcessState::Ready;
                // Сохраняем контекст процессa (нужно реализовать на ассемблере)
                // save_context(&mut current_proc.context);
            }
        }

        if let Some(next_proc) = &mut self.processes[pid] {
            next_proc.state = ProcessState::Running;
            self.current = Some(pid);
            // Восстанавливаем контекст процессa (нужно реализовать на ассемблере)
            // load_context(&next_proc.context);
        }
    }
}

// Заготовка функций для сохранения и восстановления контекста, реализуются на asm:
extern "C" {
    fn save_context(ctx: *mut Context);
    fn load_context(ctx: *const Context) -> !;
}
