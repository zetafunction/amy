extern crate libc;
extern crate nix;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod channel;
mod event;
mod frame_reader;
mod frame_writer;
mod line_reader;
mod notification;
mod poller;
mod registrar;
mod timer;
mod user_event;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod epoll;
#[cfg(any(target_os = "linux", target_os = "android"))]
#[cfg(not(feature = "no_timerfd"))]
mod timerfd;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[cfg(feature = "no_timerfd")]
mod timer_heap;

#[cfg(any(
    target_os = "bitrig",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
mod kqueue;

pub use channel::{channel, ChannelError, Receiver, Sender};
pub use event::Event;
pub use frame_reader::FrameReader;
pub use frame_writer::FrameWriter;
pub use line_reader::LineReader;
pub use notification::Notification;
pub use poller::Poller;
pub use registrar::Registrar;
pub use timer::Timer;

use nix::Error::Sys;
use std::io::{self, ErrorKind};

fn nix_err_to_io_err(err: nix::Error) -> io::Error {
    match err {
        Sys(errno) => io::Error::from(errno),
        _ => io::Error::new(ErrorKind::InvalidData, err),
    }
}
