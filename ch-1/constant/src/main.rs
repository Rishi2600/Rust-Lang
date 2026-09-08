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
