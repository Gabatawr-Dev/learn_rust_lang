#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

struct Buffer<T> {
    queue: VecDeque<T>,
    is_closed: bool,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T: Debug> {
    pub value: T,
}

pub struct Sender<T> {
    buf: Rc<RefCell<Buffer<T>>>,
}

impl<T: Debug> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let mut buf = self.buf.as_ref().borrow_mut();
        if buf.is_closed {
            Err(SendError { value })
        } else {
            buf.queue.push_back(value);
            Ok(())
        }
        
    }

    pub fn is_closed(&self) -> bool {
        let buf = self.buf.as_ref().borrow();
        buf.is_closed
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.buf, &other.buf)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            buf: self.buf.clone(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    buf: Rc<RefCell<Buffer<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        let mut buf = self.buf.as_ref().borrow_mut();
        match buf.queue.pop_front() {
            None => {
                if buf.is_closed || Rc::strong_count(&self.buf) == 1 {
                    Err(ReceiveError::Closed)
                } else {
                    Err(ReceiveError::Empty)
                }
            }
            Some(e) => Ok(e),
        }
    }

    pub fn close(&mut self) {
        let mut buf = self.buf.as_ref().borrow_mut();
        buf.is_closed = true;
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.buf, &other.buf)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let buffer = Rc::new(RefCell::new(Buffer {
        queue: VecDeque::new(),
        is_closed: false,
    }));
    (
        Sender {
            buf: buffer.clone(),
        },
        Receiver {
            buf: buffer,
        },
    )
}