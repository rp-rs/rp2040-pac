#[doc = "Register `USB_MUXING` reader"]
pub struct R(crate::R<USB_MUXING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_MUXING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_MUXING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_MUXING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_MUXING` writer"]
pub struct W(crate::W<USB_MUXING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_MUXING_SPEC>;
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
impl From<crate::W<USB_MUXING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_MUXING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_PHY` reader - "]
pub type TO_PHY_R = crate::BitReader<bool>;
#[doc = "Field `TO_PHY` writer - "]
pub type TO_PHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_MUXING_SPEC, bool, O>;
#[doc = "Field `TO_EXTPHY` reader - "]
pub type TO_EXTPHY_R = crate::BitReader<bool>;
#[doc = "Field `TO_EXTPHY` writer - "]
pub type TO_EXTPHY_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_MUXING_SPEC, bool, O>;
#[doc = "Field `TO_DIGITAL_PAD` reader - "]
pub type TO_DIGITAL_PAD_R = crate::BitReader<bool>;
#[doc = "Field `TO_DIGITAL_PAD` writer - "]
pub type TO_DIGITAL_PAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_MUXING_SPEC, bool, O>;
#[doc = "Field `SOFTCON` reader - "]
pub type SOFTCON_R = crate::BitReader<bool>;
#[doc = "Field `SOFTCON` writer - "]
pub type SOFTCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_MUXING_SPEC, bool, O>;
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
    pub fn to_phy(&mut self) -> TO_PHY_W<0> {
        TO_PHY_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn to_extphy(&mut self) -> TO_EXTPHY_W<1> {
        TO_EXTPHY_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn to_digital_pad(&mut self) -> TO_DIGITAL_PAD_W<2> {
        TO_DIGITAL_PAD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn softcon(&mut self) -> SOFTCON_W<3> {
        SOFTCON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Where to connect the USB controller. Should be to_phy by default.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usb_muxing](index.html) module"]
pub struct USB_MUXING_SPEC;
impl crate::RegisterSpec for USB_MUXING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_muxing::R](R) reader structure"]
impl crate::Readable for USB_MUXING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_muxing::W](W) writer structure"]
impl crate::Writable for USB_MUXING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_MUXING to value 0"]
impl crate::Resettable for USB_MUXING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
