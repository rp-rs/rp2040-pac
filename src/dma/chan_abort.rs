#[doc = "Reader of register CHAN_ABORT"]
pub type R = crate::R<u32, super::CHAN_ABORT>;
#[doc = "Writer for register CHAN_ABORT"]
pub type W = crate::W<u32, super::CHAN_ABORT>;
#[doc = "Register CHAN_ABORT `reset()`'s with value 0"]
impl crate::ResetValue for super::CHAN_ABORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHAN_ABORT`"]
pub type CHAN_ABORT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHAN_ABORT`"]
pub struct CHAN_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_ABORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.\\n\\n After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    pub fn chan_abort(&self) -> CHAN_ABORT_R {
        CHAN_ABORT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs.\\n\\n After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    pub fn chan_abort(&mut self) -> CHAN_ABORT_W {
        CHAN_ABORT_W { w: self }
    }
}
