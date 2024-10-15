#[doc = "Register `USB_MUXING` reader"]
pub type R = crate::R<USB_MUXING_SPEC>;
#[doc = "Register `USB_MUXING` writer"]
pub type W = crate::W<USB_MUXING_SPEC>;
#[doc = "Field `TO_PHY` reader - "]
pub type TO_PHY_R = crate::BitReader;
#[doc = "Field `TO_PHY` writer - "]
pub type TO_PHY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_EXTPHY` reader - "]
pub type TO_EXTPHY_R = crate::BitReader;
#[doc = "Field `TO_EXTPHY` writer - "]
pub type TO_EXTPHY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_DIGITAL_PAD` reader - "]
pub type TO_DIGITAL_PAD_R = crate::BitReader;
#[doc = "Field `TO_DIGITAL_PAD` writer - "]
pub type TO_DIGITAL_PAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTCON` reader - "]
pub type SOFTCON_R = crate::BitReader;
#[doc = "Field `SOFTCON` writer - "]
pub type SOFTCON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn to_phy(&self) -> TO_PHY_R {
        TO_PHY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn to_extphy(&self) -> TO_EXTPHY_R {
        TO_EXTPHY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn to_digital_pad(&self) -> TO_DIGITAL_PAD_R {
        TO_DIGITAL_PAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn softcon(&self) -> SOFTCON_R {
        SOFTCON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn to_phy(&mut self) -> TO_PHY_W<USB_MUXING_SPEC> {
        TO_PHY_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn to_extphy(&mut self) -> TO_EXTPHY_W<USB_MUXING_SPEC> {
        TO_EXTPHY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn to_digital_pad(&mut self) -> TO_DIGITAL_PAD_W<USB_MUXING_SPEC> {
        TO_DIGITAL_PAD_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn softcon(&mut self) -> SOFTCON_W<USB_MUXING_SPEC> {
        SOFTCON_W::new(self, 3)
    }
}
#[doc = "Where to connect the USB controller. Should be to_phy by default.  

You can [`read`](crate::Reg::read) this register and get [`usb_muxing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_muxing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_MUXING_SPEC;
impl crate::RegisterSpec for USB_MUXING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_muxing::R`](R) reader structure"]
impl crate::Readable for USB_MUXING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_muxing::W`](W) writer structure"]
impl crate::Writable for USB_MUXING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_MUXING to value 0"]
impl crate::Resettable for USB_MUXING_SPEC {
    const RESET_VALUE: u32 = 0;
}
