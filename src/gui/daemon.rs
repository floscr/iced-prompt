use std::{
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};

pub fn exec(program: String) {
    match unsafe { fork() }.expect("Failed to fork process") {
        ForkResult::Parent { child } => {
            println!("Try to kill me to check if the target process will be killed");

            // Do not forget to wait for the fork in order to prevent it from becoming a zombie!!!
            waitpid(Some(child), None).unwrap();

            // You have 120 seconds to kill the process :)
            // sleep(Duration::from_secs(120));
        }

        ForkResult::Child => {
            let _ = nix::unistd::setsid();
            Command::new(program)
                .spawn()
                .expect("failed to spawn the target process");
            exit(0);
        }
    }
}
