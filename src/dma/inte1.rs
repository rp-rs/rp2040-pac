#[doc = "Register `INTE1` reader"]
pub type R = crate::R<INTE1_SPEC>;
#[doc = "Register `INTE1` writer"]
pub type W = crate::W<INTE1_SPEC>;
#[doc = "Field `INTE1` reader - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
pub type INTE1_R = crate::FieldReader<u16>;
#[doc = "Field `INTE1` writer - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
pub type INTE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    pub fn inte1(&self) -> INTE1_R {
        INTE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    #[must_use]
    pub fn inte1(&mut self) -> INTE1_W<INTE1_SPEC, 0> {
        INTE1_W::new(self)
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
#[doc = "Interrupt Enables for IRQ 1  

You can [`read`](crate::generic::Reg::read) this register and get [`inte1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inte1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE1_SPEC;
impl crate::RegisterSpec for INTE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte1::R`](R) reader structure"]
impl crate::Readable for INTE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte1::W`](W) writer structure"]
impl crate::Writable for INTE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTE1 to value 0"]
impl crate::Resettable for INTE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
