use nix::fcntl::OFlag;
use nix::sys::futex::Futex;
use nix::sys::mman::{mmap, shm_open, MapFlags, ProtFlags};
use nix::sys::stat::Mode;
use std::mem::size_of;
use std::num::NonZeroUsize;

fn main() {
    // Open shared memory
    let pid = std::env::args().skip(1).next().unwrap();
    let path = format!("/{pid}");
    let fd = shm_open(path.as_str(), OFlag::O_RDWR, Mode::all()).unwrap();

    let ptr = unsafe {
        mmap(
            None,
            NonZeroUsize::try_from(size_of::<Futex<false>>()).unwrap(),
            ProtFlags::PROT_WRITE | ProtFlags::PROT_READ,
            MapFlags::MAP_SHARED,
            Some(&fd),
            0,
        )
        .unwrap()
        .cast::<Futex<false>>()
    };

    // Wait for process to be ready.
    unsafe { (*ptr).wait(0, None).unwrap() };

    drop(fd);
}
