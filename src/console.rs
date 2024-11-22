use core::fmt::{self, Write};

use crate::mem::{page_table::HYPERVISOR_PAGE_TABLE, HYPERVISOR_PAGE_TABLE_INITED};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if HYPERVISOR_PAGE_TABLE_INITED.load(core::sync::atomic::Ordering::SeqCst) {
            // after page table inited, we can use console_write to print non-ascii characters
            let str_vaddr = s.as_ptr() as usize;
            let mut global_offset = 0;
            loop {
                let range_offset = global_offset;
                let range_start_vaddr = str_vaddr + global_offset;
                let range_start_paddr = HYPERVISOR_PAGE_TABLE
                    .lock()
                    .translate(range_start_vaddr.into())
                    .expect("failed to query physical addr for printing content");
                while global_offset < s.len() {
                    let vaddr = str_vaddr + global_offset;
                    let paddr = HYPERVISOR_PAGE_TABLE
                        .lock()
                        .translate(vaddr.into())
                        .expect("failed to query physical addr for printing content");
                    if paddr.as_usize() - range_start_paddr.as_usize()
                        != global_offset - range_offset
                    {
                        break;
                    }
                    global_offset += 1;
                }

                // this is a continuous physical slice
                let ret = sbi_rt::console_write(sbi_rt::Physical::new(
                    global_offset - range_offset,
                    range_start_paddr.as_usize(),
                    0,
                ));
                if ret.is_err() {
                    panic!(
                        "[Hypervisor] failed to write to console, err: {:?}",
                        ret.err()
                    )
                }

                if global_offset == s.len() {
                    break;
                }
            }
        } else {
            console_putstr(s);
        }
        Ok(())
    }
}

// this could be used to debug logging functionality
#[allow(deprecated)]
pub fn console_putstr(s: &str) {
    for c in s.chars() {
        sbi_rt::legacy::console_putchar(c as usize);
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
