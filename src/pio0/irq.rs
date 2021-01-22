#[doc = "Reader of register IRQ"]
pub type R = crate::R<u32, super::IRQ>;
#[doc = "Writer for register IRQ"]
pub type W = crate::W<u32, super::IRQ>;
#[doc = "Register IRQ `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRQ`"]
pub type IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRQ`"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
}
