use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            sbi_rt::legacy::console_putchar(c as usize);
        }
        // let bytes = s.as_bytes();
        // let range = bytes.as_ptr_range();
        // let ret = sbi_rt::console_write(sbi_rt::Physical::new(
        //     bytes.len(),
        //     range.start as usize,
        //     range.end as usize,
        // ));
        // if ret.is_err() {
        //     panic!("[Hypervisor] failed to write to console, err: {:?}", ret.err())
        // }
        Ok(())
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
