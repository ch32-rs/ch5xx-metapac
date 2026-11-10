#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GPIO register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "The interrupt status register at the PA port."]
    #[inline(always)]
    pub const fn int_status_pa(self) -> crate::common::Reg<regs::IntStatusPa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "The interrupt enable register at the PA port."]
    #[inline(always)]
    pub const fn int_enable_pa(self) -> crate::common::Reg<regs::IntEnablePa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "The interrupt mode register at the PA port."]
    #[inline(always)]
    pub const fn int_mode_pa(self) -> crate::common::Reg<regs::IntModePa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "The interrupt polar register at the PA port."]
    #[inline(always)]
    pub const fn int_polar_pa(self) -> crate::common::Reg<regs::IntPolarPa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "The interrupt status register at the PB port."]
    #[inline(always)]
    pub const fn int_status_pb(self) -> crate::common::Reg<regs::IntStatusPb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "The interrupt enable register at the PB port."]
    #[inline(always)]
    pub const fn int_enable_pb(self) -> crate::common::Reg<regs::IntEnablePb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "The interrupt mode register at the PB port."]
    #[inline(always)]
    pub const fn int_mode_pb(self) -> crate::common::Reg<regs::IntModePb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "The interrupt polar register at the PB port."]
    #[inline(always)]
    pub const fn int_polar_pb(self) -> crate::common::Reg<regs::IntPolarPb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "The interrupt status register at the PA port."]
    #[inline(always)]
    pub const fn int_status_pd(self) -> crate::common::Reg<regs::IntStatusPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "The interrupt enable register at the PD port."]
    #[inline(always)]
    pub const fn int_enable_pd(self) -> crate::common::Reg<regs::IntEnablePd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "The interrupt mode register at the PD port."]
    #[inline(always)]
    pub const fn int_mode_pd(self) -> crate::common::Reg<regs::IntModePd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "The interrupt polar register at the PD port."]
    #[inline(always)]
    pub const fn int_polar_pd(self) -> crate::common::Reg<regs::IntPolarPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "GPIO PA I/O direction."]
    #[inline(always)]
    pub const fn pa_dir(self) -> crate::common::Reg<regs::PaDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "GPIO PA input."]
    #[inline(always)]
    pub const fn pa_pin(self) -> crate::common::Reg<regs::PaPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "GPIO PA output."]
    #[inline(always)]
    pub const fn pa_out(self) -> crate::common::Reg<regs::PaOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "GPIO PA clear output."]
    #[inline(always)]
    pub const fn pa_clr(self) -> crate::common::Reg<regs::PaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "GPIO PA pullup resistance enable."]
    #[inline(always)]
    pub const fn pa_pu(self) -> crate::common::Reg<regs::PaPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "GPIO PA output open-drain_input pulldown resistance enable."]
    #[inline(always)]
    pub const fn pa_pd(self) -> crate::common::Reg<regs::PaPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "GPIO PB I/O direction."]
    #[inline(always)]
    pub const fn pb_dir(self) -> crate::common::Reg<regs::PbDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "GPIO PB input."]
    #[inline(always)]
    pub const fn pb_pin(self) -> crate::common::Reg<regs::PbPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "GPIO PB output."]
    #[inline(always)]
    pub const fn pb_out(self) -> crate::common::Reg<regs::PbOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "GPIO PB clear output."]
    #[inline(always)]
    pub const fn pb_clr(self) -> crate::common::Reg<regs::PbClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "GPIO PB pullup resistance enable."]
    #[inline(always)]
    pub const fn pb_pu(self) -> crate::common::Reg<regs::PbPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "GPIO PB output open-drain_input pulldown resistance enable."]
    #[inline(always)]
    pub const fn pb_pd(self) -> crate::common::Reg<regs::PbPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "GPIO PD I/O direction."]
    #[inline(always)]
    pub const fn pd_dir(self) -> crate::common::Reg<regs::PdDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "GPIO PD input."]
    #[inline(always)]
    pub const fn pd_pin(self) -> crate::common::Reg<regs::PdPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "GPIO PD output."]
    #[inline(always)]
    pub const fn pd_out(self) -> crate::common::Reg<regs::PdOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "GPIO PD pullup resistance enable."]
    #[inline(always)]
    pub const fn pd_pu(self) -> crate::common::Reg<regs::PdPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "GPIO PD output open-drain_input pulldown resistance enable."]
    #[inline(always)]
    pub const fn pd_pd(self) -> crate::common::Reg<regs::PdPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[inline(always)]
    pub const fn afio_pcfr1(self) -> crate::common::Reg<regs::AfioPcfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Configure the extended control register 0."]
    #[inline(always)]
    pub const fn exten_ctlr0(self) -> crate::common::Reg<regs::ExtenCtlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Configure the extended control register 1."]
    #[inline(always)]
    pub const fn exten_ctlr1(self) -> crate::common::Reg<regs::ExtenCtlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR2)."]
    #[inline(always)]
    pub const fn afio_pcfr2(self) -> crate::common::Reg<regs::AfioPcfr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Configure the extended control register 2."]
    #[inline(always)]
    pub const fn exten_ctlr2(self) -> crate::common::Reg<regs::ExtenCtlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "ADC port mode configuration register."]
    #[inline(always)]
    pub const fn adc_mode(self) -> crate::common::Reg<regs::AdcMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016eusize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC port mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcMode(pub u8);
    impl AdcMode {
        #[doc = "ADC external channel input mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_gpio_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "ADC external channel input mode selection."]
        #[inline(always)]
        pub const fn set_adc_gpio_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for AdcMode {
        #[inline(always)]
        fn default() -> AdcMode {
            AdcMode(0)
        }
    }
    impl core::fmt::Debug for AdcMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcMode")
                .field("adc_gpio_mode", &self.adc_gpio_mode())
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR1)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AfioPcfr1(pub u32);
    impl AfioPcfr1 {
        #[doc = "SPI0 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi0_rm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SPI0 remapping."]
        #[inline(always)]
        pub const fn set_spi0_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "UART0 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart0_rm(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "UART0 remapping."]
        #[inline(always)]
        pub const fn set_uart0_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "UART1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1_rm(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "UART1 remapping."]
        #[inline(always)]
        pub const fn set_uart1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "UART2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2_rm(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "UART2 remapping."]
        #[inline(always)]
        pub const fn set_uart2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "UART3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart3_rm(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "UART3 remapping."]
        #[inline(always)]
        pub const fn set_uart3_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "UART0 MODEM remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart0_modem_rm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "UART0 MODEM remapping."]
        #[inline(always)]
        pub const fn set_uart0_modem_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "UART1 MODEM remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart1_modem_rm(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "UART1 MODEM remapping."]
        #[inline(always)]
        pub const fn set_uart1_modem_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "UART2 MODEM remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart2_modem_rm(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "UART2 MODEM remapping."]
        #[inline(always)]
        pub const fn set_uart2_modem_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "I2C remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c_rm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "I2C remapping."]
        #[inline(always)]
        pub const fn set_i2c_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "SLV interrupt pin remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_interrupt_rm(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "SLV interrupt pin remapping."]
        #[inline(always)]
        pub const fn set_slv_interrupt_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "SLV piece selection pin remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_cs_rm(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "SLV piece selection pin remapping."]
        #[inline(always)]
        pub const fn set_slv_cs_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "SLV command data selection input pin remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_addr_rm(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "SLV command data selection input pin remapping."]
        #[inline(always)]
        pub const fn set_slv_addr_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "SLV address1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_addr1_rm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "SLV address1 remapping."]
        #[inline(always)]
        pub const fn set_slv_addr1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "SLV data remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_data_rm(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "SLV data remapping."]
        #[inline(always)]
        pub const fn set_slv_data_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "SLV remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_rw_rm(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "SLV remapping."]
        #[inline(always)]
        pub const fn set_slv_rw_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "LINK LED remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn link_led_rm(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "LINK LED remapping."]
        #[inline(always)]
        pub const fn set_link_led_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for AfioPcfr1 {
        #[inline(always)]
        fn default() -> AfioPcfr1 {
            AfioPcfr1(0)
        }
    }
    impl core::fmt::Debug for AfioPcfr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AfioPcfr1")
                .field("spi0_rm", &self.spi0_rm())
                .field("uart0_rm", &self.uart0_rm())
                .field("uart1_rm", &self.uart1_rm())
                .field("uart2_rm", &self.uart2_rm())
                .field("uart3_rm", &self.uart3_rm())
                .field("uart0_modem_rm", &self.uart0_modem_rm())
                .field("uart1_modem_rm", &self.uart1_modem_rm())
                .field("uart2_modem_rm", &self.uart2_modem_rm())
                .field("i2c_rm", &self.i2c_rm())
                .field("slv_interrupt_rm", &self.slv_interrupt_rm())
                .field("slv_cs_rm", &self.slv_cs_rm())
                .field("slv_addr_rm", &self.slv_addr_rm())
                .field("slv_addr1_rm", &self.slv_addr1_rm())
                .field("slv_data_rm", &self.slv_data_rm())
                .field("slv_rw_rm", &self.slv_rw_rm())
                .field("link_led_rm", &self.link_led_rm())
                .finish()
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_PCFR2)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AfioPcfr2(pub u32);
    impl AfioPcfr2 {
        #[doc = "active led remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn act_led_rm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "active led remapping."]
        #[inline(always)]
        pub const fn set_act_led_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "reset pin remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_rm(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "reset pin remapping."]
        #[inline(always)]
        pub const fn set_rst_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "TIMER0 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn timer0_rm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER0 remapping."]
        #[inline(always)]
        pub const fn set_timer0_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIMER1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn timer1_rm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER1 remapping."]
        #[inline(always)]
        pub const fn set_timer1_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "BUSY remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn busy_rm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "BUSY remapping."]
        #[inline(always)]
        pub const fn set_busy_rm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "UART3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_rm(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "UART3 remapping."]
        #[inline(always)]
        pub const fn set_spi1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "TNOW0 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow0_rm(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "TNOW0 remapping."]
        #[inline(always)]
        pub const fn set_tnow0_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TNOW1 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow1_rm(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "TNOW1 remapping."]
        #[inline(always)]
        pub const fn set_tnow1_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "TNOW2 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow2_rm(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "TNOW2 remapping."]
        #[inline(always)]
        pub const fn set_tnow2_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "TNOW3 remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow3_rm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "TNOW3 remapping."]
        #[inline(always)]
        pub const fn set_tnow3_rm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "UART3 MODEM remapping."]
        #[must_use]
        #[inline(always)]
        pub const fn uart3_modem(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "UART3 MODEM remapping."]
        #[inline(always)]
        pub const fn set_uart3_modem(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for AfioPcfr2 {
        #[inline(always)]
        fn default() -> AfioPcfr2 {
            AfioPcfr2(0)
        }
    }
    impl core::fmt::Debug for AfioPcfr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AfioPcfr2")
                .field("act_led_rm", &self.act_led_rm())
                .field("rst_rm", &self.rst_rm())
                .field("timer0_rm", &self.timer0_rm())
                .field("timer1_rm", &self.timer1_rm())
                .field("busy_rm", &self.busy_rm())
                .field("spi1_rm", &self.spi1_rm())
                .field("tnow0_rm", &self.tnow0_rm())
                .field("tnow1_rm", &self.tnow1_rm())
                .field("tnow2_rm", &self.tnow2_rm())
                .field("tnow3_rm", &self.tnow3_rm())
                .field("uart3_modem", &self.uart3_modem())
                .finish()
        }
    }
    #[doc = "Configure the extended control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr0(pub u32);
    impl ExtenCtlr0 {
        #[doc = "Serial wire JTAG configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_cfg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Serial wire JTAG configuration."]
        #[inline(always)]
        pub const fn set_sw_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USBPLL reference clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpllsrc(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "USBPLL reference clock source."]
        #[inline(always)]
        pub const fn set_usbpllsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "select the systemitem clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "select the systemitem clock source."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "the TNOW signal output of the UART0 is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow0_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "the TNOW signal output of the UART0 is enabled."]
        #[inline(always)]
        pub const fn set_tnow0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "the TNOW signal output of the UART1 is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow1_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "the TNOW signal output of the UART1 is enabled."]
        #[inline(always)]
        pub const fn set_tnow1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "the TNOW signal output of the UART2 is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow2_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "the TNOW signal output of the UART2 is enabled."]
        #[inline(always)]
        pub const fn set_tnow2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "the TNOW signal output of the UART3 is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn tnow3_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "the TNOW signal output of the UART3 is enabled."]
        #[inline(always)]
        pub const fn set_tnow3_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "the busy signal output of spi and slv enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn busy_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "the busy signal output of spi and slv enabled."]
        #[inline(always)]
        pub const fn set_busy_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "global reset enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_cmd_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "global reset enabled."]
        #[inline(always)]
        pub const fn set_rst_cmd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USBPLL input clock frequency multiplication factor."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpllmul(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "USBPLL input clock frequency multiplication factor."]
        #[inline(always)]
        pub const fn set_usbpllmul(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "ethernet phy analog module reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn ana_eth_phy_rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ethernet phy analog module reset control."]
        #[inline(always)]
        pub const fn set_ana_eth_phy_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ethernet phy digital module reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn dig_eth_phy_rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ethernet phy digital module reset control."]
        #[inline(always)]
        pub const fn set_dig_eth_phy_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ethernet reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ethernet reset control."]
        #[inline(always)]
        pub const fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "flash clock pre-divisor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flash_pre_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "flash clock pre-divisor enable."]
        #[inline(always)]
        pub const fn set_flash_pre_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "USBPLL clock enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpllon(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "USBPLL clock enable bit."]
        #[inline(always)]
        pub const fn set_usbpllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "reset time extension control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_dleay_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "reset time extension control bit."]
        #[inline(always)]
        pub const fn set_rst_dleay_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Kernel error interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn core_halt_int_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Kernel error interrupt enabled."]
        #[inline(always)]
        pub const fn set_core_halt_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB PLL ready flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_pll_rdy(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB PLL ready flag bit."]
        #[inline(always)]
        pub const fn set_usb_pll_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ETH PLL ready flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn eth_pll_rdy(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ETH PLL ready flag bit."]
        #[inline(always)]
        pub const fn set_eth_pll_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Core protected moed status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn core_prot_status(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Core protected moed status bit."]
        #[inline(always)]
        pub const fn set_core_prot_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for ExtenCtlr0 {
        #[inline(always)]
        fn default() -> ExtenCtlr0 {
            ExtenCtlr0(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr0")
                .field("sw_cfg", &self.sw_cfg())
                .field("usbpllsrc", &self.usbpllsrc())
                .field("sw", &self.sw())
                .field("tnow0_en", &self.tnow0_en())
                .field("tnow1_en", &self.tnow1_en())
                .field("tnow2_en", &self.tnow2_en())
                .field("tnow3_en", &self.tnow3_en())
                .field("busy_en", &self.busy_en())
                .field("rst_cmd_en", &self.rst_cmd_en())
                .field("usbpllmul", &self.usbpllmul())
                .field("ana_eth_phy_rst", &self.ana_eth_phy_rst())
                .field("dig_eth_phy_rst", &self.dig_eth_phy_rst())
                .field("ethrst", &self.ethrst())
                .field("flash_pre_en", &self.flash_pre_en())
                .field("usbpllon", &self.usbpllon())
                .field("rst_dleay_en", &self.rst_dleay_en())
                .field("core_halt_int_en", &self.core_halt_int_en())
                .field("usb_pll_rdy", &self.usb_pll_rdy())
                .field("eth_pll_rdy", &self.eth_pll_rdy())
                .field("core_prot_status", &self.core_prot_status())
                .finish()
        }
    }
    #[doc = "Configure the extended control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr1(pub u32);
    impl ExtenCtlr1 {
        #[doc = "External High Speed clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock enable."]
        #[inline(always)]
        pub const fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Internal High Speed clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock enable."]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VIO status."]
        #[must_use]
        #[inline(always)]
        pub const fn vio_rdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VIO status."]
        #[inline(always)]
        pub const fn set_vio_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "External crystal current level status."]
        #[must_use]
        #[inline(always)]
        pub const fn hse_status(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "External crystal current level status."]
        #[inline(always)]
        pub const fn set_hse_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Clock signal selection."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock signal selection."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Enable the LDO to go into sleep mode when the VIO power is. down."]
        #[must_use]
        #[inline(always)]
        pub const fn ldo_slp_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the LDO to go into sleep mode when the VIO power is. down."]
        #[inline(always)]
        pub const fn set_ldo_slp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable the IO port of the peripheral to input a high level. when the VIO power is down."]
        #[must_use]
        #[inline(always)]
        pub const fn vio_pwn_io_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the IO port of the peripheral to input a high level. when the VIO power is down."]
        #[inline(always)]
        pub const fn set_vio_pwn_io_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Enable system reset when the VIO power is down."]
        #[must_use]
        #[inline(always)]
        pub const fn vio_pwn_rst_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Enable system reset when the VIO power is down."]
        #[inline(always)]
        pub const fn set_vio_pwn_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Enables an outage when the VIO power is down."]
        #[must_use]
        #[inline(always)]
        pub const fn vio_pwn_int_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Enables an outage when the VIO power is down."]
        #[inline(always)]
        pub const fn set_vio_pwn_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for ExtenCtlr1 {
        #[inline(always)]
        fn default() -> ExtenCtlr1 {
            ExtenCtlr1(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr1")
                .field("hseon", &self.hseon())
                .field("hsion", &self.hsion())
                .field("vio_rdy", &self.vio_rdy())
                .field("hse_status", &self.hse_status())
                .field("clksel", &self.clksel())
                .field("ldo_slp_en", &self.ldo_slp_en())
                .field("vio_pwn_io_en", &self.vio_pwn_io_en())
                .field("vio_pwn_rst_en", &self.vio_pwn_rst_en())
                .field("vio_pwn_int_en", &self.vio_pwn_int_en())
                .finish()
        }
    }
    #[doc = "Configure the extended control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExtenCtlr2(pub u32);
    impl ExtenCtlr2 {
        #[doc = "PD PIN PB18 high threshold input mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_in_hvt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PD PIN PB18 high threshold input mode."]
        #[inline(always)]
        pub const fn set_usbpd_in_hvt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PD PIN PB19 high threshold input mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usbpd_in_hvt1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PD PIN PB19 high threshold input mode."]
        #[inline(always)]
        pub const fn set_usbpd_in_hvt1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "XI ouput enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xi_oe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "XI ouput enable bit."]
        #[inline(always)]
        pub const fn set_xi_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "XO ouput enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xo_oe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "XO ouput enable bit."]
        #[inline(always)]
        pub const fn set_xo_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "XI/XO pin open-drain_ouput control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xixo_gpio_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "XI/XO pin open-drain_ouput control bit."]
        #[inline(always)]
        pub const fn set_xixo_gpio_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for ExtenCtlr2 {
        #[inline(always)]
        fn default() -> ExtenCtlr2 {
            ExtenCtlr2(0)
        }
    }
    impl core::fmt::Debug for ExtenCtlr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExtenCtlr2")
                .field("usbpd_in_hvt0", &self.usbpd_in_hvt0())
                .field("usbpd_in_hvt1", &self.usbpd_in_hvt1())
                .field("xi_oe", &self.xi_oe())
                .field("xo_oe", &self.xo_oe())
                .field("xixo_gpio_en", &self.xixo_gpio_en())
                .finish()
        }
    }
    #[doc = "The interrupt enable register at the PA port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEnablePa(pub u32);
    impl IntEnablePa {
        #[doc = "The interrupt enable bit of the PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x3fff;
            val as u16
        }
        #[doc = "The interrupt enable bit of the PA port."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
        }
    }
    impl Default for IntEnablePa {
        #[inline(always)]
        fn default() -> IntEnablePa {
            IntEnablePa(0)
        }
    }
    impl core::fmt::Debug for IntEnablePa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEnablePa")
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[doc = "The interrupt enable register at the PB port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEnablePb(pub u32);
    impl IntEnablePb {
        #[doc = "The interrupt enable bit of the PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn pb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "The interrupt enable bit of the PB port."]
        #[inline(always)]
        pub const fn set_pb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for IntEnablePb {
        #[inline(always)]
        fn default() -> IntEnablePb {
            IntEnablePb(0)
        }
    }
    impl core::fmt::Debug for IntEnablePb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEnablePb")
                .field("pb", &self.pb())
                .finish()
        }
    }
    #[doc = "The interrupt enable register at the PD port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEnablePd(pub u32);
    impl IntEnablePd {
        #[doc = "The interrupt enable bit of the PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn pd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt enable bit of the PD port."]
        #[inline(always)]
        pub const fn set_pd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IntEnablePd {
        #[inline(always)]
        fn default() -> IntEnablePd {
            IntEnablePd(0)
        }
    }
    impl core::fmt::Debug for IntEnablePd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEnablePd")
                .field("pd", &self.pd())
                .finish()
        }
    }
    #[doc = "The interrupt mode register at the PA port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntModePa(pub u32);
    impl IntModePa {
        #[doc = "The interrupt mode bit of the PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x3fff;
            val as u16
        }
        #[doc = "The interrupt mode bit of the PA port."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
        }
    }
    impl Default for IntModePa {
        #[inline(always)]
        fn default() -> IntModePa {
            IntModePa(0)
        }
    }
    impl core::fmt::Debug for IntModePa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntModePa").field("pa", &self.pa()).finish()
        }
    }
    #[doc = "The interrupt mode register at the PB port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntModePb(pub u32);
    impl IntModePb {
        #[doc = "The interrupt mode bit of the PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn pb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "The interrupt mode bit of the PB port."]
        #[inline(always)]
        pub const fn set_pb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for IntModePb {
        #[inline(always)]
        fn default() -> IntModePb {
            IntModePb(0)
        }
    }
    impl core::fmt::Debug for IntModePb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntModePb").field("pb", &self.pb()).finish()
        }
    }
    #[doc = "The interrupt mode register at the PD port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntModePd(pub u32);
    impl IntModePd {
        #[doc = "The interrupt mode bit of the PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn pd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt mode bit of the PD port."]
        #[inline(always)]
        pub const fn set_pd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IntModePd {
        #[inline(always)]
        fn default() -> IntModePd {
            IntModePd(0)
        }
    }
    impl core::fmt::Debug for IntModePd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntModePd").field("pd", &self.pd()).finish()
        }
    }
    #[doc = "The interrupt polar register at the PA port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntPolarPa(pub u32);
    impl IntPolarPa {
        #[doc = "The interrupt polar bit of the PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x3fff;
            val as u16
        }
        #[doc = "The interrupt polar bit of the PA port."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
        }
    }
    impl Default for IntPolarPa {
        #[inline(always)]
        fn default() -> IntPolarPa {
            IntPolarPa(0)
        }
    }
    impl core::fmt::Debug for IntPolarPa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntPolarPa")
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[doc = "The interrupt polar register at the PB port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntPolarPb(pub u32);
    impl IntPolarPb {
        #[doc = "The interrupt polar bit of the PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn pb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "The interrupt polar bit of the PB port."]
        #[inline(always)]
        pub const fn set_pb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for IntPolarPb {
        #[inline(always)]
        fn default() -> IntPolarPb {
            IntPolarPb(0)
        }
    }
    impl core::fmt::Debug for IntPolarPb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntPolarPb")
                .field("pb", &self.pb())
                .finish()
        }
    }
    #[doc = "The interrupt polar register at the PD port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntPolarPd(pub u32);
    impl IntPolarPd {
        #[doc = "The interrupt polar bit of the PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn pd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt polar bit of the PD port."]
        #[inline(always)]
        pub const fn set_pd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IntPolarPd {
        #[inline(always)]
        fn default() -> IntPolarPd {
            IntPolarPd(0)
        }
    }
    impl core::fmt::Debug for IntPolarPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntPolarPd")
                .field("pd", &self.pd())
                .finish()
        }
    }
    #[doc = "The interrupt status register at the PA port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStatusPa(pub u32);
    impl IntStatusPa {
        #[doc = "VIO power down interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn vio_pwd_if(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VIO power down interrupt flag bit."]
        #[inline(always)]
        pub const fn set_vio_pwd_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "The interrupt status bit of the PA port."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x3fff;
            val as u16
        }
        #[doc = "The interrupt status bit of the PA port."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
        }
    }
    impl Default for IntStatusPa {
        #[inline(always)]
        fn default() -> IntStatusPa {
            IntStatusPa(0)
        }
    }
    impl core::fmt::Debug for IntStatusPa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStatusPa")
                .field("vio_pwd_if", &self.vio_pwd_if())
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[doc = "The interrupt status register at the PB port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStatusPb(pub u32);
    impl IntStatusPb {
        #[doc = "The interrupt status bit of the PB port."]
        #[must_use]
        #[inline(always)]
        pub const fn pb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[doc = "The interrupt status bit of the PB port."]
        #[inline(always)]
        pub const fn set_pb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
    }
    impl Default for IntStatusPb {
        #[inline(always)]
        fn default() -> IntStatusPb {
            IntStatusPb(0)
        }
    }
    impl core::fmt::Debug for IntStatusPb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStatusPb")
                .field("pb", &self.pb())
                .finish()
        }
    }
    #[doc = "The interrupt status register at the PD port."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStatusPd(pub u32);
    impl IntStatusPd {
        #[doc = "The interrupt status bit of the PD port."]
        #[must_use]
        #[inline(always)]
        pub const fn pd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The interrupt status bit of the PD port."]
        #[inline(always)]
        pub const fn set_pd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IntStatusPd {
        #[inline(always)]
        fn default() -> IntStatusPd {
            IntStatusPd(0)
        }
    }
    impl core::fmt::Debug for IntStatusPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStatusPd")
                .field("pd", &self.pd())
                .finish()
        }
    }
    #[doc = "GPIO PA clear output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaClr(pub u32);
    impl PaClr {
        #[doc = "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_clr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pa_clr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaClr {
        #[inline(always)]
        fn default() -> PaClr {
            PaClr(0)
        }
    }
    impl core::fmt::Debug for PaClr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaClr")
                .field("pa_clr[0]", &self.pa_clr(0usize))
                .field("pa_clr[1]", &self.pa_clr(1usize))
                .field("pa_clr[2]", &self.pa_clr(2usize))
                .field("pa_clr[3]", &self.pa_clr(3usize))
                .field("pa_clr[4]", &self.pa_clr(4usize))
                .field("pa_clr[5]", &self.pa_clr(5usize))
                .field("pa_clr[6]", &self.pa_clr(6usize))
                .field("pa_clr[7]", &self.pa_clr(7usize))
                .field("pa_clr[8]", &self.pa_clr(8usize))
                .field("pa_clr[9]", &self.pa_clr(9usize))
                .field("pa_clr[10]", &self.pa_clr(10usize))
                .field("pa_clr[11]", &self.pa_clr(11usize))
                .field("pa_clr[12]", &self.pa_clr(12usize))
                .field("pa_clr[13]", &self.pa_clr(13usize))
                .field("pa_clr[14]", &self.pa_clr(14usize))
                .field("pa_clr[15]", &self.pa_clr(15usize))
                .field("pa_clr[16]", &self.pa_clr(16usize))
                .field("pa_clr[17]", &self.pa_clr(17usize))
                .field("pa_clr[18]", &self.pa_clr(18usize))
                .field("pa_clr[19]", &self.pa_clr(19usize))
                .field("pa_clr[20]", &self.pa_clr(20usize))
                .field("pa_clr[21]", &self.pa_clr(21usize))
                .field("pa_clr[22]", &self.pa_clr(22usize))
                .field("pa_clr[23]", &self.pa_clr(23usize))
                .field("pa_clr[24]", &self.pa_clr(24usize))
                .field("pa_clr[25]", &self.pa_clr(25usize))
                .field("pa_clr[26]", &self.pa_clr(26usize))
                .field("pa_clr[27]", &self.pa_clr(27usize))
                .field("pa_clr[28]", &self.pa_clr(28usize))
                .field("pa_clr[29]", &self.pa_clr(29usize))
                .field("pa_clr[30]", &self.pa_clr(30usize))
                .field("pa_clr[31]", &self.pa_clr(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA I/O direction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaDir(pub u32);
    impl PaDir {
        #[doc = "GPIO PA I/O direction: 0=input, 1=output."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn set_pa_dir(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaDir {
        #[inline(always)]
        fn default() -> PaDir {
            PaDir(0)
        }
    }
    impl core::fmt::Debug for PaDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaDir")
                .field("pa_dir[0]", &self.pa_dir(0usize))
                .field("pa_dir[1]", &self.pa_dir(1usize))
                .field("pa_dir[2]", &self.pa_dir(2usize))
                .field("pa_dir[3]", &self.pa_dir(3usize))
                .field("pa_dir[4]", &self.pa_dir(4usize))
                .field("pa_dir[5]", &self.pa_dir(5usize))
                .field("pa_dir[6]", &self.pa_dir(6usize))
                .field("pa_dir[7]", &self.pa_dir(7usize))
                .field("pa_dir[8]", &self.pa_dir(8usize))
                .field("pa_dir[9]", &self.pa_dir(9usize))
                .field("pa_dir[10]", &self.pa_dir(10usize))
                .field("pa_dir[11]", &self.pa_dir(11usize))
                .field("pa_dir[12]", &self.pa_dir(12usize))
                .field("pa_dir[13]", &self.pa_dir(13usize))
                .field("pa_dir[14]", &self.pa_dir(14usize))
                .field("pa_dir[15]", &self.pa_dir(15usize))
                .field("pa_dir[16]", &self.pa_dir(16usize))
                .field("pa_dir[17]", &self.pa_dir(17usize))
                .field("pa_dir[18]", &self.pa_dir(18usize))
                .field("pa_dir[19]", &self.pa_dir(19usize))
                .field("pa_dir[20]", &self.pa_dir(20usize))
                .field("pa_dir[21]", &self.pa_dir(21usize))
                .field("pa_dir[22]", &self.pa_dir(22usize))
                .field("pa_dir[23]", &self.pa_dir(23usize))
                .field("pa_dir[24]", &self.pa_dir(24usize))
                .field("pa_dir[25]", &self.pa_dir(25usize))
                .field("pa_dir[26]", &self.pa_dir(26usize))
                .field("pa_dir[27]", &self.pa_dir(27usize))
                .field("pa_dir[28]", &self.pa_dir(28usize))
                .field("pa_dir[29]", &self.pa_dir(29usize))
                .field("pa_dir[30]", &self.pa_dir(30usize))
                .field("pa_dir[31]", &self.pa_dir(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaOut(pub u32);
    impl PaOut {
        #[doc = "GPIO PA output level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA output level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pa_out(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaOut {
        #[inline(always)]
        fn default() -> PaOut {
            PaOut(0)
        }
    }
    impl core::fmt::Debug for PaOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaOut")
                .field("pa_out[0]", &self.pa_out(0usize))
                .field("pa_out[1]", &self.pa_out(1usize))
                .field("pa_out[2]", &self.pa_out(2usize))
                .field("pa_out[3]", &self.pa_out(3usize))
                .field("pa_out[4]", &self.pa_out(4usize))
                .field("pa_out[5]", &self.pa_out(5usize))
                .field("pa_out[6]", &self.pa_out(6usize))
                .field("pa_out[7]", &self.pa_out(7usize))
                .field("pa_out[8]", &self.pa_out(8usize))
                .field("pa_out[9]", &self.pa_out(9usize))
                .field("pa_out[10]", &self.pa_out(10usize))
                .field("pa_out[11]", &self.pa_out(11usize))
                .field("pa_out[12]", &self.pa_out(12usize))
                .field("pa_out[13]", &self.pa_out(13usize))
                .field("pa_out[14]", &self.pa_out(14usize))
                .field("pa_out[15]", &self.pa_out(15usize))
                .field("pa_out[16]", &self.pa_out(16usize))
                .field("pa_out[17]", &self.pa_out(17usize))
                .field("pa_out[18]", &self.pa_out(18usize))
                .field("pa_out[19]", &self.pa_out(19usize))
                .field("pa_out[20]", &self.pa_out(20usize))
                .field("pa_out[21]", &self.pa_out(21usize))
                .field("pa_out[22]", &self.pa_out(22usize))
                .field("pa_out[23]", &self.pa_out(23usize))
                .field("pa_out[24]", &self.pa_out(24usize))
                .field("pa_out[25]", &self.pa_out(25usize))
                .field("pa_out[26]", &self.pa_out(26usize))
                .field("pa_out[27]", &self.pa_out(27usize))
                .field("pa_out[28]", &self.pa_out(28usize))
                .field("pa_out[29]", &self.pa_out(29usize))
                .field("pa_out[30]", &self.pa_out(30usize))
                .field("pa_out[31]", &self.pa_out(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA output open-drain_input pulldown resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPd(pub u32);
    impl PaPd {
        #[doc = "GPIO PA pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pa_pd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPd {
        #[inline(always)]
        fn default() -> PaPd {
            PaPd(0)
        }
    }
    impl core::fmt::Debug for PaPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPd")
                .field("pa_pd[0]", &self.pa_pd(0usize))
                .field("pa_pd[1]", &self.pa_pd(1usize))
                .field("pa_pd[2]", &self.pa_pd(2usize))
                .field("pa_pd[3]", &self.pa_pd(3usize))
                .field("pa_pd[4]", &self.pa_pd(4usize))
                .field("pa_pd[5]", &self.pa_pd(5usize))
                .field("pa_pd[6]", &self.pa_pd(6usize))
                .field("pa_pd[7]", &self.pa_pd(7usize))
                .field("pa_pd[8]", &self.pa_pd(8usize))
                .field("pa_pd[9]", &self.pa_pd(9usize))
                .field("pa_pd[10]", &self.pa_pd(10usize))
                .field("pa_pd[11]", &self.pa_pd(11usize))
                .field("pa_pd[12]", &self.pa_pd(12usize))
                .field("pa_pd[13]", &self.pa_pd(13usize))
                .field("pa_pd[14]", &self.pa_pd(14usize))
                .field("pa_pd[15]", &self.pa_pd(15usize))
                .field("pa_pd[16]", &self.pa_pd(16usize))
                .field("pa_pd[17]", &self.pa_pd(17usize))
                .field("pa_pd[18]", &self.pa_pd(18usize))
                .field("pa_pd[19]", &self.pa_pd(19usize))
                .field("pa_pd[20]", &self.pa_pd(20usize))
                .field("pa_pd[21]", &self.pa_pd(21usize))
                .field("pa_pd[22]", &self.pa_pd(22usize))
                .field("pa_pd[23]", &self.pa_pd(23usize))
                .field("pa_pd[24]", &self.pa_pd(24usize))
                .field("pa_pd[25]", &self.pa_pd(25usize))
                .field("pa_pd[26]", &self.pa_pd(26usize))
                .field("pa_pd[27]", &self.pa_pd(27usize))
                .field("pa_pd[28]", &self.pa_pd(28usize))
                .field("pa_pd[29]", &self.pa_pd(29usize))
                .field("pa_pd[30]", &self.pa_pd(30usize))
                .field("pa_pd[31]", &self.pa_pd(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPin(pub u32);
    impl PaPin {
        #[doc = "GPIO PA input level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pa_pin(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPin {
        #[inline(always)]
        fn default() -> PaPin {
            PaPin(0)
        }
    }
    impl core::fmt::Debug for PaPin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPin")
                .field("pa_pin[0]", &self.pa_pin(0usize))
                .field("pa_pin[1]", &self.pa_pin(1usize))
                .field("pa_pin[2]", &self.pa_pin(2usize))
                .field("pa_pin[3]", &self.pa_pin(3usize))
                .field("pa_pin[4]", &self.pa_pin(4usize))
                .field("pa_pin[5]", &self.pa_pin(5usize))
                .field("pa_pin[6]", &self.pa_pin(6usize))
                .field("pa_pin[7]", &self.pa_pin(7usize))
                .field("pa_pin[8]", &self.pa_pin(8usize))
                .field("pa_pin[9]", &self.pa_pin(9usize))
                .field("pa_pin[10]", &self.pa_pin(10usize))
                .field("pa_pin[11]", &self.pa_pin(11usize))
                .field("pa_pin[12]", &self.pa_pin(12usize))
                .field("pa_pin[13]", &self.pa_pin(13usize))
                .field("pa_pin[14]", &self.pa_pin(14usize))
                .field("pa_pin[15]", &self.pa_pin(15usize))
                .field("pa_pin[16]", &self.pa_pin(16usize))
                .field("pa_pin[17]", &self.pa_pin(17usize))
                .field("pa_pin[18]", &self.pa_pin(18usize))
                .field("pa_pin[19]", &self.pa_pin(19usize))
                .field("pa_pin[20]", &self.pa_pin(20usize))
                .field("pa_pin[21]", &self.pa_pin(21usize))
                .field("pa_pin[22]", &self.pa_pin(22usize))
                .field("pa_pin[23]", &self.pa_pin(23usize))
                .field("pa_pin[24]", &self.pa_pin(24usize))
                .field("pa_pin[25]", &self.pa_pin(25usize))
                .field("pa_pin[26]", &self.pa_pin(26usize))
                .field("pa_pin[27]", &self.pa_pin(27usize))
                .field("pa_pin[28]", &self.pa_pin(28usize))
                .field("pa_pin[29]", &self.pa_pin(29usize))
                .field("pa_pin[30]", &self.pa_pin(30usize))
                .field("pa_pin[31]", &self.pa_pin(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PA pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPu(pub u32);
    impl PaPu {
        #[doc = "GPIO PA pull-up resistor: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pa_pu(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPu {
        #[inline(always)]
        fn default() -> PaPu {
            PaPu(0)
        }
    }
    impl core::fmt::Debug for PaPu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPu")
                .field("pa_pu[0]", &self.pa_pu(0usize))
                .field("pa_pu[1]", &self.pa_pu(1usize))
                .field("pa_pu[2]", &self.pa_pu(2usize))
                .field("pa_pu[3]", &self.pa_pu(3usize))
                .field("pa_pu[4]", &self.pa_pu(4usize))
                .field("pa_pu[5]", &self.pa_pu(5usize))
                .field("pa_pu[6]", &self.pa_pu(6usize))
                .field("pa_pu[7]", &self.pa_pu(7usize))
                .field("pa_pu[8]", &self.pa_pu(8usize))
                .field("pa_pu[9]", &self.pa_pu(9usize))
                .field("pa_pu[10]", &self.pa_pu(10usize))
                .field("pa_pu[11]", &self.pa_pu(11usize))
                .field("pa_pu[12]", &self.pa_pu(12usize))
                .field("pa_pu[13]", &self.pa_pu(13usize))
                .field("pa_pu[14]", &self.pa_pu(14usize))
                .field("pa_pu[15]", &self.pa_pu(15usize))
                .field("pa_pu[16]", &self.pa_pu(16usize))
                .field("pa_pu[17]", &self.pa_pu(17usize))
                .field("pa_pu[18]", &self.pa_pu(18usize))
                .field("pa_pu[19]", &self.pa_pu(19usize))
                .field("pa_pu[20]", &self.pa_pu(20usize))
                .field("pa_pu[21]", &self.pa_pu(21usize))
                .field("pa_pu[22]", &self.pa_pu(22usize))
                .field("pa_pu[23]", &self.pa_pu(23usize))
                .field("pa_pu[24]", &self.pa_pu(24usize))
                .field("pa_pu[25]", &self.pa_pu(25usize))
                .field("pa_pu[26]", &self.pa_pu(26usize))
                .field("pa_pu[27]", &self.pa_pu(27usize))
                .field("pa_pu[28]", &self.pa_pu(28usize))
                .field("pa_pu[29]", &self.pa_pu(29usize))
                .field("pa_pu[30]", &self.pa_pu(30usize))
                .field("pa_pu[31]", &self.pa_pu(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PB clear output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbClr(pub u32);
    impl PbClr {
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_clr(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pb_clr(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbClr {
        #[inline(always)]
        fn default() -> PbClr {
            PbClr(0)
        }
    }
    impl core::fmt::Debug for PbClr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbClr")
                .field("pb_clr[0]", &self.pb_clr(0usize))
                .field("pb_clr[1]", &self.pb_clr(1usize))
                .field("pb_clr[2]", &self.pb_clr(2usize))
                .field("pb_clr[3]", &self.pb_clr(3usize))
                .field("pb_clr[4]", &self.pb_clr(4usize))
                .field("pb_clr[5]", &self.pb_clr(5usize))
                .field("pb_clr[6]", &self.pb_clr(6usize))
                .field("pb_clr[7]", &self.pb_clr(7usize))
                .field("pb_clr[8]", &self.pb_clr(8usize))
                .field("pb_clr[9]", &self.pb_clr(9usize))
                .field("pb_clr[10]", &self.pb_clr(10usize))
                .field("pb_clr[11]", &self.pb_clr(11usize))
                .field("pb_clr[12]", &self.pb_clr(12usize))
                .field("pb_clr[13]", &self.pb_clr(13usize))
                .field("pb_clr[14]", &self.pb_clr(14usize))
                .field("pb_clr[15]", &self.pb_clr(15usize))
                .field("pb_clr[16]", &self.pb_clr(16usize))
                .field("pb_clr[17]", &self.pb_clr(17usize))
                .field("pb_clr[18]", &self.pb_clr(18usize))
                .field("pb_clr[19]", &self.pb_clr(19usize))
                .field("pb_clr[20]", &self.pb_clr(20usize))
                .field("pb_clr[21]", &self.pb_clr(21usize))
                .field("pb_clr[22]", &self.pb_clr(22usize))
                .finish()
        }
    }
    #[doc = "GPIO PB I/O direction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbDir(pub u32);
    impl PbDir {
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_dir(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn set_pb_dir(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Pin current intput/output direction control domain."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_dp_dir(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Pin current intput/output direction control domain."]
        #[inline(always)]
        pub const fn set_usb_dp_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Pin current intput/output direction control domain."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_dm_dir(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pin current intput/output direction control domain."]
        #[inline(always)]
        pub const fn set_usb_dm_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PbDir {
        #[inline(always)]
        fn default() -> PbDir {
            PbDir(0)
        }
    }
    impl core::fmt::Debug for PbDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbDir")
                .field("pb_dir[0]", &self.pb_dir(0usize))
                .field("pb_dir[1]", &self.pb_dir(1usize))
                .field("pb_dir[2]", &self.pb_dir(2usize))
                .field("pb_dir[3]", &self.pb_dir(3usize))
                .field("pb_dir[4]", &self.pb_dir(4usize))
                .field("pb_dir[5]", &self.pb_dir(5usize))
                .field("pb_dir[6]", &self.pb_dir(6usize))
                .field("pb_dir[7]", &self.pb_dir(7usize))
                .field("pb_dir[8]", &self.pb_dir(8usize))
                .field("pb_dir[9]", &self.pb_dir(9usize))
                .field("pb_dir[10]", &self.pb_dir(10usize))
                .field("pb_dir[11]", &self.pb_dir(11usize))
                .field("pb_dir[12]", &self.pb_dir(12usize))
                .field("pb_dir[13]", &self.pb_dir(13usize))
                .field("pb_dir[14]", &self.pb_dir(14usize))
                .field("pb_dir[15]", &self.pb_dir(15usize))
                .field("pb_dir[16]", &self.pb_dir(16usize))
                .field("pb_dir[17]", &self.pb_dir(17usize))
                .field("pb_dir[18]", &self.pb_dir(18usize))
                .field("pb_dir[19]", &self.pb_dir(19usize))
                .field("pb_dir[20]", &self.pb_dir(20usize))
                .field("pb_dir[21]", &self.pb_dir(21usize))
                .field("pb_dir[22]", &self.pb_dir(22usize))
                .field("usb_dp_dir", &self.usb_dp_dir())
                .field("usb_dm_dir", &self.usb_dm_dir())
                .finish()
        }
    }
    #[doc = "GPIO PB output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbOut(pub u32);
    impl PbOut {
        #[doc = "GPIO PB output level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_out(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB output level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pb_out(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Pin output level status."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udp_out(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Pin output level status."]
        #[inline(always)]
        pub const fn set_usb_udp_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Pin output level status."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udm_out(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pin output level status."]
        #[inline(always)]
        pub const fn set_usb_udm_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PbOut {
        #[inline(always)]
        fn default() -> PbOut {
            PbOut(0)
        }
    }
    impl core::fmt::Debug for PbOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbOut")
                .field("pb_out[0]", &self.pb_out(0usize))
                .field("pb_out[1]", &self.pb_out(1usize))
                .field("pb_out[2]", &self.pb_out(2usize))
                .field("pb_out[3]", &self.pb_out(3usize))
                .field("pb_out[4]", &self.pb_out(4usize))
                .field("pb_out[5]", &self.pb_out(5usize))
                .field("pb_out[6]", &self.pb_out(6usize))
                .field("pb_out[7]", &self.pb_out(7usize))
                .field("pb_out[8]", &self.pb_out(8usize))
                .field("pb_out[9]", &self.pb_out(9usize))
                .field("pb_out[10]", &self.pb_out(10usize))
                .field("pb_out[11]", &self.pb_out(11usize))
                .field("pb_out[12]", &self.pb_out(12usize))
                .field("pb_out[13]", &self.pb_out(13usize))
                .field("pb_out[14]", &self.pb_out(14usize))
                .field("pb_out[15]", &self.pb_out(15usize))
                .field("pb_out[16]", &self.pb_out(16usize))
                .field("pb_out[17]", &self.pb_out(17usize))
                .field("pb_out[18]", &self.pb_out(18usize))
                .field("pb_out[19]", &self.pb_out(19usize))
                .field("pb_out[20]", &self.pb_out(20usize))
                .field("pb_out[21]", &self.pb_out(21usize))
                .field("pb_out[22]", &self.pb_out(22usize))
                .field("usb_udp_out", &self.usb_udp_out())
                .field("usb_udm_out", &self.usb_udm_out())
                .finish()
        }
    }
    #[doc = "GPIO PB output open-drain_input pulldown resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPd(pub u32);
    impl PbPd {
        #[doc = "GPIO PB pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pd(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pb_pd(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Pin function enable /close control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udp_pd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Pin function enable /close control bit."]
        #[inline(always)]
        pub const fn set_usb_udp_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Pin function enable /close control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udm_pd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pin function enable /close control bit."]
        #[inline(always)]
        pub const fn set_usb_udm_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PbPd {
        #[inline(always)]
        fn default() -> PbPd {
            PbPd(0)
        }
    }
    impl core::fmt::Debug for PbPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPd")
                .field("pb_pd[0]", &self.pb_pd(0usize))
                .field("pb_pd[1]", &self.pb_pd(1usize))
                .field("pb_pd[2]", &self.pb_pd(2usize))
                .field("pb_pd[3]", &self.pb_pd(3usize))
                .field("pb_pd[4]", &self.pb_pd(4usize))
                .field("pb_pd[5]", &self.pb_pd(5usize))
                .field("pb_pd[6]", &self.pb_pd(6usize))
                .field("pb_pd[7]", &self.pb_pd(7usize))
                .field("pb_pd[8]", &self.pb_pd(8usize))
                .field("pb_pd[9]", &self.pb_pd(9usize))
                .field("pb_pd[10]", &self.pb_pd(10usize))
                .field("pb_pd[11]", &self.pb_pd(11usize))
                .field("pb_pd[12]", &self.pb_pd(12usize))
                .field("pb_pd[13]", &self.pb_pd(13usize))
                .field("pb_pd[14]", &self.pb_pd(14usize))
                .field("pb_pd[15]", &self.pb_pd(15usize))
                .field("pb_pd[16]", &self.pb_pd(16usize))
                .field("pb_pd[17]", &self.pb_pd(17usize))
                .field("pb_pd[18]", &self.pb_pd(18usize))
                .field("pb_pd[19]", &self.pb_pd(19usize))
                .field("pb_pd[20]", &self.pb_pd(20usize))
                .field("pb_pd[21]", &self.pb_pd(21usize))
                .field("pb_pd[22]", &self.pb_pd(22usize))
                .field("usb_udp_pd", &self.usb_udp_pd())
                .field("usb_udm_pd", &self.usb_udm_pd())
                .finish()
        }
    }
    #[doc = "GPIO PB input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPin(pub u32);
    impl PbPin {
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pin(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pb_pin(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Pin intput level status."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udp_in(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Pin intput level status."]
        #[inline(always)]
        pub const fn set_usb_udp_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Pin intput level status."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udm_in(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pin intput level status."]
        #[inline(always)]
        pub const fn set_usb_udm_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PbPin {
        #[inline(always)]
        fn default() -> PbPin {
            PbPin(0)
        }
    }
    impl core::fmt::Debug for PbPin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPin")
                .field("pb_pin[0]", &self.pb_pin(0usize))
                .field("pb_pin[1]", &self.pb_pin(1usize))
                .field("pb_pin[2]", &self.pb_pin(2usize))
                .field("pb_pin[3]", &self.pb_pin(3usize))
                .field("pb_pin[4]", &self.pb_pin(4usize))
                .field("pb_pin[5]", &self.pb_pin(5usize))
                .field("pb_pin[6]", &self.pb_pin(6usize))
                .field("pb_pin[7]", &self.pb_pin(7usize))
                .field("pb_pin[8]", &self.pb_pin(8usize))
                .field("pb_pin[9]", &self.pb_pin(9usize))
                .field("pb_pin[10]", &self.pb_pin(10usize))
                .field("pb_pin[11]", &self.pb_pin(11usize))
                .field("pb_pin[12]", &self.pb_pin(12usize))
                .field("pb_pin[13]", &self.pb_pin(13usize))
                .field("pb_pin[14]", &self.pb_pin(14usize))
                .field("pb_pin[15]", &self.pb_pin(15usize))
                .field("pb_pin[16]", &self.pb_pin(16usize))
                .field("pb_pin[17]", &self.pb_pin(17usize))
                .field("pb_pin[18]", &self.pb_pin(18usize))
                .field("pb_pin[19]", &self.pb_pin(19usize))
                .field("pb_pin[20]", &self.pb_pin(20usize))
                .field("pb_pin[21]", &self.pb_pin(21usize))
                .field("pb_pin[22]", &self.pb_pin(22usize))
                .field("usb_udp_in", &self.usb_udp_in())
                .field("usb_udm_in", &self.usb_udm_in())
                .finish()
        }
    }
    #[doc = "GPIO PB pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPu(pub u32);
    impl PbPu {
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pu(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pb_pu(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Pin pull-up function enable /close control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udp_pu(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Pin pull-up function enable /close control bit."]
        #[inline(always)]
        pub const fn set_usb_udp_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Pin pull-up function enable /close control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_udm_pu(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Pin pull-up function enable /close control bit."]
        #[inline(always)]
        pub const fn set_usb_udm_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PbPu {
        #[inline(always)]
        fn default() -> PbPu {
            PbPu(0)
        }
    }
    impl core::fmt::Debug for PbPu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPu")
                .field("pb_pu[0]", &self.pb_pu(0usize))
                .field("pb_pu[1]", &self.pb_pu(1usize))
                .field("pb_pu[2]", &self.pb_pu(2usize))
                .field("pb_pu[3]", &self.pb_pu(3usize))
                .field("pb_pu[4]", &self.pb_pu(4usize))
                .field("pb_pu[5]", &self.pb_pu(5usize))
                .field("pb_pu[6]", &self.pb_pu(6usize))
                .field("pb_pu[7]", &self.pb_pu(7usize))
                .field("pb_pu[8]", &self.pb_pu(8usize))
                .field("pb_pu[9]", &self.pb_pu(9usize))
                .field("pb_pu[10]", &self.pb_pu(10usize))
                .field("pb_pu[11]", &self.pb_pu(11usize))
                .field("pb_pu[12]", &self.pb_pu(12usize))
                .field("pb_pu[13]", &self.pb_pu(13usize))
                .field("pb_pu[14]", &self.pb_pu(14usize))
                .field("pb_pu[15]", &self.pb_pu(15usize))
                .field("pb_pu[16]", &self.pb_pu(16usize))
                .field("pb_pu[17]", &self.pb_pu(17usize))
                .field("pb_pu[18]", &self.pb_pu(18usize))
                .field("pb_pu[19]", &self.pb_pu(19usize))
                .field("pb_pu[20]", &self.pb_pu(20usize))
                .field("pb_pu[21]", &self.pb_pu(21usize))
                .field("pb_pu[22]", &self.pb_pu(22usize))
                .field("usb_udp_pu", &self.usb_udp_pu())
                .field("usb_udm_pu", &self.usb_udm_pu())
                .finish()
        }
    }
    #[doc = "GPIO PD I/O direction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdDir(pub u32);
    impl PdDir {
        #[doc = "GPIO PD I/O direction: 0=input, 1=output."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PD I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn set_pd_dir(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PdDir {
        #[inline(always)]
        fn default() -> PdDir {
            PdDir(0)
        }
    }
    impl core::fmt::Debug for PdDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdDir")
                .field("pd_dir[0]", &self.pd_dir(0usize))
                .field("pd_dir[1]", &self.pd_dir(1usize))
                .field("pd_dir[2]", &self.pd_dir(2usize))
                .field("pd_dir[3]", &self.pd_dir(3usize))
                .field("pd_dir[4]", &self.pd_dir(4usize))
                .field("pd_dir[5]", &self.pd_dir(5usize))
                .field("pd_dir[6]", &self.pd_dir(6usize))
                .field("pd_dir[7]", &self.pd_dir(7usize))
                .field("pd_dir[8]", &self.pd_dir(8usize))
                .field("pd_dir[9]", &self.pd_dir(9usize))
                .field("pd_dir[10]", &self.pd_dir(10usize))
                .field("pd_dir[11]", &self.pd_dir(11usize))
                .field("pd_dir[12]", &self.pd_dir(12usize))
                .field("pd_dir[13]", &self.pd_dir(13usize))
                .field("pd_dir[14]", &self.pd_dir(14usize))
                .field("pd_dir[15]", &self.pd_dir(15usize))
                .field("pd_dir[16]", &self.pd_dir(16usize))
                .field("pd_dir[17]", &self.pd_dir(17usize))
                .field("pd_dir[18]", &self.pd_dir(18usize))
                .field("pd_dir[19]", &self.pd_dir(19usize))
                .field("pd_dir[20]", &self.pd_dir(20usize))
                .field("pd_dir[21]", &self.pd_dir(21usize))
                .field("pd_dir[22]", &self.pd_dir(22usize))
                .field("pd_dir[23]", &self.pd_dir(23usize))
                .field("pd_dir[24]", &self.pd_dir(24usize))
                .field("pd_dir[25]", &self.pd_dir(25usize))
                .field("pd_dir[26]", &self.pd_dir(26usize))
                .field("pd_dir[27]", &self.pd_dir(27usize))
                .field("pd_dir[28]", &self.pd_dir(28usize))
                .field("pd_dir[29]", &self.pd_dir(29usize))
                .field("pd_dir[30]", &self.pd_dir(30usize))
                .field("pd_dir[31]", &self.pd_dir(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PD output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdOut(pub u32);
    impl PdOut {
        #[doc = "GPIO PD output level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PD output level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pd_out(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PdOut {
        #[inline(always)]
        fn default() -> PdOut {
            PdOut(0)
        }
    }
    impl core::fmt::Debug for PdOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdOut")
                .field("pd_out[0]", &self.pd_out(0usize))
                .field("pd_out[1]", &self.pd_out(1usize))
                .field("pd_out[2]", &self.pd_out(2usize))
                .field("pd_out[3]", &self.pd_out(3usize))
                .field("pd_out[4]", &self.pd_out(4usize))
                .field("pd_out[5]", &self.pd_out(5usize))
                .field("pd_out[6]", &self.pd_out(6usize))
                .field("pd_out[7]", &self.pd_out(7usize))
                .field("pd_out[8]", &self.pd_out(8usize))
                .field("pd_out[9]", &self.pd_out(9usize))
                .field("pd_out[10]", &self.pd_out(10usize))
                .field("pd_out[11]", &self.pd_out(11usize))
                .field("pd_out[12]", &self.pd_out(12usize))
                .field("pd_out[13]", &self.pd_out(13usize))
                .field("pd_out[14]", &self.pd_out(14usize))
                .field("pd_out[15]", &self.pd_out(15usize))
                .field("pd_out[16]", &self.pd_out(16usize))
                .field("pd_out[17]", &self.pd_out(17usize))
                .field("pd_out[18]", &self.pd_out(18usize))
                .field("pd_out[19]", &self.pd_out(19usize))
                .field("pd_out[20]", &self.pd_out(20usize))
                .field("pd_out[21]", &self.pd_out(21usize))
                .field("pd_out[22]", &self.pd_out(22usize))
                .field("pd_out[23]", &self.pd_out(23usize))
                .field("pd_out[24]", &self.pd_out(24usize))
                .field("pd_out[25]", &self.pd_out(25usize))
                .field("pd_out[26]", &self.pd_out(26usize))
                .field("pd_out[27]", &self.pd_out(27usize))
                .field("pd_out[28]", &self.pd_out(28usize))
                .field("pd_out[29]", &self.pd_out(29usize))
                .field("pd_out[30]", &self.pd_out(30usize))
                .field("pd_out[31]", &self.pd_out(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PD output open-drain_input pulldown resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdPd(pub u32);
    impl PdPd {
        #[doc = "GPIO PD pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_pd(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PD pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pd_pd(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PdPd {
        #[inline(always)]
        fn default() -> PdPd {
            PdPd(0)
        }
    }
    impl core::fmt::Debug for PdPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdPd")
                .field("pd_pd[0]", &self.pd_pd(0usize))
                .field("pd_pd[1]", &self.pd_pd(1usize))
                .field("pd_pd[2]", &self.pd_pd(2usize))
                .field("pd_pd[3]", &self.pd_pd(3usize))
                .field("pd_pd[4]", &self.pd_pd(4usize))
                .field("pd_pd[5]", &self.pd_pd(5usize))
                .field("pd_pd[6]", &self.pd_pd(6usize))
                .field("pd_pd[7]", &self.pd_pd(7usize))
                .field("pd_pd[8]", &self.pd_pd(8usize))
                .field("pd_pd[9]", &self.pd_pd(9usize))
                .field("pd_pd[10]", &self.pd_pd(10usize))
                .field("pd_pd[11]", &self.pd_pd(11usize))
                .field("pd_pd[12]", &self.pd_pd(12usize))
                .field("pd_pd[13]", &self.pd_pd(13usize))
                .field("pd_pd[14]", &self.pd_pd(14usize))
                .field("pd_pd[15]", &self.pd_pd(15usize))
                .field("pd_pd[16]", &self.pd_pd(16usize))
                .field("pd_pd[17]", &self.pd_pd(17usize))
                .field("pd_pd[18]", &self.pd_pd(18usize))
                .field("pd_pd[19]", &self.pd_pd(19usize))
                .field("pd_pd[20]", &self.pd_pd(20usize))
                .field("pd_pd[21]", &self.pd_pd(21usize))
                .field("pd_pd[22]", &self.pd_pd(22usize))
                .field("pd_pd[23]", &self.pd_pd(23usize))
                .field("pd_pd[24]", &self.pd_pd(24usize))
                .field("pd_pd[25]", &self.pd_pd(25usize))
                .field("pd_pd[26]", &self.pd_pd(26usize))
                .field("pd_pd[27]", &self.pd_pd(27usize))
                .field("pd_pd[28]", &self.pd_pd(28usize))
                .field("pd_pd[29]", &self.pd_pd(29usize))
                .field("pd_pd[30]", &self.pd_pd(30usize))
                .field("pd_pd[31]", &self.pd_pd(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PD input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdPin(pub u32);
    impl PdPin {
        #[doc = "GPIO PD input level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PD input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pd_pin(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PdPin {
        #[inline(always)]
        fn default() -> PdPin {
            PdPin(0)
        }
    }
    impl core::fmt::Debug for PdPin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdPin")
                .field("pd_pin[0]", &self.pd_pin(0usize))
                .field("pd_pin[1]", &self.pd_pin(1usize))
                .field("pd_pin[2]", &self.pd_pin(2usize))
                .field("pd_pin[3]", &self.pd_pin(3usize))
                .field("pd_pin[4]", &self.pd_pin(4usize))
                .field("pd_pin[5]", &self.pd_pin(5usize))
                .field("pd_pin[6]", &self.pd_pin(6usize))
                .field("pd_pin[7]", &self.pd_pin(7usize))
                .field("pd_pin[8]", &self.pd_pin(8usize))
                .field("pd_pin[9]", &self.pd_pin(9usize))
                .field("pd_pin[10]", &self.pd_pin(10usize))
                .field("pd_pin[11]", &self.pd_pin(11usize))
                .field("pd_pin[12]", &self.pd_pin(12usize))
                .field("pd_pin[13]", &self.pd_pin(13usize))
                .field("pd_pin[14]", &self.pd_pin(14usize))
                .field("pd_pin[15]", &self.pd_pin(15usize))
                .field("pd_pin[16]", &self.pd_pin(16usize))
                .field("pd_pin[17]", &self.pd_pin(17usize))
                .field("pd_pin[18]", &self.pd_pin(18usize))
                .field("pd_pin[19]", &self.pd_pin(19usize))
                .field("pd_pin[20]", &self.pd_pin(20usize))
                .field("pd_pin[21]", &self.pd_pin(21usize))
                .field("pd_pin[22]", &self.pd_pin(22usize))
                .field("pd_pin[23]", &self.pd_pin(23usize))
                .field("pd_pin[24]", &self.pd_pin(24usize))
                .field("pd_pin[25]", &self.pd_pin(25usize))
                .field("pd_pin[26]", &self.pd_pin(26usize))
                .field("pd_pin[27]", &self.pd_pin(27usize))
                .field("pd_pin[28]", &self.pd_pin(28usize))
                .field("pd_pin[29]", &self.pd_pin(29usize))
                .field("pd_pin[30]", &self.pd_pin(30usize))
                .field("pd_pin[31]", &self.pd_pin(31usize))
                .finish()
        }
    }
    #[doc = "GPIO PD pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdPu(pub u32);
    impl PdPu {
        #[doc = "GPIO PD pull-up resistor: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pd_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PD pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pd_pu(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PdPu {
        #[inline(always)]
        fn default() -> PdPu {
            PdPu(0)
        }
    }
    impl core::fmt::Debug for PdPu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdPu")
                .field("pd_pu[0]", &self.pd_pu(0usize))
                .field("pd_pu[1]", &self.pd_pu(1usize))
                .field("pd_pu[2]", &self.pd_pu(2usize))
                .field("pd_pu[3]", &self.pd_pu(3usize))
                .field("pd_pu[4]", &self.pd_pu(4usize))
                .field("pd_pu[5]", &self.pd_pu(5usize))
                .field("pd_pu[6]", &self.pd_pu(6usize))
                .field("pd_pu[7]", &self.pd_pu(7usize))
                .field("pd_pu[8]", &self.pd_pu(8usize))
                .field("pd_pu[9]", &self.pd_pu(9usize))
                .field("pd_pu[10]", &self.pd_pu(10usize))
                .field("pd_pu[11]", &self.pd_pu(11usize))
                .field("pd_pu[12]", &self.pd_pu(12usize))
                .field("pd_pu[13]", &self.pd_pu(13usize))
                .field("pd_pu[14]", &self.pd_pu(14usize))
                .field("pd_pu[15]", &self.pd_pu(15usize))
                .field("pd_pu[16]", &self.pd_pu(16usize))
                .field("pd_pu[17]", &self.pd_pu(17usize))
                .field("pd_pu[18]", &self.pd_pu(18usize))
                .field("pd_pu[19]", &self.pd_pu(19usize))
                .field("pd_pu[20]", &self.pd_pu(20usize))
                .field("pd_pu[21]", &self.pd_pu(21usize))
                .field("pd_pu[22]", &self.pd_pu(22usize))
                .field("pd_pu[23]", &self.pd_pu(23usize))
                .field("pd_pu[24]", &self.pd_pu(24usize))
                .field("pd_pu[25]", &self.pd_pu(25usize))
                .field("pd_pu[26]", &self.pd_pu(26usize))
                .field("pd_pu[27]", &self.pd_pu(27usize))
                .field("pd_pu[28]", &self.pd_pu(28usize))
                .field("pd_pu[29]", &self.pd_pu(29usize))
                .field("pd_pu[30]", &self.pd_pu(30usize))
                .field("pd_pu[31]", &self.pd_pu(31usize))
                .finish()
        }
    }
}
