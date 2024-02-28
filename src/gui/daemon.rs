use nix::sys::signal::SigSet;
use nix::unistd::{fork, setsid, ForkResult};
use std::io;
use std::os::unix::process::CommandExt;
use std::process::{self, Stdio};

/// Spawn unsupervised daemons.
///
/// This function double-forks to avoid spawning zombies and launches a program with arguments.
pub fn exec(program: &str, args: &[&str]) -> io::Result<()> {
    let mut command = process::Command::new(program);
    command.args(args);
    command.stdin(Stdio::null());
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    unsafe {
        command.pre_exec(|| {
            // Perform second fork.
            match fork() {
                Ok(ForkResult::Parent { .. }) => std::process::exit(0),
                Ok(ForkResult::Child) => (),
                Err(_) => return Err(io::Error::last_os_error()),
            }

            if setsid().is_err() {
                return Err(io::Error::last_os_error());
            }

            // Reset signal handlers.
            let signal_set = SigSet::empty();
            let _ = signal_set.thread_block();

            Ok(())
        });
    }

    command.spawn()?.wait()?;

    Ok(())
}
