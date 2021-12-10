#[doc = "Register `GPIO_STATUS` reader"]
pub struct R(crate::R<GPIO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRQTOPROC` reader - interrupt to processors, after override is applied"]
pub struct IRQTOPROC_R(crate::FieldReader<bool, bool>);
impl IRQTOPROC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQTOPROC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQTOPROC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQFROMPAD` reader - interrupt from pad before override is applied"]
pub struct IRQFROMPAD_R(crate::FieldReader<bool, bool>);
impl IRQFROMPAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQFROMPAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQFROMPAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTOPERI` reader - input signal to peripheral, after override is applied"]
pub struct INTOPERI_R(crate::FieldReader<bool, bool>);
impl INTOPERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTOPERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTOPERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INFROMPAD` reader - input signal from pad, before override is applied"]
pub struct INFROMPAD_R(crate::FieldReader<bool, bool>);
impl INFROMPAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFROMPAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFROMPAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OETOPAD` reader - output enable to pad after register override is applied"]
pub struct OETOPAD_R(crate::FieldReader<bool, bool>);
impl OETOPAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OETOPAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OETOPAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEFROMPERI` reader - output enable from selected peripheral, before register override is applied"]
pub struct OEFROMPERI_R(crate::FieldReader<bool, bool>);
impl OEFROMPERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEFROMPERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEFROMPERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTTOPAD` reader - output signal to pad after register override is applied"]
pub struct OUTTOPAD_R(crate::FieldReader<bool, bool>);
impl OUTTOPAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTTOPAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTTOPAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTFROMPERI` reader - output signal from selected peripheral, before register override is applied"]
pub struct OUTFROMPERI_R(crate::FieldReader<bool, bool>);
impl OUTFROMPERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTFROMPERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTFROMPERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "GPIO status  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_status](index.html) module"]
pub struct GPIO_STATUS_SPEC;
impl crate::RegisterSpec for GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_status::R](R) reader structure"]
impl crate::Readable for GPIO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_STATUS to value 0"]
impl crate::Resettable for GPIO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
