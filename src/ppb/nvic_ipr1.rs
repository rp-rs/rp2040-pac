#[doc = "Reader of register NVIC_IPR1"]
pub type R = crate::R<u32, super::NVIC_IPR1>;
#[doc = "Writer for register NVIC_IPR1"]
pub type W = crate::W<u32, super::NVIC_IPR1>;
#[doc = "Register NVIC_IPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_7`"]
pub type IP_7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_7`"]
pub struct IP_7_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_6`"]
pub type IP_6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_6`"]
pub struct IP_6_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_5`"]
pub type IP_5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_5`"]
pub struct IP_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_4`"]
pub type IP_4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_4`"]
pub struct IP_4_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    pub fn ip_7(&self) -> IP_7_R {
        IP_7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    pub fn ip_6(&self) -> IP_6_R {
        IP_6_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    pub fn ip_5(&self) -> IP_5_R {
        IP_5_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    pub fn ip_4(&self) -> IP_4_R {
        IP_4_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    pub fn ip_7(&mut self) -> IP_7_W {
        IP_7_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    pub fn ip_6(&mut self) -> IP_6_W {
        IP_6_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    pub fn ip_5(&mut self) -> IP_5_W {
        IP_5_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    pub fn ip_4(&mut self) -> IP_4_W {
        IP_4_W { w: self }
    }
}
