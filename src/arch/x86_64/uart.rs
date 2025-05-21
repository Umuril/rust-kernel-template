use uart_16550;

pub(crate) struct Uart {
    imp: uart_16550::SerialPort,
}

impl Uart {
    pub(crate) fn new() -> Self {
        let mut uart_16550 = unsafe { uart_16550::SerialPort::new(0x3F8) };
        uart_16550.init();
        Self { imp: uart_16550 }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.imp.write_str(s)
    }
}
