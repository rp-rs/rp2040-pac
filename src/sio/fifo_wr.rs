#[doc = "Register `FIFO_WR` reader"]
pub type R = crate::R<FIFO_WR_SPEC>;
#[doc = "Register `FIFO_WR` writer"]
pub type W = crate::W<FIFO_WR_SPEC>;
#[doc = "Field `FIFO_WR` writer - "]
pub type FIFO_WR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_wr(&mut self) -> FIFO_WR_W<FIFO_WR_SPEC> {
        FIFO_WR_W::new(self, 0)
    }
}
#[doc = "Write access to this core's TX FIFO  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_wr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_wr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_WR_SPEC;
impl crate::RegisterSpec for FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_wr::R`](R) reader structure"]
impl crate::Readable for FIFO_WR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_wr::W`](W) writer structure"]
impl crate::Writable for FIFO_WR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_WR to value 0"]
impl crate::Resettable for FIFO_WR_SPEC {
    const RESET_VALUE: u32 = 0;
}
