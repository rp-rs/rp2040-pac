#[doc = "Register `MULTI_CHAN_TRIGGER` reader"]
pub type R = crate::R<MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Register `MULTI_CHAN_TRIGGER` writer"]
pub type W = crate::W<MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Field `MULTI_CHAN_TRIGGER` reader - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
pub type MULTI_CHAN_TRIGGER_R = crate::FieldReader<u16>;
#[doc = "Field `MULTI_CHAN_TRIGGER` writer - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
pub type MULTI_CHAN_TRIGGER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    #[inline(always)]
    pub fn multi_chan_trigger(&self) -> MULTI_CHAN_TRIGGER_R {
        MULTI_CHAN_TRIGGER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    #[inline(always)]
    #[must_use]
    pub fn multi_chan_trigger(&mut self) -> MULTI_CHAN_TRIGGER_W<MULTI_CHAN_TRIGGER_SPEC> {
        MULTI_CHAN_TRIGGER_W::new(self, 0)
    }
}
#[doc = "Trigger one or more channels simultaneously  

You can [`read`](crate::Reg::read) this register and get [`multi_chan_trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`multi_chan_trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULTI_CHAN_TRIGGER_SPEC;
impl crate::RegisterSpec for MULTI_CHAN_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`multi_chan_trigger::R`](R) reader structure"]
impl crate::Readable for MULTI_CHAN_TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`multi_chan_trigger::W`](W) writer structure"]
impl crate::Writable for MULTI_CHAN_TRIGGER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MULTI_CHAN_TRIGGER to value 0"]
impl crate::Resettable for MULTI_CHAN_TRIGGER_SPEC {
    const RESET_VALUE: u32 = 0;
}
