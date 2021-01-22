#[doc = "Reader of register SM0_PINCTRL"]
pub type R = crate::R<u32, super::SM0_PINCTRL>;
#[doc = "Writer for register SM0_PINCTRL"]
pub type W = crate::W<u32, super::SM0_PINCTRL>;
#[doc = "Register SM0_PINCTRL `reset()`'s with value 0x1400_0000"]
impl crate::ResetValue for super::SM0_PINCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1400_0000
    }
}
#[doc = "Reader of field `SIDESET_COUNT`"]
pub type SIDESET_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIDESET_COUNT`"]
pub struct SIDESET_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDESET_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
#[doc = "Reader of field `SET_COUNT`"]
pub type SET_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SET_COUNT`"]
pub struct SET_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Reader of field `OUT_COUNT`"]
pub type OUT_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_COUNT`"]
pub struct OUT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `IN_BASE`"]
pub type IN_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IN_BASE`"]
pub struct IN_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `SIDESET_BASE`"]
pub type SIDESET_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIDESET_BASE`"]
pub struct SIDESET_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDESET_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SET_BASE`"]
pub type SET_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SET_BASE`"]
pub struct SET_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `OUT_BASE`"]
pub type OUT_BASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OUT_BASE`"]
pub struct OUT_BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
    #[inline(always)]
    pub fn sideset_count(&self) -> SIDESET_COUNT_R {
        SIDESET_COUNT_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. Max of 5"]
    #[inline(always)]
    pub fn set_count(&self) -> SET_COUNT_R {
        SET_COUNT_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
    #[inline(always)]
    pub fn out_count(&self) -> OUT_COUNT_R {
        OUT_COUNT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 15:19 - The virtual pin corresponding to IN bit 0"]
    #[inline(always)]
    pub fn in_base(&self) -> IN_BASE_R {
        IN_BASE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The virtual pin corresponding to delay field bit 0"]
    #[inline(always)]
    pub fn sideset_base(&self) -> SIDESET_BASE_R {
        SIDESET_BASE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The virtual pin corresponding to SET bit 0"]
    #[inline(always)]
    pub fn set_base(&self) -> SET_BASE_R {
        SET_BASE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - The virtual pin corresponding to OUT bit 0"]
    #[inline(always)]
    pub fn out_base(&self) -> OUT_BASE_R {
        OUT_BASE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - The number of delay bits co-opted for side-set. Inclusive of the enable bit, if present."]
    #[inline(always)]
    pub fn sideset_count(&mut self) -> SIDESET_COUNT_W {
        SIDESET_COUNT_W { w: self }
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. Max of 5"]
    #[inline(always)]
    pub fn set_count(&mut self) -> SET_COUNT_W {
        SET_COUNT_W { w: self }
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT. Value of 0 -> 32 pins"]
    #[inline(always)]
    pub fn out_count(&mut self) -> OUT_COUNT_W {
        OUT_COUNT_W { w: self }
    }
    #[doc = "Bits 15:19 - The virtual pin corresponding to IN bit 0"]
    #[inline(always)]
    pub fn in_base(&mut self) -> IN_BASE_W {
        IN_BASE_W { w: self }
    }
    #[doc = "Bits 10:14 - The virtual pin corresponding to delay field bit 0"]
    #[inline(always)]
    pub fn sideset_base(&mut self) -> SIDESET_BASE_W {
        SIDESET_BASE_W { w: self }
    }
    #[doc = "Bits 5:9 - The virtual pin corresponding to SET bit 0"]
    #[inline(always)]
    pub fn set_base(&mut self) -> SET_BASE_W {
        SET_BASE_W { w: self }
    }
    #[doc = "Bits 0:4 - The virtual pin corresponding to OUT bit 0"]
    #[inline(always)]
    pub fn out_base(&mut self) -> OUT_BASE_W {
        OUT_BASE_W { w: self }
    }
}
