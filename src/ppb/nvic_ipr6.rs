#[doc = "Reader of register NVIC_IPR6"]
pub type R = crate::R<u32, super::NVIC_IPR6>;
#[doc = "Writer for register NVIC_IPR6"]
pub type W = crate::W<u32, super::NVIC_IPR6>;
#[doc = "Register NVIC_IPR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_27`"]
pub type IP_27_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_27`"]
pub struct IP_27_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_26`"]
pub type IP_26_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_26`"]
pub struct IP_26_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_25`"]
pub type IP_25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_25`"]
pub struct IP_25_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_24`"]
pub type IP_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_24`"]
pub struct IP_24_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 27"]
    #[inline(always)]
    pub fn ip_27(&self) -> IP_27_R {
        IP_27_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 26"]
    #[inline(always)]
    pub fn ip_26(&self) -> IP_26_R {
        IP_26_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 25"]
    #[inline(always)]
    pub fn ip_25(&self) -> IP_25_R {
        IP_25_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 24"]
    #[inline(always)]
    pub fn ip_24(&self) -> IP_24_R {
        IP_24_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 27"]
    #[inline(always)]
    pub fn ip_27(&mut self) -> IP_27_W {
        IP_27_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 26"]
    #[inline(always)]
    pub fn ip_26(&mut self) -> IP_26_W {
        IP_26_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 25"]
    #[inline(always)]
    pub fn ip_25(&mut self) -> IP_25_W {
        IP_25_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 24"]
    #[inline(always)]
    pub fn ip_24(&mut self) -> IP_24_W {
        IP_24_W { w: self }
    }
}
