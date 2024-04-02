#[doc = "Register `TXFTLR` reader"]
pub type R = crate::R<TXFTLR_SPEC>;
#[doc = "Register `TXFTLR` writer"]
pub type W = crate::W<TXFTLR_SPEC>;
#[doc = "Field `TFT` reader - Transmit FIFO threshold"]
pub type TFT_R = crate::FieldReader;
#[doc = "Field `TFT` writer - Transmit FIFO threshold"]
pub type TFT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tft(&self) -> TFT_R {
        TFT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tft(&mut self) -> TFT_W<TXFTLR_SPEC> {
        TFT_W::new(self, 0)
    }
}
#[doc = "TX FIFO threshold level  

You can [`read`](crate::Reg::read) this register and get [`txftlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txftlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXFTLR_SPEC;
impl crate::RegisterSpec for TXFTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txftlr::R`](R) reader structure"]
impl crate::Readable for TXFTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txftlr::W`](W) writer structure"]
impl crate::Writable for TXFTLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFTLR to value 0"]
impl crate::Resettable for TXFTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
