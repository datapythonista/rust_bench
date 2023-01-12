use std::arch::asm;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

/// Get the value of the RDTSCP record on Intel x86 machines
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn rdtscp() -> u64 {
  let eax: u32;
  let _ecx: u32;
  let edx: u32;
  {
    unsafe {
      asm!(
        "rdtscp",
        lateout("eax") eax,
        lateout("ecx") _ecx,
        lateout("edx") edx,
        options(nomem, nostack)
      );
    }
  }

  (edx as u64) << 32 | eax as u64
}

/// Get the CPU user usage value from /proc/stat for the given core
pub fn cpu_user(core: u8) -> Result<u64, Error> {
    let stat = File::open("/proc/stat")?;
    let reader = BufReader::new(stat);
    for line in reader.lines() {
        let start = format!("cpu{core}");
        let line = line?;
        if let Some(all_values) = line.strip_prefix(&start) {
            let mut iter_values = all_values.split_whitespace();
            if let Some(cpu_user_value) = iter_values.next() {
                let result = cpu_user_value.parse::<u64>().unwrap();
                return Ok(result);
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "Could not get user value"))
}
