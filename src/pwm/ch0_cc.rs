#[doc = "Reader of register CH0_CC"]
pub type R = crate::R<u32, super::CH0_CC>;
#[doc = "Writer for register CH0_CC"]
pub type W = crate::W<u32, super::CH0_CC>;
#[doc = "Register CH0_CC `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0_CC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B`"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `A`"]
pub type A_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A`"]
pub struct A_W<'a> {
    w: &'a mut W,
}
impl<'a> A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&mut self) -> A_W {
        A_W { w: self }
    }
}
