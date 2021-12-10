#[doc = "Register `SM_INSTR` reader"]
pub struct R(crate::R<SM_INSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_INSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_INSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_INSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM_INSTR` writer"]
pub struct W(crate::W<SM_INSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM_INSTR_SPEC>;
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
impl From<crate::W<SM_INSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM_INSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM0_INSTR` reader - "]
pub struct SM0_INSTR_R(crate::FieldReader<u16, u16>);
impl SM0_INSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SM0_INSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_INSTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_INSTR` writer - "]
pub struct SM0_INSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_INSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sm0_instr(&self) -> SM0_INSTR_R {
        SM0_INSTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sm0_instr(&mut self) -> SM0_INSTR_W {
        SM0_INSTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter  
 Write to execute an instruction immediately (including jumps) and then resume execution.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sm_instr](index.html) module"]
pub struct SM_INSTR_SPEC;
impl crate::RegisterSpec for SM_INSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm_instr::R](R) reader structure"]
impl crate::Readable for SM_INSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm_instr::W](W) writer structure"]
impl crate::Writable for SM_INSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM_INSTR to value 0"]
impl crate::Resettable for SM_INSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
