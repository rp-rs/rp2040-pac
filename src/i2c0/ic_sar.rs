#[doc = "Reader of register IC_SAR"]
pub type R = crate::R<u32, super::IC_SAR>;
#[doc = "Writer for register IC_SAR"]
pub type W = crate::W<u32, super::IC_SAR>;
#[doc = "Register IC_SAR `reset()`'s with value 0x55"]
impl crate::ResetValue for super::IC_SAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x55
    }
}
#[doc = "Reader of field `IC_SAR`"]
pub type IC_SAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IC_SAR`"]
pub struct IC_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.\\n\\n This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    #[inline(always)]
    pub fn ic_sar(&self) -> IC_SAR_R {
        IC_SAR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used.\\n\\n This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    #[inline(always)]
    pub fn ic_sar(&mut self) -> IC_SAR_W {
        IC_SAR_W { w: self }
    }
}
