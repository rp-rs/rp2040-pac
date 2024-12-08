#[doc = "Register `GPIO_STATUS` reader"]
pub type R = crate::R<GPIO_STATUS_SPEC>;
#[doc = "Register `GPIO_STATUS` writer"]
pub type W = crate::W<GPIO_STATUS_SPEC>;
#[doc = "Field `OUTFROMPERI` reader - output signal from selected peripheral, before register override is applied"]
pub type OUTFROMPERI_R = crate::BitReader;
#[doc = "Field `OUTTOPAD` reader - output signal to pad after register override is applied"]
pub type OUTTOPAD_R = crate::BitReader;
#[doc = "Field `OEFROMPERI` reader - output enable from selected peripheral, before register override is applied"]
pub type OEFROMPERI_R = crate::BitReader;
#[doc = "Field `OETOPAD` reader - output enable to pad after register override is applied"]
pub type OETOPAD_R = crate::BitReader;
#[doc = "Field `INFROMPAD` reader - input signal from pad, before override is applied"]
pub type INFROMPAD_R = crate::BitReader;
#[doc = "Field `INTOPERI` reader - input signal to peripheral, after override is applied"]
pub type INTOPERI_R = crate::BitReader;
#[doc = "Field `IRQFROMPAD` reader - interrupt from pad before override is applied"]
pub type IRQFROMPAD_R = crate::BitReader;
#[doc = "Field `IRQTOPROC` reader - interrupt to processors, after override is applied"]
pub type IRQTOPROC_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - output signal from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub fn outfromperi(&self) -> OUTFROMPERI_R {
        OUTFROMPERI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output signal to pad after register override is applied"]
    #[inline(always)]
    pub fn outtopad(&self) -> OUTTOPAD_R {
        OUTTOPAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - output enable from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub fn oefromperi(&self) -> OEFROMPERI_R {
        OEFROMPERI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - output enable to pad after register override is applied"]
    #[inline(always)]
    pub fn oetopad(&self) -> OETOPAD_R {
        OETOPAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - input signal from pad, before override is applied"]
    #[inline(always)]
    pub fn infrompad(&self) -> INFROMPAD_R {
        INFROMPAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - input signal to peripheral, after override is applied"]
    #[inline(always)]
    pub fn intoperi(&self) -> INTOPERI_R {
        INTOPERI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - interrupt from pad before override is applied"]
    #[inline(always)]
    pub fn irqfrompad(&self) -> IRQFROMPAD_R {
        IRQFROMPAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - interrupt to processors, after override is applied"]
    #[inline(always)]
    pub fn irqtoproc(&self) -> IRQTOPROC_R {
        IRQTOPROC_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {}
#[doc = "GPIO status  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_STATUS_SPEC;
impl crate::RegisterSpec for GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_status::R`](R) reader structure"]
impl crate::Readable for GPIO_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_status::W`](W) writer structure"]
impl crate::Writable for GPIO_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_STATUS to value 0"]
impl crate::Resettable for GPIO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
