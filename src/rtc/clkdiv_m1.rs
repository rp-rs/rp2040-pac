#[doc = "Register `CLKDIV_M1` reader"]
pub type R = crate::R<CLKDIV_M1_SPEC>;
#[doc = "Register `CLKDIV_M1` writer"]
pub type W = crate::W<CLKDIV_M1_SPEC>;
#[doc = "Field `CLKDIV_M1` reader - "]
pub type CLKDIV_M1_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV_M1` writer - "]
pub type CLKDIV_M1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clkdiv_m1(&self) -> CLKDIV_M1_R {
        CLKDIV_M1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_m1(&mut self) -> CLKDIV_M1_W<CLKDIV_M1_SPEC> {
        CLKDIV_M1_W::new(self, 0)
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
#[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled.  

You can [`read`](crate::generic::Reg::read) this register and get [`clkdiv_m1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv_m1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKDIV_M1_SPEC;
impl crate::RegisterSpec for CLKDIV_M1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv_m1::R`](R) reader structure"]
impl crate::Readable for CLKDIV_M1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkdiv_m1::W`](W) writer structure"]
impl crate::Writable for CLKDIV_M1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV_M1 to value 0"]
impl crate::Resettable for CLKDIV_M1_SPEC {
    const RESET_VALUE: u32 = 0;
}
