use std::process::{Command, Stdio};

const ONE: &str = env!("CARGO_BIN_EXE_one");
const TWO: &str = env!("CARGO_BIN_EXE_two");

const SAMPLES: usize = 30;

#[test]
fn basic() {
    for _ in 0..SAMPLES {
        let one = Command::new(ONE)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        // There is still a race condition here, it is just much smaller.
        std::thread::sleep(std::time::Duration::from_millis(50));

        let two = Command::new(TWO)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg(one.id().to_string())
            .spawn()
            .unwrap();

        let two_out = two.wait_with_output().unwrap();
        assert!(two_out.status.success());
        let one_out = one.wait_with_output().unwrap();
        assert!(one_out.status.success());
    }
}
