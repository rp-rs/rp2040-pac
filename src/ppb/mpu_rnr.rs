#[doc = "Reader of register MPU_RNR"]
pub type R = crate::R<u32, super::MPU_RNR>;
#[doc = "Writer for register MPU_RNR"]
pub type W = crate::W<u32, super::MPU_RNR>;
#[doc = "Register MPU_RNR `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_RNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGION`"]
pub type REGION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGION`"]
pub struct REGION_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers.\\n The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    #[inline(always)]
    pub fn region(&self) -> REGION_R {
        REGION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers.\\n The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    #[inline(always)]
    pub fn region(&mut self) -> REGION_W {
        REGION_W { w: self }
    }
}
