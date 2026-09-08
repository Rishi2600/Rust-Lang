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
