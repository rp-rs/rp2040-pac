#[doc = "Register `INTERP0_ACCUM1_ADD` reader"]
pub type R = crate::R<INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Register `INTERP0_ACCUM1_ADD` writer"]
pub type W = crate::W<INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Field `INTERP0_ACCUM1_ADD` reader - "]
pub type INTERP0_ACCUM1_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `INTERP0_ACCUM1_ADD` writer - "]
pub type INTERP0_ACCUM1_ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn interp0_accum1_add(&self) -> INTERP0_ACCUM1_ADD_R {
        INTERP0_ACCUM1_ADD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn interp0_accum1_add(&mut self) -> INTERP0_ACCUM1_ADD_W<INTERP0_ACCUM1_ADD_SPEC> {
        INTERP0_ACCUM1_ADD_W::new(self, 0)
    }
}
#[doc = "Values written here are atomically added to ACCUM1  
 Reading yields lane 1's raw shift and mask value (BASE1 not added).  

You can [`read`](crate::Reg::read) this register and get [`interp0_accum1_add::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_accum1_add::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_ACCUM1_ADD_SPEC;
impl crate::RegisterSpec for INTERP0_ACCUM1_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_accum1_add::R`](R) reader structure"]
impl crate::Readable for INTERP0_ACCUM1_ADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_accum1_add::W`](W) writer structure"]
impl crate::Writable for INTERP0_ACCUM1_ADD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP0_ACCUM1_ADD to value 0"]
impl crate::Resettable for INTERP0_ACCUM1_ADD_SPEC {
    const RESET_VALUE: u32 = 0;
}
