#[doc = "Register `RXFTLR` reader"]
pub type R = crate::R<RXFTLR_SPEC>;
#[doc = "Register `RXFTLR` writer"]
pub type W = crate::W<RXFTLR_SPEC>;
#[doc = "Field `RFT` reader - Receive FIFO threshold"]
pub type RFT_R = crate::FieldReader;
#[doc = "Field `RFT` writer - Receive FIFO threshold"]
pub type RFT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn rft(&mut self) -> RFT_W<RXFTLR_SPEC> {
        RFT_W::new(self, 0)
    }
}
#[doc = "RX FIFO threshold level  

You can [`read`](crate::Reg::read) this register and get [`rxftlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxftlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFTLR_SPEC;
impl crate::RegisterSpec for RXFTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxftlr::R`](R) reader structure"]
impl crate::Readable for RXFTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxftlr::W`](W) writer structure"]
impl crate::Writable for RXFTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFTLR to value 0"]
impl crate::Resettable for RXFTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
