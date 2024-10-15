#[doc = "Register `SSPIMSC` reader"]
pub type R = crate::R<SSPIMSC_SPEC>;
#[doc = "Register `SSPIMSC` writer"]
pub type W = crate::W<SSPIMSC_SPEC>;
#[doc = "Field `RORIM` reader - Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
pub type RORIM_R = crate::BitReader;
#[doc = "Field `RORIM` writer - Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
pub type RORIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
pub type RTIM_R = crate::BitReader;
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
pub type RTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
pub type RXIM_R = crate::BitReader;
#[doc = "Field `RXIM` writer - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
pub type RXIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_R = crate::BitReader;
#[doc = "Field `TXIM` writer - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
pub type TXIM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn rorim(&mut self) -> RORIM_W<SSPIMSC_SPEC> {
        RORIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RTIM_W<SSPIMSC_SPEC> {
        RTIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RXIM_W<SSPIMSC_SPEC> {
        RXIM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TXIM_W<SSPIMSC_SPEC> {
        TXIM_W::new(self, 3)
    }
}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9  

You can [`read`](crate::Reg::read) this register and get [`sspimsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspimsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPIMSC_SPEC;
impl crate::RegisterSpec for SSPIMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspimsc::R`](R) reader structure"]
impl crate::Readable for SSPIMSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspimsc::W`](W) writer structure"]
impl crate::Writable for SSPIMSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPIMSC to value 0"]
impl crate::Resettable for SSPIMSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
