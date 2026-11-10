#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Systick register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systick {
    ptr: *mut u8,
}
unsafe impl Send for Systick {}
unsafe impl Sync for Systick {}
impl Systick {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Systick counter control register."]
    #[inline(always)]
    pub const fn stk_ctlr(self) -> crate::common::Reg<regs::StkCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Systick count status register."]
    #[inline(always)]
    pub const fn stk_sr(self) -> crate::common::Reg<regs::StkSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Systick counter low register."]
    #[inline(always)]
    pub const fn stk_cntl(self) -> crate::common::Reg<regs::StkCntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Count reload low register."]
    #[inline(always)]
    pub const fn stk_cmplr(self) -> crate::common::Reg<regs::StkCmplr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Systick compare high register."]
    #[inline(always)]
    pub const fn stk_cmphr(self) -> crate::common::Reg<regs::StkCmphr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "Systick compare high register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StkCmphr(pub u32);
    impl StkCmphr {
        #[doc = "RW, Set the reload counter value to the lower 32 bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cmph(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, Set the reload counter value to the lower 32 bits."]
        #[inline(always)]
        pub const fn set_cmph(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for StkCmphr {
        #[inline(always)]
        fn default() -> StkCmphr {
            StkCmphr(0)
        }
    }
    impl core::fmt::Debug for StkCmphr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StkCmphr")
                .field("cmph", &self.cmph())
                .finish()
        }
    }
    #[doc = "Count reload low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StkCmplr(pub u32);
    impl StkCmplr {
        #[doc = "RW, set the lower 32 bits of the reload counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, set the lower 32 bits of the reload counter value."]
        #[inline(always)]
        pub const fn set_cmpl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for StkCmplr {
        #[inline(always)]
        fn default() -> StkCmplr {
            StkCmplr(0)
        }
    }
    impl core::fmt::Debug for StkCmplr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StkCmplr")
                .field("cmpl", &self.cmpl())
                .finish()
        }
    }
    #[doc = "Systick counter low register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StkCntl(pub u32);
    impl StkCntl {
        #[doc = "RW, CNTL."]
        #[must_use]
        #[inline(always)]
        pub const fn cntl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, CNTL."]
        #[inline(always)]
        pub const fn set_cntl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for StkCntl {
        #[inline(always)]
        fn default() -> StkCntl {
            StkCntl(0)
        }
    }
    impl core::fmt::Debug for StkCntl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StkCntl")
                .field("cntl", &self.cntl())
                .finish()
        }
    }
    #[doc = "Systick counter control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StkCtlr(pub u32);
    impl StkCtlr {
        #[doc = "Systick counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ste(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Systick counter enable."]
        #[inline(always)]
        pub const fn set_ste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Systick counter interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn stie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Systick counter interrupt enable."]
        #[inline(always)]
        pub const fn set_stie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "System counter clock Source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn stclk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "System counter clock Source selection."]
        #[inline(always)]
        pub const fn set_stclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "System counter reload control."]
        #[must_use]
        #[inline(always)]
        pub const fn stre(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "System counter reload control."]
        #[inline(always)]
        pub const fn set_stre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "counter mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "counter mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RW0,System soft interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn swie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RW0,System soft interrupt enable."]
        #[inline(always)]
        pub const fn set_swie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for StkCtlr {
        #[inline(always)]
        fn default() -> StkCtlr {
            StkCtlr(0)
        }
    }
    impl core::fmt::Debug for StkCtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StkCtlr")
                .field("ste", &self.ste())
                .field("stie", &self.stie())
                .field("stclk", &self.stclk())
                .field("stre", &self.stre())
                .field("mode", &self.mode())
                .field("swie", &self.swie())
                .finish()
        }
    }
    #[doc = "Systick count status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StkSr(pub u32);
    impl StkSr {
        #[doc = "RW, CNTIF."]
        #[must_use]
        #[inline(always)]
        pub const fn cntif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, CNTIF."]
        #[inline(always)]
        pub const fn set_cntif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW, Enable software interrupt triggering."]
        #[must_use]
        #[inline(always)]
        pub const fn swie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Enable software interrupt triggering."]
        #[inline(always)]
        pub const fn set_swie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for StkSr {
        #[inline(always)]
        fn default() -> StkSr {
            StkSr(0)
        }
    }
    impl core::fmt::Debug for StkSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StkSr")
                .field("cntif", &self.cntif())
                .field("swie", &self.swie())
                .finish()
        }
    }
}
