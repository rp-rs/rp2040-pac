#[doc = "Register `PAUSE` reader"]
pub type R = crate::R<PAUSE_SPEC>;
#[doc = "Register `PAUSE` writer"]
pub type W = crate::W<PAUSE_SPEC>;
#[doc = "Field `PAUSE` reader - "]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - "]
pub type PAUSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<PAUSE_SPEC, 0> {
        PAUSE_W::new(self)
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
#[doc = "Set high to pause the timer  

You can [`read`](crate::generic::Reg::read) this register and get [`pause::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pause::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAUSE_SPEC;
impl crate::RegisterSpec for PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pause::R`](R) reader structure"]
impl crate::Readable for PAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pause::W`](W) writer structure"]
impl crate::Writable for PAUSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAUSE to value 0"]
impl crate::Resettable for PAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
