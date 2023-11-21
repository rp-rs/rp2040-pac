#[doc = "Register `INTERP0_ACCUM1_ADD` reader"]
pub type R = crate::R<INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Register `INTERP0_ACCUM1_ADD` writer"]
pub type W = crate::W<INTERP0_ACCUM1_ADD_SPEC>;
#[doc = "Field `INTERP0_ACCUM1_ADD` reader - "]
pub type INTERP0_ACCUM1_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `INTERP0_ACCUM1_ADD` writer - "]
pub type INTERP0_ACCUM1_ADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
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
    pub fn interp0_accum1_add(&mut self) -> INTERP0_ACCUM1_ADD_W<INTERP0_ACCUM1_ADD_SPEC, 0> {
        INTERP0_ACCUM1_ADD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Values written here are atomically added to ACCUM1  
 Reading yields lane 1's raw shift and mask value (BASE1 not added).  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_accum1_add::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_accum1_add::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_ACCUM1_ADD_SPEC;
impl crate::RegisterSpec for INTERP0_ACCUM1_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_accum1_add::R`](R) reader structure"]
impl crate::Readable for INTERP0_ACCUM1_ADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_accum1_add::W`](W) writer structure"]
impl crate::Writable for INTERP0_ACCUM1_ADD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERP0_ACCUM1_ADD to value 0"]
impl crate::Resettable for INTERP0_ACCUM1_ADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
