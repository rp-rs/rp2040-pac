#[doc = "Reader of register IC_SDA_SETUP"]
pub type R = crate::R<u32, super::IC_SDA_SETUP>;
#[doc = "Writer for register IC_SDA_SETUP"]
pub type W = crate::W<u32, super::IC_SDA_SETUP>;
#[doc = "Register IC_SDA_SETUP `reset()`'s with value 0x64"]
impl crate::ResetValue for super::IC_SDA_SETUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x64
    }
}
#[doc = "Reader of field `SDA_SETUP`"]
pub type SDA_SETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_SETUP`"]
pub struct SDA_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    #[inline(always)]
    pub fn sda_setup(&self) -> SDA_SETUP_R {
        SDA_SETUP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    #[inline(always)]
    pub fn sda_setup(&mut self) -> SDA_SETUP_W {
        SDA_SETUP_W { w: self }
    }
}
