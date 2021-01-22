#[doc = "Reader of register DIV"]
pub type R = crate::R<u32, super::DIV>;
#[doc = "Writer for register DIV"]
pub type W = crate::W<u32, super::DIV>;
#[doc = "Register DIV `reset()`'s with value 0"]
impl crate::ResetValue for super::DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "set to 0xaa0 + div where\\n div = 0 divides by 32\\n div = 1-31 divides by div\\n any other value sets div=0 and therefore divides by 32\\n this register resets to div=16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DIV_A {
    #[doc = "2720: `101010100000`"]
    PASS = 2720,
}
impl From<DIV_A> for u16 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u16, DIV_A>;
impl DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, DIV_A> {
        use crate::Variant::*;
        match self.bits {
            2720 => Val(DIV_A::PASS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == DIV_A::PASS
    }
}
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`101010100000`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(DIV_A::PASS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - set to 0xaa0 + div where\\n div = 0 divides by 32\\n div = 1-31 divides by div\\n any other value sets div=0 and therefore divides by 32\\n this register resets to div=16"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - set to 0xaa0 + div where\\n div = 0 divides by 32\\n div = 1-31 divides by div\\n any other value sets div=0 and therefore divides by 32\\n this register resets to div=16"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
}
