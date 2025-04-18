#[doc = "Register `MPU_RASR` reader"]
pub type R = crate::R<MPU_RASR_SPEC>;
#[doc = "Register `MPU_RASR` writer"]
pub type W = crate::W<MPU_RASR_SPEC>;
#[doc = "Field `ENABLE` reader - Enables the region."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enables the region."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
pub type SIZE_R = crate::FieldReader;
#[doc = "Field `SIZE` writer - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SRD` reader - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
pub type SRD_R = crate::FieldReader;
#[doc = "Field `SRD` writer - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
pub type SRD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTRS` reader - The MPU Region Attribute field. Use to define the region attribute control. 28 = XN: Instruction access disable bit: 0 = Instruction fetches enabled. 1 = Instruction fetches disabled. 26:24 = AP: Access permission field 18 = S: Shareable bit 17 = C: Cacheable bit 16 = B: Bufferable bit"]
pub type ATTRS_R = crate::FieldReader<u16>;
#[doc = "Field `ATTRS` writer - The MPU Region Attribute field. Use to define the region attribute control. 28 = XN: Instruction access disable bit: 0 = Instruction fetches enabled. 1 = Instruction fetches disabled. 26:24 = AP: Access permission field 18 = S: Shareable bit 17 = C: Cacheable bit 16 = B: Bufferable bit"]
pub type ATTRS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control. 28 = XN: Instruction access disable bit: 0 = Instruction fetches enabled. 1 = Instruction fetches disabled. 26:24 = AP: Access permission field 18 = S: Shareable bit 17 = C: Cacheable bit 16 = B: Bufferable bit"]
    #[inline(always)]
    pub fn attrs(&self) -> ATTRS_R {
        ATTRS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the region."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<MPU_RASR_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<MPU_RASR_SPEC> {
        SIZE_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SRD_W<MPU_RASR_SPEC> {
        SRD_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - The MPU Region Attribute field. Use to define the region attribute control. 28 = XN: Instruction access disable bit: 0 = Instruction fetches enabled. 1 = Instruction fetches disabled. 26:24 = AP: Access permission field 18 = S: Shareable bit 17 = C: Cacheable bit 16 = B: Bufferable bit"]
    #[inline(always)]
    #[must_use]
    pub fn attrs(&mut self) -> ATTRS_W<MPU_RASR_SPEC> {
        ATTRS_W::new(self, 16)
    }
}
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RASR_SPEC;
impl crate::RegisterSpec for MPU_RASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr::R`](R) reader structure"]
impl crate::Readable for MPU_RASR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr::W`](W) writer structure"]
impl crate::Writable for MPU_RASR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MPU_RASR_SPEC {
    const RESET_VALUE: u32 = 0;
}
