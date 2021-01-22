#[doc = "Reader of register STREAM_ADDR"]
pub type R = crate::R<u32, super::STREAM_ADDR>;
#[doc = "Writer for register STREAM_ADDR"]
pub type W = crate::W<u32, super::STREAM_ADDR>;
#[doc = "Register STREAM_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::STREAM_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STREAM_ADDR`"]
pub type STREAM_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STREAM_ADDR`"]
pub struct STREAM_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> STREAM_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - The address of the next word to be streamed from flash to the streaming FIFO.\\n Increments automatically after each flash access.\\n Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub fn stream_addr(&self) -> STREAM_ADDR_R {
        STREAM_ADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - The address of the next word to be streamed from flash to the streaming FIFO.\\n Increments automatically after each flash access.\\n Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub fn stream_addr(&mut self) -> STREAM_ADDR_W {
        STREAM_ADDR_W { w: self }
    }
}
