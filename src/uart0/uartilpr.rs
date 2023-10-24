#[doc = "Register `UARTILPR` reader"]
pub type R = crate::R<UARTILPR_SPEC>;
#[doc = "Register `UARTILPR` writer"]
pub type W = crate::W<UARTILPR_SPEC>;
#[doc = "Field `ILPDVSR` reader - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
pub type ILPDVSR_R = crate::FieldReader;
#[doc = "Field `ILPDVSR` writer - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
pub type ILPDVSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> ILPDVSR_R {
        ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    #[must_use]
    pub fn ilpdvsr(&mut self) -> ILPDVSR_W<UARTILPR_SPEC, 0> {
        ILPDVSR_W::new(self)
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
#[doc = "IrDA Low-Power Counter Register, UARTILPR  

You can [`read`](crate::generic::Reg::read) this register and get [`uartilpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartilpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTILPR_SPEC;
impl crate::RegisterSpec for UARTILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartilpr::R`](R) reader structure"]
impl crate::Readable for UARTILPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartilpr::W`](W) writer structure"]
impl crate::Writable for UARTILPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTILPR to value 0"]
impl crate::Resettable for UARTILPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
