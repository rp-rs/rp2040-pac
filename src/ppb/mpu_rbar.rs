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
#[doc = "Field `REGION` reader - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
pub type REGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGION` writer - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
pub type REGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RBAR_SPEC, u8, u8, 4, O>;
#[doc = "Field `VALID` reader - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.  
 Write:  
 0 = MPU_RNR not changed, and the processor:  
 Updates the base address for the region specified in the MPU_RNR.  
 Ignores the value of the REGION field.  
 1 = The processor:  
 Updates the value of the MPU_RNR to the value of the REGION field.  
 Updates the base address for the region specified in the REGION field.  
 Always reads as zero."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region.  
 Write:  
 0 = MPU_RNR not changed, and the processor:  
 Updates the base address for the region specified in the MPU_RNR.  
 Ignores the value of the REGION field.  
 1 = The processor:  
 Updates the value of the MPU_RNR to the value of the REGION field.  
 Updates the base address for the region specified in the REGION field.  
 Always reads as zero."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RBAR_SPEC, bool, O>;
#[doc = "Field `ADDR` reader - Base address of the region."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Base address of the region."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RBAR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
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
        VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<0> {
        REGION_W::new(self)
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
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<4> {
        VALID_W::new(self)
    }
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<8> {
        ADDR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RBAR to value 0"]
impl crate::Resettable for MPU_RBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
