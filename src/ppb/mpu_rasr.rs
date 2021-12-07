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
#[doc = "Field `ATTRS` reader - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
pub struct ATTRS_R(crate::FieldReader<u16, u16>);
impl ATTRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ATTRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTRS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTRS` writer - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
pub struct ATTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `SRD` reader - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
pub struct SRD_R(crate::FieldReader<u8, u8>);
impl SRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRD` writer - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
pub struct SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SIZE` reader - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Enables the region."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enables the region."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
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
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control.  
 28 = XN: Instruction access disable bit:  
 0 = Instruction fetches enabled.  
 1 = Instruction fetches disabled.  
 26:24 = AP: Access permission field  
 18 = S: Shareable bit  
 17 = C: Cacheable bit  
 16 = B: Bufferable bit"]
    #[inline(always)]
    pub fn attrs(&mut self) -> ATTRS_W {
        ATTRS_W { w: self }
    }
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    pub fn srd(&mut self) -> SRD_W {
        SRD_W { w: self }
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
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
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MPU_RASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
