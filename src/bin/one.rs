use nix::fcntl::OFlag;
use nix::sys::futex::Futex;
use nix::sys::mman::{mmap, shm_open, MapFlags, ProtFlags};
use nix::sys::stat::Mode;
use nix::unistd::{ftruncate, Pid};
use std::mem::size_of;
use std::num::NonZeroUsize;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let pid = Pid::this();

    // Create futex in shared memory
    let path = format!("/{pid}");
    let fd = shm_open(
        path.as_str(),
        OFlag::O_TRUNC | OFlag::O_CREAT | OFlag::O_RDWR,
        Mode::all(),
    )
    .unwrap();

    let n = size_of::<Futex<false>>();

    ftruncate(&fd, n as _).unwrap();

    let ptr = unsafe {
        let ptr = mmap(
            None,
            NonZeroUsize::try_from(n).unwrap(),
            ProtFlags::PROT_WRITE | ProtFlags::PROT_READ,
            MapFlags::MAP_SHARED,
            Some(&fd),
            0,
        )
        .unwrap();
        let futex_ptr = ptr.cast::<Futex<false>>();
        std::ptr::write(futex_ptr, Futex::<false>::new(0));
        futex_ptr
    };

    // Here we could do work that takes notable time to setup, like starting the API.
    sleep(Duration::from_millis(100));

    // Mark the state as ready and wake all waiting processes.
    unsafe {
        (*ptr.cast::<AtomicU32>()).store(1, Ordering::SeqCst);
        (*ptr).wake(u32::MAX).unwrap();
    }

    // sleep(Duration::from_secs(2));
    drop(fd);
    // nix::sys::mman::shm_unlink(path.as_str()).unwrap();
}
