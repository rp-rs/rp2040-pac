#[doc = "Register `RXF%s` reader"]
pub struct R(crate::R<RXF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rxf](index.html) module"]
pub struct RXF_SPEC;
impl crate::RegisterSpec for RXF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf::R](R) reader structure"]
impl crate::Readable for RXF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF%s to value 0"]
impl crate::Resettable for RXF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
