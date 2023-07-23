#[doc = "Register `IRQ_SETUP_1` reader"]
pub struct R(crate::R<IRQ_SETUP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_SETUP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_SETUP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_SETUP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_SETUP_1` writer"]
pub struct W(crate::W<IRQ_SETUP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_SETUP_1_SPEC>;
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
impl From<crate::W<IRQ_SETUP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_SETUP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - Seconds"]
pub type SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC` writer - Seconds"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_SETUP_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `MIN` reader - Minutes"]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - Minutes"]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_SETUP_1_SPEC, u8, u8, 6, O>;
#[doc = "Field `HOUR` reader - Hours"]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOUR` writer - Hours"]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_SETUP_1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DOTW` reader - Day of the week"]
pub type DOTW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOTW` writer - Day of the week"]
pub type DOTW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_SETUP_1_SPEC, u8, u8, 3, O>;
#[doc = "Field `SEC_ENA` reader - Enable second matching"]
pub type SEC_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SEC_ENA` writer - Enable second matching"]
pub type SEC_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_SETUP_1_SPEC, bool, O>;
#[doc = "Field `MIN_ENA` reader - Enable minute matching"]
pub type MIN_ENA_R = crate::BitReader<bool>;
#[doc = "Field `MIN_ENA` writer - Enable minute matching"]
pub type MIN_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_SETUP_1_SPEC, bool, O>;
#[doc = "Field `HOUR_ENA` reader - Enable hour matching"]
pub type HOUR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `HOUR_ENA` writer - Enable hour matching"]
pub type HOUR_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_SETUP_1_SPEC, bool, O>;
#[doc = "Field `DOTW_ENA` reader - Enable day of the week matching"]
pub type DOTW_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DOTW_ENA` writer - Enable day of the week matching"]
pub type DOTW_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_SETUP_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    pub fn sec_ena(&self) -> SEC_ENA_R {
        SEC_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    pub fn min_ena(&self) -> MIN_ENA_R {
        MIN_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    pub fn hour_ena(&self) -> HOUR_ENA_R {
        HOUR_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    pub fn dotw_ena(&self) -> DOTW_ENA_R {
        DOTW_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<8> {
        MIN_W::new(self)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<16> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    #[must_use]
    pub fn dotw(&mut self) -> DOTW_W<24> {
        DOTW_W::new(self)
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    #[must_use]
    pub fn sec_ena(&mut self) -> SEC_ENA_W<28> {
        SEC_ENA_W::new(self)
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    #[must_use]
    pub fn min_ena(&mut self) -> MIN_ENA_W<29> {
        MIN_ENA_W::new(self)
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    #[must_use]
    pub fn hour_ena(&mut self) -> HOUR_ENA_W<30> {
        HOUR_ENA_W::new(self)
    }
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    #[must_use]
    pub fn dotw_ena(&mut self) -> DOTW_ENA_W<31> {
        DOTW_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt setup register 1  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [irq_setup_1](index.html) module"]
pub struct IRQ_SETUP_1_SPEC;
impl crate::RegisterSpec for IRQ_SETUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_setup_1::R](R) reader structure"]
impl crate::Readable for IRQ_SETUP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_setup_1::W](W) writer structure"]
impl crate::Writable for IRQ_SETUP_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_SETUP_1 to value 0"]
impl crate::Resettable for IRQ_SETUP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
