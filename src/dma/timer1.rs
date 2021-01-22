#[doc = "Reader of register TIMER1"]
pub type R = crate::R<u32, super::TIMER1>;
#[doc = "Writer for register TIMER1"]
pub type W = crate::W<u32, super::TIMER1>;
#[doc = "Register TIMER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X`"]
pub type X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `X`"]
pub struct X_W<'a> {
    w: &'a mut W,
}
impl<'a> X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `Y`"]
pub type Y_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Y`"]
pub struct Y_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn x(&mut self) -> X_W {
        X_W { w: self }
    }
    #[doc = "Bits 0:15 - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn y(&mut self) -> Y_W {
        Y_W { w: self }
    }
}
