#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USBHS HSI calibration register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsHsi {
    ptr: *mut u8,
}
unsafe impl Send for UsbhsHsi {}
unsafe impl Sync for UsbhsHsi {}
impl UsbhsHsi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSI clock calibration control register."]
    #[inline(always)]
    pub const fn cal_ctrl(self) -> crate::common::Reg<regs::CalCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "HSI clock calibration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalCtrl(pub u32);
    impl CalCtrl {
        #[doc = "the calibration module is globally reset."]
        #[inline(always)]
        pub const fn cal_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the calibration module is globally reset."]
        #[inline(always)]
        pub fn set_cal_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "alignment enabled."]
        #[inline(always)]
        pub const fn cal_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "alignment enabled."]
        #[inline(always)]
        pub fn set_cal_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "exti fine calibration mode and receive that BUS_RESET. should be set."]
        #[inline(always)]
        pub const fn sft_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "exti fine calibration mode and receive that BUS_RESET. should be set."]
        #[inline(always)]
        pub fn set_sft_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the uplink interface enters the enable mode."]
        #[inline(always)]
        pub const fn sof_free(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the uplink interface enters the enable mode."]
        #[inline(always)]
        pub fn set_sof_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "clock selection."]
        #[inline(always)]
        pub const fn clk_sel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "clock selection."]
        #[inline(always)]
        pub fn set_clk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for CalCtrl {
        #[inline(always)]
        fn default() -> CalCtrl {
            CalCtrl(0)
        }
    }
}
