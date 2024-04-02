#[doc = "Register `TXF%s` writer"]
pub type W = crate::W<TXF_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TXF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO.  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXF_SPEC;
impl crate::RegisterSpec for TXF_SPEC {
    type Ux = u32;
}
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
