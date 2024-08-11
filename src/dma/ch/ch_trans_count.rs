#[doc = "Register `CH_TRANS_COUNT` reader"]
pub type R = crate::R<CH_TRANS_COUNT_SPEC>;
#[doc = "Register `CH_TRANS_COUNT` writer"]
pub type W = crate::W<CH_TRANS_COUNT_SPEC>;
#[doc = "Field `CH0_TRANS_COUNT` reader - Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub type CH0_TRANS_COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `CH0_TRANS_COUNT` writer - Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
pub type CH0_TRANS_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn ch0_trans_count(&self) -> CH0_TRANS_COUNT_R {
        CH0_TRANS_COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_trans_count(&mut self) -> CH0_TRANS_COUNT_W<CH_TRANS_COUNT_SPEC> {
        CH0_TRANS_COUNT_W::new(self, 0)
    }
}
#[doc = "DMA Channel 0 Transfer Count  

You can [`read`](crate::generic::Reg::read) this register and get [`ch_trans_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_trans_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TRANS_COUNT_SPEC;
impl crate::RegisterSpec for CH_TRANS_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_trans_count::R`](R) reader structure"]
impl crate::Readable for CH_TRANS_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_trans_count::W`](W) writer structure"]
impl crate::Writable for CH_TRANS_COUNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_TRANS_COUNT to value 0"]
impl crate::Resettable for CH_TRANS_COUNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
