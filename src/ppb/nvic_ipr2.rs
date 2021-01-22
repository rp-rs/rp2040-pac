#[doc = "Reader of register NVIC_IPR2"]
pub type R = crate::R<u32, super::NVIC_IPR2>;
#[doc = "Writer for register NVIC_IPR2"]
pub type W = crate::W<u32, super::NVIC_IPR2>;
#[doc = "Register NVIC_IPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_11`"]
pub type IP_11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_11`"]
pub struct IP_11_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_10`"]
pub type IP_10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_10`"]
pub struct IP_10_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_9`"]
pub type IP_9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_9`"]
pub struct IP_9_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_8`"]
pub type IP_8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_8`"]
pub struct IP_8_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    pub fn ip_11(&self) -> IP_11_R {
        IP_11_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    pub fn ip_10(&self) -> IP_10_R {
        IP_10_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    pub fn ip_9(&self) -> IP_9_R {
        IP_9_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn ip_8(&self) -> IP_8_R {
        IP_8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    pub fn ip_11(&mut self) -> IP_11_W {
        IP_11_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    pub fn ip_10(&mut self) -> IP_10_W {
        IP_10_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    pub fn ip_9(&mut self) -> IP_9_W {
        IP_9_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn ip_8(&mut self) -> IP_8_W {
        IP_8_W { w: self }
    }
}
