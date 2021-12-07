#[doc = "Register `PLATFORM` reader"]
pub struct R(crate::R<PLATFORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLATFORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLATFORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLATFORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ASIC` reader - "]
pub struct ASIC_R(crate::FieldReader<bool, bool>);
impl ASIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPGA` reader - "]
pub struct FPGA_R(crate::FieldReader<bool, bool>);
impl FPGA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FPGA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPGA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn asic(&self) -> ASIC_R {
        ASIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fpga(&self) -> FPGA_R {
        FPGA_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Platform register. Allows software to know what environment it is running in.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [platform](index.html) module"]
pub struct PLATFORM_SPEC;
impl crate::RegisterSpec for PLATFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [platform::R](R) reader structure"]
impl crate::Readable for PLATFORM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLATFORM to value 0"]
impl crate::Resettable for PLATFORM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
