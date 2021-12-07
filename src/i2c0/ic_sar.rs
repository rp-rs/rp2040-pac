#[doc = "Register `IC_SAR` reader"]
pub struct R(crate::R<IC_SAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_SAR` writer"]
pub struct W(crate::W<IC_SAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SAR_SPEC>;
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
impl From<crate::W<IC_SAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_SAR` reader - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
pub struct IC_SAR_R(crate::FieldReader<u16, u16>);
impl IC_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_SAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_SAR` writer - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
pub struct IC_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.  

 This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.  

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
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

 Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    #[inline(always)]
    pub fn ic_sar(&mut self) -> IC_SAR_W {
        IC_SAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Address Register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_sar](index.html) module"]
pub struct IC_SAR_SPEC;
impl crate::RegisterSpec for IC_SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_sar::R](R) reader structure"]
impl crate::Readable for IC_SAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_sar::W](W) writer structure"]
impl crate::Writable for IC_SAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_SAR to value 0x55"]
impl crate::Resettable for IC_SAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
