#[doc = "Reader of register FLUSH"]
pub type R = crate::R<u32, super::FLUSH>;
#[doc = "Writer for register FLUSH"]
pub type W = crate::W<u32, super::FLUSH>;
#[doc = "Register FLUSH `reset()`'s with value 0"]
impl crate::ResetValue for super::FLUSH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLUSH`"]
pub type FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLUSH`"]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write 1 to flush the cache. This clears the tag memory, but\\n the data memory retains its contents. (This means cache-as-SRAM\\n contents is not affected by flush or reset.)\\n Reading will hold the bus (stall the processor) until the flush\\n completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to flush the cache. This clears the tag memory, but\\n the data memory retains its contents. (This means cache-as-SRAM\\n contents is not affected by flush or reset.)\\n Reading will hold the bus (stall the processor) until the flush\\n completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
}
