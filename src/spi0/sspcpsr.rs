#[doc = "Register `SSPCPSR` reader"]
pub type R = crate::R<SSPCPSR_SPEC>;
#[doc = "Register `SSPCPSR` writer"]
pub type W = crate::W<SSPCPSR_SPEC>;
#[doc = "Field `CPSDVSR` reader - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub type CPSDVSR_R = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub type CPSDVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    #[must_use]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W<SSPCPSR_SPEC> {
        CPSDVSR_W::new(self, 0)
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
#[doc = "Clock prescale register, SSPCPSR on page 3-8  

You can [`read`](crate::generic::Reg::read) this register and get [`sspcpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspcpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPCPSR_SPEC;
impl crate::RegisterSpec for SSPCPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspcpsr::R`](R) reader structure"]
impl crate::Readable for SSPCPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspcpsr::W`](W) writer structure"]
impl crate::Writable for SSPCPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPCPSR to value 0"]
impl crate::Resettable for SSPCPSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
