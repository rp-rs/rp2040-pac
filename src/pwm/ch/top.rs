#[doc = "Reader of register top"]
pub type R = crate::R<u32, super::TOP>;
#[doc = "Writer for register top"]
pub type W = crate::W<u32, super::TOP>;
#[doc = "Register top `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
}
