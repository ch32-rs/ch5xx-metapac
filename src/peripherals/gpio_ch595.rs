#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General porpose intput output."]
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
    #[doc = "RW, GPIO PA interrupt enable."]
    #[inline(always)]
    pub const fn pa_int_en(self) -> crate::common::Reg<regs::PaIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RW, GPIO PB interrupt enable."]
    #[inline(always)]
    pub const fn pb_int_en(self) -> crate::common::Reg<regs::PbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "RW, GPIO PA interrupt mode: 0=level action, 1=edge action."]
    #[inline(always)]
    pub const fn pa_int_mode(self) -> crate::common::Reg<regs::PaIntMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read."]
    #[inline(always)]
    pub const fn pb_int_mode(self) -> crate::common::Reg<regs::PbIntMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x96usize) as _) }
    }
    #[doc = "RW, PA port interrupt edge type configuration register."]
    #[inline(always)]
    pub const fn pa_int_edge_type(
        self,
    ) -> crate::common::Reg<regs::PaIntEdgeType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "RW, PB port interrupt edge type configuration register."]
    #[inline(always)]
    pub const fn pb_int_edge_type(
        self,
    ) -> crate::common::Reg<regs::PbIntEdgeType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9ausize) as _) }
    }
    #[doc = "RW1, GPIO PA interrupt flag."]
    #[inline(always)]
    pub const fn pa_int_if(self) -> crate::common::Reg<regs::PaIntIf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "RW1, GPIO PB interrupt flag."]
    #[inline(always)]
    pub const fn pb_int_if(self) -> crate::common::Reg<regs::PbIntIf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "RW, GPIO PA I/O direction: 0=in, 1=out."]
    #[inline(always)]
    pub const fn pa_dir(self) -> crate::common::Reg<regs::PaDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RO, GPIO PA input."]
    #[inline(always)]
    pub const fn pa_pin(self) -> crate::common::Reg<regs::PaPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "RW, GPIO PA output."]
    #[inline(always)]
    pub const fn pa_out(self) -> crate::common::Reg<regs::PaOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "WZ, GPIO PA clear output: 0=keep, 1=clear."]
    #[inline(always)]
    pub const fn pa_clr(self) -> crate::common::Reg<regs::PaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "RW, GPIO PA pullup resistance enable."]
    #[inline(always)]
    pub const fn pa_pu(self) -> crate::common::Reg<regs::PaPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "RW, PA pulldown for input or PA driving capability for output."]
    #[inline(always)]
    pub const fn pa_pd_drv(self) -> crate::common::Reg<regs::PaPdDrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "WZ, PA port output set register."]
    #[inline(always)]
    pub const fn pa_set(self) -> crate::common::Reg<regs::PaSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "RW, GPIO PB I/O direction: 0=in, 1=out."]
    #[inline(always)]
    pub const fn pb_dir(self) -> crate::common::Reg<regs::PbDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "RO, GPIO PB input."]
    #[inline(always)]
    pub const fn pb_pin(self) -> crate::common::Reg<regs::PbPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "RW, GPIO PB output;RW, data for parallel slave read."]
    #[inline(always)]
    pub const fn pb_out(self) -> crate::common::Reg<regs::PbOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "WZ, GPIO PB clear output: 0=keep, 1=clear."]
    #[inline(always)]
    pub const fn pb_clr(self) -> crate::common::Reg<regs::PbClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "RW, GPIO PB pullup resistance enable."]
    #[inline(always)]
    pub const fn pb_pu(self) -> crate::common::Reg<regs::PbPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "RW, PB pulldown for input or PB driving capability for output."]
    #[inline(always)]
    pub const fn pb_pd_drv(self) -> crate::common::Reg<regs::PbPdDrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "WZ, PB port output set register."]
    #[inline(always)]
    pub const fn pb_set(self) -> crate::common::Reg<regs::PbSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
}
pub mod regs {
    #[doc = "WZ, GPIO PA clear output: 0=keep, 1=clear."]
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
    #[doc = "RW, GPIO PA I/O direction: 0=in, 1=out."]
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
    #[doc = "RW, PA port interrupt edge type configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntEdgeType(pub u16);
    impl PaIntEdgeType {
        #[doc = "PA port interrupt edge type configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_int_edge_type(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "PA port interrupt edge type configuration."]
        #[inline(always)]
        pub const fn set_pa_int_edge_type(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntEdgeType {
        #[inline(always)]
        fn default() -> PaIntEdgeType {
            PaIntEdgeType(0)
        }
    }
    impl core::fmt::Debug for PaIntEdgeType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaIntEdgeType")
                .field("pa_int_edge_type", &self.pa_int_edge_type())
                .finish()
        }
    }
    #[doc = "RW, GPIO PA interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntEn(pub u16);
    impl PaIntEn {
        #[doc = "GPIO PA interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_int_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt enable."]
        #[inline(always)]
        pub const fn set_pa_int_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntEn {
        #[inline(always)]
        fn default() -> PaIntEn {
            PaIntEn(0)
        }
    }
    impl core::fmt::Debug for PaIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaIntEn")
                .field("pa_int_en", &self.pa_int_en())
                .finish()
        }
    }
    #[doc = "RW1, GPIO PA interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntIf(pub u16);
    impl PaIntIf {
        #[doc = "GPIO PA interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_int_if(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt flag."]
        #[inline(always)]
        pub const fn set_pa_int_if(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntIf {
        #[inline(always)]
        fn default() -> PaIntIf {
            PaIntIf(0)
        }
    }
    impl core::fmt::Debug for PaIntIf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaIntIf")
                .field("pa_int_if", &self.pa_int_if())
                .finish()
        }
    }
    #[doc = "RW, GPIO PA interrupt mode: 0=level action, 1=edge action."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntMode(pub u16);
    impl PaIntMode {
        #[doc = "GPIO PA interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_int_mode(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt mode."]
        #[inline(always)]
        pub const fn set_pa_int_mode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntMode {
        #[inline(always)]
        fn default() -> PaIntMode {
            PaIntMode(0)
        }
    }
    impl core::fmt::Debug for PaIntMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaIntMode")
                .field("pa_int_mode", &self.pa_int_mode())
                .finish()
        }
    }
    #[doc = "RW, GPIO PA output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaOut(pub u32);
    impl PaOut {
        #[doc = "GPIO PA output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
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
    #[doc = "RW, PA pulldown for input or PA driving capability for output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPdDrv(pub u32);
    impl PaPdDrv {
        #[doc = "GPIO PA pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_pd_drv(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[inline(always)]
        pub const fn set_pa_pd_drv(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaPdDrv {
        #[inline(always)]
        fn default() -> PaPdDrv {
            PaPdDrv(0)
        }
    }
    impl core::fmt::Debug for PaPdDrv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaPdDrv")
                .field("pa_pd_drv[0]", &self.pa_pd_drv(0usize))
                .field("pa_pd_drv[1]", &self.pa_pd_drv(1usize))
                .field("pa_pd_drv[2]", &self.pa_pd_drv(2usize))
                .field("pa_pd_drv[3]", &self.pa_pd_drv(3usize))
                .field("pa_pd_drv[4]", &self.pa_pd_drv(4usize))
                .field("pa_pd_drv[5]", &self.pa_pd_drv(5usize))
                .field("pa_pd_drv[6]", &self.pa_pd_drv(6usize))
                .field("pa_pd_drv[7]", &self.pa_pd_drv(7usize))
                .field("pa_pd_drv[8]", &self.pa_pd_drv(8usize))
                .field("pa_pd_drv[9]", &self.pa_pd_drv(9usize))
                .field("pa_pd_drv[10]", &self.pa_pd_drv(10usize))
                .field("pa_pd_drv[11]", &self.pa_pd_drv(11usize))
                .field("pa_pd_drv[12]", &self.pa_pd_drv(12usize))
                .field("pa_pd_drv[13]", &self.pa_pd_drv(13usize))
                .field("pa_pd_drv[14]", &self.pa_pd_drv(14usize))
                .field("pa_pd_drv[15]", &self.pa_pd_drv(15usize))
                .field("pa_pd_drv[16]", &self.pa_pd_drv(16usize))
                .field("pa_pd_drv[17]", &self.pa_pd_drv(17usize))
                .field("pa_pd_drv[18]", &self.pa_pd_drv(18usize))
                .field("pa_pd_drv[19]", &self.pa_pd_drv(19usize))
                .field("pa_pd_drv[20]", &self.pa_pd_drv(20usize))
                .field("pa_pd_drv[21]", &self.pa_pd_drv(21usize))
                .field("pa_pd_drv[22]", &self.pa_pd_drv(22usize))
                .field("pa_pd_drv[23]", &self.pa_pd_drv(23usize))
                .field("pa_pd_drv[24]", &self.pa_pd_drv(24usize))
                .field("pa_pd_drv[25]", &self.pa_pd_drv(25usize))
                .field("pa_pd_drv[26]", &self.pa_pd_drv(26usize))
                .field("pa_pd_drv[27]", &self.pa_pd_drv(27usize))
                .field("pa_pd_drv[28]", &self.pa_pd_drv(28usize))
                .field("pa_pd_drv[29]", &self.pa_pd_drv(29usize))
                .field("pa_pd_drv[30]", &self.pa_pd_drv(30usize))
                .field("pa_pd_drv[31]", &self.pa_pd_drv(31usize))
                .finish()
        }
    }
    #[doc = "RO, GPIO PA input."]
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
    #[doc = "RW, GPIO PA pullup resistance enable."]
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
    #[doc = "WZ, PA port output set register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaSet(pub u32);
    impl PaSet {
        #[doc = "GPIO PA set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pa_set(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pa_set(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PaSet {
        #[inline(always)]
        fn default() -> PaSet {
            PaSet(0)
        }
    }
    impl core::fmt::Debug for PaSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PaSet")
                .field("pa_set[0]", &self.pa_set(0usize))
                .field("pa_set[1]", &self.pa_set(1usize))
                .field("pa_set[2]", &self.pa_set(2usize))
                .field("pa_set[3]", &self.pa_set(3usize))
                .field("pa_set[4]", &self.pa_set(4usize))
                .field("pa_set[5]", &self.pa_set(5usize))
                .field("pa_set[6]", &self.pa_set(6usize))
                .field("pa_set[7]", &self.pa_set(7usize))
                .field("pa_set[8]", &self.pa_set(8usize))
                .field("pa_set[9]", &self.pa_set(9usize))
                .field("pa_set[10]", &self.pa_set(10usize))
                .field("pa_set[11]", &self.pa_set(11usize))
                .field("pa_set[12]", &self.pa_set(12usize))
                .field("pa_set[13]", &self.pa_set(13usize))
                .field("pa_set[14]", &self.pa_set(14usize))
                .field("pa_set[15]", &self.pa_set(15usize))
                .field("pa_set[16]", &self.pa_set(16usize))
                .field("pa_set[17]", &self.pa_set(17usize))
                .field("pa_set[18]", &self.pa_set(18usize))
                .field("pa_set[19]", &self.pa_set(19usize))
                .field("pa_set[20]", &self.pa_set(20usize))
                .field("pa_set[21]", &self.pa_set(21usize))
                .field("pa_set[22]", &self.pa_set(22usize))
                .field("pa_set[23]", &self.pa_set(23usize))
                .field("pa_set[24]", &self.pa_set(24usize))
                .field("pa_set[25]", &self.pa_set(25usize))
                .field("pa_set[26]", &self.pa_set(26usize))
                .field("pa_set[27]", &self.pa_set(27usize))
                .field("pa_set[28]", &self.pa_set(28usize))
                .field("pa_set[29]", &self.pa_set(29usize))
                .field("pa_set[30]", &self.pa_set(30usize))
                .field("pa_set[31]", &self.pa_set(31usize))
                .finish()
        }
    }
    #[doc = "WZ, GPIO PB clear output: 0=keep, 1=clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbClr(pub u32);
    impl PbClr {
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_clr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pb_clr(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
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
                .field("pb_clr[23]", &self.pb_clr(23usize))
                .field("pb_clr[24]", &self.pb_clr(24usize))
                .field("pb_clr[25]", &self.pb_clr(25usize))
                .field("pb_clr[26]", &self.pb_clr(26usize))
                .field("pb_clr[27]", &self.pb_clr(27usize))
                .field("pb_clr[28]", &self.pb_clr(28usize))
                .field("pb_clr[29]", &self.pb_clr(29usize))
                .field("pb_clr[30]", &self.pb_clr(30usize))
                .field("pb_clr[31]", &self.pb_clr(31usize))
                .finish()
        }
    }
    #[doc = "RW, GPIO PB I/O direction: 0=in, 1=out."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbDir(pub u32);
    impl PbDir {
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn set_pb_dir(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("pb_dir[23]", &self.pb_dir(23usize))
                .field("pb_dir[24]", &self.pb_dir(24usize))
                .field("pb_dir[25]", &self.pb_dir(25usize))
                .field("pb_dir[26]", &self.pb_dir(26usize))
                .field("pb_dir[27]", &self.pb_dir(27usize))
                .field("pb_dir[28]", &self.pb_dir(28usize))
                .field("pb_dir[29]", &self.pb_dir(29usize))
                .field("pb_dir[30]", &self.pb_dir(30usize))
                .field("pb_dir[31]", &self.pb_dir(31usize))
                .finish()
        }
    }
    #[doc = "RW, PB port interrupt edge type configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntEdgeType(pub u16);
    impl PbIntEdgeType {
        #[doc = "PB port interrupt edge type configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_int_edge_type(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "PB port interrupt edge type configuration."]
        #[inline(always)]
        pub const fn set_pb_int_edge_type(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntEdgeType {
        #[inline(always)]
        fn default() -> PbIntEdgeType {
            PbIntEdgeType(0)
        }
    }
    impl core::fmt::Debug for PbIntEdgeType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbIntEdgeType")
                .field("pb_int_edge_type", &self.pb_int_edge_type())
                .finish()
        }
    }
    #[doc = "RW, GPIO PB interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntEn(pub u16);
    impl PbIntEn {
        #[doc = "GPIO PB interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_int_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt enable."]
        #[inline(always)]
        pub const fn set_pb_int_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntEn {
        #[inline(always)]
        fn default() -> PbIntEn {
            PbIntEn(0)
        }
    }
    impl core::fmt::Debug for PbIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbIntEn")
                .field("pb_int_en", &self.pb_int_en())
                .finish()
        }
    }
    #[doc = "RW1, GPIO PB interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntIf(pub u16);
    impl PbIntIf {
        #[doc = "GPIO PB interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_int_if(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt flag."]
        #[inline(always)]
        pub const fn set_pb_int_if(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntIf {
        #[inline(always)]
        fn default() -> PbIntIf {
            PbIntIf(0)
        }
    }
    impl core::fmt::Debug for PbIntIf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbIntIf")
                .field("pb_int_if", &self.pb_int_if())
                .finish()
        }
    }
    #[doc = "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntMode(pub u16);
    impl PbIntMode {
        #[doc = "GPIO PB interrupt mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_int_mode(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt mode."]
        #[inline(always)]
        pub const fn set_pb_int_mode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntMode {
        #[inline(always)]
        fn default() -> PbIntMode {
            PbIntMode(0)
        }
    }
    impl core::fmt::Debug for PbIntMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbIntMode")
                .field("pb_int_mode", &self.pb_int_mode())
                .finish()
        }
    }
    #[doc = "RW, GPIO PB output;RW, data for parallel slave read."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbOut(pub u32);
    impl PbOut {
        #[doc = "GPIO PB output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[inline(always)]
        pub const fn set_pb_out(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("pb_out[23]", &self.pb_out(23usize))
                .field("pb_out[24]", &self.pb_out(24usize))
                .field("pb_out[25]", &self.pb_out(25usize))
                .field("pb_out[26]", &self.pb_out(26usize))
                .field("pb_out[27]", &self.pb_out(27usize))
                .field("pb_out[28]", &self.pb_out(28usize))
                .field("pb_out[29]", &self.pb_out(29usize))
                .field("pb_out[30]", &self.pb_out(30usize))
                .field("pb_out[31]", &self.pb_out(31usize))
                .finish()
        }
    }
    #[doc = "RW, PB pulldown for input or PB driving capability for output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPdDrv(pub u32);
    impl PbPdDrv {
        #[doc = "GPIO PB pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pd_drv(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[inline(always)]
        pub const fn set_pb_pd_drv(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbPdDrv {
        #[inline(always)]
        fn default() -> PbPdDrv {
            PbPdDrv(0)
        }
    }
    impl core::fmt::Debug for PbPdDrv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbPdDrv")
                .field("pb_pd_drv[0]", &self.pb_pd_drv(0usize))
                .field("pb_pd_drv[1]", &self.pb_pd_drv(1usize))
                .field("pb_pd_drv[2]", &self.pb_pd_drv(2usize))
                .field("pb_pd_drv[3]", &self.pb_pd_drv(3usize))
                .field("pb_pd_drv[4]", &self.pb_pd_drv(4usize))
                .field("pb_pd_drv[5]", &self.pb_pd_drv(5usize))
                .field("pb_pd_drv[6]", &self.pb_pd_drv(6usize))
                .field("pb_pd_drv[7]", &self.pb_pd_drv(7usize))
                .field("pb_pd_drv[8]", &self.pb_pd_drv(8usize))
                .field("pb_pd_drv[9]", &self.pb_pd_drv(9usize))
                .field("pb_pd_drv[10]", &self.pb_pd_drv(10usize))
                .field("pb_pd_drv[11]", &self.pb_pd_drv(11usize))
                .field("pb_pd_drv[12]", &self.pb_pd_drv(12usize))
                .field("pb_pd_drv[13]", &self.pb_pd_drv(13usize))
                .field("pb_pd_drv[14]", &self.pb_pd_drv(14usize))
                .field("pb_pd_drv[15]", &self.pb_pd_drv(15usize))
                .field("pb_pd_drv[16]", &self.pb_pd_drv(16usize))
                .field("pb_pd_drv[17]", &self.pb_pd_drv(17usize))
                .field("pb_pd_drv[18]", &self.pb_pd_drv(18usize))
                .field("pb_pd_drv[19]", &self.pb_pd_drv(19usize))
                .field("pb_pd_drv[20]", &self.pb_pd_drv(20usize))
                .field("pb_pd_drv[21]", &self.pb_pd_drv(21usize))
                .field("pb_pd_drv[22]", &self.pb_pd_drv(22usize))
                .field("pb_pd_drv[23]", &self.pb_pd_drv(23usize))
                .field("pb_pd_drv[24]", &self.pb_pd_drv(24usize))
                .field("pb_pd_drv[25]", &self.pb_pd_drv(25usize))
                .field("pb_pd_drv[26]", &self.pb_pd_drv(26usize))
                .field("pb_pd_drv[27]", &self.pb_pd_drv(27usize))
                .field("pb_pd_drv[28]", &self.pb_pd_drv(28usize))
                .field("pb_pd_drv[29]", &self.pb_pd_drv(29usize))
                .field("pb_pd_drv[30]", &self.pb_pd_drv(30usize))
                .field("pb_pd_drv[31]", &self.pb_pd_drv(31usize))
                .finish()
        }
    }
    #[doc = "RO, GPIO PB input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPin(pub u32);
    impl PbPin {
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn set_pb_pin(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("pb_pin[23]", &self.pb_pin(23usize))
                .field("pb_pin[24]", &self.pb_pin(24usize))
                .field("pb_pin[25]", &self.pb_pin(25usize))
                .field("pb_pin[26]", &self.pb_pin(26usize))
                .field("pb_pin[27]", &self.pb_pin(27usize))
                .field("pb_pin[28]", &self.pb_pin(28usize))
                .field("pb_pin[29]", &self.pb_pin(29usize))
                .field("pb_pin[30]", &self.pb_pin(30usize))
                .field("pb_pin[31]", &self.pb_pin(31usize))
                .finish()
        }
    }
    #[doc = "RW, GPIO PB pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPu(pub u32);
    impl PbPu {
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn set_pb_pu(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("pb_pu[23]", &self.pb_pu(23usize))
                .field("pb_pu[24]", &self.pb_pu(24usize))
                .field("pb_pu[25]", &self.pb_pu(25usize))
                .field("pb_pu[26]", &self.pb_pu(26usize))
                .field("pb_pu[27]", &self.pb_pu(27usize))
                .field("pb_pu[28]", &self.pb_pu(28usize))
                .field("pb_pu[29]", &self.pb_pu(29usize))
                .field("pb_pu[30]", &self.pb_pu(30usize))
                .field("pb_pu[31]", &self.pb_pu(31usize))
                .finish()
        }
    }
    #[doc = "WZ, PB port output set register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbSet(pub u32);
    impl PbSet {
        #[doc = "GPIO PB set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn pb_set(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn set_pb_set(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for PbSet {
        #[inline(always)]
        fn default() -> PbSet {
            PbSet(0)
        }
    }
    impl core::fmt::Debug for PbSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PbSet")
                .field("pb_set[0]", &self.pb_set(0usize))
                .field("pb_set[1]", &self.pb_set(1usize))
                .field("pb_set[2]", &self.pb_set(2usize))
                .field("pb_set[3]", &self.pb_set(3usize))
                .field("pb_set[4]", &self.pb_set(4usize))
                .field("pb_set[5]", &self.pb_set(5usize))
                .field("pb_set[6]", &self.pb_set(6usize))
                .field("pb_set[7]", &self.pb_set(7usize))
                .field("pb_set[8]", &self.pb_set(8usize))
                .field("pb_set[9]", &self.pb_set(9usize))
                .field("pb_set[10]", &self.pb_set(10usize))
                .field("pb_set[11]", &self.pb_set(11usize))
                .field("pb_set[12]", &self.pb_set(12usize))
                .field("pb_set[13]", &self.pb_set(13usize))
                .field("pb_set[14]", &self.pb_set(14usize))
                .field("pb_set[15]", &self.pb_set(15usize))
                .field("pb_set[16]", &self.pb_set(16usize))
                .field("pb_set[17]", &self.pb_set(17usize))
                .field("pb_set[18]", &self.pb_set(18usize))
                .field("pb_set[19]", &self.pb_set(19usize))
                .field("pb_set[20]", &self.pb_set(20usize))
                .field("pb_set[21]", &self.pb_set(21usize))
                .field("pb_set[22]", &self.pb_set(22usize))
                .field("pb_set[23]", &self.pb_set(23usize))
                .field("pb_set[24]", &self.pb_set(24usize))
                .field("pb_set[25]", &self.pb_set(25usize))
                .field("pb_set[26]", &self.pb_set(26usize))
                .field("pb_set[27]", &self.pb_set(27usize))
                .field("pb_set[28]", &self.pb_set(28usize))
                .field("pb_set[29]", &self.pb_set(29usize))
                .field("pb_set[30]", &self.pb_set(30usize))
                .field("pb_set[31]", &self.pb_set(31usize))
                .finish()
        }
    }
}
