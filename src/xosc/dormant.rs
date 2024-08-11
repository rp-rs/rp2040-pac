#[doc = "Register `DORMANT` reader"]
pub type R = crate::R<DORMANT_SPEC>;
#[doc = "Register `DORMANT` writer"]
pub type W = crate::W<DORMANT_SPEC>;
#[doc = "This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DORMANT_A {
    #[doc = "1668246881: `1100011011011110110110101100001`"]
    DORMANT = 1668246881,
    #[doc = "2002873189: `1110111011000010110101101100101`"]
    WAKE = 2002873189,
}
impl From<DORMANT_A> for u32 {
    #[inline(always)]
    fn from(variant: DORMANT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DORMANT_A {
    type Ux = u32;
}
#[doc = "Field `DORMANT` reader - This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode"]
pub type DORMANT_R = crate::FieldReader<DORMANT_A>;
impl DORMANT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DORMANT_A> {
        match self.bits {
            1668246881 => Some(DORMANT_A::DORMANT),
            2002873189 => Some(DORMANT_A::WAKE),
            _ => None,
        }
    }
    #[doc = "`1100011011011110110110101100001`"]
    #[inline(always)]
    pub fn is_dormant(&self) -> bool {
        *self == DORMANT_A::DORMANT
    }
    #[doc = "`1110111011000010110101101100101`"]
    #[inline(always)]
    pub fn is_wake(&self) -> bool {
        *self == DORMANT_A::WAKE
    }
}
#[doc = "Field `DORMANT` writer - This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode"]
pub type DORMANT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, DORMANT_A>;
impl<'a, REG> DORMANT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "`1100011011011110110110101100001`"]
    #[inline(always)]
    pub fn dormant(self) -> &'a mut crate::W<REG> {
        self.variant(DORMANT_A::DORMANT)
    }
    #[doc = "`1110111011000010110101101100101`"]
    #[inline(always)]
    pub fn wake(self) -> &'a mut crate::W<REG> {
        self.variant(DORMANT_A::WAKE)
    }
}
impl R {
    #[doc = "Bits 0:31 - This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode"]
    #[inline(always)]
    pub fn dormant(&self) -> DORMANT_R {
        DORMANT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode"]
    #[inline(always)]
    #[must_use]
    pub fn dormant(&mut self) -> DORMANT_W<DORMANT_SPEC> {
        DORMANT_W::new(self, 0)
    }
}
#[doc = "Crystal Oscillator pause control  

You can [`read`](crate::generic::Reg::read) this register and get [`dormant::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dormant::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DORMANT_SPEC;
impl crate::RegisterSpec for DORMANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dormant::R`](R) reader structure"]
impl crate::Readable for DORMANT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dormant::W`](W) writer structure"]
impl crate::Writable for DORMANT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DORMANT to value 0"]
impl crate::Resettable for DORMANT_SPEC {
    const RESET_VALUE: u32 = 0;
}
