use libc::{
    cpu_set_t, sched_param, sched_setscheduler, CPU_SET, CPU_ZERO,
    SCHED_FIFO, SCHED_OTHER, SCHED_RR,
};
use std::io::Error;
use std::process;

fn set_cpu_affinity(pid: i32, cpu_core: usize) -> Result<(), String> {
    unsafe {
        let mut set: cpu_set_t = std::mem::zeroed();
        CPU_ZERO(&mut set);
        CPU_SET(cpu_core, &mut set);

        let res = libc::sched_setaffinity(
            pid,
            std::mem::size_of::<cpu_set_t>(),
            &set as *const cpu_set_t,
        );

        if res == 0 {
            Ok(())
        } else {
            Err(format!("Failed to set CPU affinity: {}", Error::last_os_error()))
        }
    }
}

fn set_scheduling_policy(pid: i32, policy: i32, priority: i32) -> Result<(), String> {
    unsafe {
        let param = sched_param {
            sched_priority: priority,
        };

        let res = sched_setscheduler(pid, policy, &param as *const sched_param);

        if res == 0 {
            Ok(())
        } else {
            Err(format!("Failed to set scheduling policy: {}", Error::last_os_error()))
        }
    }
}

fn set_process_nice(nice_value: i32) -> Result<(), String> {
    unsafe {
        *libc::__errno_location() = 0;
        let _res = libc::nice(nice_value);

        let errno = Error::last_os_error().raw_os_error().unwrap_or(0);
        if errno != 0 {
            Err(format!("Failed to set nice value: {}", Error::last_os_error()))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let current_pid = process::id() as i32;
    println!("Current Process PID: {}", current_pid);

    // Pin process to CPU core 0
    match set_cpu_affinity(current_pid, 0) {
        Ok(_) => println!("Successfully set CPU affinity to Core 0."),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Lower process priority via nice value (no sudo required)
    match set_process_nice(5) {
        Ok(_) => println!("Successfully adjusted nice value."),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Set real-time FIFO scheduling policy (Requires sudo / CAP_SYS_NICE)
    match set_scheduling_policy(current_pid, SCHED_FIFO, 10) {
        Ok(_) => println!("Successfully set scheduling policy to SCHED_FIFO."),
        Err(e) => eprintln!("Error (Note: Real-time policies usually require root): {}", e),
    }
}
