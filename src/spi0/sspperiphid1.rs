#[doc = "Register `SSPPERIPHID1` reader"]
pub struct R(crate::R<SSPPERIPHID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPERIPHID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPERIPHID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPERIPHID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DESIGNER0` reader - These bits read back as 0x1"]
pub struct DESIGNER0_R(crate::FieldReader<u8, u8>);
impl DESIGNER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DESIGNER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESIGNER0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTNUMBER1` reader - These bits read back as 0x0"]
pub struct PARTNUMBER1_R(crate::FieldReader<u8, u8>);
impl PARTNUMBER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARTNUMBER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNUMBER1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> DESIGNER0_R {
        DESIGNER0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - These bits read back as 0x0"]
    #[inline(always)]
    pub fn partnumber1(&self) -> PARTNUMBER1_R {
        PARTNUMBER1_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspperiphid1](index.html) module"]
pub struct SSPPERIPHID1_SPEC;
impl crate::RegisterSpec for SSPPERIPHID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspperiphid1::R](R) reader structure"]
impl crate::Readable for SSPPERIPHID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPERIPHID1 to value 0x10"]
impl crate::Resettable for SSPPERIPHID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
