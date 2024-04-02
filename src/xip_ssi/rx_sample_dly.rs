#[doc = "Register `RX_SAMPLE_DLY` reader"]
pub type R = crate::R<RX_SAMPLE_DLY_SPEC>;
#[doc = "Register `RX_SAMPLE_DLY` writer"]
pub type W = crate::W<RX_SAMPLE_DLY_SPEC>;
#[doc = "Field `RSD` reader - RXD sample delay (in SCLK cycles)"]
pub type RSD_R = crate::FieldReader;
#[doc = "Field `RSD` writer - RXD sample delay (in SCLK cycles)"]
pub type RSD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    pub fn rsd(&self) -> RSD_R {
        RSD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    #[must_use]
    pub fn rsd(&mut self) -> RSD_W<RX_SAMPLE_DLY_SPEC> {
        RSD_W::new(self, 0)
    }
}
#[doc = "RX sample delay  

You can [`read`](crate::Reg::read) this register and get [`rx_sample_dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_sample_dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_SAMPLE_DLY_SPEC;
impl crate::RegisterSpec for RX_SAMPLE_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_sample_dly::R`](R) reader structure"]
impl crate::Readable for RX_SAMPLE_DLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_sample_dly::W`](W) writer structure"]
impl crate::Writable for RX_SAMPLE_DLY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_SAMPLE_DLY to value 0"]
impl crate::Resettable for RX_SAMPLE_DLY_SPEC {
    const RESET_VALUE: u32 = 0;
}
