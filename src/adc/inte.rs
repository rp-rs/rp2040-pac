#[doc = "Reader of register INTE"]
pub type R = crate::R<u32, super::INTE>;
#[doc = "Writer for register INTE"]
pub type W = crate::W<u32, super::INTE>;
#[doc = "Register INTE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFO`"]
pub type FIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO`"]
pub struct FIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_W<'a> {
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
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level.\\n This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level.\\n This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn fifo(&mut self) -> FIFO_W {
        FIFO_W { w: self }
    }
}
