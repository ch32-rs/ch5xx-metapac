#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "UART register."]
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
    #[doc = "RW, UART modem control."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RW, UART interrupt enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "RW, UART FIFO control."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "RW, UART line control."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "RO, UART interrupt identification."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::Iir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RO, UART line status."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "RO, UART receiver buffer, receiving byte."]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "WO, UART transmitter holding, transmittal byte."]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::Thr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RO, UART receiver FIFO count."]
    #[inline(always)]
    pub const fn rfc(self) -> crate::common::Reg<regs::Rfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "RO, UART transmitter FIFO count."]
    #[inline(always)]
    pub const fn tfc(self) -> crate::common::Reg<regs::Tfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[doc = "RW, UART divisor latch."]
    #[inline(always)]
    pub const fn dl(self) -> crate::common::Reg<regs::Dl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RW, UART pre-divisor latch byte, only low 7 bit, from 1 to 128."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, UART pre-divisor latch byte, only low 7 bit, from 1 to 128."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Div(pub u8);
    impl Div {
        #[doc = "RW, UART pre-divisor latch byte, only low 7 bit, from 1 to 128."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, UART pre-divisor latch byte, only low 7 bit, from 1 to 128."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Div {
        #[inline(always)]
        fn default() -> Div {
            Div(0)
        }
    }
    #[doc = "RW, UART divisor latch."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dl(pub u16);
    impl Dl {
        #[doc = "RW, UART divisor latch."]
        #[inline(always)]
        pub const fn dl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, UART divisor latch."]
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
    #[doc = "RW, UART FIFO control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u8);
    impl Fcr {
        #[doc = "RW, UART FIFO enable."]
        #[inline(always)]
        pub const fn fcr_fifo_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART FIFO enable."]
        #[inline(always)]
        pub fn set_fcr_fifo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, UART receiver FIFO trigger level: 00-1byte, 01-2bytes, 10-4bytes, 11-7bytes."]
        #[inline(always)]
        pub const fn fcr_fifo_trig(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RW, UART receiver FIFO trigger level: 00-1byte, 01-2bytes, 10-4bytes, 11-7bytes."]
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
    #[doc = "RW, UART interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u8);
    impl Ier {
        #[doc = "RW, UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub const fn ier_recv_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for receiver data ready."]
        #[inline(always)]
        pub fn set_ier_recv_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, UART interrupt enable for THR empty."]
        #[inline(always)]
        pub const fn ier_thr_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for THR empty."]
        #[inline(always)]
        pub fn set_ier_thr_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub const fn ier_line_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt enable for receiver line status."]
        #[inline(always)]
        pub fn set_ier_line_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART TXD pin enable."]
        #[inline(always)]
        pub const fn ier_txd_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART TXD pin enable."]
        #[inline(always)]
        pub fn set_ier_txd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "RO, UART interrupt identification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iir(pub u8);
    impl Iir {
        #[doc = "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt."]
        #[inline(always)]
        pub const fn iir_no_int(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART no interrupt flag: 0=interrupt action, 1=no interrupt."]
        #[inline(always)]
        pub fn set_iir_no_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, UART interrupt flag bit mask."]
        #[inline(always)]
        pub const fn iir_int_mask(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "RO, UART interrupt flag bit mask."]
        #[inline(always)]
        pub fn set_iir_int_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
        }
        #[doc = "RO, UART FIFO enabled flag."]
        #[inline(always)]
        pub const fn iir_fifo_id(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RO, UART FIFO enabled flag."]
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
    #[doc = "RW, UART line control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcr(pub u8);
    impl Lcr {
        #[doc = "RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit."]
        #[inline(always)]
        pub const fn lcr_word_sz(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RW, UART word bit length: 00-5bit, 01-6bit, 10-7bit, 11-8bit."]
        #[inline(always)]
        pub fn set_lcr_word_sz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "RW, UART stop bit length: 0-1bit, 1-2bit."]
        #[inline(always)]
        pub const fn lcr_stop_bit(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART stop bit length: 0-1bit, 1-2bit."]
        #[inline(always)]
        pub fn set_lcr_stop_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, UART parity enable."]
        #[inline(always)]
        pub const fn lcr_par_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART parity enable."]
        #[inline(always)]
        pub fn set_lcr_par_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space."]
        #[inline(always)]
        pub const fn lcr_par_mod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RW, UART parity mode: 00-odd, 01-even, 10-mark, 11-space."]
        #[inline(always)]
        pub fn set_lcr_par_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RW, UART break control enable."]
        #[inline(always)]
        pub const fn lcr_break_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART break control enable."]
        #[inline(always)]
        pub fn set_lcr_break_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, UART divisor latch access bit."]
        #[inline(always)]
        pub const fn lcr_dlab(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART divisor latch access bit."]
        #[inline(always)]
        pub fn set_lcr_dlab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
        #[doc = "RW, UART general purpose bit."]
        #[inline(always)]
        pub const fn lcr_gp_bit(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART general purpose bit."]
        #[inline(always)]
        pub fn set_lcr_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Lcr {
        #[inline(always)]
        fn default() -> Lcr {
            Lcr(0)
        }
    }
    #[doc = "RO, UART line status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsr(pub u8);
    impl Lsr {
        #[doc = "RO, UART receiver fifo data ready status."]
        #[inline(always)]
        pub const fn lsr_data_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART receiver fifo data ready status."]
        #[inline(always)]
        pub fn set_lsr_data_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RZ, UART receiver overrun error."]
        #[inline(always)]
        pub const fn lsr_over_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver overrun error."]
        #[inline(always)]
        pub fn set_lsr_over_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RZ, UART receiver parity error."]
        #[inline(always)]
        pub const fn lsr_par_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver parity error."]
        #[inline(always)]
        pub fn set_lsr_par_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RZ, UART receiver frame error."]
        #[inline(always)]
        pub const fn lsr_frame_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver frame error."]
        #[inline(always)]
        pub fn set_lsr_frame_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RZ, UART receiver break error."]
        #[inline(always)]
        pub const fn lsr_break_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RZ, UART receiver break error."]
        #[inline(always)]
        pub fn set_lsr_break_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, UART transmitter fifo empty status."]
        #[inline(always)]
        pub const fn lsr_tx_fifo_emp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART transmitter fifo empty status."]
        #[inline(always)]
        pub fn set_lsr_tx_fifo_emp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, UART transmitter all empty status."]
        #[inline(always)]
        pub const fn lsr_tx_all_emp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, UART transmitter all empty status."]
        #[inline(always)]
        pub fn set_lsr_tx_all_emp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate error in UART receiver fifo."]
        #[inline(always)]
        pub const fn lsr_err_rx_fifo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate error in UART receiver fifo."]
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
    #[doc = "RW, UART modem control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u8);
    impl Mcr {
        #[doc = "RW, UART interrupt output enable."]
        #[inline(always)]
        pub const fn mcr_int_oe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART interrupt output enable."]
        #[inline(always)]
        pub fn set_mcr_int_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, UART control OUT2."]
        #[inline(always)]
        pub const fn mcr_out2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, UART control OUT2."]
        #[inline(always)]
        pub fn set_mcr_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    #[doc = "RO, UART receiver buffer, receiving byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbr(pub u8);
    impl Rbr {
        #[doc = "RO, UART receiver buffer, receiving byte."]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART receiver buffer, receiving byte."]
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
    #[doc = "RO, UART receiver FIFO count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc(pub u8);
    impl Rfc {
        #[doc = "RO, UART receiver FIFO count."]
        #[inline(always)]
        pub const fn rfc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART receiver FIFO count."]
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
    #[doc = "RO, UART transmitter FIFO count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tfc(pub u8);
    impl Tfc {
        #[doc = "RO, UART receiver FIFO count."]
        #[inline(always)]
        pub const fn tfc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, UART receiver FIFO count."]
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
    #[doc = "WO, UART transmitter holding, transmittal byte."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Thr(pub u8);
    impl Thr {
        #[doc = "WO, UART transmitter holding, transmittal byte."]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "WO, UART transmitter holding, transmittal byte."]
        #[inline(always)]
        pub fn set_rbr(&mut self, val: u8) {
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
