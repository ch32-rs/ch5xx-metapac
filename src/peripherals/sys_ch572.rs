#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System Control Register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sys {
    ptr: *mut u8,
}
unsafe impl Send for Sys {}
unsafe impl Sync for Sys {}
impl Sys {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WO, Watchdog key register, SAM."]
    #[inline(always)]
    pub const fn iwdg_kr(self) -> crate::common::Reg<regs::IwdgKr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, Watchdog configuration register, SAM."]
    #[inline(always)]
    pub const fn iwdg_cfg(self) -> crate::common::Reg<regs::IwdgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RWA, system clock configuration, SAM."]
    #[inline(always)]
    pub const fn clk_cfg(self) -> crate::common::Reg<regs::ClkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RWA, High frequency clock module power control register, SAM."]
    #[inline(always)]
    pub const fn hfck_pwr_ctrl(self) -> crate::common::Reg<regs::HfckPwrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "RWA, sleep clock off control byte 0, SAM."]
    #[inline(always)]
    pub const fn slp_clk_off0(self) -> crate::common::Reg<regs::SlpClkOff0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RWA, sleep clock off control byte 1, SAM."]
    #[inline(always)]
    pub const fn slp_clk_off1(self) -> crate::common::Reg<regs::SlpClkOff1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "RWA, wake control, SAM."]
    #[inline(always)]
    pub const fn slp_wake_ctrl(self) -> crate::common::Reg<regs::SlpWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "RWA, peripherals power down control, SAM."]
    #[inline(always)]
    pub const fn slp_power_ctrl(self) -> crate::common::Reg<regs::SlpPowerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "RW, function pin alternate configuration."]
    #[inline(always)]
    pub const fn pin_alternate(self) -> crate::common::Reg<regs::PinAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, function pin high alternate configuration."]
    #[inline(always)]
    pub const fn pin_alternate_h(
        self,
    ) -> crate::common::Reg<regs::PinAlternateH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "RWA, Long reset configuration register."]
    #[inline(always)]
    pub const fn long_rst_cfg(self) -> crate::common::Reg<regs::LongRstCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RWA, Sleep clock control register 2."]
    #[inline(always)]
    pub const fn slp_clk_off2(self) -> crate::common::Reg<regs::SlpClkOff2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "RWA, Sleep clock control register."]
    #[inline(always)]
    pub const fn slp_wake_cfg(self) -> crate::common::Reg<regs::SlpWakeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "RWA, power plan before sleep instruction, SAM."]
    #[inline(always)]
    pub const fn power_plan(self) -> crate::common::Reg<regs::PowerPlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RWA, aux power adjust control, SAM."]
    #[inline(always)]
    pub const fn aux_power_adj(self) -> crate::common::Reg<regs::AuxPowerAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "RWA, battery voltage detector control, SAM."]
    #[inline(always)]
    pub const fn bat_det_ctrl(self) -> crate::common::Reg<regs::BatDetCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RWA, battery voltage detector configuration, SAM."]
    #[inline(always)]
    pub const fn bat_det_cfg(self) -> crate::common::Reg<regs::BatDetCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25usize) as _) }
    }
    #[doc = "RO, battery status."]
    #[inline(always)]
    pub const fn bat_status(self) -> crate::common::Reg<regs::BatStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "RWA, LSI configure register."]
    #[inline(always)]
    pub const fn lsi_config(self) -> crate::common::Reg<regs::LsiConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2fusize) as _) }
    }
}
pub mod regs {
    #[doc = "RWA, aux power adjust control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AuxPowerAdj(pub u16);
    impl AuxPowerAdj {
        #[doc = "RWA, Output voltage regulation value of auxiliary power supply for ultra-low power consumption LDO."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpldo_adj(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RWA, Output voltage regulation value of auxiliary power supply for ultra-low power consumption LDO."]
        #[inline(always)]
        pub const fn set_ulpldo_adj(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
        }
        #[doc = "RO, retain. The original value must be kept unchanged when writing."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_ivref(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, retain. The original value must be kept unchanged when writing."]
        #[inline(always)]
        pub const fn set_cfg_ivref(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
    }
    impl Default for AuxPowerAdj {
        #[inline(always)]
        fn default() -> AuxPowerAdj {
            AuxPowerAdj(0)
        }
    }
    impl core::fmt::Debug for AuxPowerAdj {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AuxPowerAdj")
                .field("ulpldo_adj", &self.ulpldo_adj())
                .field("cfg_ivref", &self.cfg_ivref())
                .finish()
        }
    }
    #[doc = "RWA, battery voltage detector configuration, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatDetCfg(pub u8);
    impl BatDetCfg {
        #[doc = "RWA, select threshold voltage of battery voltage low."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_low_vth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, select threshold voltage of battery voltage low."]
        #[inline(always)]
        pub const fn set_bat_low_vth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for BatDetCfg {
        #[inline(always)]
        fn default() -> BatDetCfg {
            BatDetCfg(0)
        }
    }
    impl core::fmt::Debug for BatDetCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BatDetCfg")
                .field("bat_low_vth", &self.bat_low_vth())
                .finish()
        }
    }
    #[doc = "RWA, battery voltage detector control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatDetCtrl(pub u8);
    impl BatDetCtrl {
        #[doc = "RWA, -."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ldo_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, -."]
        #[inline(always)]
        pub const fn set_pwr_ldo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, Enable low-power battery voltage monitoring function."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_mon_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Enable low-power battery voltage monitoring function."]
        #[inline(always)]
        pub const fn set_bat_mon_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, Battery low voltage interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_low_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Battery low voltage interrupt enable."]
        #[inline(always)]
        pub const fn set_bat_low_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for BatDetCtrl {
        #[inline(always)]
        fn default() -> BatDetCtrl {
            BatDetCtrl(0)
        }
    }
    impl core::fmt::Debug for BatDetCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BatDetCtrl")
                .field("pwr_ldo_en", &self.pwr_ldo_en())
                .field("bat_mon_en", &self.bat_mon_en())
                .field("bat_low_ie", &self.bat_low_ie())
                .finish()
        }
    }
    #[doc = "RO, battery status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatStatus(pub u8);
    impl BatStatus {
        #[doc = "RO, battery low voltage status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_stat_low(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, battery low voltage status, high action."]
        #[inline(always)]
        pub const fn set_bat_stat_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for BatStatus {
        #[inline(always)]
        fn default() -> BatStatus {
            BatStatus(0)
        }
    }
    impl core::fmt::Debug for BatStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BatStatus")
                .field("bat_stat_low", &self.bat_stat_low())
                .finish()
        }
    }
    #[doc = "RWA, system clock configuration, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkCfg(pub u8);
    impl ClkCfg {
        #[doc = "RWA, output clock divider from PLL or CK32M."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pll_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "RWA, output clock divider from PLL or CK32M."]
        #[inline(always)]
        pub const fn set_clk_pll_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "RWA, system clock source mode: 00=divided from 32MHz."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sys_mod(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, system clock source mode: 00=divided from 32MHz."]
        #[inline(always)]
        pub const fn set_clk_sys_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for ClkCfg {
        #[inline(always)]
        fn default() -> ClkCfg {
            ClkCfg(0)
        }
    }
    impl core::fmt::Debug for ClkCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkCfg")
                .field("clk_pll_div", &self.clk_pll_div())
                .field("clk_sys_mod", &self.clk_sys_mod())
                .finish()
        }
    }
    #[doc = "RWA, High frequency clock module power control register, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HfckPwrCtrl(pub u8);
    impl HfckPwrCtrl {
        #[doc = "RWA, External 32MHz oscillator HSE power control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_xt32m_pon(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, External 32MHz oscillator HSE power control bit."]
        #[inline(always)]
        pub const fn set_clk_xt32m_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, Used to control the stopping of the clock system in pause mode."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_xt32m_keep(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Used to control the stopping of the clock system in pause mode."]
        #[inline(always)]
        pub const fn set_clk_xt32m_keep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, PLL power control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pll_pon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, PLL power control bit."]
        #[inline(always)]
        pub const fn set_clk_pll_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for HfckPwrCtrl {
        #[inline(always)]
        fn default() -> HfckPwrCtrl {
            HfckPwrCtrl(0)
        }
    }
    impl core::fmt::Debug for HfckPwrCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HfckPwrCtrl")
                .field("clk_xt32m_pon", &self.clk_xt32m_pon())
                .field("clk_xt32m_keep", &self.clk_xt32m_keep())
                .field("clk_pll_pon", &self.clk_pll_pon())
                .finish()
        }
    }
    #[doc = "RW, Watchdog configuration register, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgCfg(pub u32);
    impl IwdgCfg {
        #[doc = "RW, RL \\[11:0\\]: Watchdog counter reload."]
        #[must_use]
        #[inline(always)]
        pub const fn rlr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RW, RL \\[11:0\\]: Watchdog counter reload."]
        #[inline(always)]
        pub const fn set_rlr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "RO, Pre division factor (with write protection)."]
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "RO, Pre division factor (with write protection)."]
        #[inline(always)]
        pub const fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "RO, Configure register update flag bit (with write protection)."]
        #[must_use]
        #[inline(always)]
        pub const fn pvu(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Configure register update flag bit (with write protection)."]
        #[inline(always)]
        pub const fn set_pvu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RO, Watchdog countdown counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "RO, Watchdog countdown counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "RW, Watchdog stop enabled (with write protection)."]
        #[must_use]
        #[inline(always)]
        pub const fn stop_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Watchdog stop enabled (with write protection)."]
        #[inline(always)]
        pub const fn set_stop_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "RW, Write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_protect(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Write protection."]
        #[inline(always)]
        pub const fn set_wr_protect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "RW, Watch dog start switch: 1: ON; 0: Close."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Watch dog start switch: 1: ON; 0: Close."]
        #[inline(always)]
        pub const fn set_iwdg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IwdgCfg {
        #[inline(always)]
        fn default() -> IwdgCfg {
            IwdgCfg(0)
        }
    }
    impl core::fmt::Debug for IwdgCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgCfg")
                .field("rlr", &self.rlr())
                .field("pr", &self.pr())
                .field("pvu", &self.pvu())
                .field("count", &self.count())
                .field("stop_en", &self.stop_en())
                .field("wr_protect", &self.wr_protect())
                .field("iwdg_en", &self.iwdg_en())
                .finish()
        }
    }
    #[doc = "WO, Watchdog key register, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwdgKr(pub u32);
    impl IwdgKr {
        #[doc = "RO,KEY \\[15:0\\]: Key value."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_kr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RO,KEY \\[15:0\\]: Key value."]
        #[inline(always)]
        pub const fn set_iwdg_kr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for IwdgKr {
        #[inline(always)]
        fn default() -> IwdgKr {
            IwdgKr(0)
        }
    }
    impl core::fmt::Debug for IwdgKr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IwdgKr")
                .field("iwdg_kr", &self.iwdg_kr())
                .finish()
        }
    }
    #[doc = "RWA, Long reset configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LongRstCfg(pub u8);
    impl LongRstCfg {
        #[doc = "RWA, Long reset enable."]
        #[must_use]
        #[inline(always)]
        pub const fn long_rst_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Long reset enable."]
        #[inline(always)]
        pub const fn set_long_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, Long reset duration selection."]
        #[must_use]
        #[inline(always)]
        pub const fn long_tim_sel(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, Long reset duration selection."]
        #[inline(always)]
        pub const fn set_long_tim_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
    }
    impl Default for LongRstCfg {
        #[inline(always)]
        fn default() -> LongRstCfg {
            LongRstCfg(0)
        }
    }
    impl core::fmt::Debug for LongRstCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LongRstCfg")
                .field("long_rst_en", &self.long_rst_en())
                .field("long_tim_sel", &self.long_tim_sel())
                .finish()
        }
    }
    #[doc = "RWA, LSI configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LsiConfig(pub u8);
    impl LsiConfig {
        #[doc = "RWA, Internal low-frequency RC oscillator LSI power control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_lsi_pon(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Internal low-frequency RC oscillator LSI power control bit."]
        #[inline(always)]
        pub const fn set_clk_lsi_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RO, LSI clock pin status (asynchronous signal)."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi_clk_pin(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, LSI clock pin status (asynchronous signal)."]
        #[inline(always)]
        pub const fn set_lsi_clk_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for LsiConfig {
        #[inline(always)]
        fn default() -> LsiConfig {
            LsiConfig(0)
        }
    }
    impl core::fmt::Debug for LsiConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LsiConfig")
                .field("clk_lsi_pon", &self.clk_lsi_pon())
                .field("lsi_clk_pin", &self.lsi_clk_pin())
                .finish()
        }
    }
    #[doc = "RW, function pin alternate configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAlternate(pub u16);
    impl PinAlternate {
        #[doc = "RW, PA0-PA11 channel pin digital input enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_di_dis(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RW, PA0-PA11 channel pin digital input enable."]
        #[inline(always)]
        pub const fn set_pa_di_dis(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
        #[doc = "RW, Enable internal pull-up resistor for UDP pin of full speed USB."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_pu_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Enable internal pull-up resistor for UDP pin of full speed USB."]
        #[inline(always)]
        pub const fn set_udp_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "RW, Full speed USB pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_usb_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Full speed USB pin enable."]
        #[inline(always)]
        pub const fn set_pin_usb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "RW, Debugging interface control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_debug_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Debugging interface control bit."]
        #[inline(always)]
        pub const fn set_pin_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for PinAlternate {
        #[inline(always)]
        fn default() -> PinAlternate {
            PinAlternate(0)
        }
    }
    impl core::fmt::Debug for PinAlternate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinAlternate")
                .field("pa_di_dis", &self.pa_di_dis())
                .field("udp_pu_en", &self.udp_pu_en())
                .field("pin_usb_en", &self.pin_usb_en())
                .field("pin_debug_en", &self.pin_debug_en())
                .finish()
        }
    }
    #[doc = "RW, function pin high alternate configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAlternateH(pub u16);
    impl PinAlternateH {
        #[doc = "RW, RXD alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart_rxd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RW, RXD alternate pin enable."]
        #[inline(always)]
        pub const fn set_uart_rxd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
        }
        #[doc = "RW, TXD alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart_txd(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "RW, TXD alternate pin enable."]
        #[inline(always)]
        pub const fn set_uart_txd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u16) & 0x07) << 3usize);
        }
        #[doc = "RW, Timer function pin mapping selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_pin(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Timer function pin mapping selection bit."]
        #[inline(always)]
        pub const fn set_tmr_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[doc = "RW, SPI CS function pin mapping selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn spi_cs(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI CS function pin mapping selection bit."]
        #[inline(always)]
        pub const fn set_spi_cs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RW, I2C function pin mapping selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c_pin(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "RW, I2C function pin mapping selection bit."]
        #[inline(always)]
        pub const fn set_i2c_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u16) & 0x03) << 9usize);
        }
        #[doc = "RW, SPI CLK function pin mapping selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn spi_clk(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI CLK function pin mapping selection bit."]
        #[inline(always)]
        pub const fn set_spi_clk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "RW, Enable 25MHz clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn _25m_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Enable 25MHz clock output."]
        #[inline(always)]
        pub const fn set__25m_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
    }
    impl Default for PinAlternateH {
        #[inline(always)]
        fn default() -> PinAlternateH {
            PinAlternateH(0)
        }
    }
    impl core::fmt::Debug for PinAlternateH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinAlternateH")
                .field("uart_rxd", &self.uart_rxd())
                .field("uart_txd", &self.uart_txd())
                .field("tmr_pin", &self.tmr_pin())
                .field("spi_cs", &self.spi_cs())
                .field("i2c_pin", &self.i2c_pin())
                .field("spi_clk", &self.spi_clk())
                .field("_25m_en", &self._25m_en())
                .finish()
        }
    }
    #[doc = "RWA, power plan before sleep instruction, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerPlan(pub u16);
    impl PowerPlan {
        #[doc = "RWA, power for FlashROM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_xrom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power for FlashROM."]
        #[inline(always)]
        pub const fn set_pwr_xrom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RWA, SRAM power supply for RAM12K."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ram12k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, SRAM power supply for RAM12K."]
        #[inline(always)]
        pub const fn set_pwr_ram12k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RWA, power retention for core and base peripherals."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_core(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power retention for core and base peripherals."]
        #[inline(always)]
        pub const fn set_pwr_core(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "RWA, power retention for USB and BLE."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_extend(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power retention for USB and BLE."]
        #[inline(always)]
        pub const fn set_pwr_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "RWA, Main power selection."]
        #[must_use]
        #[inline(always)]
        pub const fn main_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Main power selection."]
        #[inline(always)]
        pub const fn set_main_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RWA, power for system."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_sys_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power for system."]
        #[inline(always)]
        pub const fn set_pwr_sys_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RWA, Internal LDO5V control."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ldo5v_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Internal LDO5V control."]
        #[inline(always)]
        pub const fn set_pwr_ldo5v_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RWA, Enable sleep power planning and control."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_plan_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Enable sleep power planning and control."]
        #[inline(always)]
        pub const fn set_pwr_plan_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for PowerPlan {
        #[inline(always)]
        fn default() -> PowerPlan {
            PowerPlan(0)
        }
    }
    impl core::fmt::Debug for PowerPlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PowerPlan")
                .field("pwr_xrom", &self.pwr_xrom())
                .field("pwr_ram12k", &self.pwr_ram12k())
                .field("pwr_core", &self.pwr_core())
                .field("pwr_extend", &self.pwr_extend())
                .field("main_act", &self.main_act())
                .field("pwr_sys_en", &self.pwr_sys_en())
                .field("pwr_ldo5v_en", &self.pwr_ldo5v_en())
                .field("pwr_plan_en", &self.pwr_plan_en())
                .finish()
        }
    }
    #[doc = "RWA, sleep clock off control byte 0, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff0(pub u8);
    impl SlpClkOff0 {
        #[doc = "RWA, Timer clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Timer clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, CMP clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_cmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, CMP clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, UART clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_uart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, KEYSCAN wakeup enable."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_keyscan_wake(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, KEYSCAN wakeup enable."]
        #[inline(always)]
        pub const fn set_slp_keyscan_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpClkOff0 {
        #[inline(always)]
        fn default() -> SlpClkOff0 {
            SlpClkOff0(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff0")
                .field("slp_clk_tmr", &self.slp_clk_tmr())
                .field("slp_clk_cmp", &self.slp_clk_cmp())
                .field("slp_clk_uart", &self.slp_clk_uart())
                .field("slp_keyscan_wake", &self.slp_keyscan_wake())
                .finish()
        }
    }
    #[doc = "RWA, sleep clock off control byte 1, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff1(pub u8);
    impl SlpClkOff1 {
        #[doc = "RWA, close SPI clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close SPI clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, close AES_CCM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_aesccm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close AES_CCM clock."]
        #[inline(always)]
        pub const fn set_clk_off_aesccm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, close PWMx clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_pwmx(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close PWMx clock."]
        #[inline(always)]
        pub const fn set_slp_clk_pwmx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, close I2C clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_i2c(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close I2C clock."]
        #[inline(always)]
        pub const fn set_slp_clk_i2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, close USB clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_usb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close USB clock."]
        #[inline(always)]
        pub const fn set_slp_clk_usb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, close BLE clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ble(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close BLE clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ble(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpClkOff1 {
        #[inline(always)]
        fn default() -> SlpClkOff1 {
            SlpClkOff1(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff1")
                .field("slp_clk_spi", &self.slp_clk_spi())
                .field("clk_off_aesccm", &self.clk_off_aesccm())
                .field("slp_clk_pwmx", &self.slp_clk_pwmx())
                .field("slp_clk_i2c", &self.slp_clk_i2c())
                .field("slp_clk_usb", &self.slp_clk_usb())
                .field("slp_clk_ble", &self.slp_clk_ble())
                .finish()
        }
    }
    #[doc = "RWA, Sleep clock control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff2(pub u8);
    impl SlpClkOff2 {
        #[doc = "RWA, Turn off the 64M or 600M clock of the FLASH controller."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_xrom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Turn off the 64M or 600M clock of the FLASH controller."]
        #[inline(always)]
        pub const fn set_clk_off_xrom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, Close the two-wire debugging 32M clock."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_debug(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Close the two-wire debugging 32M clock."]
        #[inline(always)]
        pub const fn set_clk_off_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, Turn off the HCLK clock of the FLASH controller."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_hclk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Turn off the HCLK clock of the FLASH controller."]
        #[inline(always)]
        pub const fn set_clk_off_hclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for SlpClkOff2 {
        #[inline(always)]
        fn default() -> SlpClkOff2 {
            SlpClkOff2(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff2")
                .field("clk_off_xrom", &self.clk_off_xrom())
                .field("clk_off_debug", &self.clk_off_debug())
                .field("clk_off_hclk", &self.clk_off_hclk())
                .finish()
        }
    }
    #[doc = "RWA, peripherals power down control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpPowerCtrl(pub u8);
    impl SlpPowerCtrl {
        #[doc = "RWA, wakeup delay time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_dly_mod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RWA, wakeup delay time selection."]
        #[inline(always)]
        pub const fn set_wake_dly_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "RWA, close main SRAM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ramx(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close main SRAM clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ramx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, SRAM retention voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_ret_lv(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, SRAM retention voltage selection."]
        #[inline(always)]
        pub const fn set_ram_ret_lv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for SlpPowerCtrl {
        #[inline(always)]
        fn default() -> SlpPowerCtrl {
            SlpPowerCtrl(0)
        }
    }
    impl core::fmt::Debug for SlpPowerCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpPowerCtrl")
                .field("wake_dly_mod", &self.wake_dly_mod())
                .field("slp_clk_ramx", &self.slp_clk_ramx())
                .field("ram_ret_lv", &self.ram_ret_lv())
                .finish()
        }
    }
    #[doc = "RWA, Sleep clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpWakeCfg(pub u16);
    impl SlpWakeCfg {
        #[doc = "RWA, During Halt sleep, the peripheral clock remains on."]
        #[must_use]
        #[inline(always)]
        pub const fn oscclk_rdy_keep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, During Halt sleep, the peripheral clock remains on."]
        #[inline(always)]
        pub const fn set_oscclk_rdy_keep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RWA, When waking up from sleep, it takes a certain amount of time to release HCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn preclk_cnt_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, When waking up from sleep, it takes a certain amount of time to release HCLK."]
        #[inline(always)]
        pub const fn set_preclk_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RWA, Sleep time configuration, sleep duration=number of cycles * Fsys."]
        #[must_use]
        #[inline(always)]
        pub const fn preclk_cnt_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, Sleep time configuration, sleep duration=number of cycles * Fsys."]
        #[inline(always)]
        pub const fn set_preclk_cnt_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
        }
        #[doc = "RWA, The secure register channel automatically closes."]
        #[must_use]
        #[inline(always)]
        pub const fn acauto_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, The secure register channel automatically closes."]
        #[inline(always)]
        pub const fn set_acauto_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for SlpWakeCfg {
        #[inline(always)]
        fn default() -> SlpWakeCfg {
            SlpWakeCfg(0)
        }
    }
    impl core::fmt::Debug for SlpWakeCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpWakeCfg")
                .field("oscclk_rdy_keep", &self.oscclk_rdy_keep())
                .field("preclk_cnt_en", &self.preclk_cnt_en())
                .field("preclk_cnt_sel", &self.preclk_cnt_sel())
                .field("acauto_enable", &self.acauto_enable())
                .finish()
        }
    }
    #[doc = "RWA, wake control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpWakeCtrl(pub u8);
    impl SlpWakeCtrl {
        #[doc = "RWA, enable USB waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usb_wake(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable USB waking."]
        #[inline(always)]
        pub const fn set_slp_usb_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, enable ENC waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_enc_wake(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable ENC waking."]
        #[inline(always)]
        pub const fn set_slp_enc_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, GPIO wake-up mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_gpio_edge_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, GPIO wake-up mode selection."]
        #[inline(always)]
        pub const fn set_slp_gpio_edge_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, enable RTC waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_rtc_wake(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable RTC waking."]
        #[inline(always)]
        pub const fn set_slp_rtc_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, enable GPIO waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_gpio_wake(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable GPIO waking."]
        #[inline(always)]
        pub const fn set_slp_gpio_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, enable BAT waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_bat_wake(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable BAT waking."]
        #[inline(always)]
        pub const fn set_slp_bat_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, event wakeup mode."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_ev_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, event wakeup mode."]
        #[inline(always)]
        pub const fn set_wake_ev_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, enable edge GPIO wake-up mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio_wake_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable edge GPIO wake-up mode."]
        #[inline(always)]
        pub const fn set_gpio_wake_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpWakeCtrl {
        #[inline(always)]
        fn default() -> SlpWakeCtrl {
            SlpWakeCtrl(0)
        }
    }
    impl core::fmt::Debug for SlpWakeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpWakeCtrl")
                .field("slp_usb_wake", &self.slp_usb_wake())
                .field("slp_enc_wake", &self.slp_enc_wake())
                .field("slp_gpio_edge_mode", &self.slp_gpio_edge_mode())
                .field("slp_rtc_wake", &self.slp_rtc_wake())
                .field("slp_gpio_wake", &self.slp_gpio_wake())
                .field("slp_bat_wake", &self.slp_bat_wake())
                .field("wake_ev_mode", &self.wake_ev_mode())
                .field("gpio_wake_mode", &self.gpio_wake_mode())
                .finish()
        }
    }
}
