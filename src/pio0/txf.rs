#[doc = "Register `TXF%s` reader"]
pub type R = crate::R<TXF_SPEC>;
#[doc = "Register `TXF%s` writer"]
pub type W = crate::W<TXF_SPEC>;
#[doc = "Field `TXF0` writer - "]
pub type TXF0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn txf0(&mut self) -> TXF0_W<TXF_SPEC> {
        TXF0_W::new(self, 0)
    }
}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.  

You can [`read`](crate::generic::Reg::read) this register and get [`txf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXF_SPEC;
impl crate::RegisterSpec for TXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txf::R`](R) reader structure"]
impl crate::Readable for TXF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txf::W`](W) writer structure"]
impl crate::Writable for TXF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXF%s to value 0"]
impl crate::Resettable for TXF_SPEC {
    const RESET_VALUE: u32 = 0;
}
