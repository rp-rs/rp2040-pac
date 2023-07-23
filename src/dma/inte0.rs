#[doc = "Register `INTE0` reader"]
pub struct R(crate::R<INTE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTE0` writer"]
pub struct W(crate::W<INTE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTE0_SPEC>;
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
impl From<crate::W<INTE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTE0` reader - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
pub type INTE0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTE0` writer - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
pub type INTE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTE0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    #[inline(always)]
    pub fn inte0(&self) -> INTE0_R {
        INTE0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    #[inline(always)]
    #[must_use]
    pub fn inte0(&mut self) -> INTE0_W<0> {
        INTE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enables for IRQ 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [inte0](index.html) module"]
pub struct INTE0_SPEC;
impl crate::RegisterSpec for INTE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inte0::R](R) reader structure"]
impl crate::Readable for INTE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inte0::W](W) writer structure"]
impl crate::Writable for INTE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTE0 to value 0"]
impl crate::Resettable for INTE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
