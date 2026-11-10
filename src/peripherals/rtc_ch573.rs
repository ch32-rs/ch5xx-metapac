#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RTC register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, RTC flag and clear control."]
    #[inline(always)]
    pub const fn flag_ctrl(self) -> crate::common::Reg<regs::FlagCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "RWA, RTC mode control, SAM."]
    #[inline(always)]
    pub const fn mode_ctrl(self) -> crate::common::Reg<regs::ModeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31usize) as _) }
    }
    #[doc = "RWA, RTC trigger value, SAM."]
    #[inline(always)]
    pub const fn trig(self) -> crate::common::Reg<regs::Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "RO, RTC count based 32KHz."]
    #[inline(always)]
    pub const fn cnt_32k(self) -> crate::common::Reg<regs::Cnt32k, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "RO, RTC count based 2 second."]
    #[inline(always)]
    pub const fn cnt_2s(self) -> crate::common::Reg<regs::Cnt2s, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "RO, RTC count based one day, only low 14 bit."]
    #[inline(always)]
    pub const fn cnt_day(self) -> crate::common::Reg<regs::CntDay, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RO, RTC count based 2 second."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt2s(pub u16);
    impl Cnt2s {
        #[doc = "RO, RTC count based 2 second."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_2s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RO, RTC count based 2 second."]
        #[inline(always)]
        pub const fn set_cnt_2s(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt2s {
        #[inline(always)]
        fn default() -> Cnt2s {
            Cnt2s(0)
        }
    }
    impl core::fmt::Debug for Cnt2s {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt2s")
                .field("cnt_2s", &self.cnt_2s())
                .finish()
        }
    }
    #[doc = "RO, RTC count based 32KHz."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt32k(pub u16);
    impl Cnt32k {
        #[doc = "RWA,RTC count based 32KHz."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_32k(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RWA,RTC count based 32KHz."]
        #[inline(always)]
        pub const fn set_cnt_32k(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt32k {
        #[inline(always)]
        fn default() -> Cnt32k {
            Cnt32k(0)
        }
    }
    impl core::fmt::Debug for Cnt32k {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt32k")
                .field("cnt_32k", &self.cnt_32k())
                .finish()
        }
    }
    #[doc = "RO, RTC count based one day, only low 14 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntDay(pub u32);
    impl CntDay {
        #[doc = "RWA,RTC count based one day."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_day(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "RWA,RTC count based one day."]
        #[inline(always)]
        pub const fn set_cnt_day(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for CntDay {
        #[inline(always)]
        fn default() -> CntDay {
            CntDay(0)
        }
    }
    impl core::fmt::Debug for CntDay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CntDay")
                .field("cnt_day", &self.cnt_day())
                .finish()
        }
    }
    #[doc = "RW, RTC flag and clear control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FlagCtrl(pub u8);
    impl FlagCtrl {
        #[doc = "RW, set 1 to clear RTC timer action flag, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_clr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, set 1 to clear RTC timer action flag, auto clear."]
        #[inline(always)]
        pub const fn set_tmr_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, set 1 to clear RTC trigger action flag, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn trig_clr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, set 1 to clear RTC trigger action flag, auto clear."]
        #[inline(always)]
        pub const fn set_trig_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, RTC timer action flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_flag(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, RTC timer action flag."]
        #[inline(always)]
        pub const fn set_tmr_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, RTC trigger action flag."]
        #[must_use]
        #[inline(always)]
        pub const fn trig_flag(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, RTC trigger action flag."]
        #[inline(always)]
        pub const fn set_trig_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for FlagCtrl {
        #[inline(always)]
        fn default() -> FlagCtrl {
            FlagCtrl(0)
        }
    }
    impl core::fmt::Debug for FlagCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FlagCtrl")
                .field("tmr_clr", &self.tmr_clr())
                .field("trig_clr", &self.trig_clr())
                .field("tmr_flag", &self.tmr_flag())
                .field("trig_flag", &self.trig_flag())
                .finish()
        }
    }
    #[doc = "RWA, RTC mode control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModeCtrl(pub u8);
    impl ModeCtrl {
        #[doc = "RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S."]
        #[inline(always)]
        pub const fn set_tmr_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0."]
        #[must_use]
        #[inline(always)]
        pub const fn ignore_b0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0."]
        #[inline(always)]
        pub const fn set_ignore_b0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, RTC timer mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, RTC timer mode enable."]
        #[inline(always)]
        pub const fn set_tmr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, RTC trigger mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn trig_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, RTC trigger mode enable."]
        #[inline(always)]
        pub const fn set_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded."]
        #[must_use]
        #[inline(always)]
        pub const fn load_lo(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded."]
        #[inline(always)]
        pub const fn set_load_lo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded."]
        #[must_use]
        #[inline(always)]
        pub const fn load_hi(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded."]
        #[inline(always)]
        pub const fn set_load_hi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for ModeCtrl {
        #[inline(always)]
        fn default() -> ModeCtrl {
            ModeCtrl(0)
        }
    }
    impl core::fmt::Debug for ModeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ModeCtrl")
                .field("tmr_mode", &self.tmr_mode())
                .field("ignore_b0", &self.ignore_b0())
                .field("tmr_en", &self.tmr_en())
                .field("trig_en", &self.trig_en())
                .field("load_lo", &self.load_lo())
                .field("load_hi", &self.load_hi())
                .finish()
        }
    }
    #[doc = "RWA, RTC trigger value, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trig(pub u32);
    impl Trig {
        #[doc = "RWA, RTC trigger value."]
        #[must_use]
        #[inline(always)]
        pub const fn trig(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RWA, RTC trigger value."]
        #[inline(always)]
        pub const fn set_trig(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Trig {
        #[inline(always)]
        fn default() -> Trig {
            Trig(0)
        }
    }
    impl core::fmt::Debug for Trig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trig").field("trig", &self.trig()).finish()
        }
    }
}
