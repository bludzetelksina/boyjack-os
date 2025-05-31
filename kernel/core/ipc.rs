#![no_std]

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;
use core::task::Waker;

/// Тип сообщения для IPC
#[derive(Clone)]
pub enum Message {
    Text(&'static str),
    Data(u64),
    Signal(u8),
}

/// Очередь сообщений между процессами
pub struct MessageQueue {
    queue: Mutex<VecDeque<Message>>,
    waker: Mutex<Option<Waker>>,
}

impl MessageQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            queue: Mutex::new(VecDeque::new()),
            waker: Mutex::new(None),
        })
    }

    /// Отправка сообщения в очередь
    pub fn send(&self, msg: Message) {
        let mut q = self.queue.lock();
        q.push_back(msg);
        if let Some(waker) = self.waker.lock().take() {
            waker.wake();
        }
    }

    /// Получение сообщения (если доступно)
    pub fn receive(&self) -> Option<Message> {
        self.queue.lock().pop_front()
    }

    /// Установка `waker` для пробуждения задачи при поступлении сообщения
    pub fn set_waker(&self, w: Waker) {
        *self.waker.lock() = Some(w);
    }
}

/// IPC канал между двумя процессами
pub struct Channel {
    sender: Arc<MessageQueue>,
    receiver: Arc<MessageQueue>,
}

impl Channel {
    pub fn new() -> (Self, Self) {
        let a = MessageQueue::new();
        let b = MessageQueue::new();

        (
            Self {
                sender: a.clone(),
                receiver: b.clone(),
            },
            Self {
                sender: b,
                receiver: a,
            },
        )
    }

    pub fn send(&self, msg: Message) {
        self.sender.send(msg);
    }

    pub fn receive(&self) -> Option<Message> {
        self.receiver.receive()
    }
}
