#[doc = "Register `MPU_RASR` reader"]
pub struct R(crate::R<MPU_RASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RASR` writer"]
pub struct W(crate::W<MPU_RASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RASR_SPEC>;
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
impl From<crate::W<MPU_RASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enables the region."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enables the region."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SRD` reader - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
pub type SRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRD` writer - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
pub type SRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTRS` reader - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
pub type ATTRS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ATTRS` writer - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
pub type ATTRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
    #[inline(always)]
    pub fn attrs(&self) -> ATTRS_R {
        ATTRS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<1> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SRD_W<8> {
        SRD_W::new(self)
    }
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
    #[inline(always)]
    #[must_use]
    pub fn attrs(&mut self) -> ATTRS_W<16> {
        ATTRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mpu_rasr](index.html) module"]
pub struct MPU_RASR_SPEC;
impl crate::RegisterSpec for MPU_RASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rasr::R](R) reader structure"]
impl crate::Readable for MPU_RASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rasr::W](W) writer structure"]
impl crate::Writable for MPU_RASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MPU_RASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
