#[doc = "Register `SSPCR0` reader"]
pub type R = crate::R<SSPCR0_SPEC>;
#[doc = "Register `SSPCR0` writer"]
pub type W = crate::W<SSPCR0_SPEC>;
#[doc = "Field `DSS` reader - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
pub type DSS_R = crate::FieldReader;
#[doc = "Field `DSS` writer - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
pub type DSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Frame format.  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRF_A {
    #[doc = "0: Motorola SPI frame format"]
    MOTOROLA = 0,
    #[doc = "1: Texas Instruments synchronous serial frame format"]
    TEXAS_INSTRUMENTS = 1,
    #[doc = "2: National Semiconductor Microwire frame format"]
    NATIONAL_SEMICONDUCTOR_MICROWIRE = 2,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRF_A {
    type Ux = u8;
}
impl crate::IsEnum for FRF_A {}
#[doc = "Field `FRF` reader - Frame format."]
pub type FRF_R = crate::FieldReader<FRF_A>;
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FRF_A> {
        match self.bits {
            0 => Some(FRF_A::MOTOROLA),
            1 => Some(FRF_A::TEXAS_INSTRUMENTS),
            2 => Some(FRF_A::NATIONAL_SEMICONDUCTOR_MICROWIRE),
            _ => None,
        }
    }
    #[doc = "Motorola SPI frame format"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF_A::MOTOROLA
    }
    #[doc = "Texas Instruments synchronous serial frame format"]
    #[inline(always)]
    pub fn is_texas_instruments(&self) -> bool {
        *self == FRF_A::TEXAS_INSTRUMENTS
    }
    #[doc = "National Semiconductor Microwire frame format"]
    #[inline(always)]
    pub fn is_national_semiconductor_microwire(&self) -> bool {
        *self == FRF_A::NATIONAL_SEMICONDUCTOR_MICROWIRE
    }
}
#[doc = "Field `FRF` writer - Frame format."]
pub type FRF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FRF_A>;
impl<'a, REG> FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Motorola SPI frame format"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::MOTOROLA)
    }
    #[doc = "Texas Instruments synchronous serial frame format"]
    #[inline(always)]
    pub fn texas_instruments(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::TEXAS_INSTRUMENTS)
    }
    #[doc = "National Semiconductor Microwire frame format"]
    #[inline(always)]
    pub fn national_semiconductor_microwire(self) -> &'a mut crate::W<REG> {
        self.variant(FRF_A::NATIONAL_SEMICONDUCTOR_MICROWIRE)
    }
}
#[doc = "Field `SPO` reader - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPO_R = crate::BitReader;
#[doc = "Field `SPO` writer - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPH` reader - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPH_R = crate::BitReader;
#[doc = "Field `SPH` writer - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
pub type SPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
pub type SCR_R = crate::FieldReader;
#[doc = "Field `SCR` writer - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
pub type SCR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame format."]
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
    pub fn dss(&mut self) -> DSS_W<SSPCR0_SPEC> {
        DSS_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Frame format."]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<SSPCR0_SPEC> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 6 - SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SPO_W<SSPCR0_SPEC> {
        SPO_W::new(self, 6)
    }
    #[doc = "Bit 7 - SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SPH_W<SSPCR0_SPEC> {
        SPH_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<SSPCR0_SPEC> {
        SCR_W::new(self, 8)
    }
}
#[doc = "Control register 0, SSPCR0 on page 3-4  

You can [`read`](crate::Reg::read) this register and get [`sspcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPCR0_SPEC;
impl crate::RegisterSpec for SSPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspcr0::R`](R) reader structure"]
impl crate::Readable for SSPCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspcr0::W`](W) writer structure"]
impl crate::Writable for SSPCR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPCR0 to value 0"]
impl crate::Resettable for SSPCR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
