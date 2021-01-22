#[doc = "Reader of register SSIENR"]
pub type R = crate::R<u32, super::SSIENR>;
#[doc = "Writer for register SSIENR"]
pub type W = crate::W<u32, super::SSIENR>;
#[doc = "Register SSIENR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSIENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI_EN`"]
pub type SSI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_EN`"]
pub struct SSI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_EN_W<'a> {
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
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    pub fn ssi_en(&self) -> SSI_EN_R {
        SSI_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    pub fn ssi_en(&mut self) -> SSI_EN_W {
        SSI_EN_W { w: self }
    }
}
