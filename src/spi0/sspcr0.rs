#[doc = "Register `SSPCR0` reader"]
pub struct R(crate::R<SSPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSPCR0` writer"]
pub struct W(crate::W<SSPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSPCR0_SPEC>;
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
impl From<crate::W<SSPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSPCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSS` reader - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
pub type DSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSS` writer - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
pub type DSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSPCR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FRF` reader - Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
pub type FRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRF` writer - Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
pub type FRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSPCR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPO` reader - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPO_R = crate::BitReader<bool>;
#[doc = "Field `SPO` writer - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSPCR0_SPEC, bool, O>;
#[doc = "Field `SPH` reader - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPH_R = crate::BitReader<bool>;
#[doc = "Field `SPH` writer - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSPCR0_SPEC, bool, O>;
#[doc = "Field `SCR` reader - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
pub type SCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCR` writer - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
pub type SCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSPCR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DSS_W<0> {
        DSS_W::new(self)
    }
    #[doc = "Bits 4:5 - Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SPO_W<6> {
        SPO_W::new(self)
    }
    #[doc = "Bit 7 - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SPH_W<7> {
        SPH_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<8> {
        SCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0, SSPCR0 on page 3-4  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sspcr0](index.html) module"]
pub struct SSPCR0_SPEC;
impl crate::RegisterSpec for SSPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sspcr0::R](R) reader structure"]
impl crate::Readable for SSPCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sspcr0::W](W) writer structure"]
impl crate::Writable for SSPCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSPCR0 to value 0"]
impl crate::Resettable for SSPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
