#[doc = "Register `USB_PWR` reader"]
pub type R = crate::R<USB_PWR_SPEC>;
#[doc = "Register `USB_PWR` writer"]
pub type W = crate::W<USB_PWR_SPEC>;
#[doc = "Field `VBUS_EN` reader - "]
pub type VBUS_EN_R = crate::BitReader;
#[doc = "Field `VBUS_EN` writer - "]
pub type VBUS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_EN_OVERRIDE_EN` reader - "]
pub type VBUS_EN_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `VBUS_EN_OVERRIDE_EN` writer - "]
pub type VBUS_EN_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_DETECT` reader - "]
pub type VBUS_DETECT_R = crate::BitReader;
#[doc = "Field `VBUS_DETECT` writer - "]
pub type VBUS_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_DETECT_OVERRIDE_EN` reader - "]
pub type VBUS_DETECT_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `VBUS_DETECT_OVERRIDE_EN` writer - "]
pub type VBUS_DETECT_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERCURR_DETECT` reader - "]
pub type OVERCURR_DETECT_R = crate::BitReader;
#[doc = "Field `OVERCURR_DETECT` writer - "]
pub type OVERCURR_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERCURR_DETECT_EN` reader - "]
pub type OVERCURR_DETECT_EN_R = crate::BitReader;
#[doc = "Field `OVERCURR_DETECT_EN` writer - "]
pub type OVERCURR_DETECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbus_en(&self) -> VBUS_EN_R {
        VBUS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbus_en_override_en(&self) -> VBUS_EN_OVERRIDE_EN_R {
        VBUS_EN_OVERRIDE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_detect(&self) -> VBUS_DETECT_R {
        VBUS_DETECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbus_detect_override_en(&self) -> VBUS_DETECT_OVERRIDE_EN_R {
        VBUS_DETECT_OVERRIDE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn overcurr_detect(&self) -> OVERCURR_DETECT_R {
        OVERCURR_DETECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn overcurr_detect_en(&self) -> OVERCURR_DETECT_EN_R {
        OVERCURR_DETECT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_en(&mut self) -> VBUS_EN_W<USB_PWR_SPEC> {
        VBUS_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_en_override_en(&mut self) -> VBUS_EN_OVERRIDE_EN_W<USB_PWR_SPEC> {
        VBUS_EN_OVERRIDE_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_detect(&mut self) -> VBUS_DETECT_W<USB_PWR_SPEC> {
        VBUS_DETECT_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_detect_override_en(&mut self) -> VBUS_DETECT_OVERRIDE_EN_W<USB_PWR_SPEC> {
        VBUS_DETECT_OVERRIDE_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn overcurr_detect(&mut self) -> OVERCURR_DETECT_W<USB_PWR_SPEC> {
        OVERCURR_DETECT_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn overcurr_detect_en(&mut self) -> OVERCURR_DETECT_EN_W<USB_PWR_SPEC> {
        OVERCURR_DETECT_EN_W::new(self, 5)
    }
}
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value.  

You can [`read`](crate::Reg::read) this register and get [`usb_pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_PWR_SPEC;
impl crate::RegisterSpec for USB_PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_pwr::R`](R) reader structure"]
impl crate::Readable for USB_PWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_pwr::W`](W) writer structure"]
impl crate::Writable for USB_PWR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_PWR to value 0"]
impl crate::Resettable for USB_PWR_SPEC {
    const RESET_VALUE: u32 = 0;
}
