#[doc = "Register `INTERP1_BASE0` reader"]
pub type R = crate::R<INTERP1_BASE0_SPEC>;
#[doc = "Register `INTERP1_BASE0` writer"]
pub type W = crate::W<INTERP1_BASE0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTERP1_BASE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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

You can [`read`](crate::generic::Reg::read) this register and get [`interp1_base0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_BASE0_SPEC;
impl crate::RegisterSpec for INTERP1_BASE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_base0::R`](R) reader structure"]
impl crate::Readable for INTERP1_BASE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp1_base0::W`](W) writer structure"]
impl crate::Writable for INTERP1_BASE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERP1_BASE0 to value 0"]
impl crate::Resettable for INTERP1_BASE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
