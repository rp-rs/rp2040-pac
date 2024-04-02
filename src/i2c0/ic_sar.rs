#[doc = "Register `IC_SAR` reader"]
pub type R = crate::R<IC_SAR_SPEC>;
#[doc = "Register `IC_SAR` writer"]
pub type W = crate::W<IC_SAR_SPEC>;
#[doc = "Field `IC_SAR` reader - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to &lt;&lt;table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
pub type IC_SAR_R = crate::FieldReader<u16>;
#[doc = "Field `IC_SAR` writer - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to &lt;&lt;table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
pub type IC_SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to &lt;&lt;table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    #[inline(always)]
    pub fn ic_sar(&self) -> IC_SAR_R {
        IC_SAR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to &lt;&lt;table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    #[inline(always)]
    #[must_use]
    pub fn ic_sar(&mut self) -> IC_SAR_W<IC_SAR_SPEC> {
        IC_SAR_W::new(self, 0)
    }
}
#[doc = "I2C Slave Address Register  

You can [`read`](crate::Reg::read) this register and get [`ic_sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_SAR_SPEC;
impl crate::RegisterSpec for IC_SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sar::R`](R) reader structure"]
impl crate::Readable for IC_SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_sar::W`](W) writer structure"]
impl crate::Writable for IC_SAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_SAR to value 0x55"]
impl crate::Resettable for IC_SAR_SPEC {
    const RESET_VALUE: u32 = 0x55;
}
