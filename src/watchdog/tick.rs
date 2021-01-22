#[doc = "Reader of register TICK"]
pub type R = crate::R<u32, super::TICK>;
#[doc = "Writer for register TICK"]
pub type W = crate::W<u32, super::TICK>;
#[doc = "Register TICK `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::TICK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `RUNNING`"]
pub type RUNNING_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CYCLES`"]
pub type CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CYCLES`"]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:19 - Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 10 - Is the tick generator running?"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - start / stop tick generation"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 9 - start / stop tick generation"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:8 - Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
}
