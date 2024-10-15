#[doc = "Register `INTE` reader"]
pub type R = crate::R<INTE_SPEC>;
#[doc = "Register `INTE` writer"]
pub type W = crate::W<INTE_SPEC>;
#[doc = "Field `ALARM_0` reader - "]
pub type ALARM_0_R = crate::BitReader;
#[doc = "Field `ALARM_0` writer - "]
pub type ALARM_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM_1` reader - "]
pub type ALARM_1_R = crate::BitReader;
#[doc = "Field `ALARM_1` writer - "]
pub type ALARM_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM_2` reader - "]
pub type ALARM_2_R = crate::BitReader;
#[doc = "Field `ALARM_2` writer - "]
pub type ALARM_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM_3` reader - "]
pub type ALARM_3_R = crate::BitReader;
#[doc = "Field `ALARM_3` writer - "]
pub type ALARM_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alarm_0(&self) -> ALARM_0_R {
        ALARM_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alarm_1(&self) -> ALARM_1_R {
        ALARM_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alarm_2(&self) -> ALARM_2_R {
        ALARM_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alarm_3(&self) -> ALARM_3_R {
        ALARM_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_0(&mut self) -> ALARM_0_W<INTE_SPEC> {
        ALARM_0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_1(&mut self) -> ALARM_1_W<INTE_SPEC> {
        ALARM_1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_2(&mut self) -> ALARM_2_W<INTE_SPEC> {
        ALARM_2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_3(&mut self) -> ALARM_3_W<INTE_SPEC> {
        ALARM_3_W::new(self, 3)
    }
}
#[doc = "Interrupt Enable  

You can [`read`](crate::Reg::read) this register and get [`inte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTE_SPEC;
impl crate::RegisterSpec for INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inte::R`](R) reader structure"]
impl crate::Readable for INTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inte::W`](W) writer structure"]
impl crate::Writable for INTE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTE to value 0"]
impl crate::Resettable for INTE_SPEC {
    const RESET_VALUE: u32 = 0;
}
