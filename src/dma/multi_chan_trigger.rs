#[doc = "Reader of register MULTI_CHAN_TRIGGER"]
pub type R = crate::R<u32, super::MULTI_CHAN_TRIGGER>;
#[doc = "Writer for register MULTI_CHAN_TRIGGER"]
pub type W = crate::W<u32, super::MULTI_CHAN_TRIGGER>;
#[doc = "Register MULTI_CHAN_TRIGGER `reset()`'s with value 0"]
impl crate::ResetValue for super::MULTI_CHAN_TRIGGER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MULTI_CHAN_TRIGGER`"]
pub type MULTI_CHAN_TRIGGER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MULTI_CHAN_TRIGGER`"]
pub struct MULTI_CHAN_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTI_CHAN_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
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
    pub fn multi_chan_trigger(&mut self) -> MULTI_CHAN_TRIGGER_W {
        MULTI_CHAN_TRIGGER_W { w: self }
    }
}
