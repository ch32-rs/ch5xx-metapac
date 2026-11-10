#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Touch buttons."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TouchKey {
    ptr: *mut u8,
}
unsafe impl Send for TouchKey {}
unsafe impl Sync for TouchKey {}
impl TouchKey {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, Touchkey configure."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x57usize) as _) }
    }
    #[doc = "RW, TouchKey configuration register 2."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "RW, TouchKey selection register."]
    #[inline(always)]
    pub const fn sel(self) -> crate::common::Reg<regs::Sel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, Touchkey configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u8);
    impl Cfg {
        #[doc = "RW, Touchkey drive shield enable."]
        #[must_use]
        #[inline(always)]
        pub const fn drv_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Touchkey drive shield enable."]
        #[inline(always)]
        pub const fn set_drv_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, Select the running speed of PGA in ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_pga_cfg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Select the running speed of PGA in ADC."]
        #[inline(always)]
        pub const fn set_adc_pga_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, TouchKey DMA Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TouchKey DMA Enable."]
        #[inline(always)]
        pub const fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("drv_en", &self.drv_en())
                .field("adc_pga_cfg", &self.adc_pga_cfg())
                .field("dma_en", &self.dma_en())
                .finish()
        }
    }
    #[doc = "RW, TouchKey configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "RW, Touch key Capacitive charge moving time."]
        #[must_use]
        #[inline(always)]
        pub const fn trans(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RW, Touch key Capacitive charge moving time."]
        #[inline(always)]
        pub const fn set_trans(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "RW, Touch key capacitor charging time."]
        #[must_use]
        #[inline(always)]
        pub const fn cc(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Touch key capacitor charging time."]
        #[inline(always)]
        pub const fn set_cc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "RW, Touch the key to start ADC time selection in advance, unit: moving cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn tkadc_start_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Touch the key to start ADC time selection in advance, unit: moving cycle."]
        #[inline(always)]
        pub const fn set_tkadc_start_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "RW, TKEY module enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TKEY module enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RW, TKEY module start signal and working status, automatic reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ctrans_act(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TKEY module start signal and working status, automatic reset."]
        #[inline(always)]
        pub const fn set_ctrans_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RW, Number of moving cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn transfer_n(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "RW, Number of moving cycles."]
        #[inline(always)]
        pub const fn set_transfer_n(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    impl core::fmt::Debug for Cfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg2")
                .field("trans", &self.trans())
                .field("cc", &self.cc())
                .field("tkadc_start_sel", &self.tkadc_start_sel())
                .field("en", &self.en())
                .field("ctrans_act", &self.ctrans_act())
                .field("transfer_n", &self.transfer_n())
                .finish()
        }
    }
    #[doc = "RW, TouchKey selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sel(pub u32);
    impl Sel {
        #[doc = "RW, Selection of charge transfer channel."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_cs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "RW, Selection of charge transfer channel."]
        #[inline(always)]
        pub const fn set_sel_cs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "RW, TouchKey drive shield channel selection switch."]
        #[must_use]
        #[inline(always)]
        pub const fn driverout_en(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "RW, TouchKey drive shield channel selection switch."]
        #[inline(always)]
        pub const fn set_driverout_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Sel {
        #[inline(always)]
        fn default() -> Sel {
            Sel(0)
        }
    }
    impl core::fmt::Debug for Sel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sel")
                .field("sel_cs", &self.sel_cs())
                .field("driverout_en", &self.driverout_en())
                .finish()
        }
    }
}
