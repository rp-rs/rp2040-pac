#[doc = "Register `PWR` reader"]
pub type R = crate::R<PWR_SPEC>;
#[doc = "Register `PWR` writer"]
pub type W = crate::W<PWR_SPEC>;
#[doc = "Field `PD` reader - PLL powerdown  
 To save power set high when PLL output not required."]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - PLL powerdown  
 To save power set high when PLL output not required."]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSMPD` reader - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
pub type DSMPD_R = crate::BitReader;
#[doc = "Field `DSMPD` writer - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
pub type DSMPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSTDIVPD` reader - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type POSTDIVPD_R = crate::BitReader;
#[doc = "Field `POSTDIVPD` writer - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type POSTDIVPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCOPD` reader - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type VCOPD_R = crate::BitReader;
#[doc = "Field `VCOPD` writer - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type VCOPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL powerdown  
 To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn dsmpd(&self) -> DSMPD_R {
        DSMPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn postdivpd(&self) -> POSTDIVPD_R {
        POSTDIVPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn vcopd(&self) -> VCOPD_R {
        VCOPD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL powerdown  
 To save power set high when PLL output not required."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<PWR_SPEC> {
        PD_W::new(self, 0)
    }
    #[doc = "Bit 2 - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
    #[inline(always)]
    #[must_use]
    pub fn dsmpd(&mut self) -> DSMPD_W<PWR_SPEC> {
        DSMPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    #[must_use]
    pub fn postdivpd(&mut self) -> POSTDIVPD_W<PWR_SPEC> {
        POSTDIVPD_W::new(self, 3)
    }
    #[doc = "Bit 5 - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    #[must_use]
    pub fn vcopd(&mut self) -> VCOPD_W<PWR_SPEC> {
        VCOPD_W::new(self, 5)
    }
}
#[doc = "Controls the PLL power modes.  

You can [`read`](crate::Reg::read) this register and get [`pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_SPEC;
impl crate::RegisterSpec for PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr::R`](R) reader structure"]
impl crate::Readable for PWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr::W`](W) writer structure"]
impl crate::Writable for PWR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR to value 0x2d"]
impl crate::Resettable for PWR_SPEC {
    const RESET_VALUE: u32 = 0x2d;
}
