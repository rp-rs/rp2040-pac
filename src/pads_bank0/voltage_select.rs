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
#[doc = "Field `VOLTAGE_SELECT` reader - "]
pub type VOLTAGE_SELECT_R = crate::BitReader<VOLTAGE_SELECT_A>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl VOLTAGE_SELECT_R {
    #[doc = "Get enumerated values variant"]
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
        *self == VOLTAGE_SELECT_A::_3V3
    }
    #[doc = "Checks if the value of the field is `_1V8`"]
    #[inline(always)]
    pub fn is_1v8(&self) -> bool {
        *self == VOLTAGE_SELECT_A::_1V8
    }
}
#[doc = "Field `VOLTAGE_SELECT` writer - "]
pub type VOLTAGE_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VOLTAGE_SELECT_SPEC, VOLTAGE_SELECT_A, O>;
impl<'a, const O: u8> VOLTAGE_SELECT_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn voltage_select(&self) -> VOLTAGE_SELECT_R {
        VOLTAGE_SELECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn voltage_select(&mut self) -> VOLTAGE_SELECT_W<0> {
        VOLTAGE_SELECT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VOLTAGE_SELECT to value 0"]
impl crate::Resettable for VOLTAGE_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
