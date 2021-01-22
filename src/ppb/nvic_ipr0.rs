#[doc = "Reader of register NVIC_IPR0"]
pub type R = crate::R<u32, super::NVIC_IPR0>;
#[doc = "Writer for register NVIC_IPR0"]
pub type W = crate::W<u32, super::NVIC_IPR0>;
#[doc = "Register NVIC_IPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::NVIC_IPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP_3`"]
pub type IP_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_3`"]
pub struct IP_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `IP_2`"]
pub type IP_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_2`"]
pub struct IP_2_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `IP_1`"]
pub type IP_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_1`"]
pub struct IP_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IP_0`"]
pub type IP_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IP_0`"]
pub struct IP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 3"]
    #[inline(always)]
    pub fn ip_3(&self) -> IP_3_R {
        IP_3_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 2"]
    #[inline(always)]
    pub fn ip_2(&self) -> IP_2_R {
        IP_2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 1"]
    #[inline(always)]
    pub fn ip_1(&self) -> IP_1_R {
        IP_1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 0"]
    #[inline(always)]
    pub fn ip_0(&self) -> IP_0_R {
        IP_0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 3"]
    #[inline(always)]
    pub fn ip_3(&mut self) -> IP_3_W {
        IP_3_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 2"]
    #[inline(always)]
    pub fn ip_2(&mut self) -> IP_2_W {
        IP_2_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 1"]
    #[inline(always)]
    pub fn ip_1(&mut self) -> IP_1_W {
        IP_1_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 0"]
    #[inline(always)]
    pub fn ip_0(&mut self) -> IP_0_W {
        IP_0_W { w: self }
    }
}
