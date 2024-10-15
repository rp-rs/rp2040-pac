#[doc = "Register `IC_SS_SCL_LCNT` reader"]
pub type R = crate::R<IC_SS_SCL_LCNT_SPEC>;
#[doc = "Register `IC_SS_SCL_LCNT` writer"]
pub type W = crate::W<IC_SS_SCL_LCNT_SPEC>;
#[doc = "Field `IC_SS_SCL_LCNT` reader - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
pub type IC_SS_SCL_LCNT_R = crate::FieldReader<u16>;
#[doc = "Field `IC_SS_SCL_LCNT` writer - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
pub type IC_SS_SCL_LCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&self) -> IC_SS_SCL_LCNT_R {
        IC_SS_SCL_LCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ic_ss_scl_lcnt(&mut self) -> IC_SS_SCL_LCNT_W<IC_SS_SCL_LCNT_SPEC> {
        IC_SS_SCL_LCNT_W::new(self, 0)
    }
}
#[doc = "Standard Speed I2C Clock SCL Low Count Register  

You can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_SS_SCL_LCNT_SPEC;
impl crate::RegisterSpec for IC_SS_SCL_LCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_ss_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for IC_SS_SCL_LCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_ss_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for IC_SS_SCL_LCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_SS_SCL_LCNT to value 0x2f"]
impl crate::Resettable for IC_SS_SCL_LCNT_SPEC {
    const RESET_VALUE: u32 = 0x2f;
}
