#[doc = "Register `UARTFBRD` reader"]
pub type R = crate::R<UARTFBRD_SPEC>;
#[doc = "Register `UARTFBRD` writer"]
pub type W = crate::W<UARTFBRD_SPEC>;
#[doc = "Field `BAUD_DIVFRAC` reader - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
pub type BAUD_DIVFRAC_R = crate::FieldReader;
#[doc = "Field `BAUD_DIVFRAC` writer - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
pub type BAUD_DIVFRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divfrac(&self) -> BAUD_DIVFRAC_R {
        BAUD_DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    #[must_use]
    pub fn baud_divfrac(&mut self) -> BAUD_DIVFRAC_W<UARTFBRD_SPEC, 0> {
        BAUD_DIVFRAC_W::new(self)
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
#[doc = "Fractional Baud Rate Register, UARTFBRD  

You can [`read`](crate::generic::Reg::read) this register and get [`uartfbrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartfbrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTFBRD_SPEC;
impl crate::RegisterSpec for UARTFBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartfbrd::R`](R) reader structure"]
impl crate::Readable for UARTFBRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartfbrd::W`](W) writer structure"]
impl crate::Writable for UARTFBRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTFBRD to value 0"]
impl crate::Resettable for UARTFBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
