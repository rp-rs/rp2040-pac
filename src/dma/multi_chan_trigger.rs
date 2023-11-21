#[doc = "Register `MULTI_CHAN_TRIGGER` reader"]
pub type R = crate::R<MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Register `MULTI_CHAN_TRIGGER` writer"]
pub type W = crate::W<MULTI_CHAN_TRIGGER_SPEC>;
#[doc = "Field `MULTI_CHAN_TRIGGER` reader - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
pub type MULTI_CHAN_TRIGGER_R = crate::FieldReader<u16>;
#[doc = "Field `MULTI_CHAN_TRIGGER` writer - Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
pub type MULTI_CHAN_TRIGGER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
    pub fn multi_chan_trigger(&mut self) -> MULTI_CHAN_TRIGGER_W<MULTI_CHAN_TRIGGER_SPEC, 0> {
        MULTI_CHAN_TRIGGER_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Trigger one or more channels simultaneously  

You can [`read`](crate::generic::Reg::read) this register and get [`multi_chan_trigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`multi_chan_trigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULTI_CHAN_TRIGGER_SPEC;
impl crate::RegisterSpec for MULTI_CHAN_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`multi_chan_trigger::R`](R) reader structure"]
impl crate::Readable for MULTI_CHAN_TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`multi_chan_trigger::W`](W) writer structure"]
impl crate::Writable for MULTI_CHAN_TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULTI_CHAN_TRIGGER to value 0"]
impl crate::Resettable for MULTI_CHAN_TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
