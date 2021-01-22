#[doc = "Reader of register STREAM_CTR"]
pub type R = crate::R<u32, super::STREAM_CTR>;
#[doc = "Writer for register STREAM_CTR"]
pub type W = crate::W<u32, super::STREAM_CTR>;
#[doc = "Register STREAM_CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::STREAM_CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STREAM_CTR`"]
pub type STREAM_CTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STREAM_CTR`"]
pub struct STREAM_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> STREAM_CTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Write a nonzero value to start a streaming read. This will then\\n progress in the background, using flash idle cycles to transfer\\n a linear data block from flash to the streaming FIFO.\\n Decrements automatically (1 at a time) as the stream\\n progresses, and halts on reaching 0.\\n Write 0 to halt an in-progress stream, and discard any in-flight\\n read, so that a new stream can immediately be started (after\\n draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub fn stream_ctr(&self) -> STREAM_CTR_R {
        STREAM_CTR_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Write a nonzero value to start a streaming read. This will then\\n progress in the background, using flash idle cycles to transfer\\n a linear data block from flash to the streaming FIFO.\\n Decrements automatically (1 at a time) as the stream\\n progresses, and halts on reaching 0.\\n Write 0 to halt an in-progress stream, and discard any in-flight\\n read, so that a new stream can immediately be started (after\\n draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub fn stream_ctr(&mut self) -> STREAM_CTR_W {
        STREAM_CTR_W { w: self }
    }
}
