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
    #[doc = "RWA, Long reset configuration register."]
    #[inline(always)]
    pub const fn long_rst_cfg(self) -> crate::common::Reg<regs::LongRstCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RWA, Sleep clock control register 2."]
    #[inline(always)]
    pub const fn slp_clk_off2(self) -> crate::common::Reg<regs::SlpClkOff2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x15usize) as _) }
    }
    #[doc = "RWA, Sleep configuration register."]
    #[inline(always)]
    pub const fn slp_wake_cfg(self) -> crate::common::Reg<regs::SlpWakeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "RW, function pin alternate configuration."]
    #[inline(always)]
    pub const fn pin_alternate(self) -> crate::common::Reg<regs::PinAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, Function pin configuration registere."]
    #[inline(always)]
    pub const fn pin_analog_ie(self) -> crate::common::Reg<regs::PinAnalogIe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "RW, Pin configuration register 2."]
    #[inline(always)]
    pub const fn pin_config2(self) -> crate::common::Reg<regs::PinConfig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
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
    #[doc = "RWA, internal 32KHz oscillator tune control, SAM."]
    #[inline(always)]
    pub const fn int32k_tune(self) -> crate::common::Reg<regs::Int32kTune, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "RWA, External 32KHz oscillator tune control, SAM."]
    #[inline(always)]
    pub const fn xt32k_tune(self) -> crate::common::Reg<regs::Xt32kTune, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "RWA, 32KHz oscillator configure."]
    #[inline(always)]
    pub const fn ck32k_config(self) -> crate::common::Reg<regs::Ck32kConfig, crate::common::RW> {
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
        #[doc = "RWA, Low power auxiliary DC-DC enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dcdc_charge(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Low power auxiliary DC-DC enable bit."]
        #[inline(always)]
        pub const fn set_dcdc_charge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RWA, retain. The original value must be kept unchanged when writing."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_ivref(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RWA, retain. The original value must be kept unchanged when writing."]
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
                .field("dcdc_charge", &self.dcdc_charge())
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
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RWA, select threshold voltage of battery voltage low."]
        #[inline(always)]
        pub const fn set_bat_low_vth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
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
        #[doc = "RWA, Low power consumption battery voltage monitoring function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_mon_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Low power consumption battery voltage monitoring function enable."]
        #[inline(always)]
        pub const fn set_bat_mon_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, Battery low voltage interrupt type."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_low_nmi(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Battery low voltage interrupt type."]
        #[inline(always)]
        pub const fn set_bat_low_nmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, interrupt enable for battery low voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_low_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, interrupt enable for battery low voltage."]
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
                .field("bat_mon_en", &self.bat_mon_en())
                .field("bat_low_nmi", &self.bat_low_nmi())
                .field("bat_low_ie", &self.bat_low_ie())
                .finish()
        }
    }
    #[doc = "RO, battery status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatStatus(pub u8);
    impl BatStatus {
        #[doc = "RO, battery lower voltage status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_stat_lower(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, battery lower voltage status, high action."]
        #[inline(always)]
        pub const fn set_bat_stat_lower(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
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
                .field("bat_stat_lower", &self.bat_stat_lower())
                .field("bat_stat_low", &self.bat_stat_low())
                .finish()
        }
    }
    #[doc = "RWA, 32KHz oscillator configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ck32kConfig(pub u8);
    impl Ck32kConfig {
        #[doc = "RWA, external 32KHz oscillator power on."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_xt32k_pon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, external 32KHz oscillator power on."]
        #[inline(always)]
        pub const fn set_clk_xt32k_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, internal 32KHz oscillator power on."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_int32k_pon(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, internal 32KHz oscillator power on."]
        #[inline(always)]
        pub const fn set_clk_int32k_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, 32KHz oscillator source selection: 0=RC, 1=XT."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_osc32k_xt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, 32KHz oscillator source selection: 0=RC, 1=XT."]
        #[inline(always)]
        pub const fn set_clk_osc32k_xt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, internal 32KHz oscillator low noise mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_osc32k_filt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, internal 32KHz oscillator low noise mode enable."]
        #[inline(always)]
        pub const fn set_clk_osc32k_filt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, 32KHz oscillator clock pin status."]
        #[must_use]
        #[inline(always)]
        pub const fn _32k_clk_pin(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, 32KHz oscillator clock pin status."]
        #[inline(always)]
        pub const fn set__32k_clk_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Ck32kConfig {
        #[inline(always)]
        fn default() -> Ck32kConfig {
            Ck32kConfig(0)
        }
    }
    impl core::fmt::Debug for Ck32kConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ck32kConfig")
                .field("clk_xt32k_pon", &self.clk_xt32k_pon())
                .field("clk_int32k_pon", &self.clk_int32k_pon())
                .field("clk_osc32k_xt", &self.clk_osc32k_xt())
                .field("clk_osc32k_filt", &self.clk_osc32k_filt())
                .field("_32k_clk_pin", &self._32k_clk_pin())
                .finish()
        }
    }
    #[doc = "RWA, system clock configuration, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkCfg(pub u16);
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
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
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
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
        #[doc = "RWA, Selection of PLL clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn osc32m_sel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Selection of PLL clock source."]
        #[inline(always)]
        pub const fn set_osc32m_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RO, PLL clock activation flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_gate_status(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RO, PLL clock activation flag."]
        #[inline(always)]
        pub const fn set_pll_gate_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RWA, Do you turn off the PLL clock when switching the PLL clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_gate_diss(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Do you turn off the PLL clock when switching the PLL clock source."]
        #[inline(always)]
        pub const fn set_pll_gate_diss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "RWA, When switching the PLL clock source, turn off the time selection of the PLL clock."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_gate_time(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, When switching the PLL clock source, turn off the time selection of the PLL clock."]
        #[inline(always)]
        pub const fn set_pll_gate_time(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
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
                .field("osc32m_sel", &self.osc32m_sel())
                .field("pll_gate_status", &self.pll_gate_status())
                .field("pll_gate_diss", &self.pll_gate_diss())
                .field("pll_gate_time", &self.pll_gate_time())
                .finish()
        }
    }
    #[doc = "RWA, High frequency clock module power control register, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HfckPwrCtrl(pub u8);
    impl HfckPwrCtrl {
        #[doc = "RWA,Enable items for the internal 16MHz oscillator HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_rc16m_pon(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA,Enable items for the internal 16MHz oscillator HSI."]
        #[inline(always)]
        pub const fn set_clk_rc16m_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
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
                .field("clk_rc16m_pon", &self.clk_rc16m_pon())
                .field("clk_xt32m_pon", &self.clk_xt32m_pon())
                .field("clk_xt32m_keep", &self.clk_xt32m_keep())
                .field("clk_pll_pon", &self.clk_pll_pon())
                .finish()
        }
    }
    #[doc = "RWA, internal 32KHz oscillator tune control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Int32kTune(pub u16);
    impl Int32kTune {
        #[doc = "RWA, Internal RC 32KHz clock frequency calibration value."]
        #[must_use]
        #[inline(always)]
        pub const fn int32k_tune(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "RWA, Internal RC 32KHz clock frequency calibration value."]
        #[inline(always)]
        pub const fn set_int32k_tune(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u16) & 0x1fff) << 0usize);
        }
    }
    impl Default for Int32kTune {
        #[inline(always)]
        fn default() -> Int32kTune {
            Int32kTune(0)
        }
    }
    impl core::fmt::Debug for Int32kTune {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Int32kTune")
                .field("int32k_tune", &self.int32k_tune())
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
    #[doc = "RW, function pin alternate configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAlternate(pub u16);
    impl PinAlternate {
        #[doc = "RW, TMR0 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TMR0 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW, TMR1 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TMR1 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RW, TMR2 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TMR2 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "RW, RXD0/TXD0 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_uart0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, RXD0/TXD0 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RW, RXD1/TXD1 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_uart1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, RXD1/TXD1 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "RW, RXD2/TXD2 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_uart2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, RXD2/TXD2 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RW, SCS/SCK0/MOSI/MISO alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_spi0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SCS/SCK0/MOSI/MISO alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RW, PWM4/PWM5/PWM7/PWM8/PWM9 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pwmx(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4/PWM5/PWM7/PWM8/PWM9 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_pwmx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RW, DSR/DTR alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_modem(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RW, DSR/DTR alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_modem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
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
                .field("pin_tmr0", &self.pin_tmr0())
                .field("pin_tmr1", &self.pin_tmr1())
                .field("pin_tmr2", &self.pin_tmr2())
                .field("pin_uart0", &self.pin_uart0())
                .field("pin_uart1", &self.pin_uart1())
                .field("pin_uart2", &self.pin_uart2())
                .field("pin_spi0", &self.pin_spi0())
                .field("pin_pwmx", &self.pin_pwmx())
                .field("pin_modem", &self.pin_modem())
                .finish()
        }
    }
    #[doc = "RW, Function pin configuration registere."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAnalogIe(pub u16);
    impl PinAnalogIe {
        #[doc = "RW, Selection of ENC pins."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_enc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Selection of ENC pins."]
        #[inline(always)]
        pub const fn set_pin_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RW, Selection of LED pins."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_led(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Selection of LED pins."]
        #[inline(always)]
        pub const fn set_pin_led(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "RW, Full speed USB UD+pin internal pull-up resistor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_pu_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Full speed USB UD+pin internal pull-up resistor enable."]
        #[inline(always)]
        pub const fn set_udp_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RW, Full speed USB pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_usb_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Full speed USB pin enable."]
        #[inline(always)]
        pub const fn set_pin_usb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
    }
    impl Default for PinAnalogIe {
        #[inline(always)]
        fn default() -> PinAnalogIe {
            PinAnalogIe(0)
        }
    }
    impl core::fmt::Debug for PinAnalogIe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinAnalogIe")
                .field("pin_enc", &self.pin_enc())
                .field("pin_led", &self.pin_led())
                .field("udp_pu_en", &self.udp_pu_en())
                .field("pin_usb_en", &self.pin_usb_en())
                .finish()
        }
    }
    #[doc = "RW, Pin configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinConfig2(pub u32);
    impl PinConfig2 {
        #[doc = "RW, PA4~PA15 channel pin digital input disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pa_dis415(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "RW, PA4~PA15 channel pin digital input disabled."]
        #[inline(always)]
        pub const fn set_pin_pa_dis415(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[doc = "RW, PB0 channel pin digital input disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pb_dis0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PB0 channel pin digital input disabled."]
        #[inline(always)]
        pub const fn set_pin_pb_dis0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RW, PB4 channel pin digital input disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pb_dis4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PB4 channel pin digital input disabled."]
        #[inline(always)]
        pub const fn set_pin_pb_dis4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RW, PB6~PB7 channel pin digital input disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pb_dis67(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "RW, PB6~PB7 channel pin digital input disabled."]
        #[inline(always)]
        pub const fn set_pin_pb_dis67(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "RW, PB22~PB23 channel pin digital input disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pb_dis89(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "RW, PB22~PB23 channel pin digital input disabled."]
        #[inline(always)]
        pub const fn set_pin_pb_dis89(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "RW, PB10~PB15 channel pin digital input disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_pb_dis1015(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "RW, PB10~PB15 channel pin digital input disabled."]
        #[inline(always)]
        pub const fn set_pin_pb_dis1015(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for PinConfig2 {
        #[inline(always)]
        fn default() -> PinConfig2 {
            PinConfig2(0)
        }
    }
    impl core::fmt::Debug for PinConfig2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinConfig2")
                .field("pin_pa_dis415", &self.pin_pa_dis415())
                .field("pin_pb_dis0", &self.pin_pb_dis0())
                .field("pin_pb_dis4", &self.pin_pb_dis4())
                .field("pin_pb_dis67", &self.pin_pb_dis67())
                .field("pin_pb_dis89", &self.pin_pb_dis89())
                .field("pin_pb_dis1015", &self.pin_pb_dis1015())
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
        #[doc = "RWA, SRAM power supply for RAM24K."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ram24k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, SRAM power supply for RAM24K."]
        #[inline(always)]
        pub const fn set_pwr_ram24k(&mut self, val: bool) {
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
        #[doc = "RWA, SRAM power supply for RAM8K."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ram8k(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, SRAM power supply for RAM8K."]
        #[inline(always)]
        pub const fn set_pwr_ram8k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
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
        #[doc = "RWA, Internal LDO control."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ldo_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Internal LDO control."]
        #[inline(always)]
        pub const fn set_pwr_ldo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RWA, DC/DC converter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_dcdc_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, DC/DC converter enable."]
        #[inline(always)]
        pub const fn set_pwr_dcdc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RWA, DC/DC converter pre-enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_dcdc_pre(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, DC/DC converter pre-enable."]
        #[inline(always)]
        pub const fn set_pwr_dcdc_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RWA, External 32MHz clock (HSE) pre wake-up time."]
        #[must_use]
        #[inline(always)]
        pub const fn xt_pre_cfg(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, External 32MHz clock (HSE) pre wake-up time."]
        #[inline(always)]
        pub const fn set_xt_pre_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u16) & 0x03) << 11usize);
        }
        #[doc = "RWA, External 32MHz clock (HSE) pre wake-up enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xt_pre_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, External 32MHz clock (HSE) pre wake-up enable."]
        #[inline(always)]
        pub const fn set_xt_pre_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
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
                .field("pwr_ram24k", &self.pwr_ram24k())
                .field("pwr_core", &self.pwr_core())
                .field("pwr_extend", &self.pwr_extend())
                .field("pwr_ram8k", &self.pwr_ram8k())
                .field("main_act", &self.main_act())
                .field("pwr_sys_en", &self.pwr_sys_en())
                .field("pwr_ldo_en", &self.pwr_ldo_en())
                .field("pwr_dcdc_en", &self.pwr_dcdc_en())
                .field("pwr_dcdc_pre", &self.pwr_dcdc_pre())
                .field("xt_pre_cfg", &self.xt_pre_cfg())
                .field("xt_pre_en", &self.xt_pre_en())
                .field("pwr_plan_en", &self.pwr_plan_en())
                .finish()
        }
    }
    #[doc = "RWA, sleep clock off control byte 0, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff0(pub u8);
    impl SlpClkOff0 {
        #[doc = "RWA, Timer 0 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Timer 0 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, Timer 1 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Timer 1 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, Timer 2 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Timer 2 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, Timer 3 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Timer 3 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, UART0 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART0 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, UART1 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART1 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, UART2 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART2 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, UART3 clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART3 clock source."]
        #[inline(always)]
        pub const fn set_slp_clk_uart3(&mut self, val: bool) {
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
                .field("slp_clk_tmr0", &self.slp_clk_tmr0())
                .field("slp_clk_tmr1", &self.slp_clk_tmr1())
                .field("slp_clk_tmr2", &self.slp_clk_tmr2())
                .field("slp_clk_tmr3", &self.slp_clk_tmr3())
                .field("slp_clk_uart0", &self.slp_clk_uart0())
                .field("slp_clk_uart1", &self.slp_clk_uart1())
                .field("slp_clk_uart2", &self.slp_clk_uart2())
                .field("slp_clk_uart3", &self.slp_clk_uart3())
                .finish()
        }
    }
    #[doc = "RWA, sleep clock off control byte 1, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff1(pub u8);
    impl SlpClkOff1 {
        #[doc = "RWA, close SPI0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close SPI0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, close LED clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_led(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close LED clock."]
        #[inline(always)]
        pub const fn set_slp_clk_led(&mut self, val: bool) {
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
        #[doc = "RWA, close ADC clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_adc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close ADC clock."]
        #[inline(always)]
        pub const fn set_slp_clk_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, close LCD clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_lcd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close LCD clock."]
        #[inline(always)]
        pub const fn set_slp_clk_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
                .field("slp_clk_spi0", &self.slp_clk_spi0())
                .field("slp_clk_led", &self.slp_clk_led())
                .field("slp_clk_pwmx", &self.slp_clk_pwmx())
                .field("slp_clk_i2c", &self.slp_clk_i2c())
                .field("slp_clk_usb", &self.slp_clk_usb())
                .field("slp_clk_adc", &self.slp_clk_adc())
                .field("slp_clk_lcd", &self.slp_clk_lcd())
                .field("slp_clk_ble", &self.slp_clk_ble())
                .finish()
        }
    }
    #[doc = "RWA, Sleep clock control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff2(pub u8);
    impl SlpClkOff2 {
        #[doc = "RWA, 32M clock off enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_debug(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, 32M clock off enable."]
        #[inline(always)]
        pub const fn set_clk_off_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, ENC clock off enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_enc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, ENC clock off enable."]
        #[inline(always)]
        pub const fn set_clk_off_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, KEYSCAN clock off enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_off_keyscan(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, KEYSCAN clock off enable."]
        #[inline(always)]
        pub const fn set_clk_off_keyscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, Enable encoder event wake-up system."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_enc_wake(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Enable encoder event wake-up system."]
        #[inline(always)]
        pub const fn set_slp_enc_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, Enable KEYSCAN event wake-up system."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_keyscan_wake(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Enable KEYSCAN event wake-up system."]
        #[inline(always)]
        pub const fn set_slp_keyscan_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("clk_off_debug", &self.clk_off_debug())
                .field("clk_off_enc", &self.clk_off_enc())
                .field("clk_off_keyscan", &self.clk_off_keyscan())
                .field("slp_enc_wake", &self.slp_enc_wake())
                .field("slp_keyscan_wake", &self.slp_keyscan_wake())
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
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, wakeup delay time selection."]
        #[inline(always)]
        pub const fn set_wake_dly_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
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
        #[doc = "RWA, close retention 24KB SRAM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ram24k(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close retention 24KB SRAM clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ram24k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("slp_clk_ram24k", &self.slp_clk_ram24k())
                .field("ram_ret_lv", &self.ram_ret_lv())
                .finish()
        }
    }
    #[doc = "RWA, Sleep configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpWakeCfg(pub u16);
    impl SlpWakeCfg {
        #[doc = "RWA, Whether to delay the release of fclk after sleep wake-up."]
        #[must_use]
        #[inline(always)]
        pub const fn preclk_cnt_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Whether to delay the release of fclk after sleep wake-up."]
        #[inline(always)]
        pub const fn set_preclk_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RWA, Sleep time configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn preclk_cnt_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, Sleep time configuration."]
        #[inline(always)]
        pub const fn set_preclk_cnt_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
        }
        #[doc = "RWA, Whether the safety register channel is automatically closed."]
        #[must_use]
        #[inline(always)]
        pub const fn acauto_enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Whether the safety register channel is automatically closed."]
        #[inline(always)]
        pub const fn set_acauto_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RWA, UART receive delay enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart_dly_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART receive delay enable."]
        #[inline(always)]
        pub const fn set_uart_dly_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RWA, UART wake-up interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uart_wake_int(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART wake-up interrupt flag."]
        #[inline(always)]
        pub const fn set_uart_wake_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RWA, UART receiving status after RB_UART_DLY_EN is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uart_dly_idle(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, UART receiving status after RB_UART_DLY_EN is enabled."]
        #[inline(always)]
        pub const fn set_uart_dly_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "RWA, Debug enable."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Debug enable."]
        #[inline(always)]
        pub const fn set_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "RWA, Program selection after RST external manual reset."]
        #[must_use]
        #[inline(always)]
        pub const fn manurst_sel(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, Program selection after RST external manual reset."]
        #[inline(always)]
        pub const fn set_manurst_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
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
                .field("preclk_cnt_en", &self.preclk_cnt_en())
                .field("preclk_cnt_sel", &self.preclk_cnt_sel())
                .field("acauto_enable", &self.acauto_enable())
                .field("uart_dly_en", &self.uart_dly_en())
                .field("uart_wake_int", &self.uart_wake_int())
                .field("uart_dly_idle", &self.uart_dly_idle())
                .field("debug_en", &self.debug_en())
                .field("manurst_sel", &self.manurst_sel())
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
        #[doc = "RWA, enable UART waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_uart_wake(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable UART waking."]
        #[inline(always)]
        pub const fn set_slp_uart_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, GPIO edge wake-up mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio_edge_wake(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, GPIO edge wake-up mode selection."]
        #[inline(always)]
        pub const fn set_gpio_edge_wake(&mut self, val: bool) {
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
        #[doc = "RWA, enable GPIO wake-up mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpio_wake_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable GPIO wake-up mode."]
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
                .field("slp_uart_wake", &self.slp_uart_wake())
                .field("gpio_edge_wake", &self.gpio_edge_wake())
                .field("slp_rtc_wake", &self.slp_rtc_wake())
                .field("slp_gpio_wake", &self.slp_gpio_wake())
                .field("slp_bat_wake", &self.slp_bat_wake())
                .field("wake_ev_mode", &self.wake_ev_mode())
                .field("gpio_wake_mode", &self.gpio_wake_mode())
                .finish()
        }
    }
    #[doc = "RWA, External 32KHz oscillator tune control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xt32kTune(pub u8);
    impl Xt32kTune {
        #[doc = "RWA, Selection of bias current for external 32KHz oscillator."]
        #[must_use]
        #[inline(always)]
        pub const fn xt32k_i_tune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, Selection of bias current for external 32KHz oscillator."]
        #[inline(always)]
        pub const fn set_xt32k_i_tune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "RWA, RC32K adjustment bit, internal 32KHz oscillator bias current selection."]
        #[must_use]
        #[inline(always)]
        pub const fn rc32k_i_tune(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, RC32K adjustment bit, internal 32KHz oscillator bias current selection."]
        #[inline(always)]
        pub const fn set_rc32k_i_tune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "RWA, Choose a built-in load capacitor that matches the external 32KHz crystal."]
        #[must_use]
        #[inline(always)]
        pub const fn xt32k_c_load(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RWA, Choose a built-in load capacitor that matches the external 32KHz crystal."]
        #[inline(always)]
        pub const fn set_xt32k_c_load(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Xt32kTune {
        #[inline(always)]
        fn default() -> Xt32kTune {
            Xt32kTune(0)
        }
    }
    impl core::fmt::Debug for Xt32kTune {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xt32kTune")
                .field("xt32k_i_tune", &self.xt32k_i_tune())
                .field("rc32k_i_tune", &self.rc32k_i_tune())
                .field("xt32k_c_load", &self.xt32k_c_load())
                .finish()
        }
    }
}
