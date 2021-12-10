#[doc = "Register `STARTUP` reader"]
pub struct R(crate::R<STARTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTUP` writer"]
pub struct W(crate::W<STARTUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTUP_SPEC>;
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
impl From<crate::W<STARTUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X4` reader - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
pub struct X4_R(crate::FieldReader<bool, bool>);
impl X4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        X4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X4` writer - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
pub struct X4_W<'a> {
    w: &'a mut W,
}
impl<'a> X4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DELAY` reader - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
pub struct DELAY_R(crate::FieldReader<u16, u16>);
impl DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY` writer - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
    #[inline(always)]
    pub fn x4(&self) -> X4_R {
        X4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:13 - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
    #[inline(always)]
    pub fn x4(&mut self) -> X4_W {
        X4_W { w: self }
    }
    #[doc = "Bits 0:13 - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the startup delay  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [startup](index.html) module"]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startup::R](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startup::W](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTUP to value 0xc4"]
impl crate::Resettable for STARTUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc4
    }
}
