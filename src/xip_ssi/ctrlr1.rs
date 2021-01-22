#[doc = "Reader of register CTRLR1"]
pub type R = crate::R<u32, super::CTRLR1>;
#[doc = "Writer for register CTRLR1"]
pub type W = crate::W<u32, super::CTRLR1>;
#[doc = "Register CTRLR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NDF`"]
pub type NDF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NDF`"]
pub struct NDF_W<'a> {
    w: &'a mut W,
}
impl<'a> NDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of data frames"]
    #[inline(always)]
    pub fn ndf(&self) -> NDF_R {
        NDF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data frames"]
    #[inline(always)]
    pub fn ndf(&mut self) -> NDF_W {
        NDF_W { w: self }
    }
}
