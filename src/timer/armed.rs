#[doc = "Reader of register ARMED"]
pub type R = crate::R<u32, super::ARMED>;
#[doc = "Writer for register ARMED"]
pub type W = crate::W<u32, super::ARMED>;
#[doc = "Register ARMED `reset()`'s with value 0"]
impl crate::ResetValue for super::ARMED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARMED`"]
pub type ARMED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARMED`"]
pub struct ARMED_W<'a> {
    w: &'a mut W,
}
impl<'a> ARMED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn armed(&self) -> ARMED_R {
        ARMED_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn armed(&mut self) -> ARMED_W {
        ARMED_W { w: self }
    }
}
