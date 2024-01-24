#[doc = "Register `IRQ_SETUP_1` reader"]
pub type R = crate::R<IRQ_SETUP_1_SPEC>;
#[doc = "Register `IRQ_SETUP_1` writer"]
pub type W = crate::W<IRQ_SETUP_1_SPEC>;
#[doc = "Field `SEC` reader - Seconds"]
pub type SEC_R = crate::FieldReader;
#[doc = "Field `SEC` writer - Seconds"]
pub type SEC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MIN` reader - Minutes"]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - Minutes"]
pub type MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HOUR` reader - Hours"]
pub type HOUR_R = crate::FieldReader;
#[doc = "Field `HOUR` writer - Hours"]
pub type HOUR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DOTW` reader - Day of the week"]
pub type DOTW_R = crate::FieldReader;
#[doc = "Field `DOTW` writer - Day of the week"]
pub type DOTW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEC_ENA` reader - Enable second matching"]
pub type SEC_ENA_R = crate::BitReader;
#[doc = "Field `SEC_ENA` writer - Enable second matching"]
pub type SEC_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIN_ENA` reader - Enable minute matching"]
pub type MIN_ENA_R = crate::BitReader;
#[doc = "Field `MIN_ENA` writer - Enable minute matching"]
pub type MIN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR_ENA` reader - Enable hour matching"]
pub type HOUR_ENA_R = crate::BitReader;
#[doc = "Field `HOUR_ENA` writer - Enable hour matching"]
pub type HOUR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOTW_ENA` reader - Enable day of the week matching"]
pub type DOTW_ENA_R = crate::BitReader;
#[doc = "Field `DOTW_ENA` writer - Enable day of the week matching"]
pub type DOTW_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn sec(&mut self) -> SEC_W<IRQ_SETUP_1_SPEC> {
        SEC_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<IRQ_SETUP_1_SPEC> {
        MIN_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<IRQ_SETUP_1_SPEC> {
        HOUR_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    #[must_use]
    pub fn dotw(&mut self) -> DOTW_W<IRQ_SETUP_1_SPEC> {
        DOTW_W::new(self, 24)
    }
    #[doc = "Bit 28 - Enable second matching"]
    #[inline(always)]
    #[must_use]
    pub fn sec_ena(&mut self) -> SEC_ENA_W<IRQ_SETUP_1_SPEC> {
        SEC_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable minute matching"]
    #[inline(always)]
    #[must_use]
    pub fn min_ena(&mut self) -> MIN_ENA_W<IRQ_SETUP_1_SPEC> {
        MIN_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable hour matching"]
    #[inline(always)]
    #[must_use]
    pub fn hour_ena(&mut self) -> HOUR_ENA_W<IRQ_SETUP_1_SPEC> {
        HOUR_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable day of the week matching"]
    #[inline(always)]
    #[must_use]
    pub fn dotw_ena(&mut self) -> DOTW_ENA_W<IRQ_SETUP_1_SPEC> {
        DOTW_ENA_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt setup register 1  

You can [`read`](crate::generic::Reg::read) this register and get [`irq_setup_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_setup_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_SETUP_1_SPEC;
impl crate::RegisterSpec for IRQ_SETUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_setup_1::R`](R) reader structure"]
impl crate::Readable for IRQ_SETUP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_setup_1::W`](W) writer structure"]
impl crate::Writable for IRQ_SETUP_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_SETUP_1 to value 0"]
impl crate::Resettable for IRQ_SETUP_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
