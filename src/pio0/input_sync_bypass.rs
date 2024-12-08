#[doc = "Register `INPUT_SYNC_BYPASS` reader"]
pub type R = crate::R<INPUT_SYNC_BYPASS_SPEC>;
#[doc = "Register `INPUT_SYNC_BYPASS` writer"]
pub type W = crate::W<INPUT_SYNC_BYPASS_SPEC>;
#[doc = "Field `INPUT_SYNC_BYPASS` reader - "]
pub type INPUT_SYNC_BYPASS_R = crate::FieldReader<u32>;
#[doc = "Field `INPUT_SYNC_BYPASS` writer - "]
pub type INPUT_SYNC_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn input_sync_bypass(&self) -> INPUT_SYNC_BYPASS_R {
        INPUT_SYNC_BYPASS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn input_sync_bypass(&mut self) -> INPUT_SYNC_BYPASS_W<INPUT_SYNC_BYPASS_SPEC> {
        INPUT_SYNC_BYPASS_W::new(self, 0)
    }
}
#[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes.  

You can [`read`](crate::generic::Reg::read) this register and get [`input_sync_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`input_sync_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPUT_SYNC_BYPASS_SPEC;
impl crate::RegisterSpec for INPUT_SYNC_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_sync_bypass::R`](R) reader structure"]
impl crate::Readable for INPUT_SYNC_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`input_sync_bypass::W`](W) writer structure"]
impl crate::Writable for INPUT_SYNC_BYPASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPUT_SYNC_BYPASS to value 0"]
impl crate::Resettable for INPUT_SYNC_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0;
}
