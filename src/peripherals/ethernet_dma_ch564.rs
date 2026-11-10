#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet: DMA controller operation."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetDma {
    ptr: *mut u8,
}
unsafe impl Send for EthernetDma {}
unsafe impl Sync for EthernetDma {}
impl EthernetDma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet DMA bus mode register."]
    #[inline(always)]
    pub const fn dmabmr(self) -> crate::common::Reg<regs::Dmabmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Ethernet DMA transmit poll demand. register."]
    #[inline(always)]
    pub const fn dmatpdr(self) -> crate::common::Reg<regs::Dmatpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "EHERNET DMA receive poll demand. register."]
    #[inline(always)]
    pub const fn dmarpdr(self) -> crate::common::Reg<regs::Dmarpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Ethernet DMA receive descriptor list address. register."]
    #[inline(always)]
    pub const fn dmardlar(self) -> crate::common::Reg<regs::Dmardlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Ethernet DMA transmit descriptor list. address register."]
    #[inline(always)]
    pub const fn dmatdlar(self) -> crate::common::Reg<regs::Dmatdlar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Ethernet DMA status register."]
    #[inline(always)]
    pub const fn dmasr(self) -> crate::common::Reg<regs::Dmasr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Ethernet DMA operation mode. register."]
    #[inline(always)]
    pub const fn dmaomr(self) -> crate::common::Reg<regs::Dmaomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Ethernet DMA interrupt enable. register."]
    #[inline(always)]
    pub const fn dmaier(self) -> crate::common::Reg<regs::Dmaier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Ethernet DMA missed frame and buffer. overflow counter register."]
    #[inline(always)]
    pub const fn dmamfbocr(self) -> crate::common::Reg<regs::Dmamfbocr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Ethernet DMA current host transmit. descriptor register."]
    #[inline(always)]
    pub const fn dmachtdr(self) -> crate::common::Reg<regs::Dmachtdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Ethernet DMA current host receive descriptor. register."]
    #[inline(always)]
    pub const fn dmachrdr(self) -> crate::common::Reg<regs::Dmachrdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Ethernet DMA current host transmit buffer. address register."]
    #[inline(always)]
    pub const fn dmachtbar(self) -> crate::common::Reg<regs::Dmachtbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Ethernet DMA current host receive buffer. address register."]
    #[inline(always)]
    pub const fn dmachrbar(self) -> crate::common::Reg<regs::Dmachrbar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
}
pub mod regs {
    #[doc = "Ethernet DMA bus mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmabmr(pub u32);
    impl Dmabmr {
        #[doc = "Software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Descriptor skip length."]
        #[must_use]
        #[inline(always)]
        pub const fn dsl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "Descriptor skip length."]
        #[inline(always)]
        pub const fn set_dsl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
    }
    impl Default for Dmabmr {
        #[inline(always)]
        fn default() -> Dmabmr {
            Dmabmr(0)
        }
    }
    impl core::fmt::Debug for Dmabmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmabmr")
                .field("sr", &self.sr())
                .field("dsl", &self.dsl())
                .finish()
        }
    }
    #[doc = "Ethernet DMA current host receive buffer. address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrbar(pub u32);
    impl Dmachrbar {
        #[doc = "Host receive buffer address. pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn hrbap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host receive buffer address. pointer."]
        #[inline(always)]
        pub const fn set_hrbap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrbar {
        #[inline(always)]
        fn default() -> Dmachrbar {
            Dmachrbar(0)
        }
    }
    impl core::fmt::Debug for Dmachrbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachrbar")
                .field("hrbap", &self.hrbap())
                .finish()
        }
    }
    #[doc = "Ethernet DMA current host receive descriptor. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachrdr(pub u32);
    impl Dmachrdr {
        #[doc = "Host receive descriptor address. pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn hrdap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host receive descriptor address. pointer."]
        #[inline(always)]
        pub const fn set_hrdap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachrdr {
        #[inline(always)]
        fn default() -> Dmachrdr {
            Dmachrdr(0)
        }
    }
    impl core::fmt::Debug for Dmachrdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachrdr")
                .field("hrdap", &self.hrdap())
                .finish()
        }
    }
    #[doc = "Ethernet DMA current host transmit buffer. address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtbar(pub u32);
    impl Dmachtbar {
        #[doc = "Host transmit buffer address. pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn htbap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host transmit buffer address. pointer."]
        #[inline(always)]
        pub const fn set_htbap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtbar {
        #[inline(always)]
        fn default() -> Dmachtbar {
            Dmachtbar(0)
        }
    }
    impl core::fmt::Debug for Dmachtbar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachtbar")
                .field("htbap", &self.htbap())
                .finish()
        }
    }
    #[doc = "Ethernet DMA current host transmit. descriptor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmachtdr(pub u32);
    impl Dmachtdr {
        #[doc = "Host transmit descriptor address. pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn htdap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Host transmit descriptor address. pointer."]
        #[inline(always)]
        pub const fn set_htdap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmachtdr {
        #[inline(always)]
        fn default() -> Dmachtdr {
            Dmachtdr(0)
        }
    }
    impl core::fmt::Debug for Dmachtdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmachtdr")
                .field("htdap", &self.htdap())
                .finish()
        }
    }
    #[doc = "Ethernet DMA interrupt enable. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaier(pub u32);
    impl Dmaier {
        #[doc = "Transmit interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit interrupt enable."]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tpsie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped interrupt. enable."]
        #[inline(always)]
        pub const fn set_tpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tbuie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable interrupt. enable."]
        #[inline(always)]
        pub const fn set_tbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tjtie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout interrupt. enable."]
        #[inline(always)]
        pub const fn set_tjtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn roie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_roie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Underflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tuie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Underflow interrupt enable."]
        #[inline(always)]
        pub const fn set_tuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive interrupt enable."]
        #[inline(always)]
        pub const fn set_rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rbuie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable interrupt. enable."]
        #[inline(always)]
        pub const fn set_rbuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rpsie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped interrupt. enable."]
        #[inline(always)]
        pub const fn set_rpsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "receive watchdog timeout interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rwtie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "receive watchdog timeout interrupt. enable."]
        #[inline(always)]
        pub const fn set_rwtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn etie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit interrupt. enable."]
        #[inline(always)]
        pub const fn set_etie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Early send interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn phylink_ier(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Early send interrupt enable bit."]
        #[inline(always)]
        pub const fn set_phylink_ier(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Early receive interrupt. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive interrupt. enable."]
        #[inline(always)]
        pub const fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aise(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary. enable."]
        #[inline(always)]
        pub const fn set_aise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nise(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary. enable."]
        #[inline(always)]
        pub const fn set_nise(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Dmaier {
        #[inline(always)]
        fn default() -> Dmaier {
            Dmaier(0)
        }
    }
    impl core::fmt::Debug for Dmaier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaier")
                .field("tie", &self.tie())
                .field("tpsie", &self.tpsie())
                .field("tbuie", &self.tbuie())
                .field("tjtie", &self.tjtie())
                .field("roie", &self.roie())
                .field("tuie", &self.tuie())
                .field("rie", &self.rie())
                .field("rbuie", &self.rbuie())
                .field("rpsie", &self.rpsie())
                .field("rwtie", &self.rwtie())
                .field("etie", &self.etie())
                .field("phylink_ier", &self.phylink_ier())
                .field("ers", &self.ers())
                .field("aise", &self.aise())
                .field("nise", &self.nise())
                .finish()
        }
    }
    #[doc = "Ethernet DMA missed frame and buffer. overflow counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmamfbocr(pub u32);
    impl Dmamfbocr {
        #[doc = "Missed frames by the. controller."]
        #[must_use]
        #[inline(always)]
        pub const fn mfc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Missed frames by the. controller."]
        #[inline(always)]
        pub const fn set_mfc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Overflow bit for missed frame. counter."]
        #[must_use]
        #[inline(always)]
        pub const fn omfc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow bit for missed frame. counter."]
        #[inline(always)]
        pub const fn set_omfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Missed frames by the. application."]
        #[must_use]
        #[inline(always)]
        pub const fn mfa(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x07ff;
            val as u16
        }
        #[doc = "Missed frames by the. application."]
        #[inline(always)]
        pub const fn set_mfa(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 17usize)) | (((val as u32) & 0x07ff) << 17usize);
        }
        #[doc = "Overflow bit for FIFO overflow. counter."]
        #[must_use]
        #[inline(always)]
        pub const fn ofoc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow bit for FIFO overflow. counter."]
        #[inline(always)]
        pub const fn set_ofoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Dmamfbocr {
        #[inline(always)]
        fn default() -> Dmamfbocr {
            Dmamfbocr(0)
        }
    }
    impl core::fmt::Debug for Dmamfbocr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmamfbocr")
                .field("mfc", &self.mfc())
                .field("omfc", &self.omfc())
                .field("mfa", &self.mfa())
                .field("ofoc", &self.ofoc())
                .finish()
        }
    }
    #[doc = "Ethernet DMA operation mode. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaomr(pub u32);
    impl Dmaomr {
        #[doc = "SR."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SR."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FUGF."]
        #[must_use]
        #[inline(always)]
        pub const fn fugf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FUGF."]
        #[inline(always)]
        pub const fn set_fugf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FEF."]
        #[must_use]
        #[inline(always)]
        pub const fn fef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FEF."]
        #[inline(always)]
        pub const fn set_fef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ST."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ST."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "FTF."]
        #[must_use]
        #[inline(always)]
        pub const fn ftf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "FTF."]
        #[inline(always)]
        pub const fn set_ftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TSF."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TSF."]
        #[inline(always)]
        pub const fn set_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DTCEFD."]
        #[must_use]
        #[inline(always)]
        pub const fn dtcefd(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "DTCEFD."]
        #[inline(always)]
        pub const fn set_dtcefd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dmaomr {
        #[inline(always)]
        fn default() -> Dmaomr {
            Dmaomr(0)
        }
    }
    impl core::fmt::Debug for Dmaomr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaomr")
                .field("sr", &self.sr())
                .field("fugf", &self.fugf())
                .field("fef", &self.fef())
                .field("st", &self.st())
                .field("ftf", &self.ftf())
                .field("tsf", &self.tsf())
                .field("dtcefd", &self.dtcefd())
                .finish()
        }
    }
    #[doc = "Ethernet DMA receive descriptor list address. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmardlar(pub u32);
    impl Dmardlar {
        #[doc = "Start of receive list."]
        #[must_use]
        #[inline(always)]
        pub const fn rdlar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of receive list."]
        #[inline(always)]
        pub const fn set_rdlar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmardlar {
        #[inline(always)]
        fn default() -> Dmardlar {
            Dmardlar(0)
        }
    }
    impl core::fmt::Debug for Dmardlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmardlar")
                .field("rdlar", &self.rdlar())
                .finish()
        }
    }
    #[doc = "EHERNET DMA receive poll demand. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmarpdr(pub u32);
    impl Dmarpdr {
        #[doc = "Receive poll demand."]
        #[must_use]
        #[inline(always)]
        pub const fn rpdr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive poll demand."]
        #[inline(always)]
        pub const fn set_rpdr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmarpdr {
        #[inline(always)]
        fn default() -> Dmarpdr {
            Dmarpdr(0)
        }
    }
    impl core::fmt::Debug for Dmarpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmarpdr")
                .field("rpdr", &self.rpdr())
                .finish()
        }
    }
    #[doc = "Ethernet DMA status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasr(pub u32);
    impl Dmasr {
        #[doc = "Transmit status."]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit status."]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit process stopped. status."]
        #[must_use]
        #[inline(always)]
        pub const fn tpss(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit process stopped. status."]
        #[inline(always)]
        pub const fn set_tpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit buffer unavailable. status."]
        #[must_use]
        #[inline(always)]
        pub const fn tbus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit buffer unavailable. status."]
        #[inline(always)]
        pub const fn set_tbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmit jabber timeout. status."]
        #[must_use]
        #[inline(always)]
        pub const fn tjts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit jabber timeout. status."]
        #[inline(always)]
        pub const fn set_tjts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive overflow status."]
        #[must_use]
        #[inline(always)]
        pub const fn ros(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive overflow status."]
        #[inline(always)]
        pub const fn set_ros(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmit underflow status."]
        #[must_use]
        #[inline(always)]
        pub const fn tus(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit underflow status."]
        #[inline(always)]
        pub const fn set_tus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Receive status."]
        #[must_use]
        #[inline(always)]
        pub const fn rs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Receive status."]
        #[inline(always)]
        pub const fn set_rs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receive buffer unavailable. status."]
        #[must_use]
        #[inline(always)]
        pub const fn rbus(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receive buffer unavailable. status."]
        #[inline(always)]
        pub const fn set_rbus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Receive process stopped. status."]
        #[must_use]
        #[inline(always)]
        pub const fn rpss(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive process stopped. status."]
        #[inline(always)]
        pub const fn set_rpss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive watchdog timeout. status."]
        #[must_use]
        #[inline(always)]
        pub const fn pwts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive watchdog timeout. status."]
        #[inline(always)]
        pub const fn set_pwts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Early transmit status."]
        #[must_use]
        #[inline(always)]
        pub const fn ets(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early transmit status."]
        #[inline(always)]
        pub const fn set_ets(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "read interrupt source."]
        #[must_use]
        #[inline(always)]
        pub const fn phylink_sr(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "read interrupt source."]
        #[inline(always)]
        pub const fn set_phylink_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Early receive status."]
        #[must_use]
        #[inline(always)]
        pub const fn ers(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Early receive status."]
        #[inline(always)]
        pub const fn set_ers(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Abnormal interrupt summary."]
        #[must_use]
        #[inline(always)]
        pub const fn ais(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Abnormal interrupt summary."]
        #[inline(always)]
        pub const fn set_ais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Normal interrupt summary."]
        #[must_use]
        #[inline(always)]
        pub const fn nis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Normal interrupt summary."]
        #[inline(always)]
        pub const fn set_nis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive process state."]
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Receive process state."]
        #[inline(always)]
        pub const fn set_rps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Transmit process state."]
        #[must_use]
        #[inline(always)]
        pub const fn tps(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Transmit process state."]
        #[inline(always)]
        pub const fn set_tps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "Error bits status."]
        #[must_use]
        #[inline(always)]
        pub const fn ebs(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "Error bits status."]
        #[inline(always)]
        pub const fn set_ebs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "MMC status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status."]
        #[inline(always)]
        pub const fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Time stamp trigger status."]
        #[must_use]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger status."]
        #[inline(always)]
        pub const fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Dmasr {
        #[inline(always)]
        fn default() -> Dmasr {
            Dmasr(0)
        }
    }
    impl core::fmt::Debug for Dmasr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmasr")
                .field("ts", &self.ts())
                .field("tpss", &self.tpss())
                .field("tbus", &self.tbus())
                .field("tjts", &self.tjts())
                .field("ros", &self.ros())
                .field("tus", &self.tus())
                .field("rs", &self.rs())
                .field("rbus", &self.rbus())
                .field("rpss", &self.rpss())
                .field("pwts", &self.pwts())
                .field("ets", &self.ets())
                .field("phylink_sr", &self.phylink_sr())
                .field("ers", &self.ers())
                .field("ais", &self.ais())
                .field("nis", &self.nis())
                .field("rps", &self.rps())
                .field("tps", &self.tps())
                .field("ebs", &self.ebs())
                .field("mmcs", &self.mmcs())
                .field("tsts", &self.tsts())
                .finish()
        }
    }
    #[doc = "Ethernet DMA transmit descriptor list. address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatdlar(pub u32);
    impl Dmatdlar {
        #[doc = "Start of transmit list."]
        #[must_use]
        #[inline(always)]
        pub const fn tdlar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Start of transmit list."]
        #[inline(always)]
        pub const fn set_tdlar(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatdlar {
        #[inline(always)]
        fn default() -> Dmatdlar {
            Dmatdlar(0)
        }
    }
    impl core::fmt::Debug for Dmatdlar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatdlar")
                .field("tdlar", &self.tdlar())
                .finish()
        }
    }
    #[doc = "Ethernet DMA transmit poll demand. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmatpdr(pub u32);
    impl Dmatpdr {
        #[doc = "Transmit poll demand."]
        #[must_use]
        #[inline(always)]
        pub const fn tpdr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmit poll demand."]
        #[inline(always)]
        pub const fn set_tpdr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dmatpdr {
        #[inline(always)]
        fn default() -> Dmatpdr {
            Dmatpdr(0)
        }
    }
    impl core::fmt::Debug for Dmatpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmatpdr")
                .field("tpdr", &self.tpdr())
                .finish()
        }
    }
}
