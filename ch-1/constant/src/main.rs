fn set_process_nice(nice_value: i32) -> Result<(), String> {
    unsafe {
        // Reset errno before libc call
        *libc::__errno_location() = 0;
        let res = libc::nice(nice_value);

        if res == -1 && Error::last_os_error().raw_os_error().unwrap_or(0) != 0 {
            Err(format!("Failed to set nice value: {}", Error::last_os_error()))
        } else {
            Ok(())
        }
    }
}
