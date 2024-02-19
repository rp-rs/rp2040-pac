#[doc = "Register `INTERP0_BASE0` reader"]
pub type R = crate::R<INTERP0_BASE0_SPEC>;
#[doc = "Register `INTERP0_BASE0` writer"]
pub type W = crate::W<INTERP0_BASE0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTERP0_BASE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "Read/write access to BASE0 register.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_base0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_base0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_BASE0_SPEC;
impl crate::RegisterSpec for INTERP0_BASE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_base0::R`](R) reader structure"]
impl crate::Readable for INTERP0_BASE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_base0::W`](W) writer structure"]
impl crate::Writable for INTERP0_BASE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP0_BASE0 to value 0"]
impl crate::Resettable for INTERP0_BASE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
