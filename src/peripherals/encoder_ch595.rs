#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ENCODER register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Encoder {
    ptr: *mut u8,
}
unsafe impl Send for Encoder {}
unsafe impl Sync for Encoder {}
impl Encoder {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, Encoder control register."]
    #[inline(always)]
    pub const fn reg_ctrl(self) -> crate::common::Reg<regs::RegCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, Encoder interrupt enable register."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, Encoder interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, Encoder final value configuration register."]
    #[inline(always)]
    pub const fn reg_cend(self) -> crate::common::Reg<regs::RegCend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RW, Encoder current value configuration register."]
    #[inline(always)]
    pub const fn reg_ccnt(self) -> crate::common::Reg<regs::RegCcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, Encoder interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RW, Encoder forward interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dir_inc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder forward interrupt flag."]
        #[inline(always)]
        pub const fn set_if_dir_inc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Encoder backward interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dir_dec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder backward interrupt flag."]
        #[inline(always)]
        pub const fn set_if_dir_dec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
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
                .field("if_dir_inc", &self.if_dir_inc())
                .field("if_dir_dec", &self.if_dir_dec())
                .finish()
        }
    }
    #[doc = "RW, Encoder interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEn(pub u8);
    impl InterEn {
        #[doc = "RW, Encoder forward interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dir_inc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder forward interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_dir_inc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Encoder backward interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dir_dec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder backward interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_dir_dec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for InterEn {
        #[inline(always)]
        fn default() -> InterEn {
            InterEn(0)
        }
    }
    impl core::fmt::Debug for InterEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InterEn")
                .field("ie_dir_inc", &self.ie_dir_inc())
                .field("ie_dir_dec", &self.ie_dir_dec())
                .finish()
        }
    }
    #[doc = "RW, Encoder current value configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegCcnt(pub u32);
    impl RegCcnt {
        #[doc = "RW, Current encoder value."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_ccnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, Current encoder value."]
        #[inline(always)]
        pub const fn set_reg_ccnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RegCcnt {
        #[inline(always)]
        fn default() -> RegCcnt {
            RegCcnt(0)
        }
    }
    impl core::fmt::Debug for RegCcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegCcnt")
                .field("reg_ccnt", &self.reg_ccnt())
                .finish()
        }
    }
    #[doc = "RW, Encoder final value configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegCend(pub u32);
    impl RegCend {
        #[doc = "RW, Encoder final value configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn reg_cend(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, Encoder final value configuration."]
        #[inline(always)]
        pub const fn set_reg_cend(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RegCend {
        #[inline(always)]
        fn default() -> RegCend {
            RegCend(0)
        }
    }
    impl core::fmt::Debug for RegCend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegCend")
                .field("reg_cend", &self.reg_cend())
                .finish()
        }
    }
    #[doc = "RW, Encoder control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegCtrl(pub u8);
    impl RegCtrl {
        #[doc = "RW, Encoder work enable."]
        #[must_use]
        #[inline(always)]
        pub const fn start_enc_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder work enable."]
        #[inline(always)]
        pub const fn set_start_enc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Encoder edge working mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sms_mode(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Encoder edge working mode."]
        #[inline(always)]
        pub const fn set_sms_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
        #[doc = "RW, Encoder reads count and resets."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_clr_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder reads count and resets."]
        #[inline(always)]
        pub const fn set_rd_clr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, Current direction of encoder."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Current direction of encoder."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, Encoder captures the value of channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn in_ch1(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder captures the value of channel 1."]
        #[inline(always)]
        pub const fn set_in_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, Encoder captures the value of channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn in_ch2(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Encoder captures the value of channel 2."]
        #[inline(always)]
        pub const fn set_in_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for RegCtrl {
        #[inline(always)]
        fn default() -> RegCtrl {
            RegCtrl(0)
        }
    }
    impl core::fmt::Debug for RegCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegCtrl")
                .field("start_enc_en", &self.start_enc_en())
                .field("sms_mode", &self.sms_mode())
                .field("rd_clr_en", &self.rd_clr_en())
                .field("dir", &self.dir())
                .field("in_ch1", &self.in_ch1())
                .field("in_ch2", &self.in_ch2())
                .finish()
        }
    }
}
