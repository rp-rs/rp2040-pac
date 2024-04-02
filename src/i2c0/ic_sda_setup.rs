#[doc = "Register `IC_SDA_SETUP` reader"]
pub type R = crate::R<IC_SDA_SETUP_SPEC>;
#[doc = "Register `IC_SDA_SETUP` writer"]
pub type W = crate::W<IC_SDA_SETUP_SPEC>;
#[doc = "Field `SDA_SETUP` reader - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
pub type SDA_SETUP_R = crate::FieldReader;
#[doc = "Field `SDA_SETUP` writer - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
pub type SDA_SETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    #[inline(always)]
    pub fn sda_setup(&self) -> SDA_SETUP_R {
        SDA_SETUP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    #[inline(always)]
    #[must_use]
    pub fn sda_setup(&mut self) -> SDA_SETUP_W<IC_SDA_SETUP_SPEC> {
        SDA_SETUP_W::new(self, 0)
    }
}
#[doc = "I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter.  

You can [`read`](crate::Reg::read) this register and get [`ic_sda_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_SDA_SETUP_SPEC;
impl crate::RegisterSpec for IC_SDA_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sda_setup::R`](R) reader structure"]
impl crate::Readable for IC_SDA_SETUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_sda_setup::W`](W) writer structure"]
impl crate::Writable for IC_SDA_SETUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_SDA_SETUP to value 0x64"]
impl crate::Resettable for IC_SDA_SETUP_SPEC {
    const RESET_VALUE: u32 = 0x64;
}
