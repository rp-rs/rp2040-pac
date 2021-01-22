#[doc = "Reader of register NVIC_IPR7"]
pub type R = crate::R<u32, super::NVIC_IPR7>;
#[doc = "Writer for register NVIC_IPR7"]
pub type W = crate::W<u32, super::NVIC_IPR7>;
#[doc = "Register NVIC_IPR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_31`"]
pub type IP_31_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_31`"]
pub struct IP_31_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_30`"]
pub type IP_30_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_30`"]
pub struct IP_30_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_29`"]
pub type IP_29_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_29`"]
pub struct IP_29_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_28`"]
pub type IP_28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_28`"]
pub struct IP_28_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 31"]
    #[inline(always)]
    pub fn ip_31(&self) -> IP_31_R {
        IP_31_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 30"]
    #[inline(always)]
    pub fn ip_30(&self) -> IP_30_R {
        IP_30_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 29"]
    #[inline(always)]
    pub fn ip_29(&self) -> IP_29_R {
        IP_29_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 28"]
    #[inline(always)]
    pub fn ip_28(&self) -> IP_28_R {
        IP_28_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 31"]
    #[inline(always)]
    pub fn ip_31(&mut self) -> IP_31_W {
        IP_31_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 30"]
    #[inline(always)]
    pub fn ip_30(&mut self) -> IP_30_W {
        IP_30_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 29"]
    #[inline(always)]
    pub fn ip_29(&mut self) -> IP_29_W {
        IP_29_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 28"]
    #[inline(always)]
    pub fn ip_28(&mut self) -> IP_28_W {
        IP_28_W { w: self }
    }
}
