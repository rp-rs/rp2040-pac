#[doc = "Register `DONE` reader"]
pub struct R(crate::R<DONE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DONE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DONE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DONE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `proc1` reader - "]
pub struct PROC1_R(crate::FieldReader<bool, bool>);
impl PROC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `proc0` reader - "]
pub struct PROC0_R(crate::FieldReader<bool, bool>);
impl PROC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sio` reader - "]
pub struct SIO_R(crate::FieldReader<bool, bool>);
impl SIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vreg_and_chip_reset` reader - "]
pub struct VREG_AND_CHIP_RESET_R(crate::FieldReader<bool, bool>);
impl VREG_AND_CHIP_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREG_AND_CHIP_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG_AND_CHIP_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xip` reader - "]
pub struct XIP_R(crate::FieldReader<bool, bool>);
impl XIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram5` reader - "]
pub struct SRAM5_R(crate::FieldReader<bool, bool>);
impl SRAM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram4` reader - "]
pub struct SRAM4_R(crate::FieldReader<bool, bool>);
impl SRAM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram3` reader - "]
pub struct SRAM3_R(crate::FieldReader<bool, bool>);
impl SRAM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram2` reader - "]
pub struct SRAM2_R(crate::FieldReader<bool, bool>);
impl SRAM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram1` reader - "]
pub struct SRAM1_R(crate::FieldReader<bool, bool>);
impl SRAM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram0` reader - "]
pub struct SRAM0_R(crate::FieldReader<bool, bool>);
impl SRAM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rom` reader - "]
pub struct ROM_R(crate::FieldReader<bool, bool>);
impl ROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `busfabric` reader - "]
pub struct BUSFABRIC_R(crate::FieldReader<bool, bool>);
impl BUSFABRIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSFABRIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSFABRIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `resets` reader - "]
pub struct RESETS_R(crate::FieldReader<bool, bool>);
impl RESETS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESETS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESETS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clocks` reader - "]
pub struct CLOCKS_R(crate::FieldReader<bool, bool>);
impl CLOCKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLOCKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xosc` reader - "]
pub struct XOSC_R(crate::FieldReader<bool, bool>);
impl XOSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosc` reader - "]
pub struct ROSC_R(crate::FieldReader<bool, bool>);
impl ROSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&self) -> VREG_AND_CHIP_RESET_R {
        VREG_AND_CHIP_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&self) -> BUSFABRIC_R {
        BUSFABRIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&self) -> RESETS_R {
        RESETS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&self) -> CLOCKS_R {
        CLOCKS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&self) -> ROSC_R {
        ROSC_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Indicates the peripheral's registers are ready to access.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [done](index.html) module"]
pub struct DONE_SPEC;
impl crate::RegisterSpec for DONE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [done::R](R) reader structure"]
impl crate::Readable for DONE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DONE to value 0"]
impl crate::Resettable for DONE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
