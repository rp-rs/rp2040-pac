#[doc = "Reader of register PRIM"]
pub type R = crate::R<u32, super::PRIM>;
#[doc = "Writer for register PRIM"]
pub type W = crate::W<u32, super::PRIM>;
#[doc = "Register PRIM `reset()`'s with value 0x0007_7000"]
impl crate::ResetValue for super::PRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_7000
    }
}
#[doc = "Reader of field `POSTDIV1`"]
pub type POSTDIV1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSTDIV1`"]
pub struct POSTDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `POSTDIV2`"]
pub type POSTDIV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POSTDIV2`"]
pub struct POSTDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&self) -> POSTDIV2_R {
        POSTDIV2_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&mut self) -> POSTDIV1_W {
        POSTDIV1_W { w: self }
    }
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&mut self) -> POSTDIV2_W {
        POSTDIV2_W { w: self }
    }
}
