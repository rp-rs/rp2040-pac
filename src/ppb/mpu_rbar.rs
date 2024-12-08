#[doc = "Register `MPU_RBAR` reader"]
pub type R = crate::R<MPU_RBAR_SPEC>;
#[doc = "Register `MPU_RBAR` writer"]
pub type W = crate::W<MPU_RBAR_SPEC>;
#[doc = "Field `REGION` reader - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
pub type REGION_R = crate::FieldReader;
#[doc = "Field `REGION` writer - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
pub type REGION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VALID` reader - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region. Write: 0 = MPU_RNR not changed, and the processor: Updates the base address for the region specified in the MPU_RNR. Ignores the value of the REGION field. 1 = The processor: Updates the value of the MPU_RNR to the value of the REGION field. Updates the base address for the region specified in the REGION field. Always reads as zero."]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region. Write: 0 = MPU_RNR not changed, and the processor: Updates the base address for the region specified in the MPU_RNR. Ignores the value of the REGION field. 1 = The processor: Updates the value of the MPU_RNR to the value of the REGION field. Updates the base address for the region specified in the REGION field. Always reads as zero."]
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Base address of the region."]
pub type ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Base address of the region."]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region. Write: 0 = MPU_RNR not changed, and the processor: Updates the base address for the region specified in the MPU_RNR. Ignores the value of the REGION field. 1 = The processor: Updates the value of the MPU_RNR to the value of the REGION field. Updates the base address for the region specified in the REGION field. Always reads as zero."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits \\[3:0\\]
of MPU_RNR."]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<MPU_RBAR_SPEC> {
        REGION_W::new(self, 0)
    }
    #[doc = "Bit 4 - On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region. Write: 0 = MPU_RNR not changed, and the processor: Updates the base address for the region specified in the MPU_RNR. Ignores the value of the REGION field. 1 = The processor: Updates the value of the MPU_RNR to the value of the REGION field. Updates the base address for the region specified in the REGION field. Always reads as zero."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<MPU_RBAR_SPEC> {
        VALID_W::new(self, 4)
    }
    #[doc = "Bits 8:31 - Base address of the region."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<MPU_RBAR_SPEC> {
        ADDR_W::new(self, 8)
    }
}
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RBAR_SPEC;
impl crate::RegisterSpec for MPU_RBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rbar::R`](R) reader structure"]
impl crate::Readable for MPU_RBAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rbar::W`](W) writer structure"]
impl crate::Writable for MPU_RBAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RBAR to value 0"]
impl crate::Resettable for MPU_RBAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
