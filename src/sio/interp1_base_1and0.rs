#[doc = "Register `INTERP1_BASE_1AND0` writer"]
pub type W = crate::W<INTERP1_BASE_1AND0_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<INTERP1_BASE_1AND0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
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
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously.  
 Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp1_base_1and0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_BASE_1AND0_SPEC;
impl crate::RegisterSpec for INTERP1_BASE_1AND0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`interp1_base_1and0::W`](W) writer structure"]
impl crate::Writable for INTERP1_BASE_1AND0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERP1_BASE_1AND0 to value 0"]
impl crate::Resettable for INTERP1_BASE_1AND0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
