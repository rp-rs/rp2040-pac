#[doc = "Register `IC_FS_SPKLEN` reader"]
pub struct R(crate::R<IC_FS_SPKLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_FS_SPKLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_FS_SPKLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_FS_SPKLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_FS_SPKLEN` writer"]
pub struct W(crate::W<IC_FS_SPKLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_FS_SPKLEN_SPEC>;
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
impl From<crate::W<IC_FS_SPKLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_FS_SPKLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_FS_SPKLEN` reader - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
pub struct IC_FS_SPKLEN_R(crate::FieldReader<u8, u8>);
impl IC_FS_SPKLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IC_FS_SPKLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_FS_SPKLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_FS_SPKLEN` writer - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
pub struct IC_FS_SPKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_FS_SPKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn ic_fs_spklen(&mut self) -> IC_FS_SPKLEN_W {
        IC_FS_SPKLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SS, FS or FM+ spike suppression limit  

 This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_fs_spklen](index.html) module"]
pub struct IC_FS_SPKLEN_SPEC;
impl crate::RegisterSpec for IC_FS_SPKLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_fs_spklen::R](R) reader structure"]
impl crate::Readable for IC_FS_SPKLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_fs_spklen::W](W) writer structure"]
impl crate::Writable for IC_FS_SPKLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_FS_SPKLEN to value 0x07"]
impl crate::Resettable for IC_FS_SPKLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
