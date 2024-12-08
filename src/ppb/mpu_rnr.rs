#[doc = "Register `MPU_RNR` reader"]
pub type R = crate::R<MPU_RNR_SPEC>;
#[doc = "Register `MPU_RNR` writer"]
pub type W = crate::W<MPU_RNR_SPEC>;
#[doc = "Field `REGION` reader - Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
pub type REGION_R = crate::FieldReader;
#[doc = "Field `REGION` writer - Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
pub type REGION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> REGION_W<MPU_RNR_SPEC> {
        REGION_W::new(self, 0)
    }
}
#[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR.  

You can [`read`](crate::generic::Reg::read) this register and get [`mpu_rnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPU_RNR_SPEC;
impl crate::RegisterSpec for MPU_RNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rnr::R`](R) reader structure"]
impl crate::Readable for MPU_RNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mpu_rnr::W`](W) writer structure"]
impl crate::Writable for MPU_RNR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RNR to value 0"]
impl crate::Resettable for MPU_RNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
