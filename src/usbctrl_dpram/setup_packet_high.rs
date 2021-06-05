#[doc = "Reader of register SETUP_PACKET_HIGH"]
pub type R = crate::R<u32, super::SETUP_PACKET_HIGH>;
#[doc = "Writer for register SETUP_PACKET_HIGH"]
pub type W = crate::W<u32, super::SETUP_PACKET_HIGH>;
#[doc = "Register SETUP_PACKET_HIGH `reset()`'s with value 0"]
impl crate::ResetValue for super::SETUP_PACKET_HIGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WLENGTH`"]
pub type WLENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WLENGTH`"]
pub struct WLENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WLENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WINDEX`"]
pub type WINDEX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINDEX`"]
pub struct WINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDEX_W<'a> {
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
    pub fn wlength(&self) -> WLENGTH_R {
        WLENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn windex(&self) -> WINDEX_R {
        WINDEX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wlength(&mut self) -> WLENGTH_W {
        WLENGTH_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn windex(&mut self) -> WINDEX_W {
        WINDEX_W { w: self }
    }
}
