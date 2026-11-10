#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SLV register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slv {
    ptr: *mut u8,
}
unsafe impl Send for Slv {}
unsafe impl Sync for Slv {}
impl Slv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SLV configuration register."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SLV data output register."]
    #[inline(always)]
    pub const fn dout(self) -> crate::common::Reg<regs::Dout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "SLV status output register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x17usize) as _) }
    }
    #[doc = "SLV interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "SLV data input register."]
    #[inline(always)]
    pub const fn int_din(self) -> crate::common::Reg<regs::IntDin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x43usize) as _) }
    }
    #[doc = "SLV DMA enable register."]
    #[inline(always)]
    pub const fn dma_en(self) -> crate::common::Reg<regs::DmaEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "SLV DMA mode control register."]
    #[inline(always)]
    pub const fn dma_mode_ctrl(self) -> crate::common::Reg<regs::DmaModeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0181usize) as _) }
    }
    #[doc = "SLV DMA mode enable register."]
    #[inline(always)]
    pub const fn dma_mode_en(self) -> crate::common::Reg<regs::DmaModeEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0182usize) as _) }
    }
    #[doc = "SLV DMA interrupt flag register."]
    #[inline(always)]
    pub const fn dma_int_flag(self) -> crate::common::Reg<regs::DmaIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "DMA receive mode start address register."]
    #[inline(always)]
    pub const fn wr_dma_start_addr(
        self,
    ) -> crate::common::Reg<regs::WrDmaStartAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "DMA receive mode end address register."]
    #[inline(always)]
    pub const fn wr_dma_end_addr(
        self,
    ) -> crate::common::Reg<regs::WrDmaEndAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "DMA send mode start address register."]
    #[inline(always)]
    pub const fn rd_dma_start_addr(
        self,
    ) -> crate::common::Reg<regs::RdDmaStartAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "DMA send mode end address register."]
    #[inline(always)]
    pub const fn rd_dma_end_addr(
        self,
    ) -> crate::common::Reg<regs::RdDmaEndAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "DMA current address register."]
    #[inline(always)]
    pub const fn dma_end_now(self) -> crate::common::Reg<regs::DmaEndNow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "DMA mode command word0 register."]
    #[inline(always)]
    pub const fn dma_cmd0(self) -> crate::common::Reg<regs::DmaCmd0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "DMA mode command word1 register."]
    #[inline(always)]
    pub const fn dma_cmd1(self) -> crate::common::Reg<regs::DmaCmd1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a1usize) as _) }
    }
    #[doc = "DMA reset the command word register."]
    #[inline(always)]
    pub const fn reset_cmd(self) -> crate::common::Reg<regs::ResetCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "SLV busy flag register."]
    #[inline(always)]
    pub const fn busy(self) -> crate::common::Reg<regs::Busy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a5usize) as _) }
    }
    #[doc = "Non-DMA writes to the data register."]
    #[inline(always)]
    pub const fn other_data(self) -> crate::common::Reg<regs::OtherData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "DMA parses length register."]
    #[inline(always)]
    pub const fn dma_dec_len(self) -> crate::common::Reg<regs::DmaDecLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "DMA resolves the offset address register."]
    #[inline(always)]
    pub const fn dma_dec_offset(self) -> crate::common::Reg<regs::DmaDecOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b6usize) as _) }
    }
}
pub mod regs {
    #[doc = "SLV busy flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Busy(pub u8);
    impl Busy {
        #[doc = "SLV busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SLV busy flag."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Busy {
        #[inline(always)]
        fn default() -> Busy {
            Busy(0)
        }
    }
    impl core::fmt::Debug for Busy {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Busy").field("busy", &self.busy()).finish()
        }
    }
    #[doc = "SLV configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u8);
    impl Config {
        #[doc = "SLV enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SLV enable bit."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The command writes the interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cmd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The command writes the interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ie_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Data writes the interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_wr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data writes the interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ie_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Data read the interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_rd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data read the interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ie_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("enable", &self.enable())
                .field("ie_cmd", &self.ie_cmd())
                .field("ie_wr", &self.ie_wr())
                .field("ie_rd", &self.ie_rd())
                .finish()
        }
    }
    #[doc = "DMA mode command word0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCmd0(pub u8);
    impl DmaCmd0 {
        #[doc = "command word required to start DMA mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_cmd0_slv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "command word required to start DMA mode."]
        #[inline(always)]
        pub const fn set_dma_cmd0_slv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for DmaCmd0 {
        #[inline(always)]
        fn default() -> DmaCmd0 {
            DmaCmd0(0)
        }
    }
    impl core::fmt::Debug for DmaCmd0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCmd0")
                .field("dma_cmd0_slv", &self.dma_cmd0_slv())
                .finish()
        }
    }
    #[doc = "DMA mode command word1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCmd1(pub u8);
    impl DmaCmd1 {
        #[doc = "command word required to start DMA mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_cmd1_slv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "command word required to start DMA mode."]
        #[inline(always)]
        pub const fn set_dma_cmd1_slv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for DmaCmd1 {
        #[inline(always)]
        fn default() -> DmaCmd1 {
            DmaCmd1(0)
        }
    }
    impl core::fmt::Debug for DmaCmd1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCmd1")
                .field("dma_cmd1_slv", &self.dma_cmd1_slv())
                .finish()
        }
    }
    #[doc = "DMA parses length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaDecLen(pub u16);
    impl DmaDecLen {
        #[doc = "parse the resulting DMA length."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_dec_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "parse the resulting DMA length."]
        #[inline(always)]
        pub const fn set_dma_dec_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for DmaDecLen {
        #[inline(always)]
        fn default() -> DmaDecLen {
            DmaDecLen(0)
        }
    }
    impl core::fmt::Debug for DmaDecLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaDecLen")
                .field("dma_dec_len", &self.dma_dec_len())
                .finish()
        }
    }
    #[doc = "DMA resolves the offset address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaDecOffset(pub u16);
    impl DmaDecOffset {
        #[doc = "Resolved DMA address offset."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_dec_offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Resolved DMA address offset."]
        #[inline(always)]
        pub const fn set_dma_dec_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for DmaDecOffset {
        #[inline(always)]
        fn default() -> DmaDecOffset {
            DmaDecOffset(0)
        }
    }
    impl core::fmt::Debug for DmaDecOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaDecOffset")
                .field("dma_dec_offset", &self.dma_dec_offset())
                .finish()
        }
    }
    #[doc = "SLV DMA enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEn(pub u8);
    impl DmaEn {
        #[doc = "SLV DMA mode enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_en_slv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SLV DMA mode enable control bit."]
        #[inline(always)]
        pub const fn set_dma_en_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "SLV DMA error transmission interrupts enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_resp_ie_slv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SLV DMA error transmission interrupts enable control bit."]
        #[inline(always)]
        pub const fn set_dma_resp_ie_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "SLV DMA error transmission end enable control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end_ie_slv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SLV DMA error transmission end enable control bit."]
        #[inline(always)]
        pub const fn set_dma_end_ie_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "SLV DMA cyclic address mode control bits."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop_en_slv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SLV DMA cyclic address mode control bits."]
        #[inline(always)]
        pub const fn set_dma_loop_en_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "The FIFO full state interrupts the enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_fifo_ov_ie_slv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "The FIFO full state interrupts the enable bit."]
        #[inline(always)]
        pub const fn set_dma_fifo_ov_ie_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "when in compatibility mode, Transmission of non-DMA data. state interrupts the enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_other_ie_slv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "when in compatibility mode, Transmission of non-DMA data. state interrupts the enable bit."]
        #[inline(always)]
        pub const fn set_dma_other_ie_slv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "selection of transfer data length for DMA write operations."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_wr_8bit(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "selection of transfer data length for DMA write operations."]
        #[inline(always)]
        pub const fn set_dma_wr_8bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for DmaEn {
        #[inline(always)]
        fn default() -> DmaEn {
            DmaEn(0)
        }
    }
    impl core::fmt::Debug for DmaEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaEn")
                .field("dma_en_slv", &self.dma_en_slv())
                .field("dma_resp_ie_slv", &self.dma_resp_ie_slv())
                .field("dma_end_ie_slv", &self.dma_end_ie_slv())
                .field("dma_loop_en_slv", &self.dma_loop_en_slv())
                .field("dma_fifo_ov_ie_slv", &self.dma_fifo_ov_ie_slv())
                .field("dma_other_ie_slv", &self.dma_other_ie_slv())
                .field("dma_wr_8bit", &self.dma_wr_8bit())
                .finish()
        }
    }
    #[doc = "DMA current address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEndNow(pub u32);
    impl DmaEndNow {
        #[doc = "DMA current address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now_addr_slv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA current address."]
        #[inline(always)]
        pub const fn set_dma_now_addr_slv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for DmaEndNow {
        #[inline(always)]
        fn default() -> DmaEndNow {
            DmaEndNow(0)
        }
    }
    impl core::fmt::Debug for DmaEndNow {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaEndNow")
                .field("dma_now_addr_slv", &self.dma_now_addr_slv())
                .finish()
        }
    }
    #[doc = "SLV DMA interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaIntFlag(pub u8);
    impl DmaIntFlag {
        #[doc = "DMA receive mode transmission end flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_dma_end_if(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA receive mode transmission end flag bit."]
        #[inline(always)]
        pub const fn set_wr_dma_end_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DMA receive mode transmission error flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_dma_resp_if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA receive mode transmission error flag bit."]
        #[inline(always)]
        pub const fn set_wr_dma_resp_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DMA occurrence mode transmission end flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_dma_end_if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA occurrence mode transmission end flag bit."]
        #[inline(always)]
        pub const fn set_rd_dma_end_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DMA occurrence mode transmission error flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_dma_resp_if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA occurrence mode transmission error flag bit."]
        #[inline(always)]
        pub const fn set_rd_dma_resp_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "input FIFO full flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_dma_ov_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "input FIFO full flag."]
        #[inline(always)]
        pub const fn set_wr_dma_ov_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "data are written to the flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_other_if(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "data are written to the flag bit."]
        #[inline(always)]
        pub const fn set_wr_other_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Whether DATA_IN_FIFO is empty."]
        #[must_use]
        #[inline(always)]
        pub const fn data_in_fifo_empty(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Whether DATA_IN_FIFO is empty."]
        #[inline(always)]
        pub const fn set_data_in_fifo_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for DmaIntFlag {
        #[inline(always)]
        fn default() -> DmaIntFlag {
            DmaIntFlag(0)
        }
    }
    impl core::fmt::Debug for DmaIntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaIntFlag")
                .field("wr_dma_end_if", &self.wr_dma_end_if())
                .field("wr_dma_resp_if", &self.wr_dma_resp_if())
                .field("rd_dma_end_if", &self.rd_dma_end_if())
                .field("rd_dma_resp_if", &self.rd_dma_resp_if())
                .field("wr_dma_ov_if", &self.wr_dma_ov_if())
                .field("wr_other_if", &self.wr_other_if())
                .field("data_in_fifo_empty", &self.data_in_fifo_empty())
                .finish()
        }
    }
    #[doc = "SLV DMA mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaModeCtrl(pub u8);
    impl DmaModeCtrl {
        #[doc = "DMA transfer command word 0 read and write type."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd0_wr_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer command word 0 read and write type."]
        #[inline(always)]
        pub const fn set_cmd0_wr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "DMA transfer command word 1 read and write type."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd1_wr_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer command word 1 read and write type."]
        #[inline(always)]
        pub const fn set_cmd1_wr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for DmaModeCtrl {
        #[inline(always)]
        fn default() -> DmaModeCtrl {
            DmaModeCtrl(0)
        }
    }
    impl core::fmt::Debug for DmaModeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaModeCtrl")
                .field("cmd0_wr_mode", &self.cmd0_wr_mode())
                .field("cmd1_wr_mode", &self.cmd1_wr_mode())
                .finish()
        }
    }
    #[doc = "SLV DMA mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaModeEn(pub u8);
    impl DmaModeEn {
        #[doc = "DMA transfer command word 0 control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd0_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer command word 0 control bit."]
        #[inline(always)]
        pub const fn set_cmd0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DMA transfer command word 1 control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd1_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer command word 1 control bit."]
        #[inline(always)]
        pub const fn set_cmd1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DMA transfer command word mode1."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_cmd_mode1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer command word mode1."]
        #[inline(always)]
        pub const fn set_dma_cmd_mode1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for DmaModeEn {
        #[inline(always)]
        fn default() -> DmaModeEn {
            DmaModeEn(0)
        }
    }
    impl core::fmt::Debug for DmaModeEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaModeEn")
                .field("cmd0_en", &self.cmd0_en())
                .field("cmd1_en", &self.cmd1_en())
                .field("dma_cmd_mode1", &self.dma_cmd_mode1())
                .finish()
        }
    }
    #[doc = "SLV data output register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dout(pub u8);
    impl Dout {
        #[doc = "SLV data output."]
        #[must_use]
        #[inline(always)]
        pub const fn dout(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SLV data output."]
        #[inline(always)]
        pub const fn set_dout(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Dout {
        #[inline(always)]
        fn default() -> Dout {
            Dout(0)
        }
    }
    impl core::fmt::Debug for Dout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dout").field("dout", &self.dout()).finish()
        }
    }
    #[doc = "SLV data input register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntDin(pub u8);
    impl IntDin {
        #[doc = "SLV data input."]
        #[must_use]
        #[inline(always)]
        pub const fn int_slv_din(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SLV data input."]
        #[inline(always)]
        pub const fn set_int_slv_din(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for IntDin {
        #[inline(always)]
        fn default() -> IntDin {
            IntDin(0)
        }
    }
    impl core::fmt::Debug for IntDin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntDin")
                .field("int_slv_din", &self.int_slv_din())
                .finish()
        }
    }
    #[doc = "SLV interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "The state of the original SLV address line 1."]
        #[must_use]
        #[inline(always)]
        pub const fn if_slv_cmd1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "The state of the original SLV address line 1."]
        #[inline(always)]
        pub const fn set_if_slv_cmd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "The status of the SLV address line 1 after synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn if_slv_cmd1_sync(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the SLV address line 1 after synchronization."]
        #[inline(always)]
        pub const fn set_if_slv_cmd1_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "The original SLV command input flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_slv_cmd0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "The original SLV command input flag bit."]
        #[inline(always)]
        pub const fn set_if_slv_cmd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "The synchronized SLV command input flag bits."]
        #[must_use]
        #[inline(always)]
        pub const fn if_slv_cmd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "The synchronized SLV command input flag bits."]
        #[inline(always)]
        pub const fn set_if_slv_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SLV data input interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_slv_wr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SLV data input interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_slv_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SLV data output interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_slv_rd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SLV data output interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_slv_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("if_slv_cmd1", &self.if_slv_cmd1())
                .field("if_slv_cmd1_sync", &self.if_slv_cmd1_sync())
                .field("if_slv_cmd0", &self.if_slv_cmd0())
                .field("if_slv_cmd", &self.if_slv_cmd())
                .field("if_slv_wr", &self.if_slv_wr())
                .field("if_slv_rd", &self.if_slv_rd())
                .finish()
        }
    }
    #[doc = "Non-DMA writes to the data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtherData(pub u8);
    impl OtherData {
        #[doc = "Non-DMA writes to the data."]
        #[must_use]
        #[inline(always)]
        pub const fn other_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Non-DMA writes to the data."]
        #[inline(always)]
        pub const fn set_other_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for OtherData {
        #[inline(always)]
        fn default() -> OtherData {
            OtherData(0)
        }
    }
    impl core::fmt::Debug for OtherData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtherData")
                .field("other_data", &self.other_data())
                .finish()
        }
    }
    #[doc = "DMA send mode end address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdDmaEndAddr(pub u32);
    impl RdDmaEndAddr {
        #[doc = "DMA send mode end address."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_dma_start_addr_slv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA send mode end address."]
        #[inline(always)]
        pub const fn set_rd_dma_start_addr_slv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for RdDmaEndAddr {
        #[inline(always)]
        fn default() -> RdDmaEndAddr {
            RdDmaEndAddr(0)
        }
    }
    impl core::fmt::Debug for RdDmaEndAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RdDmaEndAddr")
                .field("rd_dma_start_addr_slv", &self.rd_dma_start_addr_slv())
                .finish()
        }
    }
    #[doc = "DMA send mode start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdDmaStartAddr(pub u32);
    impl RdDmaStartAddr {
        #[doc = "DMA send mode start address."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_dma_start_addr_slv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA send mode start address."]
        #[inline(always)]
        pub const fn set_rd_dma_start_addr_slv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for RdDmaStartAddr {
        #[inline(always)]
        fn default() -> RdDmaStartAddr {
            RdDmaStartAddr(0)
        }
    }
    impl core::fmt::Debug for RdDmaStartAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RdDmaStartAddr")
                .field("rd_dma_start_addr_slv", &self.rd_dma_start_addr_slv())
                .finish()
        }
    }
    #[doc = "DMA reset the command word register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetCmd(pub u8);
    impl ResetCmd {
        #[doc = "A global reset is generated upon receipt of this command."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_cmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "A global reset is generated upon receipt of this command."]
        #[inline(always)]
        pub const fn set_reset_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ResetCmd {
        #[inline(always)]
        fn default() -> ResetCmd {
            ResetCmd(0)
        }
    }
    impl core::fmt::Debug for ResetCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetCmd")
                .field("reset_cmd", &self.reset_cmd())
                .finish()
        }
    }
    #[doc = "SLV status output register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u8);
    impl Status {
        #[doc = "SLV status output."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SLV status output."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("status", &self.status())
                .finish()
        }
    }
    #[doc = "DMA receive mode end address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WrDmaEndAddr(pub u32);
    impl WrDmaEndAddr {
        #[doc = "DMA receive mode end address."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_dma_start_addr_slv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA receive mode end address."]
        #[inline(always)]
        pub const fn set_wr_dma_start_addr_slv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for WrDmaEndAddr {
        #[inline(always)]
        fn default() -> WrDmaEndAddr {
            WrDmaEndAddr(0)
        }
    }
    impl core::fmt::Debug for WrDmaEndAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WrDmaEndAddr")
                .field("wr_dma_start_addr_slv", &self.wr_dma_start_addr_slv())
                .finish()
        }
    }
    #[doc = "DMA receive mode start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WrDmaStartAddr(pub u32);
    impl WrDmaStartAddr {
        #[doc = "DMA receive mode start address."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_dma_start_addr_slv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA receive mode start address."]
        #[inline(always)]
        pub const fn set_wr_dma_start_addr_slv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for WrDmaStartAddr {
        #[inline(always)]
        fn default() -> WrDmaStartAddr {
            WrDmaStartAddr(0)
        }
    }
    impl core::fmt::Debug for WrDmaStartAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WrDmaStartAddr")
                .field("wr_dma_start_addr_slv", &self.wr_dma_start_addr_slv())
                .finish()
        }
    }
}
