#[doc = "Register `IC_SS_SCL_LCNT` reader"]
pub struct R(crate::R<IC_SS_SCL_LCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SS_SCL_LCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SS_SCL_LCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SS_SCL_LCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_SS_SCL_LCNT` writer"]
pub struct W(crate::W<IC_SS_SCL_LCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SS_SCL_LCNT_SPEC>;
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
impl From<crate::W<IC_SS_SCL_LCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SS_SCL_LCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_SS_SCL_LCNT` reader - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
pub struct IC_SS_SCL_LCNT_R(crate::FieldReader<u16, u16>);
impl IC_SS_SCL_LCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_SS_SCL_LCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_SS_SCL_LCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_SS_SCL_LCNT` writer - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'  

 This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
pub struct IC_SS_SCL_LCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SS_SCL_LCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn ic_ss_scl_lcnt(&mut self) -> IC_SS_SCL_LCNT_W {
        IC_SS_SCL_LCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard Speed I2C Clock SCL Low Count Register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_ss_scl_lcnt](index.html) module"]
pub struct IC_SS_SCL_LCNT_SPEC;
impl crate::RegisterSpec for IC_SS_SCL_LCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_ss_scl_lcnt::R](R) reader structure"]
impl crate::Readable for IC_SS_SCL_LCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_ss_scl_lcnt::W](W) writer structure"]
impl crate::Writable for IC_SS_SCL_LCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_SS_SCL_LCNT to value 0x2f"]
impl crate::Resettable for IC_SS_SCL_LCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2f
    }
}
