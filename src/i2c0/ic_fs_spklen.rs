#[doc = "Reader of register IC_FS_SPKLEN"]
pub type R = crate::R<u32, super::IC_FS_SPKLEN>;
#[doc = "Writer for register IC_FS_SPKLEN"]
pub type W = crate::W<u32, super::IC_FS_SPKLEN>;
#[doc = "Register IC_FS_SPKLEN `reset()`'s with value 0x07"]
impl crate::ResetValue for super::IC_FS_SPKLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `IC_FS_SPKLEN`"]
pub type IC_FS_SPKLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC_FS_SPKLEN`"]
pub struct IC_FS_SPKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_FS_SPKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
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
}
