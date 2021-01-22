#[doc = "Reader of register INTS"]
pub type R = crate::R<u32, super::INTS>;
#[doc = "Reader of field `ALARM_3`"]
pub type ALARM_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM_2`"]
pub type ALARM_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM_1`"]
pub type ALARM_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM_0`"]
pub type ALARM_0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn alarm_3(&self) -> ALARM_3_R {
        ALARM_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alarm_2(&self) -> ALARM_2_R {
        ALARM_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn alarm_1(&self) -> ALARM_1_R {
        ALARM_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alarm_0(&self) -> ALARM_0_R {
        ALARM_0_R::new((self.bits & 0x01) != 0)
    }
}
