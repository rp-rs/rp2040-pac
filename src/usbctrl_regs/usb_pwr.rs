#[doc = "Register `USB_PWR` reader"]
pub struct R(crate::R<USB_PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_PWR` writer"]
pub struct W(crate::W<USB_PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_PWR_SPEC>;
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
impl From<crate::W<USB_PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUS_EN` reader - "]
pub type VBUS_EN_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_EN` writer - "]
pub type VBUS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_PWR_SPEC, bool, O>;
#[doc = "Field `VBUS_EN_OVERRIDE_EN` reader - "]
pub type VBUS_EN_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_EN_OVERRIDE_EN` writer - "]
pub type VBUS_EN_OVERRIDE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_PWR_SPEC, bool, O>;
#[doc = "Field `VBUS_DETECT` reader - "]
pub type VBUS_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_DETECT` writer - "]
pub type VBUS_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_PWR_SPEC, bool, O>;
#[doc = "Field `VBUS_DETECT_OVERRIDE_EN` reader - "]
pub type VBUS_DETECT_OVERRIDE_EN_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_DETECT_OVERRIDE_EN` writer - "]
pub type VBUS_DETECT_OVERRIDE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_PWR_SPEC, bool, O>;
#[doc = "Field `OVERCURR_DETECT` reader - "]
pub type OVERCURR_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `OVERCURR_DETECT` writer - "]
pub type OVERCURR_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_PWR_SPEC, bool, O>;
#[doc = "Field `OVERCURR_DETECT_EN` reader - "]
pub type OVERCURR_DETECT_EN_R = crate::BitReader<bool>;
#[doc = "Field `OVERCURR_DETECT_EN` writer - "]
pub type OVERCURR_DETECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_PWR_SPEC, bool, O>;
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
    pub fn vbus_en(&mut self) -> VBUS_EN_W<0> {
        VBUS_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_en_override_en(&mut self) -> VBUS_EN_OVERRIDE_EN_W<1> {
        VBUS_EN_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_detect(&mut self) -> VBUS_DETECT_W<2> {
        VBUS_DETECT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_detect_override_en(&mut self) -> VBUS_DETECT_OVERRIDE_EN_W<3> {
        VBUS_DETECT_OVERRIDE_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn overcurr_detect(&mut self) -> OVERCURR_DETECT_W<4> {
        OVERCURR_DETECT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn overcurr_detect_en(&mut self) -> OVERCURR_DETECT_EN_W<5> {
        OVERCURR_DETECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usb_pwr](index.html) module"]
pub struct USB_PWR_SPEC;
impl crate::RegisterSpec for USB_PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_pwr::R](R) reader structure"]
impl crate::Readable for USB_PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_pwr::W](W) writer structure"]
impl crate::Writable for USB_PWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_PWR to value 0"]
impl crate::Resettable for USB_PWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
