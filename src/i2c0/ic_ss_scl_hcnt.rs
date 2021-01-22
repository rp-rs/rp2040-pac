#[doc = "Reader of register IC_SS_SCL_HCNT"]
pub type R = crate::R<u32, super::IC_SS_SCL_HCNT>;
#[doc = "Writer for register IC_SS_SCL_HCNT"]
pub type W = crate::W<u32, super::IC_SS_SCL_HCNT>;
#[doc = "Register IC_SS_SCL_HCNT `reset()`'s with value 0x28"]
impl crate::ResetValue for super::IC_SS_SCL_HCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x28
    }
}
#[doc = "Reader of field `IC_SS_SCL_HCNT`"]
pub type IC_SS_SCL_HCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IC_SS_SCL_HCNT`"]
pub struct IC_SS_SCL_HCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SS_SCL_HCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.\\n\\n This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.\\n\\n NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&self) -> IC_SS_SCL_HCNT_R {
        IC_SS_SCL_HCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'.\\n\\n This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed.\\n\\n NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&mut self) -> IC_SS_SCL_HCNT_W {
        IC_SS_SCL_HCNT_W { w: self }
    }
}
