#[doc = "Reader of register NVIC_IPR4"]
pub type R = crate::R<u32, super::NVIC_IPR4>;
#[doc = "Writer for register NVIC_IPR4"]
pub type W = crate::W<u32, super::NVIC_IPR4>;
#[doc = "Register NVIC_IPR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_19`"]
pub type IP_19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_19`"]
pub struct IP_19_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_18`"]
pub type IP_18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_18`"]
pub struct IP_18_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_17`"]
pub type IP_17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_17`"]
pub struct IP_17_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_16`"]
pub type IP_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_16`"]
pub struct IP_16_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 19"]
    #[inline(always)]
    pub fn ip_19(&self) -> IP_19_R {
        IP_19_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 18"]
    #[inline(always)]
    pub fn ip_18(&self) -> IP_18_R {
        IP_18_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 17"]
    #[inline(always)]
    pub fn ip_17(&self) -> IP_17_R {
        IP_17_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 16"]
    #[inline(always)]
    pub fn ip_16(&self) -> IP_16_R {
        IP_16_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 19"]
    #[inline(always)]
    pub fn ip_19(&mut self) -> IP_19_W {
        IP_19_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 18"]
    #[inline(always)]
    pub fn ip_18(&mut self) -> IP_18_W {
        IP_18_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 17"]
    #[inline(always)]
    pub fn ip_17(&mut self) -> IP_17_W {
        IP_17_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 16"]
    #[inline(always)]
    pub fn ip_16(&mut self) -> IP_16_W {
        IP_16_W { w: self }
    }
}
