#![no_std]

use crate::core::process::{Process, ProcessId, ProcessState, ProcessTable};

pub enum SchedulingAlgorithm {
    RoundRobin,
    PriorityBased,
}

pub struct Scheduler {
    process_table: ProcessTable,
    algorithm: SchedulingAlgorithm,
    last_index: usize, // Для round-robin
}

impl Scheduler {
    pub fn new(algorithm: SchedulingAlgorithm) -> Self {
        Self {
            process_table: ProcessTable::new(),
            algorithm,
            last_index: 0,
        }
    }

    pub fn add_process(&mut self, process: Process) -> Result<(), &'static str> {
        self.process_table.add_process(process)
    }

    pub fn tick(&mut self) {
        match self.algorithm {
            SchedulingAlgorithm::RoundRobin => self.schedule_round_robin(),
            SchedulingAlgorithm::PriorityBased => self.schedule_priority_based(),
        }
    }

    fn schedule_round_robin(&mut self) {
        let len = self.process_table.processes.len();
        let mut next_index = (self.last_index + 1) % len;

        for _ in 0..len {
            if let Some(proc) = &self.process_table.processes[next_index] {
                if proc.state == ProcessState::Ready {
                    self.process_table.switch_to(next_index);
                    self.last_index = next_index;
                    return;
                }
            }
            next_index = (next_index + 1) % len;
        }
    }

    fn schedule_priority_based(&mut self) {
        let mut best_pid: Option<usize> = None;
        let mut highest_priority: i32 = i32::MIN;

        for (pid, opt_proc) in self.process_table.processes.iter().enumerate() {
            if let Some(proc) = opt_proc {
                if proc.state == ProcessState::Ready {
                    let priority = self.get_priority(proc);
                    if priority > highest_priority {
                        highest_priority = priority;
                        best_pid = Some(pid);
                    }
                }
            }
        }

        if let Some(pid) = best_pid {
            self.process_table.switch_to(pid);
        }
    }

    fn get_priority(&self, _process: &Process) -> i32 {
        // Заглушка: позже можно добавить приоритет как поле в `Process`
        1
    }
}
