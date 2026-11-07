#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWMX register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmx {
    ptr: *mut u8,
}
unsafe impl Send for Pwmx {}
unsafe impl Sync for Pwmx {}
impl Pwmx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PWM mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWM configuration control."]
    #[inline(always)]
    pub const fn ctrl_cfg(self) -> crate::common::Reg<regs::CtrlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "PWM clock divisor."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "PWM data holding."]
    #[inline(always)]
    pub const fn pwm0_data(self) -> crate::common::Reg<regs::Pwm0Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PWM1 data holding."]
    #[inline(always)]
    pub const fn pwm1_data(self) -> crate::common::Reg<regs::Pwm1Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "PWM2 data holding."]
    #[inline(always)]
    pub const fn pwm2_data(self) -> crate::common::Reg<regs::Pwm2Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "PWM3 data holding."]
    #[inline(always)]
    pub const fn pwm3_data(self) -> crate::common::Reg<regs::Pwm3Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWM clock divisor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "PWM clock divisor."]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PWM clock divisor."]
        #[inline(always)]
        pub fn set_clock_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ClockDiv {
        #[inline(always)]
        fn default() -> ClockDiv {
            ClockDiv(0)
        }
    }
    #[doc = "PWM configuration control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlCfg(pub u8);
    impl CtrlCfg {
        #[doc = "PWM cycle selection."]
        #[inline(always)]
        pub const fn cycle_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PWM cycle selection."]
        #[inline(always)]
        pub fn set_cycle_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for CtrlCfg {
        #[inline(always)]
        fn default() -> CtrlCfg {
            CtrlCfg(0)
        }
    }
    #[doc = "PWM mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod(pub u8);
    impl CtrlMod {
        #[doc = "PWM0 output enable."]
        #[inline(always)]
        pub const fn pwm0_out_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PWM0 output enable."]
        #[inline(always)]
        pub fn set_pwm0_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PWM1 output enable."]
        #[inline(always)]
        pub const fn pwm1_out_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PWM1 output enable."]
        #[inline(always)]
        pub fn set_pwm1_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PWM2 output enable."]
        #[inline(always)]
        pub const fn pwm2_out_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWM2 output enable."]
        #[inline(always)]
        pub fn set_pwm2_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PWM3 output enable."]
        #[inline(always)]
        pub const fn pwm3_out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PWM3 output enable."]
        #[inline(always)]
        pub fn set_pwm3_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "PWM0 output polarity."]
        #[inline(always)]
        pub const fn pwm0_polar(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PWM0 output polarity."]
        #[inline(always)]
        pub fn set_pwm0_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PWM1 output polarity."]
        #[inline(always)]
        pub const fn pwm1_polar(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PWM1 output polarity."]
        #[inline(always)]
        pub fn set_pwm1_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "PWM2 output polarity."]
        #[inline(always)]
        pub const fn pwm2_polar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PWM2 output polarity."]
        #[inline(always)]
        pub fn set_pwm2_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "PWM3 output polarity."]
        #[inline(always)]
        pub const fn pwm3_polar(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PWM3 output polarity."]
        #[inline(always)]
        pub fn set_pwm3_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlMod {
        #[inline(always)]
        fn default() -> CtrlMod {
            CtrlMod(0)
        }
    }
    #[doc = "PWM data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm0Data(pub u8);
    impl Pwm0Data {
        #[doc = "PWM0 data holding."]
        #[inline(always)]
        pub const fn pwm0_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PWM0 data holding."]
        #[inline(always)]
        pub fn set_pwm0_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm0Data {
        #[inline(always)]
        fn default() -> Pwm0Data {
            Pwm0Data(0)
        }
    }
    #[doc = "PWM1 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm1Data(pub u8);
    impl Pwm1Data {
        #[doc = "PWM1 data holding."]
        #[inline(always)]
        pub const fn pwm1_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PWM1 data holding."]
        #[inline(always)]
        pub fn set_pwm1_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm1Data {
        #[inline(always)]
        fn default() -> Pwm1Data {
            Pwm1Data(0)
        }
    }
    #[doc = "PWM2 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm2Data(pub u8);
    impl Pwm2Data {
        #[doc = "PWM2 data holding."]
        #[inline(always)]
        pub const fn pwm2_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PWM2 data holding."]
        #[inline(always)]
        pub fn set_pwm2_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm2Data {
        #[inline(always)]
        fn default() -> Pwm2Data {
            Pwm2Data(0)
        }
    }
    #[doc = "PWM3 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm3Data(pub u8);
    impl Pwm3Data {
        #[doc = "PWM3 data holding."]
        #[inline(always)]
        pub const fn pwm3_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PWM3 data holding."]
        #[inline(always)]
        pub fn set_pwm3_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm3Data {
        #[inline(always)]
        fn default() -> Pwm3Data {
            Pwm3Data(0)
        }
    }
}
