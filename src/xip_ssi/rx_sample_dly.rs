#[doc = "Register `RX_SAMPLE_DLY` reader"]
pub type R = crate::R<RX_SAMPLE_DLY_SPEC>;
#[doc = "Register `RX_SAMPLE_DLY` writer"]
pub type W = crate::W<RX_SAMPLE_DLY_SPEC>;
#[doc = "Field `RSD` reader - RXD sample delay (in SCLK cycles)"]
pub type RSD_R = crate::FieldReader;
#[doc = "Field `RSD` writer - RXD sample delay (in SCLK cycles)"]
pub type RSD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn rsd(&mut self) -> RSD_W<RX_SAMPLE_DLY_SPEC, 0> {
        RSD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RX sample delay  

You can [`read`](crate::generic::Reg::read) this register and get [`rx_sample_dly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_sample_dly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_SAMPLE_DLY_SPEC;
impl crate::RegisterSpec for RX_SAMPLE_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_sample_dly::R`](R) reader structure"]
impl crate::Readable for RX_SAMPLE_DLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_sample_dly::W`](W) writer structure"]
impl crate::Writable for RX_SAMPLE_DLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_SAMPLE_DLY to value 0"]
impl crate::Resettable for RX_SAMPLE_DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
