#[doc = "Register `UARTIBRD` reader"]
pub type R = crate::R<UARTIBRD_SPEC>;
#[doc = "Register `UARTIBRD` writer"]
pub type W = crate::W<UARTIBRD_SPEC>;
#[doc = "Field `BAUD_DIVINT` reader - The integer baud rate divisor. These bits are cleared to 0 on reset."]
pub type BAUD_DIVINT_R = crate::FieldReader<u16>;
#[doc = "Field `BAUD_DIVINT` writer - The integer baud rate divisor. These bits are cleared to 0 on reset."]
pub type BAUD_DIVINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divint(&self) -> BAUD_DIVINT_R {
        BAUD_DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    #[must_use]
    pub fn baud_divint(&mut self) -> BAUD_DIVINT_W<UARTIBRD_SPEC> {
        BAUD_DIVINT_W::new(self, 0)
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
#[doc = "Integer Baud Rate Register, UARTIBRD  

You can [`read`](crate::generic::Reg::read) this register and get [`uartibrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartibrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTIBRD_SPEC;
impl crate::RegisterSpec for UARTIBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartibrd::R`](R) reader structure"]
impl crate::Readable for UARTIBRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartibrd::W`](W) writer structure"]
impl crate::Writable for UARTIBRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTIBRD to value 0"]
impl crate::Resettable for UARTIBRD_SPEC {
    const RESET_VALUE: u32 = 0;
}
