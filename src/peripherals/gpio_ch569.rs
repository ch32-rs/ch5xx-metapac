#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GPIO register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO interrupt flag; write 1 to clear."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPIO interrupt enable."]
    #[inline(always)]
    pub const fn int_enable(self) -> crate::common::Reg<regs::IntEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "GPIO interrupt mode."]
    #[inline(always)]
    pub const fn int_mode(self) -> crate::common::Reg<regs::IntMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "GPIO interrupt polarity."]
    #[inline(always)]
    pub const fn int_polar(self) -> crate::common::Reg<regs::IntPolar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "GPIO PA I/O direction."]
    #[inline(always)]
    pub const fn pa_dir(self) -> crate::common::Reg<regs::PaDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "GPIO PA input."]
    #[inline(always)]
    pub const fn pa_pin(self) -> crate::common::Reg<regs::PaPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GPIO PA output."]
    #[inline(always)]
    pub const fn pa_out(self) -> crate::common::Reg<regs::PaOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "GPIO PA clear output."]
    #[inline(always)]
    pub const fn pa_clr(self) -> crate::common::Reg<regs::PaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "GPIO PA pullup resistance enable."]
    #[inline(always)]
    pub const fn pa_pu(self) -> crate::common::Reg<regs::PaPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GPIO PA output open-drain and input pulldown resistance enable."]
    #[inline(always)]
    pub const fn pa_pd(self) -> crate::common::Reg<regs::PaPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "GPIO PA driving capability."]
    #[inline(always)]
    pub const fn pa_drv(self) -> crate::common::Reg<regs::PaDrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "GPIO PA output slew rate and input schmitt trigger."]
    #[inline(always)]
    pub const fn pa_smt(self) -> crate::common::Reg<regs::PaSmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "GPIO PB I/O direction."]
    #[inline(always)]
    pub const fn pb_dir(self) -> crate::common::Reg<regs::PbDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "GPIO PB input."]
    #[inline(always)]
    pub const fn pb_pin(self) -> crate::common::Reg<regs::PbPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "GPIO PB output."]
    #[inline(always)]
    pub const fn pb_out(self) -> crate::common::Reg<regs::PbOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "GPIO PB clear output."]
    #[inline(always)]
    pub const fn pb_clr(self) -> crate::common::Reg<regs::PbClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "GPIO PB pullup resistance enable."]
    #[inline(always)]
    pub const fn pb_pu(self) -> crate::common::Reg<regs::PbPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "GPIO PB output open-drain and input pulldown resistance enable."]
    #[inline(always)]
    pub const fn pb_pd(self) -> crate::common::Reg<regs::PbPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "GPIO PB driving capability."]
    #[inline(always)]
    pub const fn pb_drv(self) -> crate::common::Reg<regs::PbDrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "GPIO PB output slew rate and input schmitt trigger."]
    #[inline(always)]
    pub const fn pb_smt(self) -> crate::common::Reg<regs::PbSmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "GPIO interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEnable(pub u8);
    impl IntEnable {
        #[doc = "PA2 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pa2_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA2 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pa2_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PA3 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pa3_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA3 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pa3_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PA4 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pa4_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA4 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pa4_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PB3 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pb3_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PB3 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pb3_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "PB4 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pb4_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PB4 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pb4_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PB11 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PB11 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pb11_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "PB12 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PB12 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pb12_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "PB15 pin interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PB15 pin interrupt enable."]
        #[inline(always)]
        pub const fn set_pb15_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntEnable {
        #[inline(always)]
        fn default() -> IntEnable {
            IntEnable(0)
        }
    }
    impl core::fmt::Debug for IntEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEnable")
                .field("pa2_ie", &self.pa2_ie())
                .field("pa3_ie", &self.pa3_ie())
                .field("pa4_ie", &self.pa4_ie())
                .field("pb3_ie", &self.pb3_ie())
                .field("pb4_ie", &self.pb4_ie())
                .field("pb11_ie", &self.pb11_ie())
                .field("pb12_ie", &self.pb12_ie())
                .field("pb15_ie", &self.pb15_ie())
                .finish()
        }
    }
    #[doc = "GPIO interrupt flag; write 1 to clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "PA2 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pa2_if(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA2 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pa2_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PA3 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pa3_if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA3 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pa3_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PA4 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pa4_if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA4 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pa4_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PB3 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pb3_if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PB3 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pb3_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "PB4 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pb4_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PB4 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pb4_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PB11 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11_if(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PB11 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pb11_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "PB12 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PB12 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pb12_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "PB15 pin interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15_if(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PB15 pin interrupt flag."]
        #[inline(always)]
        pub const fn set_pb15_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntFlag {
        #[inline(always)]
        fn default() -> IntFlag {
            IntFlag(0)
        }
    }
    impl core::fmt::Debug for IntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntFlag")
                .field("pa2_if", &self.pa2_if())
                .field("pa3_if", &self.pa3_if())
                .field("pa4_if", &self.pa4_if())
                .field("pb3_if", &self.pb3_if())
                .field("pb4_if", &self.pb4_if())
                .field("pb11_if", &self.pb11_if())
                .field("pb12_if", &self.pb12_if())
                .field("pb15_if", &self.pb15_if())
                .finish()
        }
    }
    #[doc = "GPIO interrupt trigger mode: 1=edge, 0=level."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMode(pub u8);
    impl IntMode {
        #[doc = "PA2 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pa2_im(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA2 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pa2_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PA3 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pa3_im(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA3 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pa3_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PA4 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pa4_im(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA4 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pa4_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PB3 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pb3_im(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PB3 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pb3_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "PB4 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pb4_im(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PB4 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pb4_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PB11 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11_im(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PB11 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pb11_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "PB12 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12_im(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PB12 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pb12_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "PB15 pin interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15_im(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PB15 pin interrupt mode."]
        #[inline(always)]
        pub const fn set_pb15_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntMode {
        #[inline(always)]
        fn default() -> IntMode {
            IntMode(0)
        }
    }
    impl core::fmt::Debug for IntMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntMode")
                .field("pa2_im", &self.pa2_im())
                .field("pa3_im", &self.pa3_im())
                .field("pa4_im", &self.pa4_im())
                .field("pb3_im", &self.pb3_im())
                .field("pb4_im", &self.pb4_im())
                .field("pb11_im", &self.pb11_im())
                .field("pb12_im", &self.pb12_im())
                .field("pb15_im", &self.pb15_im())
                .finish()
        }
    }
    #[doc = "GPIO interrupt polarity: 1=high level or rising edge, 0=low level or falling edge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntPolar(pub u8);
    impl IntPolar {
        #[doc = "PA2 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pa2_ip(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA2 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pa2_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PA3 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pa3_ip(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PA3 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pa3_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PA4 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pa4_ip(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PA4 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pa4_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PB3 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pb3_ip(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PB3 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pb3_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "PB4 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pb4_ip(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PB4 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pb4_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PB11 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pb11_ip(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PB11 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pb11_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "PB12 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pb12_ip(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PB12 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pb12_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "PB15 pin interrupt polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pb15_ip(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PB15 pin interrupt polarity."]
        #[inline(always)]
        pub const fn set_pb15_ip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntPolar {
        #[inline(always)]
        fn default() -> IntPolar {
            IntPolar(0)
        }
    }
    impl core::fmt::Debug for IntPolar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntPolar")
                .field("pa2_ip", &self.pa2_ip())
                .field("pa3_ip", &self.pa3_ip())
                .field("pa4_ip", &self.pa4_ip())
                .field("pb3_ip", &self.pb3_ip())
                .field("pb4_ip", &self.pb4_ip())
                .field("pb11_ip", &self.pb11_ip())
                .field("pb12_ip", &self.pb12_ip())
                .field("pb15_ip", &self.pb15_ip())
                .finish()
        }
    }
    #[doc = "GPIO PA clear output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaClr(pub u32);
    impl PaClr {
        #[doc = "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_clr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pa_clr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaClr {
        #[inline(always)]
        fn default() -> PaClr {
            PaClr(0)
        }
    }
    impl core::fmt::Debug for PaClr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaClr")
                .field("pa_clr[0]", &self.pa_clr(0usize))
                .field("pa_clr[1]", &self.pa_clr(1usize))
                .field("pa_clr[2]", &self.pa_clr(2usize))
                .field("pa_clr[3]", &self.pa_clr(3usize))
                .field("pa_clr[4]", &self.pa_clr(4usize))
                .field("pa_clr[5]", &self.pa_clr(5usize))
                .field("pa_clr[6]", &self.pa_clr(6usize))
                .field("pa_clr[7]", &self.pa_clr(7usize))
                .field("pa_clr[8]", &self.pa_clr(8usize))
                .field("pa_clr[9]", &self.pa_clr(9usize))
                .field("pa_clr[10]", &self.pa_clr(10usize))
                .field("pa_clr[11]", &self.pa_clr(11usize))
                .field("pa_clr[12]", &self.pa_clr(12usize))
                .field("pa_clr[13]", &self.pa_clr(13usize))
                .field("pa_clr[14]", &self.pa_clr(14usize))
                .field("pa_clr[15]", &self.pa_clr(15usize))
                .field("pa_clr[16]", &self.pa_clr(16usize))
                .field("pa_clr[17]", &self.pa_clr(17usize))
                .field("pa_clr[18]", &self.pa_clr(18usize))
                .field("pa_clr[19]", &self.pa_clr(19usize))
                .field("pa_clr[20]", &self.pa_clr(20usize))
                .field("pa_clr[21]", &self.pa_clr(21usize))
                .field("pa_clr[22]", &self.pa_clr(22usize))
                .field("pa_clr[23]", &self.pa_clr(23usize))
                .field("pa_clr[24]", &self.pa_clr(24usize))
                .field("pa_clr[25]", &self.pa_clr(25usize))
                .field("pa_clr[26]", &self.pa_clr(26usize))
                .field("pa_clr[27]", &self.pa_clr(27usize))
                .field("pa_clr[28]", &self.pa_clr(28usize))
                .field("pa_clr[29]", &self.pa_clr(29usize))
                .field("pa_clr[30]", &self.pa_clr(30usize))
                .field("pa_clr[31]", &self.pa_clr(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA I/O direction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaDir(pub u32);
    impl PaDir {
        #[doc = "GPIO PA I/O direction: 0=input, 1=output."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn set_pa_dir(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaDir {
        #[inline(always)]
        fn default() -> PaDir {
            PaDir(0)
        }
    }
    impl core::fmt::Debug for PaDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaDir")
                .field("pa_dir[0]", &self.pa_dir(0usize))
                .field("pa_dir[1]", &self.pa_dir(1usize))
                .field("pa_dir[2]", &self.pa_dir(2usize))
                .field("pa_dir[3]", &self.pa_dir(3usize))
                .field("pa_dir[4]", &self.pa_dir(4usize))
                .field("pa_dir[5]", &self.pa_dir(5usize))
                .field("pa_dir[6]", &self.pa_dir(6usize))
                .field("pa_dir[7]", &self.pa_dir(7usize))
                .field("pa_dir[8]", &self.pa_dir(8usize))
                .field("pa_dir[9]", &self.pa_dir(9usize))
                .field("pa_dir[10]", &self.pa_dir(10usize))
                .field("pa_dir[11]", &self.pa_dir(11usize))
                .field("pa_dir[12]", &self.pa_dir(12usize))
                .field("pa_dir[13]", &self.pa_dir(13usize))
                .field("pa_dir[14]", &self.pa_dir(14usize))
                .field("pa_dir[15]", &self.pa_dir(15usize))
                .field("pa_dir[16]", &self.pa_dir(16usize))
                .field("pa_dir[17]", &self.pa_dir(17usize))
                .field("pa_dir[18]", &self.pa_dir(18usize))
                .field("pa_dir[19]", &self.pa_dir(19usize))
                .field("pa_dir[20]", &self.pa_dir(20usize))
                .field("pa_dir[21]", &self.pa_dir(21usize))
                .field("pa_dir[22]", &self.pa_dir(22usize))
                .field("pa_dir[23]", &self.pa_dir(23usize))
                .field("pa_dir[24]", &self.pa_dir(24usize))
                .field("pa_dir[25]", &self.pa_dir(25usize))
                .field("pa_dir[26]", &self.pa_dir(26usize))
                .field("pa_dir[27]", &self.pa_dir(27usize))
                .field("pa_dir[28]", &self.pa_dir(28usize))
                .field("pa_dir[29]", &self.pa_dir(29usize))
                .field("pa_dir[30]", &self.pa_dir(30usize))
                .field("pa_dir[31]", &self.pa_dir(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA driving capability."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaDrv(pub u32);
    impl PaDrv {
        #[doc = "GPIO PA output drive current: 1=max 16mA, 0=max 8mA."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_drv(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA output drive current: 1=max 16mA, 0=max 8mA."]
        #[inline(always)]
        pub const fn set_pa_drv(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaDrv {
        #[inline(always)]
        fn default() -> PaDrv {
            PaDrv(0)
        }
    }
    impl core::fmt::Debug for PaDrv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaDrv")
                .field("pa_drv[0]", &self.pa_drv(0usize))
                .field("pa_drv[1]", &self.pa_drv(1usize))
                .field("pa_drv[2]", &self.pa_drv(2usize))
                .field("pa_drv[3]", &self.pa_drv(3usize))
                .field("pa_drv[4]", &self.pa_drv(4usize))
                .field("pa_drv[5]", &self.pa_drv(5usize))
                .field("pa_drv[6]", &self.pa_drv(6usize))
                .field("pa_drv[7]", &self.pa_drv(7usize))
                .field("pa_drv[8]", &self.pa_drv(8usize))
                .field("pa_drv[9]", &self.pa_drv(9usize))
                .field("pa_drv[10]", &self.pa_drv(10usize))
                .field("pa_drv[11]", &self.pa_drv(11usize))
                .field("pa_drv[12]", &self.pa_drv(12usize))
                .field("pa_drv[13]", &self.pa_drv(13usize))
                .field("pa_drv[14]", &self.pa_drv(14usize))
                .field("pa_drv[15]", &self.pa_drv(15usize))
                .field("pa_drv[16]", &self.pa_drv(16usize))
                .field("pa_drv[17]", &self.pa_drv(17usize))
                .field("pa_drv[18]", &self.pa_drv(18usize))
                .field("pa_drv[19]", &self.pa_drv(19usize))
                .field("pa_drv[20]", &self.pa_drv(20usize))
                .field("pa_drv[21]", &self.pa_drv(21usize))
                .field("pa_drv[22]", &self.pa_drv(22usize))
                .field("pa_drv[23]", &self.pa_drv(23usize))
                .field("pa_drv[24]", &self.pa_drv(24usize))
                .field("pa_drv[25]", &self.pa_drv(25usize))
                .field("pa_drv[26]", &self.pa_drv(26usize))
                .field("pa_drv[27]", &self.pa_drv(27usize))
                .field("pa_drv[28]", &self.pa_drv(28usize))
                .field("pa_drv[29]", &self.pa_drv(29usize))
                .field("pa_drv[30]", &self.pa_drv(30usize))
                .field("pa_drv[31]", &self.pa_drv(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaOut(pub u32);
    impl PaOut {
        #[doc = "GPIO PA output level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA output level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pa_out(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaOut {
        #[inline(always)]
        fn default() -> PaOut {
            PaOut(0)
        }
    }
    impl core::fmt::Debug for PaOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaOut")
                .field("pa_out[0]", &self.pa_out(0usize))
                .field("pa_out[1]", &self.pa_out(1usize))
                .field("pa_out[2]", &self.pa_out(2usize))
                .field("pa_out[3]", &self.pa_out(3usize))
                .field("pa_out[4]", &self.pa_out(4usize))
                .field("pa_out[5]", &self.pa_out(5usize))
                .field("pa_out[6]", &self.pa_out(6usize))
                .field("pa_out[7]", &self.pa_out(7usize))
                .field("pa_out[8]", &self.pa_out(8usize))
                .field("pa_out[9]", &self.pa_out(9usize))
                .field("pa_out[10]", &self.pa_out(10usize))
                .field("pa_out[11]", &self.pa_out(11usize))
                .field("pa_out[12]", &self.pa_out(12usize))
                .field("pa_out[13]", &self.pa_out(13usize))
                .field("pa_out[14]", &self.pa_out(14usize))
                .field("pa_out[15]", &self.pa_out(15usize))
                .field("pa_out[16]", &self.pa_out(16usize))
                .field("pa_out[17]", &self.pa_out(17usize))
                .field("pa_out[18]", &self.pa_out(18usize))
                .field("pa_out[19]", &self.pa_out(19usize))
                .field("pa_out[20]", &self.pa_out(20usize))
                .field("pa_out[21]", &self.pa_out(21usize))
                .field("pa_out[22]", &self.pa_out(22usize))
                .field("pa_out[23]", &self.pa_out(23usize))
                .field("pa_out[24]", &self.pa_out(24usize))
                .field("pa_out[25]", &self.pa_out(25usize))
                .field("pa_out[26]", &self.pa_out(26usize))
                .field("pa_out[27]", &self.pa_out(27usize))
                .field("pa_out[28]", &self.pa_out(28usize))
                .field("pa_out[29]", &self.pa_out(29usize))
                .field("pa_out[30]", &self.pa_out(30usize))
                .field("pa_out[31]", &self.pa_out(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA output open-drain and input pulldown resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPd(pub u32);
    impl PaPd {
        #[doc = "GPIO PA open-drain output (output mode) / pull-down resistor (input mode): 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA open-drain output (output mode) / pull-down resistor (input mode): 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pa_pd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPd {
        #[inline(always)]
        fn default() -> PaPd {
            PaPd(0)
        }
    }
    impl core::fmt::Debug for PaPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPd")
                .field("pa_pd[0]", &self.pa_pd(0usize))
                .field("pa_pd[1]", &self.pa_pd(1usize))
                .field("pa_pd[2]", &self.pa_pd(2usize))
                .field("pa_pd[3]", &self.pa_pd(3usize))
                .field("pa_pd[4]", &self.pa_pd(4usize))
                .field("pa_pd[5]", &self.pa_pd(5usize))
                .field("pa_pd[6]", &self.pa_pd(6usize))
                .field("pa_pd[7]", &self.pa_pd(7usize))
                .field("pa_pd[8]", &self.pa_pd(8usize))
                .field("pa_pd[9]", &self.pa_pd(9usize))
                .field("pa_pd[10]", &self.pa_pd(10usize))
                .field("pa_pd[11]", &self.pa_pd(11usize))
                .field("pa_pd[12]", &self.pa_pd(12usize))
                .field("pa_pd[13]", &self.pa_pd(13usize))
                .field("pa_pd[14]", &self.pa_pd(14usize))
                .field("pa_pd[15]", &self.pa_pd(15usize))
                .field("pa_pd[16]", &self.pa_pd(16usize))
                .field("pa_pd[17]", &self.pa_pd(17usize))
                .field("pa_pd[18]", &self.pa_pd(18usize))
                .field("pa_pd[19]", &self.pa_pd(19usize))
                .field("pa_pd[20]", &self.pa_pd(20usize))
                .field("pa_pd[21]", &self.pa_pd(21usize))
                .field("pa_pd[22]", &self.pa_pd(22usize))
                .field("pa_pd[23]", &self.pa_pd(23usize))
                .field("pa_pd[24]", &self.pa_pd(24usize))
                .field("pa_pd[25]", &self.pa_pd(25usize))
                .field("pa_pd[26]", &self.pa_pd(26usize))
                .field("pa_pd[27]", &self.pa_pd(27usize))
                .field("pa_pd[28]", &self.pa_pd(28usize))
                .field("pa_pd[29]", &self.pa_pd(29usize))
                .field("pa_pd[30]", &self.pa_pd(30usize))
                .field("pa_pd[31]", &self.pa_pd(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPin(pub u32);
    impl PaPin {
        #[doc = "GPIO PA input level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pa_pin(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPin {
        #[inline(always)]
        fn default() -> PaPin {
            PaPin(0)
        }
    }
    impl core::fmt::Debug for PaPin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPin")
                .field("pa_pin[0]", &self.pa_pin(0usize))
                .field("pa_pin[1]", &self.pa_pin(1usize))
                .field("pa_pin[2]", &self.pa_pin(2usize))
                .field("pa_pin[3]", &self.pa_pin(3usize))
                .field("pa_pin[4]", &self.pa_pin(4usize))
                .field("pa_pin[5]", &self.pa_pin(5usize))
                .field("pa_pin[6]", &self.pa_pin(6usize))
                .field("pa_pin[7]", &self.pa_pin(7usize))
                .field("pa_pin[8]", &self.pa_pin(8usize))
                .field("pa_pin[9]", &self.pa_pin(9usize))
                .field("pa_pin[10]", &self.pa_pin(10usize))
                .field("pa_pin[11]", &self.pa_pin(11usize))
                .field("pa_pin[12]", &self.pa_pin(12usize))
                .field("pa_pin[13]", &self.pa_pin(13usize))
                .field("pa_pin[14]", &self.pa_pin(14usize))
                .field("pa_pin[15]", &self.pa_pin(15usize))
                .field("pa_pin[16]", &self.pa_pin(16usize))
                .field("pa_pin[17]", &self.pa_pin(17usize))
                .field("pa_pin[18]", &self.pa_pin(18usize))
                .field("pa_pin[19]", &self.pa_pin(19usize))
                .field("pa_pin[20]", &self.pa_pin(20usize))
                .field("pa_pin[21]", &self.pa_pin(21usize))
                .field("pa_pin[22]", &self.pa_pin(22usize))
                .field("pa_pin[23]", &self.pa_pin(23usize))
                .field("pa_pin[24]", &self.pa_pin(24usize))
                .field("pa_pin[25]", &self.pa_pin(25usize))
                .field("pa_pin[26]", &self.pa_pin(26usize))
                .field("pa_pin[27]", &self.pa_pin(27usize))
                .field("pa_pin[28]", &self.pa_pin(28usize))
                .field("pa_pin[29]", &self.pa_pin(29usize))
                .field("pa_pin[30]", &self.pa_pin(30usize))
                .field("pa_pin[31]", &self.pa_pin(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPu(pub u32);
    impl PaPu {
        #[doc = "GPIO PA pull-up resistor: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pa_pu(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPu {
        #[inline(always)]
        fn default() -> PaPu {
            PaPu(0)
        }
    }
    impl core::fmt::Debug for PaPu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPu")
                .field("pa_pu[0]", &self.pa_pu(0usize))
                .field("pa_pu[1]", &self.pa_pu(1usize))
                .field("pa_pu[2]", &self.pa_pu(2usize))
                .field("pa_pu[3]", &self.pa_pu(3usize))
                .field("pa_pu[4]", &self.pa_pu(4usize))
                .field("pa_pu[5]", &self.pa_pu(5usize))
                .field("pa_pu[6]", &self.pa_pu(6usize))
                .field("pa_pu[7]", &self.pa_pu(7usize))
                .field("pa_pu[8]", &self.pa_pu(8usize))
                .field("pa_pu[9]", &self.pa_pu(9usize))
                .field("pa_pu[10]", &self.pa_pu(10usize))
                .field("pa_pu[11]", &self.pa_pu(11usize))
                .field("pa_pu[12]", &self.pa_pu(12usize))
                .field("pa_pu[13]", &self.pa_pu(13usize))
                .field("pa_pu[14]", &self.pa_pu(14usize))
                .field("pa_pu[15]", &self.pa_pu(15usize))
                .field("pa_pu[16]", &self.pa_pu(16usize))
                .field("pa_pu[17]", &self.pa_pu(17usize))
                .field("pa_pu[18]", &self.pa_pu(18usize))
                .field("pa_pu[19]", &self.pa_pu(19usize))
                .field("pa_pu[20]", &self.pa_pu(20usize))
                .field("pa_pu[21]", &self.pa_pu(21usize))
                .field("pa_pu[22]", &self.pa_pu(22usize))
                .field("pa_pu[23]", &self.pa_pu(23usize))
                .field("pa_pu[24]", &self.pa_pu(24usize))
                .field("pa_pu[25]", &self.pa_pu(25usize))
                .field("pa_pu[26]", &self.pa_pu(26usize))
                .field("pa_pu[27]", &self.pa_pu(27usize))
                .field("pa_pu[28]", &self.pa_pu(28usize))
                .field("pa_pu[29]", &self.pa_pu(29usize))
                .field("pa_pu[30]", &self.pa_pu(30usize))
                .field("pa_pu[31]", &self.pa_pu(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA output slew rate and input schmitt trigger."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaSmt(pub u32);
    impl PaSmt {
        #[doc = "GPIO PA low-slew-rate output (output mode) / Schmitt-trigger input (input mode): 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_smt(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA low-slew-rate output (output mode) / Schmitt-trigger input (input mode): 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pa_smt(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaSmt {
        #[inline(always)]
        fn default() -> PaSmt {
            PaSmt(0)
        }
    }
    impl core::fmt::Debug for PaSmt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaSmt")
                .field("pa_smt[0]", &self.pa_smt(0usize))
                .field("pa_smt[1]", &self.pa_smt(1usize))
                .field("pa_smt[2]", &self.pa_smt(2usize))
                .field("pa_smt[3]", &self.pa_smt(3usize))
                .field("pa_smt[4]", &self.pa_smt(4usize))
                .field("pa_smt[5]", &self.pa_smt(5usize))
                .field("pa_smt[6]", &self.pa_smt(6usize))
                .field("pa_smt[7]", &self.pa_smt(7usize))
                .field("pa_smt[8]", &self.pa_smt(8usize))
                .field("pa_smt[9]", &self.pa_smt(9usize))
                .field("pa_smt[10]", &self.pa_smt(10usize))
                .field("pa_smt[11]", &self.pa_smt(11usize))
                .field("pa_smt[12]", &self.pa_smt(12usize))
                .field("pa_smt[13]", &self.pa_smt(13usize))
                .field("pa_smt[14]", &self.pa_smt(14usize))
                .field("pa_smt[15]", &self.pa_smt(15usize))
                .field("pa_smt[16]", &self.pa_smt(16usize))
                .field("pa_smt[17]", &self.pa_smt(17usize))
                .field("pa_smt[18]", &self.pa_smt(18usize))
                .field("pa_smt[19]", &self.pa_smt(19usize))
                .field("pa_smt[20]", &self.pa_smt(20usize))
                .field("pa_smt[21]", &self.pa_smt(21usize))
                .field("pa_smt[22]", &self.pa_smt(22usize))
                .field("pa_smt[23]", &self.pa_smt(23usize))
                .field("pa_smt[24]", &self.pa_smt(24usize))
                .field("pa_smt[25]", &self.pa_smt(25usize))
                .field("pa_smt[26]", &self.pa_smt(26usize))
                .field("pa_smt[27]", &self.pa_smt(27usize))
                .field("pa_smt[28]", &self.pa_smt(28usize))
                .field("pa_smt[29]", &self.pa_smt(29usize))
                .field("pa_smt[30]", &self.pa_smt(30usize))
                .field("pa_smt[31]", &self.pa_smt(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB clear output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbClr(pub u32);
    impl PbClr {
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_clr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pb_clr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbClr {
        #[inline(always)]
        fn default() -> PbClr {
            PbClr(0)
        }
    }
    impl core::fmt::Debug for PbClr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbClr")
                .field("pb_clr[0]", &self.pb_clr(0usize))
                .field("pb_clr[1]", &self.pb_clr(1usize))
                .field("pb_clr[2]", &self.pb_clr(2usize))
                .field("pb_clr[3]", &self.pb_clr(3usize))
                .field("pb_clr[4]", &self.pb_clr(4usize))
                .field("pb_clr[5]", &self.pb_clr(5usize))
                .field("pb_clr[6]", &self.pb_clr(6usize))
                .field("pb_clr[7]", &self.pb_clr(7usize))
                .field("pb_clr[8]", &self.pb_clr(8usize))
                .field("pb_clr[9]", &self.pb_clr(9usize))
                .field("pb_clr[10]", &self.pb_clr(10usize))
                .field("pb_clr[11]", &self.pb_clr(11usize))
                .field("pb_clr[12]", &self.pb_clr(12usize))
                .field("pb_clr[13]", &self.pb_clr(13usize))
                .field("pb_clr[14]", &self.pb_clr(14usize))
                .field("pb_clr[15]", &self.pb_clr(15usize))
                .field("pb_clr[16]", &self.pb_clr(16usize))
                .field("pb_clr[17]", &self.pb_clr(17usize))
                .field("pb_clr[18]", &self.pb_clr(18usize))
                .field("pb_clr[19]", &self.pb_clr(19usize))
                .field("pb_clr[20]", &self.pb_clr(20usize))
                .field("pb_clr[21]", &self.pb_clr(21usize))
                .field("pb_clr[22]", &self.pb_clr(22usize))
                .field("pb_clr[23]", &self.pb_clr(23usize))
                .field("pb_clr[24]", &self.pb_clr(24usize))
                .field("pb_clr[25]", &self.pb_clr(25usize))
                .field("pb_clr[26]", &self.pb_clr(26usize))
                .field("pb_clr[27]", &self.pb_clr(27usize))
                .field("pb_clr[28]", &self.pb_clr(28usize))
                .field("pb_clr[29]", &self.pb_clr(29usize))
                .field("pb_clr[30]", &self.pb_clr(30usize))
                .field("pb_clr[31]", &self.pb_clr(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB I/O direction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbDir(pub u32);
    impl PbDir {
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn set_pb_dir(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbDir {
        #[inline(always)]
        fn default() -> PbDir {
            PbDir(0)
        }
    }
    impl core::fmt::Debug for PbDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbDir")
                .field("pb_dir[0]", &self.pb_dir(0usize))
                .field("pb_dir[1]", &self.pb_dir(1usize))
                .field("pb_dir[2]", &self.pb_dir(2usize))
                .field("pb_dir[3]", &self.pb_dir(3usize))
                .field("pb_dir[4]", &self.pb_dir(4usize))
                .field("pb_dir[5]", &self.pb_dir(5usize))
                .field("pb_dir[6]", &self.pb_dir(6usize))
                .field("pb_dir[7]", &self.pb_dir(7usize))
                .field("pb_dir[8]", &self.pb_dir(8usize))
                .field("pb_dir[9]", &self.pb_dir(9usize))
                .field("pb_dir[10]", &self.pb_dir(10usize))
                .field("pb_dir[11]", &self.pb_dir(11usize))
                .field("pb_dir[12]", &self.pb_dir(12usize))
                .field("pb_dir[13]", &self.pb_dir(13usize))
                .field("pb_dir[14]", &self.pb_dir(14usize))
                .field("pb_dir[15]", &self.pb_dir(15usize))
                .field("pb_dir[16]", &self.pb_dir(16usize))
                .field("pb_dir[17]", &self.pb_dir(17usize))
                .field("pb_dir[18]", &self.pb_dir(18usize))
                .field("pb_dir[19]", &self.pb_dir(19usize))
                .field("pb_dir[20]", &self.pb_dir(20usize))
                .field("pb_dir[21]", &self.pb_dir(21usize))
                .field("pb_dir[22]", &self.pb_dir(22usize))
                .field("pb_dir[23]", &self.pb_dir(23usize))
                .field("pb_dir[24]", &self.pb_dir(24usize))
                .field("pb_dir[25]", &self.pb_dir(25usize))
                .field("pb_dir[26]", &self.pb_dir(26usize))
                .field("pb_dir[27]", &self.pb_dir(27usize))
                .field("pb_dir[28]", &self.pb_dir(28usize))
                .field("pb_dir[29]", &self.pb_dir(29usize))
                .field("pb_dir[30]", &self.pb_dir(30usize))
                .field("pb_dir[31]", &self.pb_dir(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB driving capability."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbDrv(pub u32);
    impl PbDrv {
        #[doc = "GPIO PB output drive current: 1=max 16mA, 0=max 8mA."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_drv(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB output drive current: 1=max 16mA, 0=max 8mA."]
        #[inline(always)]
        pub const fn set_pb_drv(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbDrv {
        #[inline(always)]
        fn default() -> PbDrv {
            PbDrv(0)
        }
    }
    impl core::fmt::Debug for PbDrv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbDrv")
                .field("pb_drv[0]", &self.pb_drv(0usize))
                .field("pb_drv[1]", &self.pb_drv(1usize))
                .field("pb_drv[2]", &self.pb_drv(2usize))
                .field("pb_drv[3]", &self.pb_drv(3usize))
                .field("pb_drv[4]", &self.pb_drv(4usize))
                .field("pb_drv[5]", &self.pb_drv(5usize))
                .field("pb_drv[6]", &self.pb_drv(6usize))
                .field("pb_drv[7]", &self.pb_drv(7usize))
                .field("pb_drv[8]", &self.pb_drv(8usize))
                .field("pb_drv[9]", &self.pb_drv(9usize))
                .field("pb_drv[10]", &self.pb_drv(10usize))
                .field("pb_drv[11]", &self.pb_drv(11usize))
                .field("pb_drv[12]", &self.pb_drv(12usize))
                .field("pb_drv[13]", &self.pb_drv(13usize))
                .field("pb_drv[14]", &self.pb_drv(14usize))
                .field("pb_drv[15]", &self.pb_drv(15usize))
                .field("pb_drv[16]", &self.pb_drv(16usize))
                .field("pb_drv[17]", &self.pb_drv(17usize))
                .field("pb_drv[18]", &self.pb_drv(18usize))
                .field("pb_drv[19]", &self.pb_drv(19usize))
                .field("pb_drv[20]", &self.pb_drv(20usize))
                .field("pb_drv[21]", &self.pb_drv(21usize))
                .field("pb_drv[22]", &self.pb_drv(22usize))
                .field("pb_drv[23]", &self.pb_drv(23usize))
                .field("pb_drv[24]", &self.pb_drv(24usize))
                .field("pb_drv[25]", &self.pb_drv(25usize))
                .field("pb_drv[26]", &self.pb_drv(26usize))
                .field("pb_drv[27]", &self.pb_drv(27usize))
                .field("pb_drv[28]", &self.pb_drv(28usize))
                .field("pb_drv[29]", &self.pb_drv(29usize))
                .field("pb_drv[30]", &self.pb_drv(30usize))
                .field("pb_drv[31]", &self.pb_drv(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbOut(pub u32);
    impl PbOut {
        #[doc = "GPIO PB output level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB output level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pb_out(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbOut {
        #[inline(always)]
        fn default() -> PbOut {
            PbOut(0)
        }
    }
    impl core::fmt::Debug for PbOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbOut")
                .field("pb_out[0]", &self.pb_out(0usize))
                .field("pb_out[1]", &self.pb_out(1usize))
                .field("pb_out[2]", &self.pb_out(2usize))
                .field("pb_out[3]", &self.pb_out(3usize))
                .field("pb_out[4]", &self.pb_out(4usize))
                .field("pb_out[5]", &self.pb_out(5usize))
                .field("pb_out[6]", &self.pb_out(6usize))
                .field("pb_out[7]", &self.pb_out(7usize))
                .field("pb_out[8]", &self.pb_out(8usize))
                .field("pb_out[9]", &self.pb_out(9usize))
                .field("pb_out[10]", &self.pb_out(10usize))
                .field("pb_out[11]", &self.pb_out(11usize))
                .field("pb_out[12]", &self.pb_out(12usize))
                .field("pb_out[13]", &self.pb_out(13usize))
                .field("pb_out[14]", &self.pb_out(14usize))
                .field("pb_out[15]", &self.pb_out(15usize))
                .field("pb_out[16]", &self.pb_out(16usize))
                .field("pb_out[17]", &self.pb_out(17usize))
                .field("pb_out[18]", &self.pb_out(18usize))
                .field("pb_out[19]", &self.pb_out(19usize))
                .field("pb_out[20]", &self.pb_out(20usize))
                .field("pb_out[21]", &self.pb_out(21usize))
                .field("pb_out[22]", &self.pb_out(22usize))
                .field("pb_out[23]", &self.pb_out(23usize))
                .field("pb_out[24]", &self.pb_out(24usize))
                .field("pb_out[25]", &self.pb_out(25usize))
                .field("pb_out[26]", &self.pb_out(26usize))
                .field("pb_out[27]", &self.pb_out(27usize))
                .field("pb_out[28]", &self.pb_out(28usize))
                .field("pb_out[29]", &self.pb_out(29usize))
                .field("pb_out[30]", &self.pb_out(30usize))
                .field("pb_out[31]", &self.pb_out(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB output open-drain and input pulldown resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPd(pub u32);
    impl PbPd {
        #[doc = "GPIO PB open-drain output (output mode) / pull-down resistor (input mode): 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB open-drain output (output mode) / pull-down resistor (input mode): 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pb_pd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbPd {
        #[inline(always)]
        fn default() -> PbPd {
            PbPd(0)
        }
    }
    impl core::fmt::Debug for PbPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPd")
                .field("pb_pd[0]", &self.pb_pd(0usize))
                .field("pb_pd[1]", &self.pb_pd(1usize))
                .field("pb_pd[2]", &self.pb_pd(2usize))
                .field("pb_pd[3]", &self.pb_pd(3usize))
                .field("pb_pd[4]", &self.pb_pd(4usize))
                .field("pb_pd[5]", &self.pb_pd(5usize))
                .field("pb_pd[6]", &self.pb_pd(6usize))
                .field("pb_pd[7]", &self.pb_pd(7usize))
                .field("pb_pd[8]", &self.pb_pd(8usize))
                .field("pb_pd[9]", &self.pb_pd(9usize))
                .field("pb_pd[10]", &self.pb_pd(10usize))
                .field("pb_pd[11]", &self.pb_pd(11usize))
                .field("pb_pd[12]", &self.pb_pd(12usize))
                .field("pb_pd[13]", &self.pb_pd(13usize))
                .field("pb_pd[14]", &self.pb_pd(14usize))
                .field("pb_pd[15]", &self.pb_pd(15usize))
                .field("pb_pd[16]", &self.pb_pd(16usize))
                .field("pb_pd[17]", &self.pb_pd(17usize))
                .field("pb_pd[18]", &self.pb_pd(18usize))
                .field("pb_pd[19]", &self.pb_pd(19usize))
                .field("pb_pd[20]", &self.pb_pd(20usize))
                .field("pb_pd[21]", &self.pb_pd(21usize))
                .field("pb_pd[22]", &self.pb_pd(22usize))
                .field("pb_pd[23]", &self.pb_pd(23usize))
                .field("pb_pd[24]", &self.pb_pd(24usize))
                .field("pb_pd[25]", &self.pb_pd(25usize))
                .field("pb_pd[26]", &self.pb_pd(26usize))
                .field("pb_pd[27]", &self.pb_pd(27usize))
                .field("pb_pd[28]", &self.pb_pd(28usize))
                .field("pb_pd[29]", &self.pb_pd(29usize))
                .field("pb_pd[30]", &self.pb_pd(30usize))
                .field("pb_pd[31]", &self.pb_pd(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPin(pub u32);
    impl PbPin {
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pb_pin(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbPin {
        #[inline(always)]
        fn default() -> PbPin {
            PbPin(0)
        }
    }
    impl core::fmt::Debug for PbPin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPin")
                .field("pb_pin[0]", &self.pb_pin(0usize))
                .field("pb_pin[1]", &self.pb_pin(1usize))
                .field("pb_pin[2]", &self.pb_pin(2usize))
                .field("pb_pin[3]", &self.pb_pin(3usize))
                .field("pb_pin[4]", &self.pb_pin(4usize))
                .field("pb_pin[5]", &self.pb_pin(5usize))
                .field("pb_pin[6]", &self.pb_pin(6usize))
                .field("pb_pin[7]", &self.pb_pin(7usize))
                .field("pb_pin[8]", &self.pb_pin(8usize))
                .field("pb_pin[9]", &self.pb_pin(9usize))
                .field("pb_pin[10]", &self.pb_pin(10usize))
                .field("pb_pin[11]", &self.pb_pin(11usize))
                .field("pb_pin[12]", &self.pb_pin(12usize))
                .field("pb_pin[13]", &self.pb_pin(13usize))
                .field("pb_pin[14]", &self.pb_pin(14usize))
                .field("pb_pin[15]", &self.pb_pin(15usize))
                .field("pb_pin[16]", &self.pb_pin(16usize))
                .field("pb_pin[17]", &self.pb_pin(17usize))
                .field("pb_pin[18]", &self.pb_pin(18usize))
                .field("pb_pin[19]", &self.pb_pin(19usize))
                .field("pb_pin[20]", &self.pb_pin(20usize))
                .field("pb_pin[21]", &self.pb_pin(21usize))
                .field("pb_pin[22]", &self.pb_pin(22usize))
                .field("pb_pin[23]", &self.pb_pin(23usize))
                .field("pb_pin[24]", &self.pb_pin(24usize))
                .field("pb_pin[25]", &self.pb_pin(25usize))
                .field("pb_pin[26]", &self.pb_pin(26usize))
                .field("pb_pin[27]", &self.pb_pin(27usize))
                .field("pb_pin[28]", &self.pb_pin(28usize))
                .field("pb_pin[29]", &self.pb_pin(29usize))
                .field("pb_pin[30]", &self.pb_pin(30usize))
                .field("pb_pin[31]", &self.pb_pin(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPu(pub u32);
    impl PbPu {
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pb_pu(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbPu {
        #[inline(always)]
        fn default() -> PbPu {
            PbPu(0)
        }
    }
    impl core::fmt::Debug for PbPu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPu")
                .field("pb_pu[0]", &self.pb_pu(0usize))
                .field("pb_pu[1]", &self.pb_pu(1usize))
                .field("pb_pu[2]", &self.pb_pu(2usize))
                .field("pb_pu[3]", &self.pb_pu(3usize))
                .field("pb_pu[4]", &self.pb_pu(4usize))
                .field("pb_pu[5]", &self.pb_pu(5usize))
                .field("pb_pu[6]", &self.pb_pu(6usize))
                .field("pb_pu[7]", &self.pb_pu(7usize))
                .field("pb_pu[8]", &self.pb_pu(8usize))
                .field("pb_pu[9]", &self.pb_pu(9usize))
                .field("pb_pu[10]", &self.pb_pu(10usize))
                .field("pb_pu[11]", &self.pb_pu(11usize))
                .field("pb_pu[12]", &self.pb_pu(12usize))
                .field("pb_pu[13]", &self.pb_pu(13usize))
                .field("pb_pu[14]", &self.pb_pu(14usize))
                .field("pb_pu[15]", &self.pb_pu(15usize))
                .field("pb_pu[16]", &self.pb_pu(16usize))
                .field("pb_pu[17]", &self.pb_pu(17usize))
                .field("pb_pu[18]", &self.pb_pu(18usize))
                .field("pb_pu[19]", &self.pb_pu(19usize))
                .field("pb_pu[20]", &self.pb_pu(20usize))
                .field("pb_pu[21]", &self.pb_pu(21usize))
                .field("pb_pu[22]", &self.pb_pu(22usize))
                .field("pb_pu[23]", &self.pb_pu(23usize))
                .field("pb_pu[24]", &self.pb_pu(24usize))
                .field("pb_pu[25]", &self.pb_pu(25usize))
                .field("pb_pu[26]", &self.pb_pu(26usize))
                .field("pb_pu[27]", &self.pb_pu(27usize))
                .field("pb_pu[28]", &self.pb_pu(28usize))
                .field("pb_pu[29]", &self.pb_pu(29usize))
                .field("pb_pu[30]", &self.pb_pu(30usize))
                .field("pb_pu[31]", &self.pb_pu(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB output slew rate and input schmitt trigger."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbSmt(pub u32);
    impl PbSmt {
        #[doc = "GPIO PB low-slew-rate output (output mode) / Schmitt-trigger input (input mode): 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_smt(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB low-slew-rate output (output mode) / Schmitt-trigger input (input mode): 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pb_smt(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbSmt {
        #[inline(always)]
        fn default() -> PbSmt {
            PbSmt(0)
        }
    }
    impl core::fmt::Debug for PbSmt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbSmt")
                .field("pb_smt[0]", &self.pb_smt(0usize))
                .field("pb_smt[1]", &self.pb_smt(1usize))
                .field("pb_smt[2]", &self.pb_smt(2usize))
                .field("pb_smt[3]", &self.pb_smt(3usize))
                .field("pb_smt[4]", &self.pb_smt(4usize))
                .field("pb_smt[5]", &self.pb_smt(5usize))
                .field("pb_smt[6]", &self.pb_smt(6usize))
                .field("pb_smt[7]", &self.pb_smt(7usize))
                .field("pb_smt[8]", &self.pb_smt(8usize))
                .field("pb_smt[9]", &self.pb_smt(9usize))
                .field("pb_smt[10]", &self.pb_smt(10usize))
                .field("pb_smt[11]", &self.pb_smt(11usize))
                .field("pb_smt[12]", &self.pb_smt(12usize))
                .field("pb_smt[13]", &self.pb_smt(13usize))
                .field("pb_smt[14]", &self.pb_smt(14usize))
                .field("pb_smt[15]", &self.pb_smt(15usize))
                .field("pb_smt[16]", &self.pb_smt(16usize))
                .field("pb_smt[17]", &self.pb_smt(17usize))
                .field("pb_smt[18]", &self.pb_smt(18usize))
                .field("pb_smt[19]", &self.pb_smt(19usize))
                .field("pb_smt[20]", &self.pb_smt(20usize))
                .field("pb_smt[21]", &self.pb_smt(21usize))
                .field("pb_smt[22]", &self.pb_smt(22usize))
                .field("pb_smt[23]", &self.pb_smt(23usize))
                .field("pb_smt[24]", &self.pb_smt(24usize))
                .field("pb_smt[25]", &self.pb_smt(25usize))
                .field("pb_smt[26]", &self.pb_smt(26usize))
                .field("pb_smt[27]", &self.pb_smt(27usize))
                .field("pb_smt[28]", &self.pb_smt(28usize))
                .field("pb_smt[29]", &self.pb_smt(29usize))
                .field("pb_smt[30]", &self.pb_smt(30usize))
                .field("pb_smt[31]", &self.pb_smt(31usize))
                .finish()
        }
    }
}
