#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "UART_FULL register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartFull {
    ptr: *mut u8,
}
unsafe impl Send for UartFull {}
unsafe impl Sync for UartFull {}
impl UartFull {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, UART_FULL modem control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::McrFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, UART_FULL interrupt enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::IerFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, UART_FULL FIFO control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, UART_FULL line control."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RO, UART_FULL interrupt identification."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::IirFull, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RO, UART_FULL line status."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RO, UART_FULL modem status."]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "RO, UART_FULL receiver buffer, receiving byte."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "WO, UART_FULL transmitter holding, transmittal byte."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::ThrFull, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RO, UART_FULL receiver FIFO count."]
    #[inline(always)]
    pub const fn rfc(self) -> crate::common::Reg<regs::Rfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "RO, UART_FULL transmitter FIFO count."]
    #[inline(always)]
    pub const fn tfc(self) -> crate::common::Reg<regs::Tfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "RW, UART_FULL divisor latch."]
    #[inline(always)]
    pub const fn dl(self) -> crate::common::Reg<regs::Dl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RW, UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "RW, UART_FULL slave address: 0xFF=disable, other=enable."]
    #[inline(always)]
    pub const fn adr(self) -> crate::common::Reg<regs::Adr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
}
#[doc = "UART_NOMODEM register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartNomodem {
    ptr: *mut u8,
}
unsafe impl Send for UartNomodem {}
unsafe impl Sync for UartNomodem {}
impl UartNomodem {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, UART_NOMODEM modem control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::McrNomodem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, UART_NOMODEM interrupt enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::IerNomodem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, UART_NOMODEM FIFO control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, UART_NOMODEM line control."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RO, UART_NOMODEM interrupt identification."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::IirNomodem, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RO, UART_NOMODEM line status."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RO, UART_NOMODEM receiver buffer, receiving byte."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "WO, UART_NOMODEM transmitter holding, transmittal byte."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::ThrNomodem, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RO, UART_NOMODEM receiver FIFO count."]
    #[inline(always)]
    pub const fn rfc(self) -> crate::common::Reg<regs::Rfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "RO, UART_NOMODEM transmitter FIFO count."]
    #[inline(always)]
    pub const fn tfc(self) -> crate::common::Reg<regs::Tfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "RW, UART_NOMODEM divisor latch."]
    #[inline(always)]
    pub const fn dl(self) -> crate::common::Reg<regs::Dl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RW, UART_NOMODEM pre-divisor latch byte, only low 7 bit, from 1 to 128."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, UART_FULL slave address: 0xFF=disable, other=enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adr(pub u8);
    impl Adr {
        #[doc = "RW,UART_FULL slave address: 0xFF=disable, other=enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW,UART_FULL slave address: 0xFF=disable, other=enable."]
        #[inline(always)]
        pub const fn set_adr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Adr {
        #[inline(always)]
        fn default() -> Adr {
            Adr(0)
        }
    }
    impl core::fmt::Debug for Adr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adr").field("adr", &self.adr()).finish()
        }
    }
    #[doc = "RW, UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div(pub u8);
    impl Div {
        #[doc = "RW,UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW,UART_FULL pre-divisor latch byte, only low 7 bit, from 1 to 0/128."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Div {
        #[inline(always)]
        fn default() -> Div {
            Div(0)
        }
    }
    impl core::fmt::Debug for Div {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Div").field("div", &self.div()).finish()
        }
    }
    #[doc = "RW, UART_FULL divisor latch."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dl(pub u16);
    impl Dl {
        #[doc = "RW, UART_FULL divisor latch."]
        #[must_use]
        #[inline(always)]
        pub const fn dl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, UART_FULL divisor latch."]
        #[inline(always)]
        pub const fn set_dl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Dl {
        #[inline(always)]
        fn default() -> Dl {
            Dl(0)
        }
    }
    impl core::fmt::Debug for Dl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dl").field("dl", &self.dl()).finish()
        }
    }
    #[doc = "RW, UART_FULL FIFO control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u8);
    impl Fcr {
        #[doc = "RW, UART FIFO enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fcr_fifo_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART FIFO enable."]
        #[inline(always)]
        pub const fn set_fcr_fifo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "WZ, clear UART receiver FIFO, high action, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn fcr_rx_fifo_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "WZ, clear UART receiver FIFO, high action, auto clear."]
        #[inline(always)]
        pub const fn set_fcr_rx_fifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "WZ, clear UART transmitter FIFO, high action, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn fcr_tx_fifo_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WZ, clear UART transmitter FIFO, high action, auto clear."]
        #[inline(always)]
        pub const fn set_fcr_tx_fifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART receiver FIFO trigger level: 00-1byte, 01-2bytes, 10-4bytes, 11-7bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn fcr_fifo_trig(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RW, UART receiver FIFO trigger level: 00-1byte, 01-2bytes, 10-4bytes, 11-7bytes."]
        #[inline(always)]
        pub const fn set_fcr_fifo_trig(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("fcr_fifo_en", &self.fcr_fifo_en())
                .field("fcr_rx_fifo_clr", &self.fcr_rx_fifo_clr())
                .field("fcr_tx_fifo_clr", &self.fcr_tx_fifo_clr())
                .field("fcr_fifo_trig", &self.fcr_fifo_trig())
                .finish()
        }
    }
    #[doc = "RW, UART_FULL interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IerFull(pub u8);
    impl IerFull {
        #[doc = "RW, UART interrupt enable for receiver data ready."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_recv_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub const fn set_ier_recv_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, UART interrupt enable for THR empty."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_thr_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for THR empty."]
        #[inline(always)]
        pub const fn set_ier_thr_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, UART interrupt enable for receiver line status."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_line_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub const fn set_ier_line_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART_FULL interrupt enable for modem status change."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_modem_chg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL interrupt enable for modem status change."]
        #[inline(always)]
        pub const fn set_ier_modem_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, UART_FULL DTR/TNOW output pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_dtr_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL DTR/TNOW output pin enable."]
        #[inline(always)]
        pub const fn set_ier_dtr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, UART_FULL RTS output pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_rts_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL RTS output pin enable."]
        #[inline(always)]
        pub const fn set_ier_rts_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, UART TXD pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_txd_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART TXD pin enable."]
        #[inline(always)]
        pub const fn set_ier_txd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "WZ, UART software reset control, high action, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "WZ, UART software reset control, high action, auto clear."]
        #[inline(always)]
        pub const fn set_ier_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IerFull {
        #[inline(always)]
        fn default() -> IerFull {
            IerFull(0)
        }
    }
    impl core::fmt::Debug for IerFull {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IerFull")
                .field("ier_recv_rdy", &self.ier_recv_rdy())
                .field("ier_thr_empty", &self.ier_thr_empty())
                .field("ier_line_stat", &self.ier_line_stat())
                .field("ier_modem_chg", &self.ier_modem_chg())
                .field("ier_dtr_en", &self.ier_dtr_en())
                .field("ier_rts_en", &self.ier_rts_en())
                .field("ier_txd_en", &self.ier_txd_en())
                .field("ier_reset", &self.ier_reset())
                .finish()
        }
    }
    #[doc = "RW, UART_NOMODEM interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IerNomodem(pub u8);
    impl IerNomodem {
        #[doc = "RW, UART interrupt enable for receiver data ready."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_recv_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub const fn set_ier_recv_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, UART interrupt enable for THR empty."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_thr_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for THR empty."]
        #[inline(always)]
        pub const fn set_ier_thr_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, UART interrupt enable for receiver line status."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_line_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub const fn set_ier_line_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART TXD pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_txd_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART TXD pin enable."]
        #[inline(always)]
        pub const fn set_ier_txd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "WZ, UART software reset control, high action, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn ier_reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "WZ, UART software reset control, high action, auto clear."]
        #[inline(always)]
        pub const fn set_ier_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IerNomodem {
        #[inline(always)]
        fn default() -> IerNomodem {
            IerNomodem(0)
        }
    }
    impl core::fmt::Debug for IerNomodem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IerNomodem")
                .field("ier_recv_rdy", &self.ier_recv_rdy())
                .field("ier_thr_empty", &self.ier_thr_empty())
                .field("ier_line_stat", &self.ier_line_stat())
                .field("ier_txd_en", &self.ier_txd_en())
                .field("ier_reset", &self.ier_reset())
                .finish()
        }
    }
    #[doc = "RO, UART_FULL interrupt identification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IirFull(pub u8);
    impl IirFull {
        #[doc = "RO, UART interrupt flag bit mask."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_int_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, UART interrupt flag bit mask."]
        #[inline(always)]
        pub const fn set_iir_int_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_no_int(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt."]
        #[inline(always)]
        pub const fn set_iir_no_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, UART FIFO enabled flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_fifo_id(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RO, UART FIFO enabled flag."]
        #[inline(always)]
        pub const fn set_iir_fifo_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for IirFull {
        #[inline(always)]
        fn default() -> IirFull {
            IirFull(0)
        }
    }
    impl core::fmt::Debug for IirFull {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IirFull")
                .field("iir_int_mask", &self.iir_int_mask())
                .field("iir_no_int", &self.iir_no_int())
                .field("iir_fifo_id", &self.iir_fifo_id())
                .finish()
        }
    }
    #[doc = "RO, UART_NOMODEM interrupt identification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IirNomodem(pub u8);
    impl IirNomodem {
        #[doc = "RO, UART interrupt flag bit mask."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_int_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, UART interrupt flag bit mask."]
        #[inline(always)]
        pub const fn set_iir_int_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_no_int(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt."]
        #[inline(always)]
        pub const fn set_iir_no_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, UART FIFO enabled flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_fifo_id(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART FIFO enabled flag."]
        #[inline(always)]
        pub const fn set_iir_fifo_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IirNomodem {
        #[inline(always)]
        fn default() -> IirNomodem {
            IirNomodem(0)
        }
    }
    impl core::fmt::Debug for IirNomodem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IirNomodem")
                .field("iir_int_mask", &self.iir_int_mask())
                .field("iir_no_int", &self.iir_no_int())
                .field("iir_fifo_id", &self.iir_fifo_id())
                .finish()
        }
    }
    #[doc = "RW, UART_FULL line control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcr(pub u8);
    impl Lcr {
        #[doc = "RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lcr_word_sz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit."]
        #[inline(always)]
        pub const fn set_lcr_word_sz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "RW, UART stop bit length: 0-1bit, 1-2bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lcr_stop_bit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART stop bit length: 0-1bit, 1-2bit."]
        #[inline(always)]
        pub const fn set_lcr_stop_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART parity enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lcr_par_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART parity enable."]
        #[inline(always)]
        pub const fn set_lcr_par_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space."]
        #[must_use]
        #[inline(always)]
        pub const fn lcr_par_mod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space."]
        #[inline(always)]
        pub const fn set_lcr_par_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RW, UART break control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lcr_break_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART break control enable."]
        #[inline(always)]
        pub const fn set_lcr_break_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, UART general purpose bit;RW, UART reserved bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lcr_gp_bit_rb_lcr_dlab(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART general purpose bit;RW, UART reserved bit."]
        #[inline(always)]
        pub const fn set_lcr_gp_bit_rb_lcr_dlab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Lcr {
        #[inline(always)]
        fn default() -> Lcr {
            Lcr(0)
        }
    }
    impl core::fmt::Debug for Lcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lcr")
                .field("lcr_word_sz", &self.lcr_word_sz())
                .field("lcr_stop_bit", &self.lcr_stop_bit())
                .field("lcr_par_en", &self.lcr_par_en())
                .field("lcr_par_mod", &self.lcr_par_mod())
                .field("lcr_break_en", &self.lcr_break_en())
                .field("lcr_gp_bit_rb_lcr_dlab", &self.lcr_gp_bit_rb_lcr_dlab())
                .finish()
        }
    }
    #[doc = "RO, UART_FULL line status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsr(pub u8);
    impl Lsr {
        #[doc = "RO, UART receiver fifo data ready status."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_data_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART receiver fifo data ready status."]
        #[inline(always)]
        pub const fn set_lsr_data_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RZ, UART receiver overrun error."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_over_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver overrun error."]
        #[inline(always)]
        pub const fn set_lsr_over_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RZ, UART receiver parity error."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_par_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver parity error."]
        #[inline(always)]
        pub const fn set_lsr_par_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RZ, UART receiver frame error."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_frame_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver frame error."]
        #[inline(always)]
        pub const fn set_lsr_frame_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RZ, UART receiver break error."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_break_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver break error."]
        #[inline(always)]
        pub const fn set_lsr_break_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, UART transmitter fifo empty status."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_tx_fifo_emp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART transmitter fifo empty status."]
        #[inline(always)]
        pub const fn set_lsr_tx_fifo_emp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, UART transmitter all empty status."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_tx_all_emp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART transmitter all empty status."]
        #[inline(always)]
        pub const fn set_lsr_tx_all_emp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate error in UART receiver fifo."]
        #[must_use]
        #[inline(always)]
        pub const fn lsr_err_rx_fifo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate error in UART receiver fifo."]
        #[inline(always)]
        pub const fn set_lsr_err_rx_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Lsr {
        #[inline(always)]
        fn default() -> Lsr {
            Lsr(0)
        }
    }
    impl core::fmt::Debug for Lsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lsr")
                .field("lsr_data_rdy", &self.lsr_data_rdy())
                .field("lsr_over_err", &self.lsr_over_err())
                .field("lsr_par_err", &self.lsr_par_err())
                .field("lsr_frame_err", &self.lsr_frame_err())
                .field("lsr_break_err", &self.lsr_break_err())
                .field("lsr_tx_fifo_emp", &self.lsr_tx_fifo_emp())
                .field("lsr_tx_all_emp", &self.lsr_tx_all_emp())
                .field("lsr_err_rx_fifo", &self.lsr_err_rx_fifo())
                .finish()
        }
    }
    #[doc = "RW, UART_FULL modem control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McrFull(pub u8);
    impl McrFull {
        #[doc = "RW, UART_FULL control DTR."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_dtr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL control DTR."]
        #[inline(always)]
        pub const fn set_mcr_dtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, UART_FULL control RTS."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_rts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL control RTS."]
        #[inline(always)]
        pub const fn set_mcr_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, UART_FULL control OUT1."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_out1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL control OUT1."]
        #[inline(always)]
        pub const fn set_mcr_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART control OUT2/ UART interrupt output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_out2_rb_mcr_int_oe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART control OUT2/ UART interrupt output enable."]
        #[inline(always)]
        pub const fn set_mcr_out2_rb_mcr_int_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, UART_FULL enable local loop back."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_loop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL enable local loop back."]
        #[inline(always)]
        pub const fn set_mcr_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, UART_FULL enable autoflow control."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_au_flow_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL enable autoflow control."]
        #[inline(always)]
        pub const fn set_mcr_au_flow_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, UART_FULL enable TNOW output on DTR pin."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_tnow(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL enable TNOW output on DTR pin."]
        #[inline(always)]
        pub const fn set_mcr_tnow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, UART_FULL enable half-duplex."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_half(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART_FULL enable half-duplex."]
        #[inline(always)]
        pub const fn set_mcr_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for McrFull {
        #[inline(always)]
        fn default() -> McrFull {
            McrFull(0)
        }
    }
    impl core::fmt::Debug for McrFull {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McrFull")
                .field("mcr_dtr", &self.mcr_dtr())
                .field("mcr_rts", &self.mcr_rts())
                .field("mcr_out1", &self.mcr_out1())
                .field("mcr_out2_rb_mcr_int_oe", &self.mcr_out2_rb_mcr_int_oe())
                .field("mcr_loop", &self.mcr_loop())
                .field("mcr_au_flow_en", &self.mcr_au_flow_en())
                .field("mcr_tnow", &self.mcr_tnow())
                .field("mcr_half", &self.mcr_half())
                .finish()
        }
    }
    #[doc = "RW, UART_NOMODEM modem control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McrNomodem(pub u8);
    impl McrNomodem {
        #[doc = "RW, UART control OUT2/UART interrupt output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcr_out2_rb_mcr_int_oe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART control OUT2/UART interrupt output enable."]
        #[inline(always)]
        pub const fn set_mcr_out2_rb_mcr_int_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for McrNomodem {
        #[inline(always)]
        fn default() -> McrNomodem {
            McrNomodem(0)
        }
    }
    impl core::fmt::Debug for McrNomodem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("McrNomodem")
                .field("mcr_out2_rb_mcr_int_oe", &self.mcr_out2_rb_mcr_int_oe())
                .finish()
        }
    }
    #[doc = "RO, UART_FULL modem status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u8);
    impl Msr {
        #[doc = "RZ, UART_FULL CTS changed status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_cts_chg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART_FULL CTS changed status, high action."]
        #[inline(always)]
        pub const fn set_msr_cts_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RZ, UART_FULL DSR changed status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_dsr_chg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART_FULL DSR changed status, high action."]
        #[inline(always)]
        pub const fn set_msr_dsr_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RZ, UART_FULL RI changed status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_ri_chg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART_FULL RI changed status, high action."]
        #[inline(always)]
        pub const fn set_msr_ri_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RZ, UART_FULL DCD changed status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_dcd_chg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART_FULL DCD changed status, high action."]
        #[inline(always)]
        pub const fn set_msr_dcd_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, UART_FULL CTS action status."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_cts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART_FULL CTS action status."]
        #[inline(always)]
        pub const fn set_msr_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, UART_FULL DSR action statusv."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_dsr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART_FULL DSR action statusv."]
        #[inline(always)]
        pub const fn set_msr_dsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, UART_FULL RI action status."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART_FULL RI action status."]
        #[inline(always)]
        pub const fn set_msr_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, UART_FULL DCD action status."]
        #[must_use]
        #[inline(always)]
        pub const fn msr_dcd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART_FULL DCD action status."]
        #[inline(always)]
        pub const fn set_msr_dcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    impl core::fmt::Debug for Msr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msr")
                .field("msr_cts_chg", &self.msr_cts_chg())
                .field("msr_dsr_chg", &self.msr_dsr_chg())
                .field("msr_ri_chg", &self.msr_ri_chg())
                .field("msr_dcd_chg", &self.msr_dcd_chg())
                .field("msr_cts", &self.msr_cts())
                .field("msr_dsr", &self.msr_dsr())
                .field("msr_ri", &self.msr_ri())
                .field("msr_dcd", &self.msr_dcd())
                .finish()
        }
    }
    #[doc = "RO, UART_FULL receiver buffer, receiving byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbr(pub u8);
    impl Rbr {
        #[doc = "RO, UART_FULL receiver buffer, receiving byte."]
        #[must_use]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART_FULL receiver buffer, receiving byte."]
        #[inline(always)]
        pub const fn set_rbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Rbr {
        #[inline(always)]
        fn default() -> Rbr {
            Rbr(0)
        }
    }
    impl core::fmt::Debug for Rbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rbr").field("rbr", &self.rbr()).finish()
        }
    }
    #[doc = "RO, UART_FULL receiver FIFO count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc(pub u8);
    impl Rfc {
        #[doc = "RO, UART_FULL receiver FIFO count."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART_FULL receiver FIFO count."]
        #[inline(always)]
        pub const fn set_rfc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Rfc {
        #[inline(always)]
        fn default() -> Rfc {
            Rfc(0)
        }
    }
    impl core::fmt::Debug for Rfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfc").field("rfc", &self.rfc()).finish()
        }
    }
    #[doc = "RO, UART_FULL transmitter FIFO count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tfc(pub u8);
    impl Tfc {
        #[doc = "RO, UART_FULL transmitter FIFO count."]
        #[must_use]
        #[inline(always)]
        pub const fn tfc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART_FULL transmitter FIFO count."]
        #[inline(always)]
        pub const fn set_tfc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Tfc {
        #[inline(always)]
        fn default() -> Tfc {
            Tfc(0)
        }
    }
    impl core::fmt::Debug for Tfc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tfc").field("tfc", &self.tfc()).finish()
        }
    }
    #[doc = "WO, UART_FULL transmitter holding, transmittal byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ThrFull(pub u8);
    impl ThrFull {
        #[doc = "RO, UART_FULL transmitter holding, transmittal byte."]
        #[must_use]
        #[inline(always)]
        pub const fn thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART_FULL transmitter holding, transmittal byte."]
        #[inline(always)]
        pub const fn set_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ThrFull {
        #[inline(always)]
        fn default() -> ThrFull {
            ThrFull(0)
        }
    }
    impl core::fmt::Debug for ThrFull {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ThrFull").field("thr", &self.thr()).finish()
        }
    }
    #[doc = "WO, UART_NOMODEM transmitter holding, transmittal byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ThrNomodem(pub u8);
    impl ThrNomodem {
        #[doc = "WO, UART_NOMODEM transmitter holding, transmittal byte."]
        #[must_use]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "WO, UART_NOMODEM transmitter holding, transmittal byte."]
        #[inline(always)]
        pub const fn set_rbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ThrNomodem {
        #[inline(always)]
        fn default() -> ThrNomodem {
            ThrNomodem(0)
        }
    }
    impl core::fmt::Debug for ThrNomodem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ThrNomodem")
                .field("rbr", &self.rbr())
                .finish()
        }
    }
}
