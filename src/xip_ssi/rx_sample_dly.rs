#[doc = "Reader of register RX_SAMPLE_DLY"]
pub type R = crate::R<u32, super::RX_SAMPLE_DLY>;
#[doc = "Writer for register RX_SAMPLE_DLY"]
pub type W = crate::W<u32, super::RX_SAMPLE_DLY>;
#[doc = "Register RX_SAMPLE_DLY `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_SAMPLE_DLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSD`"]
pub type RSD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSD`"]
pub struct RSD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    pub fn rsd(&self) -> RSD_R {
        RSD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    pub fn rsd(&mut self) -> RSD_W {
        RSD_W { w: self }
    }
}
