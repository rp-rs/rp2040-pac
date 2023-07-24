#[doc = "Register `INTE1` reader"]
pub struct R(crate::R<INTE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTE1` writer"]
pub struct W(crate::W<INTE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTE1_SPEC>;
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
impl From<crate::W<INTE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTE1` reader - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
pub type INTE1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTE1` writer - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
pub type INTE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTE1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    pub fn inte1(&self) -> INTE1_R {
        INTE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    #[inline(always)]
    #[must_use]
    pub fn inte1(&mut self) -> INTE1_W<0> {
        INTE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enables for IRQ 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [inte1](index.html) module"]
pub struct INTE1_SPEC;
impl crate::RegisterSpec for INTE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inte1::R](R) reader structure"]
impl crate::Readable for INTE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inte1::W](W) writer structure"]
impl crate::Writable for INTE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTE1 to value 0"]
impl crate::Resettable for INTE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
