#[doc = "Reader of register IC_SS_SCL_LCNT"]
pub type R = crate::R<u32, super::IC_SS_SCL_LCNT>;
#[doc = "Writer for register IC_SS_SCL_LCNT"]
pub type W = crate::W<u32, super::IC_SS_SCL_LCNT>;
#[doc = "Register IC_SS_SCL_LCNT `reset()`'s with value 0x2f"]
impl crate::ResetValue for super::IC_SS_SCL_LCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2f
    }
}
#[doc = "Reader of field `IC_SS_SCL_LCNT`"]
pub type IC_SS_SCL_LCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IC_SS_SCL_LCNT`"]
pub struct IC_SS_SCL_LCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SS_SCL_LCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'\\n\\n This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&self) -> IC_SS_SCL_LCNT_R {
        IC_SS_SCL_LCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'\\n\\n This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE\\[0\\]
register being set to 0. Writes at other times have no effect.\\n\\n The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
    #[inline(always)]
    pub fn ic_ss_scl_lcnt(&mut self) -> IC_SS_SCL_LCNT_W {
        IC_SS_SCL_LCNT_W { w: self }
    }
}
