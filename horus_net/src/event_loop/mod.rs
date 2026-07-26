//! Event-driven I/O — epoll (Linux) + kqueue (macOS). Unix-only: there is no
//! Windows backend.
//!
//! No busy-polling. Zero wakeups when idle.

#[cfg(target_os = "linux")]
pub mod epoll;

#[cfg(target_os = "macos")]
pub mod kqueue;

/// Event source — what woke us up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSource {
    /// UDP socket has data ready to read (import path).
    UdpSocket,
    /// SHM eventfd/pipe signaled — new local data to export.
    ShmNotify,
    /// Timer fired — periodic tasks (discovery, heartbeat).
    Timer,
}

/// A single event from the event loop.
#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub source: EventSource,
}

/// Platform-dispatched event loop.
///
/// Linux: epoll. macOS: kqueue. There is no other backend — this used to say
/// "On other: polling fallback", but no fallback was ever written, so on any
/// other target `PlatformEventLoop` simply does not exist and horus_net fails
/// to compile (Windows also trips over the bare `libc::write` in
/// replicator.rs). horus_net is Unix-only until an IOCP backend lands; the
/// Windows job in multi-platform.yml excludes it for that reason.
#[cfg(target_os = "linux")]
pub type PlatformEventLoop = epoll::EpollLoop;

#[cfg(target_os = "macos")]
pub type PlatformEventLoop = kqueue::KqueueLoop;
