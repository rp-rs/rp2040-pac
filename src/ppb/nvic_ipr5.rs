#[doc = "Reader of register NVIC_IPR5"]
pub type R = crate::R<u32, super::NVIC_IPR5>;
#[doc = "Writer for register NVIC_IPR5"]
pub type W = crate::W<u32, super::NVIC_IPR5>;
#[doc = "Register NVIC_IPR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_23`"]
pub type IP_23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_23`"]
pub struct IP_23_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_22`"]
pub type IP_22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_22`"]
pub struct IP_22_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_21`"]
pub type IP_21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_21`"]
pub struct IP_21_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_20`"]
pub type IP_20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_20`"]
pub struct IP_20_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    pub fn ip_23(&self) -> IP_23_R {
        IP_23_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    pub fn ip_22(&self) -> IP_22_R {
        IP_22_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    pub fn ip_21(&self) -> IP_21_R {
        IP_21_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    pub fn ip_20(&self) -> IP_20_R {
        IP_20_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    pub fn ip_23(&mut self) -> IP_23_W {
        IP_23_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    pub fn ip_22(&mut self) -> IP_22_W {
        IP_22_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    pub fn ip_21(&mut self) -> IP_21_W {
        IP_21_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    pub fn ip_20(&mut self) -> IP_20_W {
        IP_20_W { w: self }
    }
}
