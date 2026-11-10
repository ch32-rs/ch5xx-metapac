#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWMx register."]
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
    #[doc = "RW, PWM output enable control."]
    #[inline(always)]
    pub const fn out_en(self) -> crate::common::Reg<regs::OutEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, PWM output polarity control."]
    #[inline(always)]
    pub const fn polar(self) -> crate::common::Reg<regs::Polar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, PWM configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, PWM clock divisor."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, PWM4 data holding."]
    #[inline(always)]
    pub const fn pwm4_data(self) -> crate::common::Reg<regs::Pwm4Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RW, PWM5 data holding."]
    #[inline(always)]
    pub const fn pwm5_data(self) -> crate::common::Reg<regs::Pwm5Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RW, PWM6 data holding."]
    #[inline(always)]
    pub const fn pwm6_data(self) -> crate::common::Reg<regs::Pwm6Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "RW, PWM7 data holding."]
    #[inline(always)]
    pub const fn pwm7_data(self) -> crate::common::Reg<regs::Pwm7Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "RW, PWM8 data holding."]
    #[inline(always)]
    pub const fn pwm8_data(self) -> crate::common::Reg<regs::Pwm8Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RW, PWM9 data holding."]
    #[inline(always)]
    pub const fn pwm9_data(self) -> crate::common::Reg<regs::Pwm9Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "RW, PWM10 data holding."]
    #[inline(always)]
    pub const fn pwm10_data(self) -> crate::common::Reg<regs::Pwm10Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "RW, PWM11 data holding."]
    #[inline(always)]
    pub const fn pwm11_data(self) -> crate::common::Reg<regs::Pwm11Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "RW, PWM interrupt control."]
    #[inline(always)]
    pub const fn int_ctrl(self) -> crate::common::Reg<regs::IntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, PWM clock divisor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "RW, PWM clock divisor."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM clock divisor."]
        #[inline(always)]
        pub const fn set_clock_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ClockDiv {
        #[inline(always)]
        fn default() -> ClockDiv {
            ClockDiv(0)
        }
    }
    impl core::fmt::Debug for ClockDiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClockDiv")
                .field("clock_div", &self.clock_div())
                .finish()
        }
    }
    #[doc = "RW, PWM configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u8);
    impl Config {
        #[doc = "RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks."]
        #[must_use]
        #[inline(always)]
        pub const fn cycle_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks."]
        #[inline(always)]
        pub const fn set_cycle_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, PWM stagger cycle status."]
        #[must_use]
        #[inline(always)]
        pub const fn stag_st(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, PWM stagger cycle status."]
        #[inline(always)]
        pub const fn set_stag_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_mod(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data."]
        #[inline(always)]
        pub const fn set_cyc_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_5_stag_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output."]
        #[inline(always)]
        pub const fn set_pwm4_5_stag_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm6_7_stag_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output."]
        #[inline(always)]
        pub const fn set_pwm6_7_stag_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm8_9_stag_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output."]
        #[inline(always)]
        pub const fn set_pwm8_9_stag_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm10_11_stag_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output."]
        #[inline(always)]
        pub const fn set_pwm10_11_stag_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("cycle_sel", &self.cycle_sel())
                .field("stag_st", &self.stag_st())
                .field("cyc_mod", &self.cyc_mod())
                .field("pwm4_5_stag_en", &self.pwm4_5_stag_en())
                .field("pwm6_7_stag_en", &self.pwm6_7_stag_en())
                .field("pwm8_9_stag_en", &self.pwm8_9_stag_en())
                .field("pwm10_11_stag_en", &self.pwm10_11_stag_en())
                .finish()
        }
    }
    #[doc = "RW, PWM interrupt control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntCtrl(pub u8);
    impl IntCtrl {
        #[doc = "RW, enable interrupt for PWM cycle end."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for PWM cycle end."]
        #[inline(always)]
        pub const fn set_ie_cyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, select PWM cycle interrupt point."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_pre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, select PWM cycle interrupt point."]
        #[inline(always)]
        pub const fn set_cyc_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW1, interrupt flag for PWM cycle end."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cyc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for PWM cycle end."]
        #[inline(always)]
        pub const fn set_if_cyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntCtrl {
        #[inline(always)]
        fn default() -> IntCtrl {
            IntCtrl(0)
        }
    }
    impl core::fmt::Debug for IntCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntCtrl")
                .field("ie_cyc", &self.ie_cyc())
                .field("cyc_pre", &self.cyc_pre())
                .field("if_cyc", &self.if_cyc())
                .finish()
        }
    }
    #[doc = "RW, PWM output enable control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutEn(pub u8);
    impl OutEn {
        #[doc = "RW, PWM4 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_out_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4 output enable."]
        #[inline(always)]
        pub const fn set_pwm4_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, PWM5 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_out_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM5 output enable."]
        #[inline(always)]
        pub const fn set_pwm5_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM6 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm6_out_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM6 output enable."]
        #[inline(always)]
        pub const fn set_pwm6_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, PWM7 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm7_out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM7 output enable."]
        #[inline(always)]
        pub const fn set_pwm7_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, PWM8 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm8_out_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM8 output enable."]
        #[inline(always)]
        pub const fn set_pwm8_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, PWM9 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm9_out_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM9 output enable."]
        #[inline(always)]
        pub const fn set_pwm9_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, PWM10 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm10_out_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM10 output enable."]
        #[inline(always)]
        pub const fn set_pwm10_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, PWM11 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm11_out_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM11 output enable."]
        #[inline(always)]
        pub const fn set_pwm11_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for OutEn {
        #[inline(always)]
        fn default() -> OutEn {
            OutEn(0)
        }
    }
    impl core::fmt::Debug for OutEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutEn")
                .field("pwm4_out_en", &self.pwm4_out_en())
                .field("pwm5_out_en", &self.pwm5_out_en())
                .field("pwm6_out_en", &self.pwm6_out_en())
                .field("pwm7_out_en", &self.pwm7_out_en())
                .field("pwm8_out_en", &self.pwm8_out_en())
                .field("pwm9_out_en", &self.pwm9_out_en())
                .field("pwm10_out_en", &self.pwm10_out_en())
                .field("pwm11_out_en", &self.pwm11_out_en())
                .finish()
        }
    }
    #[doc = "RW, PWM output polarity control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Polar(pub u8);
    impl Polar {
        #[doc = "RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_polar(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm4_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_polar(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm5_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm6_polar(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm6_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm7_polar(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm7_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm8_polar(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm8_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm9_polar(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm9_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm10_polar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm10_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm11_polar(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm11_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Polar {
        #[inline(always)]
        fn default() -> Polar {
            Polar(0)
        }
    }
    impl core::fmt::Debug for Polar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Polar")
                .field("pwm4_polar", &self.pwm4_polar())
                .field("pwm5_polar", &self.pwm5_polar())
                .field("pwm6_polar", &self.pwm6_polar())
                .field("pwm7_polar", &self.pwm7_polar())
                .field("pwm8_polar", &self.pwm8_polar())
                .field("pwm9_polar", &self.pwm9_polar())
                .field("pwm10_polar", &self.pwm10_polar())
                .field("pwm11_polar", &self.pwm11_polar())
                .finish()
        }
    }
    #[doc = "RW, PWM10 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm10Data(pub u8);
    impl Pwm10Data {
        #[doc = "RW, PWM10 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm10_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM10 data holding."]
        #[inline(always)]
        pub const fn set_pwm10_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm10Data {
        #[inline(always)]
        fn default() -> Pwm10Data {
            Pwm10Data(0)
        }
    }
    impl core::fmt::Debug for Pwm10Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm10Data")
                .field("pwm10_data", &self.pwm10_data())
                .finish()
        }
    }
    #[doc = "RW, PWM11 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm11Data(pub u8);
    impl Pwm11Data {
        #[doc = "RW, PWM11 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm11_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM11 data holding."]
        #[inline(always)]
        pub const fn set_pwm11_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm11Data {
        #[inline(always)]
        fn default() -> Pwm11Data {
            Pwm11Data(0)
        }
    }
    impl core::fmt::Debug for Pwm11Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm11Data")
                .field("pwm11_data", &self.pwm11_data())
                .finish()
        }
    }
    #[doc = "RW, PWM4 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm4Data(pub u8);
    impl Pwm4Data {
        #[doc = "RW, PWM4 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM4 data holding."]
        #[inline(always)]
        pub const fn set_pwm4_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm4Data {
        #[inline(always)]
        fn default() -> Pwm4Data {
            Pwm4Data(0)
        }
    }
    impl core::fmt::Debug for Pwm4Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm4Data")
                .field("pwm4_data", &self.pwm4_data())
                .finish()
        }
    }
    #[doc = "RW, PWM5 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm5Data(pub u8);
    impl Pwm5Data {
        #[doc = "RW, PWM5 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM5 data holding."]
        #[inline(always)]
        pub const fn set_pwm5_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm5Data {
        #[inline(always)]
        fn default() -> Pwm5Data {
            Pwm5Data(0)
        }
    }
    impl core::fmt::Debug for Pwm5Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm5Data")
                .field("pwm5_data", &self.pwm5_data())
                .finish()
        }
    }
    #[doc = "RW, PWM6 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm6Data(pub u8);
    impl Pwm6Data {
        #[doc = "RW, PWM6 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm6_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM6 data holding."]
        #[inline(always)]
        pub const fn set_pwm6_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm6Data {
        #[inline(always)]
        fn default() -> Pwm6Data {
            Pwm6Data(0)
        }
    }
    impl core::fmt::Debug for Pwm6Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm6Data")
                .field("pwm6_data", &self.pwm6_data())
                .finish()
        }
    }
    #[doc = "RW, PWM7 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm7Data(pub u8);
    impl Pwm7Data {
        #[doc = "RW, PWM7 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm7_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM7 data holding."]
        #[inline(always)]
        pub const fn set_pwm7_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm7Data {
        #[inline(always)]
        fn default() -> Pwm7Data {
            Pwm7Data(0)
        }
    }
    impl core::fmt::Debug for Pwm7Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm7Data")
                .field("pwm7_data", &self.pwm7_data())
                .finish()
        }
    }
    #[doc = "RW, PWM8 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm8Data(pub u8);
    impl Pwm8Data {
        #[doc = "RW, PWM8 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm8_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM8 data holding."]
        #[inline(always)]
        pub const fn set_pwm8_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm8Data {
        #[inline(always)]
        fn default() -> Pwm8Data {
            Pwm8Data(0)
        }
    }
    impl core::fmt::Debug for Pwm8Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm8Data")
                .field("pwm8_data", &self.pwm8_data())
                .finish()
        }
    }
    #[doc = "RW, PWM9 data holding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm9Data(pub u8);
    impl Pwm9Data {
        #[doc = "RW, PWM9 data holding."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm9_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM9 data holding."]
        #[inline(always)]
        pub const fn set_pwm9_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Pwm9Data {
        #[inline(always)]
        fn default() -> Pwm9Data {
            Pwm9Data(0)
        }
    }
    impl core::fmt::Debug for Pwm9Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm9Data")
                .field("pwm9_data", &self.pwm9_data())
                .finish()
        }
    }
}
