#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System_control register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemControl {
    ptr: *mut u8,
}
unsafe impl Send for SystemControl {}
unsafe impl Sync for SystemControl {}
impl SystemControl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode."]
    #[inline(always)]
    pub const fn safe_access_sig(
        self,
    ) -> crate::common::Reg<regs::SafeAccessSig, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "RF, chip ID register, always is ID_CH57*."]
    #[inline(always)]
    pub const fn chip_id(self) -> crate::common::Reg<regs::ChipId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x41usize) as _) }
    }
    #[doc = "RF, safe accessing ID register, always 0x04."]
    #[inline(always)]
    pub const fn safe_access_id(self) -> crate::common::Reg<regs::SafeAccessId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "RW, watch-dog count, count by clock frequency Fsys/131072."]
    #[inline(always)]
    pub const fn wdog_count(self) -> crate::common::Reg<regs::WdogCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x43usize) as _) }
    }
    #[doc = "RO, global configuration information and status."]
    #[inline(always)]
    pub const fn glob_cfg_info(self) -> crate::common::Reg<regs::GlobCfgInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x45usize) as _) }
    }
    #[doc = "RWA, reset and watch-dog control, SAM."]
    #[inline(always)]
    pub const fn rst_wdog_ctrl(self) -> crate::common::Reg<regs::RstWdogCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "RW, value keeper during global reset."]
    #[inline(always)]
    pub const fn glob_reset_keep(
        self,
    ) -> crate::common::Reg<regs::GlobResetKeep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x47usize) as _) }
    }
    #[doc = "RWA, PLL configuration control, SAM."]
    #[inline(always)]
    pub const fn pll_config(self) -> crate::common::Reg<regs::PllConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4busize) as _) }
    }
    #[doc = "RWA, external 32MHz oscillator tune control, SAM."]
    #[inline(always)]
    pub const fn xt32m_tune(self) -> crate::common::Reg<regs::Xt32mTune, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4eusize) as _) }
    }
    #[doc = "RO, system clock count value for 32KHz 5 cycles."]
    #[inline(always)]
    pub const fn osc_cal_cnt(self) -> crate::common::Reg<regs::OscCalCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RWA, oscillator frequency calibration control, SAM."]
    #[inline(always)]
    pub const fn osc_cal_ctrl(self) -> crate::common::Reg<regs::OscCalCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
}
pub mod regs {
    #[doc = "RF, chip ID register, always is ID_CH57*."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChipId(pub u8);
    impl ChipId {
        #[doc = "RF,chip ID register."]
        #[must_use]
        #[inline(always)]
        pub const fn chip_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RF,chip ID register."]
        #[inline(always)]
        pub const fn set_chip_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ChipId {
        #[inline(always)]
        fn default() -> ChipId {
            ChipId(0)
        }
    }
    impl core::fmt::Debug for ChipId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChipId")
                .field("chip_id", &self.chip_id())
                .finish()
        }
    }
    #[doc = "RO, global configuration information and status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobCfgInfo(pub u8);
    impl GlobCfgInfo {
        #[doc = "RO, indicate protected status of Flash ROM code and data: 0=reading protect, 1=enable read by external programmer."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_rom_read(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate protected status of Flash ROM code and data: 0=reading protect, 1=enable read by external programmer."]
        #[inline(always)]
        pub const fn set_cfg_rom_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, manual reset input enable status."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_reset_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, manual reset input enable status."]
        #[inline(always)]
        pub const fn set_cfg_reset_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RO, boot-loader enable status."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_boot_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, boot-loader enable status."]
        #[inline(always)]
        pub const fn set_cfg_boot_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, debug enable status."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_debug_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, debug enable status."]
        #[inline(always)]
        pub const fn set_cfg_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate boot loader status: 0=application status (by software reset), 1=boot loader status."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_loader(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate boot loader status: 0=application status (by software reset), 1=boot loader status."]
        #[inline(always)]
        pub const fn set_boot_loader(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for GlobCfgInfo {
        #[inline(always)]
        fn default() -> GlobCfgInfo {
            GlobCfgInfo(0)
        }
    }
    impl core::fmt::Debug for GlobCfgInfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobCfgInfo")
                .field("cfg_rom_read", &self.cfg_rom_read())
                .field("cfg_reset_en", &self.cfg_reset_en())
                .field("cfg_boot_en", &self.cfg_boot_en())
                .field("cfg_debug_en", &self.cfg_debug_en())
                .field("boot_loader", &self.boot_loader())
                .finish()
        }
    }
    #[doc = "RW, value keeper during global reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobResetKeep(pub u8);
    impl GlobResetKeep {
        #[doc = "RW, value keeper during global reset."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_reset_keep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, value keeper during global reset."]
        #[inline(always)]
        pub const fn set_glob_reset_keep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for GlobResetKeep {
        #[inline(always)]
        fn default() -> GlobResetKeep {
            GlobResetKeep(0)
        }
    }
    impl core::fmt::Debug for GlobResetKeep {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobResetKeep")
                .field("glob_reset_keep", &self.glob_reset_keep())
                .finish()
        }
    }
    #[doc = "RO, system clock count value for 32KHz 5 cycles."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OscCalCnt(pub u16);
    impl OscCalCnt {
        #[doc = "RO, system clock count value for 32KHz 5 cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn osc_cal_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "RO, system clock count value for 32KHz 5 cycles."]
        #[inline(always)]
        pub const fn set_osc_cal_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u16) & 0x3fff) << 0usize);
        }
    }
    impl Default for OscCalCnt {
        #[inline(always)]
        fn default() -> OscCalCnt {
            OscCalCnt(0)
        }
    }
    impl core::fmt::Debug for OscCalCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OscCalCnt")
                .field("osc_cal_cnt", &self.osc_cal_cnt())
                .finish()
        }
    }
    #[doc = "RWA, oscillator frequency calibration control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OscCalCtrl(pub u8);
    impl OscCalCtrl {
        #[doc = "RWA, calibration counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn osc_cnt_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, calibration counter enable."]
        #[inline(always)]
        pub const fn set_osc_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, calibration counter halt status: 0=counting, 1=halt for reading count value."]
        #[must_use]
        #[inline(always)]
        pub const fn osc_cnt_halt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, calibration counter halt status: 0=counting, 1=halt for reading count value."]
        #[inline(always)]
        pub const fn set_osc_cnt_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for OscCalCtrl {
        #[inline(always)]
        fn default() -> OscCalCtrl {
            OscCalCtrl(0)
        }
    }
    impl core::fmt::Debug for OscCalCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OscCalCtrl")
                .field("osc_cnt_en", &self.osc_cnt_en())
                .field("osc_cnt_halt", &self.osc_cnt_halt())
                .finish()
        }
    }
    #[doc = "RWA, PLL configuration control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PllConfig(pub u8);
    impl PllConfig {
        #[doc = "RWA, PLL configure data."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_cfg_dat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "RWA, PLL configure data."]
        #[inline(always)]
        pub const fn set_pll_cfg_dat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for PllConfig {
        #[inline(always)]
        fn default() -> PllConfig {
            PllConfig(0)
        }
    }
    impl core::fmt::Debug for PllConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PllConfig")
                .field("pll_cfg_dat", &self.pll_cfg_dat())
                .finish()
        }
    }
    #[doc = "RWA, reset and watch-dog control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RstWdogCtrl(pub u8);
    impl RstWdogCtrl {
        #[doc = "WA or WZ, global software reset, high action, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn software_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WA or WZ, global software reset, high action, auto clear."]
        #[inline(always)]
        pub const fn set_software_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_rst_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow."]
        #[inline(always)]
        pub const fn set_wdog_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_int_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable."]
        #[inline(always)]
        pub const fn set_wdog_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event)."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_int_flag(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event)."]
        #[inline(always)]
        pub const fn set_wdog_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for RstWdogCtrl {
        #[inline(always)]
        fn default() -> RstWdogCtrl {
            RstWdogCtrl(0)
        }
    }
    impl core::fmt::Debug for RstWdogCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RstWdogCtrl")
                .field("software_reset", &self.software_reset())
                .field("wdog_rst_en", &self.wdog_rst_en())
                .field("wdog_int_en", &self.wdog_int_en())
                .field("wdog_int_flag", &self.wdog_int_flag())
                .finish()
        }
    }
    #[doc = "RF, safe accessing ID register, always 0x04."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SafeAccessId(pub u8);
    impl SafeAccessId {
        #[doc = "RF,safe accessing ID register."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_access_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RF,safe accessing ID register."]
        #[inline(always)]
        pub const fn set_safe_access_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for SafeAccessId {
        #[inline(always)]
        fn default() -> SafeAccessId {
            SafeAccessId(0)
        }
    }
    impl core::fmt::Debug for SafeAccessId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SafeAccessId")
                .field("safe_access_id", &self.safe_access_id())
                .finish()
        }
    }
    #[doc = "WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SafeAccessSig(pub u8);
    impl SafeAccessSig {
        #[doc = "RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11)."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_acc_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11)."]
        #[inline(always)]
        pub const fn set_safe_acc_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_access_sig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode."]
        #[inline(always)]
        pub const fn set_safe_access_sig(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
        #[doc = "RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_acc_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled."]
        #[inline(always)]
        pub const fn set_safe_acc_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, safe accessing timer bit mask (16*clock number)."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_acc_timer(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "RO, safe accessing timer bit mask (16*clock number)."]
        #[inline(always)]
        pub const fn set_safe_acc_timer(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for SafeAccessSig {
        #[inline(always)]
        fn default() -> SafeAccessSig {
            SafeAccessSig(0)
        }
    }
    impl core::fmt::Debug for SafeAccessSig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SafeAccessSig")
                .field("safe_acc_mode", &self.safe_acc_mode())
                .field("safe_access_sig", &self.safe_access_sig())
                .field("safe_acc_act", &self.safe_acc_act())
                .field("safe_acc_timer", &self.safe_acc_timer())
                .finish()
        }
    }
    #[doc = "RW, watch-dog count, count by clock frequency Fsys/131072."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdogCount(pub u8);
    impl WdogCount {
        #[doc = "RF,watch-dog count, count by clock frequency Fsys/131072."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RF,watch-dog count, count by clock frequency Fsys/131072."]
        #[inline(always)]
        pub const fn set_wdog_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for WdogCount {
        #[inline(always)]
        fn default() -> WdogCount {
            WdogCount(0)
        }
    }
    impl core::fmt::Debug for WdogCount {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdogCount")
                .field("wdog_count", &self.wdog_count())
                .finish()
        }
    }
    #[doc = "RWA, external 32MHz oscillator tune control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xt32mTune(pub u8);
    impl Xt32mTune {
        #[doc = "RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current."]
        #[must_use]
        #[inline(always)]
        pub const fn xt32m_i_bias(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current."]
        #[inline(always)]
        pub const fn set_xt32m_i_bias(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF."]
        #[must_use]
        #[inline(always)]
        pub const fn xt32m_c_load(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF."]
        #[inline(always)]
        pub const fn set_xt32m_c_load(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
        }
    }
    impl Default for Xt32mTune {
        #[inline(always)]
        fn default() -> Xt32mTune {
            Xt32mTune(0)
        }
    }
    impl core::fmt::Debug for Xt32mTune {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xt32mTune")
                .field("xt32m_i_bias", &self.xt32m_i_bias())
                .field("xt32m_c_load", &self.xt32m_c_load())
                .finish()
        }
    }
}
