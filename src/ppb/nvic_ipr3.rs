#[doc = "Reader of register NVIC_IPR3"]
pub type R = crate::R<u32, super::NVIC_IPR3>;
#[doc = "Writer for register NVIC_IPR3"]
pub type W = crate::W<u32, super::NVIC_IPR3>;
#[doc = "Register NVIC_IPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_15`"]
pub type IP_15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_15`"]
pub struct IP_15_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_14`"]
pub type IP_14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_14`"]
pub struct IP_14_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_13`"]
pub type IP_13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_13`"]
pub struct IP_13_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_12`"]
pub type IP_12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_12`"]
pub struct IP_12_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    pub fn ip_15(&self) -> IP_15_R {
        IP_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    pub fn ip_14(&self) -> IP_14_R {
        IP_14_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    pub fn ip_13(&self) -> IP_13_R {
        IP_13_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    pub fn ip_12(&self) -> IP_12_R {
        IP_12_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    pub fn ip_15(&mut self) -> IP_15_W {
        IP_15_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    pub fn ip_14(&mut self) -> IP_14_W {
        IP_14_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    pub fn ip_13(&mut self) -> IP_13_W {
        IP_13_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    pub fn ip_12(&mut self) -> IP_12_W {
        IP_12_W { w: self }
    }
}
