#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<SHCSR_SPEC>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<SHCSR_SPEC>;
#[doc = "Field `SVCALLPENDED` reader - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
pub type SVCALLPENDED_R = crate::BitReader;
#[doc = "Field `SVCALLPENDED` writer - Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
pub type SVCALLPENDED_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W<SHCSR_SPEC> {
        SVCALLPENDED_W::new(self, 15)
    }
}
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall.  

You can [`read`](crate::Reg::read) this register and get [`shcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
