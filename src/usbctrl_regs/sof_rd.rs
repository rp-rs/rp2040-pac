#[doc = "Register `SOF_RD` reader"]
pub struct R(crate::R<SOF_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOF_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOF_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOF_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - "]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sof_rd](index.html) module"]
pub struct SOF_RD_SPEC;
impl crate::RegisterSpec for SOF_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sof_rd::R](R) reader structure"]
impl crate::Readable for SOF_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SOF_RD to value 0"]
impl crate::Resettable for SOF_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
