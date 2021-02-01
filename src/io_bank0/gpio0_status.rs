#[doc = "Reader of register GPIO0_STATUS"]
pub type R = crate::R<u32, super::GPIO0_STATUS>;
#[doc = "Reader of field `IRQTOPROC`"]
pub type IRQTOPROC_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRQFROMPAD`"]
pub type IRQFROMPAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `INTOPERI`"]
pub type INTOPERI_R = crate::R<bool, bool>;
#[doc = "Reader of field `INFROMPAD`"]
pub type INFROMPAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `OETOPAD`"]
pub type OETOPAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `OEFROMPERI`"]
pub type OEFROMPERI_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTTOPAD`"]
pub type OUTTOPAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `OUTFROMPERI`"]
pub type OUTFROMPERI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 26 - interrupt to processors, after override is applied"]
    #[inline(always)]
    pub fn irqtoproc(&self) -> IRQTOPROC_R {
        IRQTOPROC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 24 - interrupt from pad before override is applied"]
    #[inline(always)]
    pub fn irqfrompad(&self) -> IRQFROMPAD_R {
        IRQFROMPAD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19 - input signal to peripheral, after override is applied"]
    #[inline(always)]
    pub fn intoperi(&self) -> INTOPERI_R {
        INTOPERI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - input signal from pad, before override is applied"]
    #[inline(always)]
    pub fn infrompad(&self) -> INFROMPAD_R {
        INFROMPAD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 13 - output enable to pad after register override is applied"]
    #[inline(always)]
    pub fn oetopad(&self) -> OETOPAD_R {
        OETOPAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - output enable from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub fn oefromperi(&self) -> OEFROMPERI_R {
        OEFROMPERI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - output signal to pad after register override is applied"]
    #[inline(always)]
    pub fn outtopad(&self) -> OUTTOPAD_R {
        OUTTOPAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - output signal from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub fn outfromperi(&self) -> OUTFROMPERI_R {
        OUTFROMPERI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
