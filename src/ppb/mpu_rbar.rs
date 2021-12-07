#[doc = "Register `MPU_RBAR` reader"]
pub struct R(crate::R<MPU_RBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RBAR` writer"]
pub struct W(crate::W<MPU_RBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MPU_RBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Base address of the region."]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Base address of the region."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `VALID` reader - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.  
 Write:  
 0 = MPU_RNR not changed, and the processor:  
 Updates the base address for the region specified in the MPU_RNR.  
 Ignores the value of the REGION field.  
 1 = The processor:  
 Updates the value of the MPU_RNR to the value of the REGION field.  
 Updates the base address for the region specified in the REGION field.  
 Always reads as zero."]
pub struct VALID_R(crate::FieldReader<bool, bool>);
impl VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID` writer - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.  
 Write:  
 0 = MPU_RNR not changed, and the processor:  
 Updates the base address for the region specified in the MPU_RNR.  
 Ignores the value of the REGION field.  
 1 = The processor:  
 Updates the value of the MPU_RNR to the value of the REGION field.  
 Updates the base address for the region specified in the REGION field.  
 Always reads as zero."]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `REGION` reader - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
pub struct REGION_R(crate::FieldReader<u8, u8>);
impl REGION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REGION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION` writer - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
pub struct REGION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 4 - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.  
 Write:  
 0 = MPU_RNR not changed, and the processor:  
 Updates the base address for the region specified in the MPU_RNR.  
 Ignores the value of the REGION field.  
 1 = The processor:  
 Updates the value of the MPU_RNR to the value of the REGION field.  
 Updates the base address for the region specified in the REGION field.  
 Always reads as zero."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 4 - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.  
 Write:  
 0 = MPU_RNR not changed, and the processor:  
 Updates the base address for the region specified in the MPU_RNR.  
 Ignores the value of the REGION field.  
 1 = The processor:  
 Updates the value of the MPU_RNR to the value of the REGION field.  
 Updates the base address for the region specified in the REGION field.  
 Always reads as zero."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W {
        REGION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mpu_rbar](index.html) module"]
pub struct MPU_RBAR_SPEC;
impl crate::RegisterSpec for MPU_RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rbar::R](R) reader structure"]
impl crate::Readable for MPU_RBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rbar::W](W) writer structure"]
impl crate::Writable for MPU_RBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPU_RBAR to value 0"]
impl crate::Resettable for MPU_RBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
