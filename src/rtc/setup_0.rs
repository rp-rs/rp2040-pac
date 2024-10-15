#[doc = "Register `SETUP_0` reader"]
pub type R = crate::R<SETUP_0_SPEC>;
#[doc = "Register `SETUP_0` writer"]
pub type W = crate::W<SETUP_0_SPEC>;
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
}
impl W {
    #[doc = "Bits 0:4 - Day of the month (1..31)"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<SETUP_0_SPEC> {
        DAY_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Month (1..12)"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<SETUP_0_SPEC> {
        MONTH_W::new(self, 8)
    }
    #[doc = "Bits 12:23 - Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<SETUP_0_SPEC> {
        YEAR_W::new(self, 12)
    }
}
#[doc = "RTC setup register 0  

You can [`read`](crate::Reg::read) this register and get [`setup_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SETUP_0_SPEC;
impl crate::RegisterSpec for SETUP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setup_0::R`](R) reader structure"]
impl crate::Readable for SETUP_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`setup_0::W`](W) writer structure"]
impl crate::Writable for SETUP_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETUP_0 to value 0"]
impl crate::Resettable for SETUP_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
