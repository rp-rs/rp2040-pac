#[doc = "Register `CH_AL1_TRANS_COUNT_TRIG` reader"]
pub type R = crate::R<CH_AL1_TRANS_COUNT_TRIG_SPEC>;
#[doc = "Register `CH_AL1_TRANS_COUNT_TRIG` writer"]
pub type W = crate::W<CH_AL1_TRANS_COUNT_TRIG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Alias for channel 0 TRANS_COUNT register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel.  

You can [`read`](crate::Reg::read) this register and get [`ch_al1_trans_count_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_al1_trans_count_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_AL1_TRANS_COUNT_TRIG_SPEC;
impl crate::RegisterSpec for CH_AL1_TRANS_COUNT_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_al1_trans_count_trig::R`](R) reader structure"]
impl crate::Readable for CH_AL1_TRANS_COUNT_TRIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_al1_trans_count_trig::W`](W) writer structure"]
impl crate::Writable for CH_AL1_TRANS_COUNT_TRIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_AL1_TRANS_COUNT_TRIG to value 0"]
impl crate::Resettable for CH_AL1_TRANS_COUNT_TRIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
