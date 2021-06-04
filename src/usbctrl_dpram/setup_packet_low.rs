#[doc = "Reader of register SETUP_PACKET_LOW"]
pub type R = crate::R<u32, super::SETUP_PACKET_LOW>;
#[doc = "Writer for register SETUP_PACKET_LOW"]
pub type W = crate::W<u32, super::SETUP_PACKET_LOW>;
#[doc = "Register SETUP_PACKET_LOW `reset()`'s with value 0"]
impl crate::ResetValue for super::SETUP_PACKET_LOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WVALUE`"]
pub type WVALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WVALUE`"]
pub struct WVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> WVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BREQUEST`"]
pub type BREQUEST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BREQUEST`"]
pub struct BREQUEST_W<'a> {
    w: &'a mut W,
}
impl<'a> BREQUEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BMREQUESTTYPE`"]
pub type BMREQUESTTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BMREQUESTTYPE`"]
pub struct BMREQUESTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMREQUESTTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wvalue(&self) -> WVALUE_R {
        WVALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bmrequesttype(&self) -> BMREQUESTTYPE_R {
        BMREQUESTTYPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wvalue(&mut self) -> WVALUE_W {
        WVALUE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn brequest(&mut self) -> BREQUEST_W {
        BREQUEST_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bmrequesttype(&mut self) -> BMREQUESTTYPE_W {
        BMREQUESTTYPE_W { w: self }
    }
}
