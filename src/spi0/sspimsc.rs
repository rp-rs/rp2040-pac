#[doc = "Register `SSPIMSC` reader"]
pub struct R(crate::R<SSPIMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPIMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPIMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPIMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPIMSC` writer"]
pub struct W(crate::W<SSPIMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPIMSC_SPEC>;
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
impl From<crate::W<SSPIMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPIMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RORIM` reader - Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
pub type RORIM_R = crate::BitReader<bool>;
#[doc = "Field `RORIM` writer - Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
pub type RORIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSPIMSC_SPEC, bool, O>;
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
pub type RTIM_R = crate::BitReader<bool>;
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
pub type RTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSPIMSC_SPEC, bool, O>;
#[doc = "Field `RXIM` reader - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
pub type RXIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIM` writer - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
pub type RXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSPIMSC_SPEC, bool, O>;
#[doc = "Field `TXIM` reader - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_R = crate::BitReader<bool>;
#[doc = "Field `TXIM` writer - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSPIMSC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RORIM_W<0> {
        RORIM_W::new(self)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<1> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<2> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<3> {
        TXIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspimsc](index.html) module"]
pub struct SSPIMSC_SPEC;
impl crate::RegisterSpec for SSPIMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspimsc::R](R) reader structure"]
impl crate::Readable for SSPIMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspimsc::W](W) writer structure"]
impl crate::Writable for SSPIMSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPIMSC to value 0"]
impl crate::Resettable for SSPIMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
