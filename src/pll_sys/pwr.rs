#[doc = "Reader of register PWR"]
pub type R = crate::R<u32, super::PWR>;
#[doc = "Writer for register PWR"]
pub type W = crate::W<u32, super::PWR>;
#[doc = "Register PWR `reset()`'s with value 0x2d"]
impl crate::ResetValue for super::PWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2d
    }
}
#[doc = "Reader of field `VCOPD`"]
pub type VCOPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCOPD`"]
pub struct VCOPD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `POSTDIVPD`"]
pub type POSTDIVPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POSTDIVPD`"]
pub struct POSTDIVPD_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIVPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DSMPD`"]
pub type DSMPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSMPD`"]
pub struct DSMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSMPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - PLL VCO powerdown\\n To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn vcopd(&self) -> VCOPD_R {
        VCOPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PLL post divider powerdown\\n To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn postdivpd(&self) -> POSTDIVPD_R {
        POSTDIVPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PLL DSM powerdown\\n Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn dsmpd(&self) -> DSMPD_R {
        DSMPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PLL powerdown\\n To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - PLL VCO powerdown\\n To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn vcopd(&mut self) -> VCOPD_W {
        VCOPD_W { w: self }
    }
    #[doc = "Bit 3 - PLL post divider powerdown\\n To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn postdivpd(&mut self) -> POSTDIVPD_W {
        POSTDIVPD_W { w: self }
    }
    #[doc = "Bit 2 - PLL DSM powerdown\\n Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn dsmpd(&mut self) -> DSMPD_W {
        DSMPD_W { w: self }
    }
    #[doc = "Bit 0 - PLL powerdown\\n To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
}
