#[doc = "Register `VOLTAGE_SELECT` reader"]
pub struct R(crate::R<VOLTAGE_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOLTAGE_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOLTAGE_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOLTAGE_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOLTAGE_SELECT` writer"]
pub struct W(crate::W<VOLTAGE_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOLTAGE_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<VOLTAGE_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOLTAGE_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_SELECT_A {
    #[doc = "0: Set voltage to 3.3V (DVDD >= 2V5)"]
    _3V3 = 0,
    #[doc = "1: Set voltage to 1.8V (DVDD <= 1V8)"]
    _1V8 = 1,
}
impl From<VOLTAGE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE_SELECT` reader - "]
pub struct VOLTAGE_SELECT_R(crate::FieldReader<bool, VOLTAGE_SELECT_A>);
impl VOLTAGE_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VOLTAGE_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOLTAGE_SELECT_A {
        match self.bits {
            false => VOLTAGE_SELECT_A::_3V3,
            true => VOLTAGE_SELECT_A::_1V8,
        }
    }
    #[doc = "Checks if the value of the field is `_3V3`"]
    #[inline(always)]
    pub fn is_3v3(&self) -> bool {
        **self == VOLTAGE_SELECT_A::_3V3
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        **self == VOLTAGE_SELECT_A::_1V8
    }
}
impl core::ops::Deref for VOLTAGE_SELECT_R {
    type Target = crate::FieldReader<bool, VOLTAGE_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOLTAGE_SELECT` writer - "]
pub struct VOLTAGE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLTAGE_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOLTAGE_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set voltage to 3.3V (DVDD >= 2V5)"]
    #[inline(always)]
    pub fn _3v3(self) -> &'a mut W {
        self.variant(VOLTAGE_SELECT_A::_3V3)
    }
    #[doc = "Set voltage to 1.8V (DVDD <= 1V8)"]
    #[inline(always)]
    pub fn _1v8(self) -> &'a mut W {
        self.variant(VOLTAGE_SELECT_A::_1V8)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn voltage_select(&self) -> VOLTAGE_SELECT_R {
        VOLTAGE_SELECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn voltage_select(&mut self) -> VOLTAGE_SELECT_W {
        VOLTAGE_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage select. Per bank control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [voltage_select](index.html) module"]
pub struct VOLTAGE_SELECT_SPEC;
impl crate::RegisterSpec for VOLTAGE_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voltage_select::R](R) reader structure"]
impl crate::Readable for VOLTAGE_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voltage_select::W](W) writer structure"]
impl crate::Writable for VOLTAGE_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOLTAGE_SELECT to value 0"]
impl crate::Resettable for VOLTAGE_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
