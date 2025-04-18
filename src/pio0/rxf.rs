#[doc = "Register `RXF%s` reader"]
pub type R = crate::R<RXF_SPEC>;
#[doc = "Register `RXF%s` writer"]
pub type W = crate::W<RXF_SPEC>;
#[doc = "Field `RXF0` reader -   

The field is **modified** in some way after a read operation."]
pub type RXF0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxf0(&self) -> RXF0_R {
        RXF0_R::new(self.bits)
    }
}
impl W {}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.  

You can [`read`](crate::generic::Reg::read) this register and get [`rxf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF_SPEC;
impl crate::RegisterSpec for RXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf::R`](R) reader structure"]
impl crate::Readable for RXF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf::W`](W) writer structure"]
impl crate::Writable for RXF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXF%s to value 0"]
impl crate::Resettable for RXF_SPEC {
    const RESET_VALUE: u32 = 0;
}
