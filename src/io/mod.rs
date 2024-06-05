pub mod serial;
pub mod stdout;
pub mod stdin;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (
        {
            use crate::{std_print, serial_print};
            std_print!($($arg)*);
            serial_print!($($arg)*);
        }
    );
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
