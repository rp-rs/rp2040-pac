#[doc = "Writer for register IRQ_FORCE"]
pub type W = crate::W<u32, super::IRQ_FORCE>;
#[doc = "Register IRQ_FORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ_FORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `IRQ_FORCE`"]
pub struct IRQ_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn irq_force(&mut self) -> IRQ_FORCE_W {
        IRQ_FORCE_W { w: self }
    }
}
