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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RW, GPIO PB interrupt enable."]
    #[inline(always)]
    pub const fn pb_int_en(self) -> crate::common::Reg<regs::PbIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x92usize) as _) }
    }
    #[doc = "RW, GPIO PA interrupt mode: 0=level action, 1=edge action."]
    #[inline(always)]
    pub const fn pa_int_mode(self) -> crate::common::Reg<regs::PaIntMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read."]
    #[inline(always)]
    pub const fn pb_int_mode(self) -> crate::common::Reg<regs::PbIntMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x96usize) as _) }
    }
    #[doc = "RW, GPIO PA interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
    #[inline(always)]
    pub const fn pa_int_edge_type(
        self,
    ) -> crate::common::Reg<regs::PaIntEdgeType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "RW, GPIO PB interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
    #[inline(always)]
    pub const fn pb_int_edge_type(
        self,
    ) -> crate::common::Reg<regs::PbIntEdgeType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9ausize) as _) }
    }
    #[doc = "RW1, GPIO PA interrupt flag."]
    #[inline(always)]
    pub const fn pa_int_if(self) -> crate::common::Reg<regs::PaIntIf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "RW1, GPIO PB interrupt flag."]
    #[inline(always)]
    pub const fn pb_int_if(self) -> crate::common::Reg<regs::PbIntIf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9eusize) as _) }
    }
    #[doc = "RW, GPIO PA I/O direction: 0=in, 1=out."]
    #[inline(always)]
    pub const fn pa_dir(self) -> crate::common::Reg<regs::PaDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "RO, GPIO PA input."]
    #[inline(always)]
    pub const fn pa_pin(self) -> crate::common::Reg<regs::PaPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RW, GPIO PA output."]
    #[inline(always)]
    pub const fn pa_out(self) -> crate::common::Reg<regs::PaOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "WZ, GPIO PA clear output: 0=keep, 1=clear."]
    #[inline(always)]
    pub const fn pa_clr(self) -> crate::common::Reg<regs::PaClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "RW, GPIO PA pullup resistance enable."]
    #[inline(always)]
    pub const fn pa_pu(self) -> crate::common::Reg<regs::PaPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RW, PA pulldown for input or PA driving capability for output."]
    #[inline(always)]
    pub const fn pa_pd_drv(self) -> crate::common::Reg<regs::PaPdDrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "WZ, GPIO PA set output: 0=keep, 1=set high."]
    #[inline(always)]
    pub const fn pa_set(self) -> crate::common::Reg<regs::PaSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "RW, GPIO PB I/O direction: 0=in, 1=out."]
    #[inline(always)]
    pub const fn pb_dir(self) -> crate::common::Reg<regs::PbDir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "RO, GPIO PB input."]
    #[inline(always)]
    pub const fn pb_pin(self) -> crate::common::Reg<regs::PbPin, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "RW, GPIO PB output;RW, data for parallel slave read."]
    #[inline(always)]
    pub const fn pb_out(self) -> crate::common::Reg<regs::PbOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "WZ, GPIO PB clear output: 0=keep, 1=clear."]
    #[inline(always)]
    pub const fn pb_clr(self) -> crate::common::Reg<regs::PbClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "RW, GPIO PB pullup resistance enable."]
    #[inline(always)]
    pub const fn pb_pu(self) -> crate::common::Reg<regs::PbPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "RW, PB pulldown for input or PB driving capability for output."]
    #[inline(always)]
    pub const fn pb_pd_drv(self) -> crate::common::Reg<regs::PbPdDrv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "WZ, GPIO PB set output: 0=keep, 1=set high."]
    #[inline(always)]
    pub const fn pb_set(self) -> crate::common::Reg<regs::PbSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
}
pub mod regs {
    #[doc = "WZ, GPIO PA clear output: 0=keep, 1=clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaClr(pub u32);
    impl PaClr {
        #[doc = "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn pa_clr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub fn set_pa_clr(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, GPIO PA I/O direction: 0=in, 1=out."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaDir(pub u32);
    impl PaDir {
        #[doc = "GPIO PA I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn pa_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub fn set_pa_dir(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, GPIO PA interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntEdgeType(pub u16);
    impl PaIntEdgeType {
        #[doc = "GPIO PA interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
        #[inline(always)]
        pub const fn pa_int_edge_type(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
        #[inline(always)]
        pub fn set_pa_int_edge_type(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntEdgeType {
        #[inline(always)]
        fn default() -> PaIntEdgeType {
            PaIntEdgeType(0)
        }
    }
    #[doc = "RW, GPIO PA interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntEn(pub u16);
    impl PaIntEn {
        #[doc = "GPIO PA interrupt enable."]
        #[inline(always)]
        pub const fn pa_int_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt enable."]
        #[inline(always)]
        pub fn set_pa_int_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntEn {
        #[inline(always)]
        fn default() -> PaIntEn {
            PaIntEn(0)
        }
    }
    #[doc = "RW1, GPIO PA interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntIf(pub u16);
    impl PaIntIf {
        #[doc = "GPIO PA interrupt flag."]
        #[inline(always)]
        pub const fn pa_int_if(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt flag."]
        #[inline(always)]
        pub fn set_pa_int_if(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntIf {
        #[inline(always)]
        fn default() -> PaIntIf {
            PaIntIf(0)
        }
    }
    #[doc = "RW, GPIO PA interrupt mode: 0=level action, 1=edge action."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaIntMode(pub u16);
    impl PaIntMode {
        #[doc = "GPIO PA interrupt mode."]
        #[inline(always)]
        pub const fn pa_int_mode(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PA interrupt mode."]
        #[inline(always)]
        pub fn set_pa_int_mode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PaIntMode {
        #[inline(always)]
        fn default() -> PaIntMode {
            PaIntMode(0)
        }
    }
    #[doc = "RW, GPIO PA output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaOut(pub u32);
    impl PaOut {
        #[doc = "GPIO PA output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[inline(always)]
        pub const fn pa_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[inline(always)]
        pub fn set_pa_out(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, PA pulldown for input or PA driving capability for output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPdDrv(pub u32);
    impl PaPdDrv {
        #[doc = "GPIO PA pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[inline(always)]
        pub const fn pa_pd_drv(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[inline(always)]
        pub fn set_pa_pd_drv(&mut self, n: usize, val: bool) {
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
    #[doc = "RO, GPIO PA input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPin(pub u32);
    impl PaPin {
        #[doc = "GPIO PA input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn pa_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA input level: 1=high, 0=low."]
        #[inline(always)]
        pub fn set_pa_pin(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, GPIO PA pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaPu(pub u32);
    impl PaPu {
        #[doc = "GPIO PA pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn pa_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub fn set_pa_pu(&mut self, n: usize, val: bool) {
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
    #[doc = "WZ, GPIO PA set output: 0=keep, 1=set high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PaSet(pub u32);
    impl PaSet {
        #[doc = "GPIO PA set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn pa_set(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PA set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[inline(always)]
        pub fn set_pa_set(&mut self, n: usize, val: bool) {
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
    #[doc = "WZ, GPIO PB clear output: 0=keep, 1=clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbClr(pub u32);
    impl PbClr {
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn pb_clr(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged."]
        #[inline(always)]
        pub fn set_pb_clr(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, GPIO PB I/O direction: 0=in, 1=out."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbDir(pub u32);
    impl PbDir {
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub const fn pb_dir(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB I/O direction: 0=input, 1=output."]
        #[inline(always)]
        pub fn set_pb_dir(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, GPIO PB interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntEdgeType(pub u16);
    impl PbIntEdgeType {
        #[doc = "GPIO PB interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
        #[inline(always)]
        pub const fn pb_int_edge_type(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt edge type: 0=polarity sensitive, 1=non-polarity sensitive."]
        #[inline(always)]
        pub fn set_pb_int_edge_type(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntEdgeType {
        #[inline(always)]
        fn default() -> PbIntEdgeType {
            PbIntEdgeType(0)
        }
    }
    #[doc = "RW, GPIO PB interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntEn(pub u16);
    impl PbIntEn {
        #[doc = "GPIO PB interrupt enable."]
        #[inline(always)]
        pub const fn pb_int_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt enable."]
        #[inline(always)]
        pub fn set_pb_int_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntEn {
        #[inline(always)]
        fn default() -> PbIntEn {
            PbIntEn(0)
        }
    }
    #[doc = "RW1, GPIO PB interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntIf(pub u16);
    impl PbIntIf {
        #[doc = "GPIO PB interrupt flag."]
        #[inline(always)]
        pub const fn pb_int_if(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt flag."]
        #[inline(always)]
        pub fn set_pb_int_if(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntIf {
        #[inline(always)]
        fn default() -> PbIntIf {
            PbIntIf(0)
        }
    }
    #[doc = "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbIntMode(pub u16);
    impl PbIntMode {
        #[doc = "GPIO PB interrupt mode."]
        #[inline(always)]
        pub const fn pb_int_mode(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "GPIO PB interrupt mode."]
        #[inline(always)]
        pub fn set_pb_int_mode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PbIntMode {
        #[inline(always)]
        fn default() -> PbIntMode {
            PbIntMode(0)
        }
    }
    #[doc = "RW, GPIO PB output;RW, data for parallel slave read."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbOut(pub u32);
    impl PbOut {
        #[doc = "GPIO PB output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[inline(always)]
        pub const fn pb_out(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge)."]
        #[inline(always)]
        pub fn set_pb_out(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, PB pulldown for input or PB driving capability for output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPdDrv(pub u32);
    impl PbPdDrv {
        #[doc = "GPIO PB pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[inline(always)]
        pub const fn pb_pd_drv(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA)."]
        #[inline(always)]
        pub fn set_pb_pd_drv(&mut self, n: usize, val: bool) {
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
    #[doc = "RO, GPIO PB input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPin(pub u32);
    impl PbPin {
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[inline(always)]
        pub const fn pb_pin(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB input level: 1=high, 0=low."]
        #[inline(always)]
        pub fn set_pb_pin(&mut self, n: usize, val: bool) {
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
    #[doc = "RW, GPIO PB pullup resistance enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbPu(pub u32);
    impl PbPu {
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub const fn pb_pu(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB pull-up resistor: 1=enabled, 0=disabled."]
        #[inline(always)]
        pub fn set_pb_pu(&mut self, n: usize, val: bool) {
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
    #[doc = "WZ, GPIO PB set output: 0=keep, 1=set high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PbSet(pub u32);
    impl PbSet {
        #[doc = "GPIO PB set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[inline(always)]
        pub const fn pb_set(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPIO PB set output: write 1 to drive the pin output high, write 0 leaves it unchanged."]
        #[inline(always)]
        pub fn set_pb_set(&mut self, n: usize, val: bool) {
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
}
