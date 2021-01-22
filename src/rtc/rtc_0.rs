#[doc = "Reader of register RTC_0"]
pub type R = crate::R<u32, super::RTC_0>;
#[doc = "Reader of field `DOTW`"]
pub type DOTW_R = crate::R<u8, u8>;
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:26 - Day of the week"]
    #[inline(always)]
    pub fn dotw(&self) -> DOTW_R {
        DOTW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
}
