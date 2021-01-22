#[doc = "Reader of register SER"]
pub type R = crate::R<u32, super::SER>;
#[doc = "Writer for register SER"]
pub type W = crate::W<u32, super::SER>;
#[doc = "Register SER `reset()`'s with value 0"]
impl crate::ResetValue for super::SER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SER`"]
pub type SER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SER`"]
pub struct SER_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_W<'a> {
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
    #[doc = "Bit 0 - For each bit:\\n 0 -> slave not selected\\n 1 -> slave selected"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For each bit:\\n 0 -> slave not selected\\n 1 -> slave selected"]
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W {
        SER_W { w: self }
    }
}
