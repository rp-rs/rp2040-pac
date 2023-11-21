#[doc = "Register `RXFTLR` reader"]
pub type R = crate::R<RXFTLR_SPEC>;
#[doc = "Register `RXFTLR` writer"]
pub type W = crate::W<RXFTLR_SPEC>;
#[doc = "Field `RFT` reader - Receive FIFO threshold"]
pub type RFT_R = crate::FieldReader;
#[doc = "Field `RFT` writer - Receive FIFO threshold"]
pub type RFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO threshold"]
    #[inline(always)]
    pub fn rft(&self) -> RFT_R {
        RFT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rft(&mut self) -> RFT_W<RXFTLR_SPEC, 0> {
        RFT_W::new(self)
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
#[doc = "RX FIFO threshold level  

You can [`read`](crate::generic::Reg::read) this register and get [`rxftlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxftlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFTLR_SPEC;
impl crate::RegisterSpec for RXFTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxftlr::R`](R) reader structure"]
impl crate::Readable for RXFTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxftlr::W`](W) writer structure"]
impl crate::Writable for RXFTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFTLR to value 0"]
impl crate::Resettable for RXFTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
