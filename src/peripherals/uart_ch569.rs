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
    #[doc = "UART_FULL modem control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::McrFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "UART_FULL interrupt enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::IerFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "UART_FULL FIFO control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "UART_FULL line control."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "UART_FULL interrupt identification."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::Iir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "UART_FULL line status."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "UART_FULL modem status."]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "UART_FULL receiver buffer, receiving byte."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "UART_FULL transmitter holding, transmittal byte."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::Thr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "UART_FULL receiver FIFO count."]
    #[inline(always)]
    pub const fn rfc(self) -> crate::common::Reg<regs::Rfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "UART_FULL transmitter FIFO count."]
    #[inline(always)]
    pub const fn tfc(self) -> crate::common::Reg<regs::Tfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "UART_FULL divisor latch."]
    #[inline(always)]
    pub const fn dl(self) -> crate::common::Reg<regs::Dl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "UART_FULL pre-divisor latch byte."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::DivFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "UART_FULL slave address."]
    #[inline(always)]
    pub const fn adr(self) -> crate::common::Reg<regs::Adr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fusize) as _) }
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
    #[doc = "UART_NOMODEM modem control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::McrNomodem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "UART_NOMODEM interrupt enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::IerNomodem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "UART_NOMODEM FIFO control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "UART_NOMODEM line control."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "UART_NOMODEM interrupt identification."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::Iir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "UART_NOMODEM line status."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "UART_NOMODEM receiver buffer, receiving byte."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "UART_NOMODEM transmitter holding, transmittal byte."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::Thr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "UART_NOMODEM receiver FIFO count."]
    #[inline(always)]
    pub const fn rfc(self) -> crate::common::Reg<regs::Rfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "UART_NOMODEM transmitter FIFO count."]
    #[inline(always)]
    pub const fn tfc(self) -> crate::common::Reg<regs::Tfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "UART_NOMODEM divisor latch."]
    #[inline(always)]
    pub const fn dl(self) -> crate::common::Reg<regs::Dl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "UART_NOMODEM pre-divisor latch byte."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::DivNomodem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
}
pub mod regs {
    #[doc = "UART_FULL slave address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adr(pub u8);
    impl Adr {
        #[doc = "UART_FULL slave address."]
        #[inline(always)]
        pub const fn adr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART_FULL slave address."]
        #[inline(always)]
        pub fn set_adr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Adr {
        #[inline(always)]
        fn default() -> Adr {
            Adr(0)
        }
    }
    #[doc = "UART_FULL pre-divisor latch byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DivFull(pub u8);
    impl DivFull {
        #[doc = "UART pre-divisor latch byte."]
        #[inline(always)]
        pub const fn adr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART pre-divisor latch byte."]
        #[inline(always)]
        pub fn set_adr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for DivFull {
        #[inline(always)]
        fn default() -> DivFull {
            DivFull(0)
        }
    }
    #[doc = "UART_NOMODEM pre-divisor latch byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DivNomodem(pub u8);
    impl DivNomodem {
        #[doc = "UART pre-divisor latch byte."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART pre-divisor latch byte."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for DivNomodem {
        #[inline(always)]
        fn default() -> DivNomodem {
            DivNomodem(0)
        }
    }
    #[doc = "UART_FULL divisor latch."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dl(pub u16);
    impl Dl {
        #[doc = "UART divisor latch."]
        #[inline(always)]
        pub const fn dl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "UART divisor latch."]
        #[inline(always)]
        pub fn set_dl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Dl {
        #[inline(always)]
        fn default() -> Dl {
            Dl(0)
        }
    }
    #[doc = "UART_FULL FIFO control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u8);
    impl Fcr {
        #[doc = "UART FIFO enable."]
        #[inline(always)]
        pub const fn fcr_fifo_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART FIFO enable."]
        #[inline(always)]
        pub fn set_fcr_fifo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "clear UART receiver FIFO, high action, auto clear."]
        #[inline(always)]
        pub const fn fcr_rx_fifo_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear UART receiver FIFO, high action, auto clear."]
        #[inline(always)]
        pub fn set_fcr_rx_fifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear UART transmitter FIFO, high action, auto clear."]
        #[inline(always)]
        pub const fn fcr_tx_fifo_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear UART transmitter FIFO, high action, auto clear."]
        #[inline(always)]
        pub fn set_fcr_tx_fifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART receiver FIFO trigger level."]
        #[inline(always)]
        pub const fn fcr_fifo_trig(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "UART receiver FIFO trigger level."]
        #[inline(always)]
        pub fn set_fcr_fifo_trig(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    #[doc = "UART_FULL interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IerFull(pub u8);
    impl IerFull {
        #[doc = "UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub const fn ier_recv_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub fn set_ier_recv_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "UART interrupt enable for THR empty."]
        #[inline(always)]
        pub const fn ier_thr_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UART interrupt enable for THR empty."]
        #[inline(always)]
        pub fn set_ier_thr_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub const fn ier_line_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub fn set_ier_line_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART_FULL interrupt enable for modem status change."]
        #[inline(always)]
        pub const fn ier_modem_chg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL interrupt enable for modem status change."]
        #[inline(always)]
        pub fn set_ier_modem_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "UART_FULL DTR/TNOW output pin enable."]
        #[inline(always)]
        pub const fn ier_dtr_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL DTR/TNOW output pin enable."]
        #[inline(always)]
        pub fn set_ier_dtr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "UART_FULL RTS output pin enable."]
        #[inline(always)]
        pub const fn ier_rts_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL RTS output pin enable."]
        #[inline(always)]
        pub fn set_ier_rts_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "UART TXD pin enable."]
        #[inline(always)]
        pub const fn ier_txd_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART TXD pin enable."]
        #[inline(always)]
        pub fn set_ier_txd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "UART software reset control, high action, auto clear."]
        #[inline(always)]
        pub const fn ier_reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "UART software reset control, high action, auto clear."]
        #[inline(always)]
        pub fn set_ier_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IerFull {
        #[inline(always)]
        fn default() -> IerFull {
            IerFull(0)
        }
    }
    #[doc = "UART_NOMODEM interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IerNomodem(pub u8);
    impl IerNomodem {
        #[doc = "UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub const fn ier_recv_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub fn set_ier_recv_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "UART interrupt enable for THR empty."]
        #[inline(always)]
        pub const fn ier_thr_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UART interrupt enable for THR empty."]
        #[inline(always)]
        pub fn set_ier_thr_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub const fn ier_line_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub fn set_ier_line_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART TXD pin enable."]
        #[inline(always)]
        pub const fn ier_txd_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART TXD pin enable."]
        #[inline(always)]
        pub fn set_ier_txd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "UART software reset control, high action, auto clear."]
        #[inline(always)]
        pub const fn ier_reset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "UART software reset control, high action, auto clear."]
        #[inline(always)]
        pub fn set_ier_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IerNomodem {
        #[inline(always)]
        fn default() -> IerNomodem {
            IerNomodem(0)
        }
    }
    #[doc = "UART_FULL interrupt identification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iir(pub u8);
    impl Iir {
        #[doc = "UART no interrupt flag."]
        #[inline(always)]
        pub const fn iir_no_int(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART no interrupt flag."]
        #[inline(always)]
        pub fn set_iir_no_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "UART interrupt flag bit mask."]
        #[inline(always)]
        pub const fn iir_int_mask(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "UART interrupt flag bit mask."]
        #[inline(always)]
        pub fn set_iir_int_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
        }
        #[doc = "UART FIFO enabled flag."]
        #[inline(always)]
        pub const fn iir_fifo_id(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "UART FIFO enabled flag."]
        #[inline(always)]
        pub fn set_iir_fifo_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Iir {
        #[inline(always)]
        fn default() -> Iir {
            Iir(0)
        }
    }
    #[doc = "UART_FULL line control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcr(pub u8);
    impl Lcr {
        #[doc = "UART word bit length."]
        #[inline(always)]
        pub const fn lcr_word_sz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "UART word bit length."]
        #[inline(always)]
        pub fn set_lcr_word_sz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "UART stop bit length."]
        #[inline(always)]
        pub const fn lcr_stop_bit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UART stop bit length."]
        #[inline(always)]
        pub fn set_lcr_stop_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART parity enable."]
        #[inline(always)]
        pub const fn lcr_par_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UART parity enable."]
        #[inline(always)]
        pub fn set_lcr_par_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "UART parity mode."]
        #[inline(always)]
        pub const fn lcr_par_mod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "UART parity mode."]
        #[inline(always)]
        pub fn set_lcr_par_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "UART break control enable."]
        #[inline(always)]
        pub const fn lcr_break_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART break control enable."]
        #[inline(always)]
        pub fn set_lcr_break_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "UART reserved bit _UART general purpose bit."]
        #[inline(always)]
        pub const fn lcr_dlab_rb_lcr_gp_bit(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "UART reserved bit _UART general purpose bit."]
        #[inline(always)]
        pub fn set_lcr_dlab_rb_lcr_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Lcr {
        #[inline(always)]
        fn default() -> Lcr {
            Lcr(0)
        }
    }
    #[doc = "UART_FULL line status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsr(pub u8);
    impl Lsr {
        #[doc = "UART receiver fifo data ready status."]
        #[inline(always)]
        pub const fn lsr_data_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART receiver fifo data ready status."]
        #[inline(always)]
        pub fn set_lsr_data_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "UART receiver overrun error."]
        #[inline(always)]
        pub const fn lsr_over_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UART receiver overrun error."]
        #[inline(always)]
        pub fn set_lsr_over_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "UART receiver frame error."]
        #[inline(always)]
        pub const fn lsr_par_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UART receiver frame error."]
        #[inline(always)]
        pub fn set_lsr_par_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART receiver frame error."]
        #[inline(always)]
        pub const fn lsr_frame_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UART receiver frame error."]
        #[inline(always)]
        pub fn set_lsr_frame_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "UART receiver break error."]
        #[inline(always)]
        pub const fn lsr_break_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UART receiver break error."]
        #[inline(always)]
        pub fn set_lsr_break_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "UART transmitter fifo empty status."]
        #[inline(always)]
        pub const fn lsr_tx_fifo_emp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UART transmitter fifo empty status."]
        #[inline(always)]
        pub fn set_lsr_tx_fifo_emp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "UART transmitter all empty status."]
        #[inline(always)]
        pub const fn lsr_tx_all_emp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART transmitter all empty status."]
        #[inline(always)]
        pub fn set_lsr_tx_all_emp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "indicate error in UART receiver fifo."]
        #[inline(always)]
        pub const fn lsr_err_rx_fifo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "indicate error in UART receiver fifo."]
        #[inline(always)]
        pub fn set_lsr_err_rx_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Lsr {
        #[inline(always)]
        fn default() -> Lsr {
            Lsr(0)
        }
    }
    #[doc = "UART_FULL modem control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McrFull(pub u8);
    impl McrFull {
        #[doc = "UART_FULL control DTR."]
        #[inline(always)]
        pub const fn mcr_dtr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL control DTR."]
        #[inline(always)]
        pub fn set_mcr_dtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "UART_FULL control RTS."]
        #[inline(always)]
        pub const fn mcr_rts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL control RTS."]
        #[inline(always)]
        pub fn set_mcr_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "UART_FULL control OUT1."]
        #[inline(always)]
        pub const fn mcr_out1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL control OUT1."]
        #[inline(always)]
        pub fn set_mcr_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART control OUT2."]
        #[inline(always)]
        pub const fn mcr_out2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UART control OUT2."]
        #[inline(always)]
        pub fn set_mcr_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "UART_FULL enable local loop back."]
        #[inline(always)]
        pub const fn mcr_loop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL enable local loop back."]
        #[inline(always)]
        pub fn set_mcr_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "UART_FULL enable autoflow control."]
        #[inline(always)]
        pub const fn mcr_au_flow_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL enable autoflow control."]
        #[inline(always)]
        pub fn set_mcr_au_flow_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "UART_FULL enable TNOW output on DTR pin."]
        #[inline(always)]
        pub const fn mcr_tnow(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL enable TNOW output on DTR pin."]
        #[inline(always)]
        pub fn set_mcr_tnow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "UART_FULL enable half-duplex."]
        #[inline(always)]
        pub const fn mcr_half(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL enable half-duplex."]
        #[inline(always)]
        pub fn set_mcr_half(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for McrFull {
        #[inline(always)]
        fn default() -> McrFull {
            McrFull(0)
        }
    }
    #[doc = "UART_NOMODEM modem control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McrNomodem(pub u8);
    impl McrNomodem {
        #[doc = "UART_NOMODEM control OUT2."]
        #[inline(always)]
        pub const fn mcr_out2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UART_NOMODEM control OUT2."]
        #[inline(always)]
        pub fn set_mcr_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for McrNomodem {
        #[inline(always)]
        fn default() -> McrNomodem {
            McrNomodem(0)
        }
    }
    #[doc = "UART_FULL modem status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u8);
    impl Msr {
        #[doc = "UART_FULL CTS changed status, high action."]
        #[inline(always)]
        pub const fn msr_cts_chg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL CTS changed status, high action."]
        #[inline(always)]
        pub fn set_msr_cts_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "UART_FULL DSR changed status, high action."]
        #[inline(always)]
        pub const fn msr_dsr_chg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL DSR changed status, high action."]
        #[inline(always)]
        pub fn set_msr_dsr_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "UART_FULL RI changed status, high action."]
        #[inline(always)]
        pub const fn msr_ri_chg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL RI changed status, high action."]
        #[inline(always)]
        pub fn set_msr_ri_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "UART_FULL DCD changed status, high action."]
        #[inline(always)]
        pub const fn msr_dcd_chg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL DCD changed status, high action."]
        #[inline(always)]
        pub fn set_msr_dcd_chg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "UART_FULL CTS action status."]
        #[inline(always)]
        pub const fn msr_cts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL CTS action status."]
        #[inline(always)]
        pub fn set_msr_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "UART_FULL DSR action status."]
        #[inline(always)]
        pub const fn msr_dsr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL DSR action status."]
        #[inline(always)]
        pub fn set_msr_dsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "UART_FULL RI action status."]
        #[inline(always)]
        pub const fn msr_ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL RI action status."]
        #[inline(always)]
        pub fn set_msr_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "UART_FULL DCD action status."]
        #[inline(always)]
        pub const fn msr_dcd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "UART_FULL DCD action status."]
        #[inline(always)]
        pub fn set_msr_dcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    #[doc = "UART receiver buffer, receiving byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbr(pub u8);
    impl Rbr {
        #[doc = "UART receiver buffer, receiving byte."]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART receiver buffer, receiving byte."]
        #[inline(always)]
        pub fn set_rbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Rbr {
        #[inline(always)]
        fn default() -> Rbr {
            Rbr(0)
        }
    }
    #[doc = "UART_FULL receiver FIFO count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc(pub u8);
    impl Rfc {
        #[doc = "UART receiver FIFO count."]
        #[inline(always)]
        pub const fn rfc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART receiver FIFO count."]
        #[inline(always)]
        pub fn set_rfc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Rfc {
        #[inline(always)]
        fn default() -> Rfc {
            Rfc(0)
        }
    }
    #[doc = "UART_FULL transmitter FIFO count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tfc(pub u8);
    impl Tfc {
        #[doc = "UART transmitter FIFO count."]
        #[inline(always)]
        pub const fn tfc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART transmitter FIFO count."]
        #[inline(always)]
        pub fn set_tfc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Tfc {
        #[inline(always)]
        fn default() -> Tfc {
            Tfc(0)
        }
    }
    #[doc = "UART transmitter holding, transmittal byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Thr(pub u8);
    impl Thr {
        #[doc = "UART transmitter holding, transmittal byte."]
        #[inline(always)]
        pub const fn thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "UART transmitter holding, transmittal byte."]
        #[inline(always)]
        pub fn set_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Thr {
        #[inline(always)]
        fn default() -> Thr {
            Thr(0)
        }
    }
}
