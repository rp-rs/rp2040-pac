#[doc = "Register `FIFO_RD` reader"]
pub type R = crate::R<FIFO_RD_SPEC>;
#[doc = "Register `FIFO_RD` writer"]
pub type W = crate::W<FIFO_RD_SPEC>;
#[doc = "Field `FIFO_RD` reader -   

The field is **modified** in some way after a read operation."]
pub type FIFO_RD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fifo_rd(&self) -> FIFO_RD_R {
        FIFO_RD_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read access to this core's RX FIFO  

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_RD_SPEC;
impl crate::RegisterSpec for FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_rd::R`](R) reader structure"]
impl crate::Readable for FIFO_RD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_rd::W`](W) writer structure"]
impl crate::Writable for FIFO_RD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_RD to value 0"]
impl crate::Resettable for FIFO_RD_SPEC {
    const RESET_VALUE: u32 = 0;
}
