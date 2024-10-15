#[doc = "Register `RXF%s` reader"]
pub type R = crate::R<RXF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.  

You can [`read`](crate::Reg::read) this register and get [`rxf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF_SPEC;
impl crate::RegisterSpec for RXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf::R`](R) reader structure"]
impl crate::Readable for RXF_SPEC {}
#[doc = "`reset()` method sets RXF%s to value 0"]
impl crate::Resettable for RXF_SPEC {
    const RESET_VALUE: u32 = 0;
}
