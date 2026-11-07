#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "I2C register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, I2C control 1."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RW, I2C control 2."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RW, I2C own address register 1."]
    #[inline(always)]
    pub const fn oaddr1(self) -> crate::common::Reg<regs::Oaddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RW, I2C own address register 2."]
    #[inline(always)]
    pub const fn oaddr2(self) -> crate::common::Reg<regs::Oaddr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RW, I2C data register."]
    #[inline(always)]
    pub const fn datar(self) -> crate::common::Reg<regs::Datar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RO, I2C stauts register 1."]
    #[inline(always)]
    pub const fn star1(self) -> crate::common::Reg<regs::Star1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "RO, I2C status register 2."]
    #[inline(always)]
    pub const fn star2(self) -> crate::common::Reg<regs::Star2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "RW, I2C clock control register."]
    #[inline(always)]
    pub const fn ckcfgr(self) -> crate::common::Reg<regs::Ckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RW, I2C trise register."]
    #[inline(always)]
    pub const fn rtr(self) -> crate::common::Reg<regs::Rtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, I2C clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcfgr(pub u16);
    impl Ckcfgr {
        #[doc = "RW, Controls the SCL clock in Fm/Sm mode (Master mode)."]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RW, Controls the SCL clock in Fm/Sm mode (Master mode)."]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
        #[doc = "RW, Fm mode duty cycle: 0=L/H=2, 1=L/H=16/9."]
        #[inline(always)]
        pub const fn duty(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Fm mode duty cycle: 0=L/H=2, 1=L/H=16/9."]
        #[inline(always)]
        pub fn set_duty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "RW, I2C master mode selection: 0=standard mode, 1=fast mode."]
        #[inline(always)]
        pub const fn f_s(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RW, I2C master mode selection: 0=standard mode, 1=fast mode."]
        #[inline(always)]
        pub fn set_f_s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Ckcfgr {
        #[inline(always)]
        fn default() -> Ckcfgr {
            Ckcfgr(0)
        }
    }
    #[doc = "RW, I2C control 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u16);
    impl Ctrl1 {
        #[doc = "RW, Peripheral enable."]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Peripheral enable."]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW, SMBUS mode: 0=I2C mode, 1=SMBUS mode."]
        #[inline(always)]
        pub const fn smbus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SMBUS mode: 0=I2C mode, 1=SMBUS mode."]
        #[inline(always)]
        pub fn set_smbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RW, SMBus type: 0=Device, 1=Host."]
        #[inline(always)]
        pub const fn smbtype(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SMBus type: 0=Device, 1=Host."]
        #[inline(always)]
        pub fn set_smbtype(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "RW, ARP enable."]
        #[inline(always)]
        pub const fn ebarp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ARP enable."]
        #[inline(always)]
        pub fn set_ebarp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RW, PEC ebable."]
        #[inline(always)]
        pub const fn enpec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PEC ebable."]
        #[inline(always)]
        pub fn set_enpec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "RW, General call enable."]
        #[inline(always)]
        pub const fn engc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, General call enable."]
        #[inline(always)]
        pub fn set_engc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RW, Clock stretching disable (Slave mode)."]
        #[inline(always)]
        pub const fn nostretch(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Clock stretching disable (Slave mode)."]
        #[inline(always)]
        pub fn set_nostretch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RW, Start generation: master mode: 0=no start, 1=repeated start; slave mode: 0=no start, 1=start at bus free."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Start generation: master mode: 0=no start, 1=repeated start; slave mode: 0=no start, 1=start at bus free."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RW, Stop generation: master mode: 0=no stop, 1=stop after the current byte transfer or after the current Start condition is sent; slave mode: 0=no stop, 1=Release the SCL and SDA lines after the current byte transfer."]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Stop generation: master mode: 0=no stop, 1=stop after the current byte transfer or after the current Start condition is sent; slave mode: 0=no stop, 1=Release the SCL and SDA lines after the current byte transfer."]
        #[inline(always)]
        pub fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RW, Acknowledge enable."]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Acknowledge enable."]
        #[inline(always)]
        pub fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RW, Acknowledge/PEC Position (for data reception)."]
        #[inline(always)]
        pub const fn pos(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Acknowledge/PEC Position (for data reception)."]
        #[inline(always)]
        pub fn set_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "RW, Packet error checking: 0=No PEC transfer, 1=PEC transfer (in Tx or Rx mode)."]
        #[inline(always)]
        pub const fn pec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Packet error checking: 0=No PEC transfer, 1=PEC transfer (in Tx or Rx mode)."]
        #[inline(always)]
        pub fn set_pec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "RW, SMBus alert: 0=Releases SMBA pin high, 1=Drives SMBA pin low."]
        #[inline(always)]
        pub const fn alert(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SMBus alert: 0=Releases SMBA pin high, 1=Drives SMBA pin low."]
        #[inline(always)]
        pub fn set_alert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "RW, Software reset."]
        #[inline(always)]
        pub const fn swrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Software reset."]
        #[inline(always)]
        pub fn set_swrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    #[doc = "RW, I2C control 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl2(pub u16);
    impl Ctrl2 {
        #[doc = "RW, Peripheral clock frequency, The minimum allowed frequency is 2 MHz,the maximum frequency is 36 MHz."]
        #[inline(always)]
        pub const fn freq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "RW, Peripheral clock frequency, The minimum allowed frequency is 2 MHz,the maximum frequency is 36 MHz."]
        #[inline(always)]
        pub fn set_freq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[doc = "RW, Error interrupt enable."]
        #[inline(always)]
        pub const fn iterren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Error interrupt enable."]
        #[inline(always)]
        pub fn set_iterren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RW, Event interrupt enable."]
        #[inline(always)]
        pub const fn itevten(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Event interrupt enable."]
        #[inline(always)]
        pub fn set_itevten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RW, Buffer interrupt enable."]
        #[inline(always)]
        pub const fn itbufen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Buffer interrupt enable."]
        #[inline(always)]
        pub fn set_itbufen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
    }
    impl Default for Ctrl2 {
        #[inline(always)]
        fn default() -> Ctrl2 {
            Ctrl2(0)
        }
    }
    #[doc = "RW, I2C data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datar(pub u16);
    impl Datar {
        #[doc = "RW, I2C data register."]
        #[inline(always)]
        pub const fn datar(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, I2C data register."]
        #[inline(always)]
        pub fn set_datar(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for Datar {
        #[inline(always)]
        fn default() -> Datar {
            Datar(0)
        }
    }
    #[doc = "RW, I2C own address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oaddr1(pub u16);
    impl Oaddr1 {
        #[doc = "RW, bit0 of address in 10-bit addressing mode."]
        #[inline(always)]
        pub const fn add0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, bit0 of address in 10-bit addressing mode."]
        #[inline(always)]
        pub fn set_add0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW, bit\\[7:1\\]
of address."]
        #[inline(always)]
        pub const fn add7_1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "RW, bit\\[7:1\\]
of address."]
        #[inline(always)]
        pub fn set_add7_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
        #[doc = "RW, bit\\[9:8\\]
of address in 10-bit addressing mode."]
        #[inline(always)]
        pub const fn add9_8(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "RW, bit\\[9:8\\]
of address in 10-bit addressing mode."]
        #[inline(always)]
        pub fn set_add9_8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "RW, Addressing mode (slave mode): 0=7-bit slave address, 1=10-bit slave address."]
        #[inline(always)]
        pub const fn addmode(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Addressing mode (slave mode): 0=7-bit slave address, 1=10-bit slave address."]
        #[inline(always)]
        pub fn set_addmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Oaddr1 {
        #[inline(always)]
        fn default() -> Oaddr1 {
            Oaddr1(0)
        }
    }
    #[doc = "RW, I2C own address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oaddr2(pub u16);
    impl Oaddr2 {
        #[doc = "RW, Dual addressing mode enable."]
        #[inline(always)]
        pub const fn endual(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Dual addressing mode enable."]
        #[inline(always)]
        pub fn set_endual(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW, bit\\[7:1\\]
of address2."]
        #[inline(always)]
        pub const fn add2(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "RW, bit\\[7:1\\]
of address2."]
        #[inline(always)]
        pub fn set_add2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
    }
    impl Default for Oaddr2 {
        #[inline(always)]
        fn default() -> Oaddr2 {
            Oaddr2(0)
        }
    }
    #[doc = "RW, I2C trise register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtr(pub u16);
    impl Rtr {
        #[doc = "RW, Maximum rise time in Fm/Sm mode (Master mode)."]
        #[inline(always)]
        pub const fn trise(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "RW, Maximum rise time in Fm/Sm mode (Master mode)."]
        #[inline(always)]
        pub fn set_trise(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
    }
    impl Default for Rtr {
        #[inline(always)]
        fn default() -> Rtr {
            Rtr(0)
        }
    }
    #[doc = "RO, I2C stauts register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star1(pub u16);
    impl Star1 {
        #[doc = "RW0, Start bit flag (Master mode)."]
        #[inline(always)]
        pub const fn sb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Start bit flag (Master mode)."]
        #[inline(always)]
        pub fn set_sb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW0, Address sent (master mode)/matched (slave mode) flag."]
        #[inline(always)]
        pub const fn addr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Address sent (master mode)/matched (slave mode) flag."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RO, Byte transfer finished flag."]
        #[inline(always)]
        pub const fn btf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Byte transfer finished flag."]
        #[inline(always)]
        pub fn set_btf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "RO, 10-bit header sent flag (Master mode)."]
        #[inline(always)]
        pub const fn add10(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, 10-bit header sent flag (Master mode)."]
        #[inline(always)]
        pub fn set_add10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "RO, Stop detection flag (slave mode)."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Stop detection flag (slave mode)."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RO, Data register not empty flag (receivers)."]
        #[inline(always)]
        pub const fn rx_ne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Data register not empty flag (receivers)."]
        #[inline(always)]
        pub fn set_rx_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RO, Data register empty flag (transmitters)."]
        #[inline(always)]
        pub const fn tx_e(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Data register empty flag (transmitters)."]
        #[inline(always)]
        pub fn set_tx_e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RW0, Bus error flag."]
        #[inline(always)]
        pub const fn berr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Bus error flag."]
        #[inline(always)]
        pub fn set_berr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RW0, Arbitration lost flag (master mode)."]
        #[inline(always)]
        pub const fn arlo(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Arbitration lost flag (master mode)."]
        #[inline(always)]
        pub fn set_arlo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RW0, Acknowledge failure flag."]
        #[inline(always)]
        pub const fn af(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Acknowledge failure flag."]
        #[inline(always)]
        pub fn set_af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RW0, Overrun/Underrun flag."]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Overrun/Underrun flag."]
        #[inline(always)]
        pub fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "RW0, PEC Error flag in reception."]
        #[inline(always)]
        pub const fn pecerr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, PEC Error flag in reception."]
        #[inline(always)]
        pub fn set_pecerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "RW0, Timeout or Tlow error flag."]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, Timeout or Tlow error flag."]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "RW0, SMBus alert flag."]
        #[inline(always)]
        pub const fn smbalert(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RW0, SMBus alert flag."]
        #[inline(always)]
        pub fn set_smbalert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Star1 {
        #[inline(always)]
        fn default() -> Star1 {
            Star1(0)
        }
    }
    #[doc = "RO, I2C status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star2(pub u16);
    impl Star2 {
        #[doc = "RO, Mode statu: 0=Slave mode, 1=Master mode."]
        #[inline(always)]
        pub const fn msl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Mode statu: 0=Slave mode, 1=Master mode."]
        #[inline(always)]
        pub fn set_msl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RO, Bus busy flag."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Bus busy flag."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RO, Trans flag: 0=data bytes received, 1=data bytes transmitted."]
        #[inline(always)]
        pub const fn tra(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Trans flag: 0=data bytes received, 1=data bytes transmitted."]
        #[inline(always)]
        pub fn set_tra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "RO, General call address (Slave mode) received flag."]
        #[inline(always)]
        pub const fn gencall(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, General call address (Slave mode) received flag."]
        #[inline(always)]
        pub fn set_gencall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RO, SMBus device default address (Slave mode) received flag."]
        #[inline(always)]
        pub const fn smbdefault(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, SMBus device default address (Slave mode) received flag."]
        #[inline(always)]
        pub fn set_smbdefault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "RO, SMBus host header (Slave mode) received flag."]
        #[inline(always)]
        pub const fn smbhost(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, SMBus host header (Slave mode) received flag."]
        #[inline(always)]
        pub fn set_smbhost(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RO, Dual flag (Slave mode): 0=Received address matched with OAR1, 1=Received address matched with OAR2."]
        #[inline(always)]
        pub const fn dualf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Dual flag (Slave mode): 0=Received address matched with OAR1, 1=Received address matched with OAR2."]
        #[inline(always)]
        pub fn set_dualf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RO, Packet error checking register."]
        #[inline(always)]
        pub const fn pecx(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RO, Packet error checking register."]
        #[inline(always)]
        pub fn set_pecx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Star2 {
        #[inline(always)]
        fn default() -> Star2 {
            Star2(0)
        }
    }
}
