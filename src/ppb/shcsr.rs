#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<SHCSR_SPEC>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<SHCSR_SPEC>;
#[doc = "Field `SVCALLPENDED` reader - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
pub type SVCALLPENDED_R = crate::BitReader;
#[doc = "Field `SVCALLPENDED` writer - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
pub type SVCALLPENDED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 15 - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<SHCSR_SPEC, 15> {
        SVCALLPENDED_W::new(self)
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
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall.  

You can [`read`](crate::generic::Reg::read) this register and get [`shcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
