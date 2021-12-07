#[doc = "Register `INTF1` reader"]
pub struct R(crate::R<INTF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTF1` writer"]
pub struct W(crate::W<INTF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTF1_SPEC>;
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
impl From<crate::W<INTF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTF1` reader - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
pub struct INTF1_R(crate::FieldReader<u16, u16>);
impl INTF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INTF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTF1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTF1` writer - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
pub struct INTF1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf1(&self) -> INTF1_R {
        INTF1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn intf1(&mut self) -> INTF1_W {
        INTF1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Interrupts for IRQ 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [intf1](index.html) module"]
pub struct INTF1_SPEC;
impl crate::RegisterSpec for INTF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf1::R](R) reader structure"]
impl crate::Readable for INTF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intf1::W](W) writer structure"]
impl crate::Writable for INTF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTF1 to value 0"]
impl crate::Resettable for INTF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
