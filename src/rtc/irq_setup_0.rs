#[doc = "Register `IRQ_SETUP_0` reader"]
pub type R = crate::R<IRQ_SETUP_0_SPEC>;
#[doc = "Register `IRQ_SETUP_0` writer"]
pub type W = crate::W<IRQ_SETUP_0_SPEC>;
#[doc = "Field `DAY` reader - Day of the month (1..31)"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `DAY` writer - Day of the month (1..31)"]
pub type DAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MONTH` reader - Month (1..12)"]
pub type MONTH_R = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month (1..12)"]
pub type MONTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader<u16>;
#[doc = "Field `YEAR` writer - Year"]
pub type YEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DAY_ENA` reader - Enable day matching"]
pub type DAY_ENA_R = crate::BitReader;
#[doc = "Field `DAY_ENA` writer - Enable day matching"]
pub type DAY_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONTH_ENA` reader - Enable month matching"]
pub type MONTH_ENA_R = crate::BitReader;
#[doc = "Field `MONTH_ENA` writer - Enable month matching"]
pub type MONTH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YEAR_ENA` reader - Enable year matching"]
pub type YEAR_ENA_R = crate::BitReader;
#[doc = "Field `YEAR_ENA` writer - Enable year matching"]
pub type YEAR_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_ENA` reader - Global match enable. Don't change any other value while this one is enabled"]
pub type MATCH_ENA_R = crate::BitReader;
#[doc = "Field `MATCH_ENA` writer - Global match enable. Don't change any other value while this one is enabled"]
pub type MATCH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_ACTIVE` reader - "]
pub type MATCH_ACTIVE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Enable day matching"]
    #[inline(always)]
    pub fn day_ena(&self) -> DAY_ENA_R {
        DAY_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable month matching"]
    #[inline(always)]
    pub fn month_ena(&self) -> MONTH_ENA_R {
        MONTH_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable year matching"]
    #[inline(always)]
    pub fn year_ena(&self) -> YEAR_ENA_R {
        YEAR_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Global match enable. Don't change any other value while this one is enabled"]
    #[inline(always)]
    pub fn match_ena(&self) -> MATCH_ENA_R {
        MATCH_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn match_active(&self) -> MATCH_ACTIVE_R {
        MATCH_ACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<IRQ_SETUP_0_SPEC> {
        DAY_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<IRQ_SETUP_0_SPEC> {
        MONTH_W::new(self, 8)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<IRQ_SETUP_0_SPEC> {
        YEAR_W::new(self, 12)
    }
    #[doc = "Bit 24 - Enable day matching"]
    #[inline(always)]
    #[must_use]
    pub fn day_ena(&mut self) -> DAY_ENA_W<IRQ_SETUP_0_SPEC> {
        DAY_ENA_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable month matching"]
    #[inline(always)]
    #[must_use]
    pub fn month_ena(&mut self) -> MONTH_ENA_W<IRQ_SETUP_0_SPEC> {
        MONTH_ENA_W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable year matching"]
    #[inline(always)]
    #[must_use]
    pub fn year_ena(&mut self) -> YEAR_ENA_W<IRQ_SETUP_0_SPEC> {
        YEAR_ENA_W::new(self, 26)
    }
    #[doc = "Bit 28 - Global match enable. Don't change any other value while this one is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn match_ena(&mut self) -> MATCH_ENA_W<IRQ_SETUP_0_SPEC> {
        MATCH_ENA_W::new(self, 28)
    }
}
#[doc = "Interrupt setup register 0  

You can [`read`](crate::Reg::read) this register and get [`irq_setup_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_setup_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_SETUP_0_SPEC;
impl crate::RegisterSpec for IRQ_SETUP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_setup_0::R`](R) reader structure"]
impl crate::Readable for IRQ_SETUP_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_setup_0::W`](W) writer structure"]
impl crate::Writable for IRQ_SETUP_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_SETUP_0 to value 0"]
impl crate::Resettable for IRQ_SETUP_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
