#[doc = "Reader of register CTR"]
pub type R = crate::R<u32, super::CTR>;
#[doc = "Writer for register CTR"]
pub type W = crate::W<u32, super::CTR>;
#[doc = "Register CTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTR`"]
pub type CTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTR`"]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
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
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
}
