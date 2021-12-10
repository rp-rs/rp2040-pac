#[doc = "Register `TIMER3` reader"]
pub struct R(crate::R<TIMER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER3` writer"]
pub struct W(crate::W<TIMER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER3_SPEC>;
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
impl From<crate::W<TIMER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X` reader - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
pub struct X_R(crate::FieldReader<u16, u16>);
impl X_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X` writer - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
pub struct X_W<'a> {
    w: &'a mut W,
}
impl<'a> X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `Y` reader - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
pub struct Y_R(crate::FieldReader<u16, u16>);
impl Y_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        Y_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Y` writer - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
pub struct Y_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timer3](index.html) module"]
pub struct TIMER3_SPEC;
impl crate::RegisterSpec for TIMER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer3::R](R) reader structure"]
impl crate::Readable for TIMER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer3::W](W) writer structure"]
impl crate::Writable for TIMER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER3 to value 0"]
impl crate::Resettable for TIMER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
