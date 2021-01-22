#[doc = "Reader of register BUFF_STATUS"]
pub type R = crate::R<u32, super::BUFF_STATUS>;
#[doc = "Reader of field `EP15_OUT`"]
pub type EP15_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP15_IN`"]
pub type EP15_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP14_OUT`"]
pub type EP14_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP14_IN`"]
pub type EP14_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP13_OUT`"]
pub type EP13_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP13_IN`"]
pub type EP13_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP12_OUT`"]
pub type EP12_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP12_IN`"]
pub type EP12_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP11_OUT`"]
pub type EP11_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP11_IN`"]
pub type EP11_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP10_OUT`"]
pub type EP10_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP10_IN`"]
pub type EP10_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP9_OUT`"]
pub type EP9_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP9_IN`"]
pub type EP9_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP8_OUT`"]
pub type EP8_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP8_IN`"]
pub type EP8_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7_OUT`"]
pub type EP7_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7_IN`"]
pub type EP7_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6_OUT`"]
pub type EP6_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6_IN`"]
pub type EP6_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5_OUT`"]
pub type EP5_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5_IN`"]
pub type EP5_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4_OUT`"]
pub type EP4_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4_IN`"]
pub type EP4_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3_OUT`"]
pub type EP3_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3_IN`"]
pub type EP3_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2_OUT`"]
pub type EP2_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2_IN`"]
pub type EP2_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1_OUT`"]
pub type EP1_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1_IN`"]
pub type EP1_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0_OUT`"]
pub type EP0_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0_IN`"]
pub type EP0_IN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep15_out(&self) -> EP15_OUT_R {
        EP15_OUT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep15_in(&self) -> EP15_IN_R {
        EP15_IN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ep14_out(&self) -> EP14_OUT_R {
        EP14_OUT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ep14_in(&self) -> EP14_IN_R {
        EP14_IN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ep13_out(&self) -> EP13_OUT_R {
        EP13_OUT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ep13_in(&self) -> EP13_IN_R {
        EP13_IN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ep12_out(&self) -> EP12_OUT_R {
        EP12_OUT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ep12_in(&self) -> EP12_IN_R {
        EP12_IN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep11_out(&self) -> EP11_OUT_R {
        EP11_OUT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep11_in(&self) -> EP11_IN_R {
        EP11_IN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep10_out(&self) -> EP10_OUT_R {
        EP10_OUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep10_in(&self) -> EP10_IN_R {
        EP10_IN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep9_out(&self) -> EP9_OUT_R {
        EP9_OUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep9_in(&self) -> EP9_IN_R {
        EP9_IN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep8_out(&self) -> EP8_OUT_R {
        EP8_OUT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep8_in(&self) -> EP8_IN_R {
        EP8_IN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep7_out(&self) -> EP7_OUT_R {
        EP7_OUT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep7_in(&self) -> EP7_IN_R {
        EP7_IN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep6_out(&self) -> EP6_OUT_R {
        EP6_OUT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep6_in(&self) -> EP6_IN_R {
        EP6_IN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep5_out(&self) -> EP5_OUT_R {
        EP5_OUT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep5_in(&self) -> EP5_IN_R {
        EP5_IN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep4_out(&self) -> EP4_OUT_R {
        EP4_OUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep4_in(&self) -> EP4_IN_R {
        EP4_IN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep3_out(&self) -> EP3_OUT_R {
        EP3_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep3_in(&self) -> EP3_IN_R {
        EP3_IN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep2_out(&self) -> EP2_OUT_R {
        EP2_OUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep2_in(&self) -> EP2_IN_R {
        EP2_IN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep1_out(&self) -> EP1_OUT_R {
        EP1_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep1_in(&self) -> EP1_IN_R {
        EP1_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep0_out(&self) -> EP0_OUT_R {
        EP0_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep0_in(&self) -> EP0_IN_R {
        EP0_IN_R::new((self.bits & 0x01) != 0)
    }
}
