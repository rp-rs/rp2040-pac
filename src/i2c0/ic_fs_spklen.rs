#[doc = "Register `IC_FS_SPKLEN` reader"]
pub type R = crate::R<IC_FS_SPKLEN_SPEC>;
#[doc = "Register `IC_FS_SPKLEN` writer"]
pub type W = crate::W<IC_FS_SPKLEN_SPEC>;
#[doc = "Field `IC_FS_SPKLEN` reader - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
pub type IC_FS_SPKLEN_R = crate::FieldReader;
#[doc = "Field `IC_FS_SPKLEN` writer - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
pub type IC_FS_SPKLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
    #[inline(always)]
    pub fn ic_fs_spklen(&self) -> IC_FS_SPKLEN_R {
        IC_FS_SPKLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
    #[inline(always)]
    #[must_use]
    pub fn ic_fs_spklen(&mut self) -> IC_FS_SPKLEN_W<IC_FS_SPKLEN_SPEC> {
        IC_FS_SPKLEN_W::new(self, 0)
    }
}
#[doc = "I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1.  

You can [`read`](crate::Reg::read) this register and get [`ic_fs_spklen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_spklen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_FS_SPKLEN_SPEC;
impl crate::RegisterSpec for IC_FS_SPKLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_fs_spklen::R`](R) reader structure"]
impl crate::Readable for IC_FS_SPKLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_fs_spklen::W`](W) writer structure"]
impl crate::Writable for IC_FS_SPKLEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_FS_SPKLEN to value 0x07"]
impl crate::Resettable for IC_FS_SPKLEN_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
