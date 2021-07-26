#[doc = "Register `CH_TRANS_COUNT` reader"]
pub struct R(crate::R<CH_TRANS_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TRANS_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TRANS_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TRANS_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_TRANS_COUNT` writer"]
pub struct W(crate::W<CH_TRANS_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_TRANS_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH_TRANS_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_TRANS_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel 0 Transfer Count  
 Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE).  

 When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes.  

 Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write.  

 The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch_trans_count](index.html) module"]
pub struct CH_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_trans_count::R](R) reader structure"]
impl crate::Readable for CH_TRANS_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_trans_count::W](W) writer structure"]
impl crate::Writable for CH_TRANS_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_TRANS_COUNT to value 0"]
impl crate::Resettable for CH_TRANS_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
