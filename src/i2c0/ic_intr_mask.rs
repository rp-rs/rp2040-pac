#[doc = "Register `IC_INTR_MASK` reader"]
pub struct R(crate::R<IC_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_INTR_MASK` writer"]
pub struct W(crate::W<IC_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IC_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_RX_UNDER` reader - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_UNDER_R = crate::BitReader<M_RX_UNDER_A>;
#[doc = "This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RX_UNDER_A {
    #[doc = "0: RX_UNDER interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: RX_UNDER interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_RX_UNDER_A> for bool {
    #[inline(always)]
    fn from(variant: M_RX_UNDER_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RX_UNDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RX_UNDER_A {
        match self.bits {
            false => M_RX_UNDER_A::ENABLED,
            true => M_RX_UNDER_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_UNDER_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_UNDER_A::DISABLED
    }
}
#[doc = "Field `M_RX_UNDER` writer - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_UNDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_RX_UNDER_A, O>;
impl<'a, const O: u8> M_RX_UNDER_W<'a, O> {
    #[doc = "RX_UNDER interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_RX_UNDER_A::ENABLED)
    }
    #[doc = "RX_UNDER interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_RX_UNDER_A::DISABLED)
    }
}
#[doc = "Field `M_RX_OVER` reader - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_OVER_R = crate::BitReader<M_RX_OVER_A>;
#[doc = "This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RX_OVER_A {
    #[doc = "0: RX_OVER interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: RX_OVER interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_RX_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: M_RX_OVER_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RX_OVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RX_OVER_A {
        match self.bits {
            false => M_RX_OVER_A::ENABLED,
            true => M_RX_OVER_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_OVER_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_OVER_A::DISABLED
    }
}
#[doc = "Field `M_RX_OVER` writer - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_OVER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_RX_OVER_A, O>;
impl<'a, const O: u8> M_RX_OVER_W<'a, O> {
    #[doc = "RX_OVER interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_RX_OVER_A::ENABLED)
    }
    #[doc = "RX_OVER interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_RX_OVER_A::DISABLED)
    }
}
#[doc = "Field `M_RX_FULL` reader - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_FULL_R = crate::BitReader<M_RX_FULL_A>;
#[doc = "This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RX_FULL_A {
    #[doc = "0: RX_FULL interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: RX_FULL interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_RX_FULL_A> for bool {
    #[inline(always)]
    fn from(variant: M_RX_FULL_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RX_FULL_A {
        match self.bits {
            false => M_RX_FULL_A::ENABLED,
            true => M_RX_FULL_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_FULL_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_FULL_A::DISABLED
    }
}
#[doc = "Field `M_RX_FULL` writer - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_RX_FULL_A, O>;
impl<'a, const O: u8> M_RX_FULL_W<'a, O> {
    #[doc = "RX_FULL interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_RX_FULL_A::ENABLED)
    }
    #[doc = "RX_FULL interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_RX_FULL_A::DISABLED)
    }
}
#[doc = "Field `M_TX_OVER` reader - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_OVER_R = crate::BitReader<M_TX_OVER_A>;
#[doc = "This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_TX_OVER_A {
    #[doc = "0: TX_OVER interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: TX_OVER interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_TX_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: M_TX_OVER_A) -> Self {
        variant as u8 != 0
    }
}
impl M_TX_OVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_TX_OVER_A {
        match self.bits {
            false => M_TX_OVER_A::ENABLED,
            true => M_TX_OVER_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_TX_OVER_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_TX_OVER_A::DISABLED
    }
}
#[doc = "Field `M_TX_OVER` writer - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_OVER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_TX_OVER_A, O>;
impl<'a, const O: u8> M_TX_OVER_W<'a, O> {
    #[doc = "TX_OVER interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_TX_OVER_A::ENABLED)
    }
    #[doc = "TX_OVER interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_TX_OVER_A::DISABLED)
    }
}
#[doc = "Field `M_TX_EMPTY` reader - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_EMPTY_R = crate::BitReader<M_TX_EMPTY_A>;
#[doc = "This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_TX_EMPTY_A {
    #[doc = "0: TX_EMPTY interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: TX_EMPTY interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_TX_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: M_TX_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl M_TX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_TX_EMPTY_A {
        match self.bits {
            false => M_TX_EMPTY_A::ENABLED,
            true => M_TX_EMPTY_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_TX_EMPTY_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_TX_EMPTY_A::DISABLED
    }
}
#[doc = "Field `M_TX_EMPTY` writer - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_EMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_TX_EMPTY_A, O>;
impl<'a, const O: u8> M_TX_EMPTY_W<'a, O> {
    #[doc = "TX_EMPTY interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_TX_EMPTY_A::ENABLED)
    }
    #[doc = "TX_EMPTY interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_TX_EMPTY_A::DISABLED)
    }
}
#[doc = "Field `M_RD_REQ` reader - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RD_REQ_R = crate::BitReader<M_RD_REQ_A>;
#[doc = "This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RD_REQ_A {
    #[doc = "0: RD_REQ interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: RD_REQ interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_RD_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: M_RD_REQ_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RD_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RD_REQ_A {
        match self.bits {
            false => M_RD_REQ_A::ENABLED,
            true => M_RD_REQ_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RD_REQ_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RD_REQ_A::DISABLED
    }
}
#[doc = "Field `M_RD_REQ` writer - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RD_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_RD_REQ_A, O>;
impl<'a, const O: u8> M_RD_REQ_W<'a, O> {
    #[doc = "RD_REQ interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_RD_REQ_A::ENABLED)
    }
    #[doc = "RD_REQ interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_RD_REQ_A::DISABLED)
    }
}
#[doc = "Field `M_TX_ABRT` reader - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_ABRT_R = crate::BitReader<M_TX_ABRT_A>;
#[doc = "This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_TX_ABRT_A {
    #[doc = "0: TX_ABORT interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: TX_ABORT interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_TX_ABRT_A> for bool {
    #[inline(always)]
    fn from(variant: M_TX_ABRT_A) -> Self {
        variant as u8 != 0
    }
}
impl M_TX_ABRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_TX_ABRT_A {
        match self.bits {
            false => M_TX_ABRT_A::ENABLED,
            true => M_TX_ABRT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_TX_ABRT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_TX_ABRT_A::DISABLED
    }
}
#[doc = "Field `M_TX_ABRT` writer - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_ABRT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_TX_ABRT_A, O>;
impl<'a, const O: u8> M_TX_ABRT_W<'a, O> {
    #[doc = "TX_ABORT interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_TX_ABRT_A::ENABLED)
    }
    #[doc = "TX_ABORT interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_TX_ABRT_A::DISABLED)
    }
}
#[doc = "Field `M_RX_DONE` reader - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_DONE_R = crate::BitReader<M_RX_DONE_A>;
#[doc = "This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RX_DONE_A {
    #[doc = "0: RX_DONE interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: RX_DONE interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_RX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: M_RX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RX_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RX_DONE_A {
        match self.bits {
            false => M_RX_DONE_A::ENABLED,
            true => M_RX_DONE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_DONE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_DONE_A::DISABLED
    }
}
#[doc = "Field `M_RX_DONE` writer - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_RX_DONE_A, O>;
impl<'a, const O: u8> M_RX_DONE_W<'a, O> {
    #[doc = "RX_DONE interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_RX_DONE_A::ENABLED)
    }
    #[doc = "RX_DONE interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_RX_DONE_A::DISABLED)
    }
}
#[doc = "Field `M_ACTIVITY` reader - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_ACTIVITY_R = crate::BitReader<M_ACTIVITY_A>;
#[doc = "This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_ACTIVITY_A {
    #[doc = "0: ACTIVITY interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: ACTIVITY interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_ACTIVITY_A> for bool {
    #[inline(always)]
    fn from(variant: M_ACTIVITY_A) -> Self {
        variant as u8 != 0
    }
}
impl M_ACTIVITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_ACTIVITY_A {
        match self.bits {
            false => M_ACTIVITY_A::ENABLED,
            true => M_ACTIVITY_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_ACTIVITY_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_ACTIVITY_A::DISABLED
    }
}
#[doc = "Field `M_ACTIVITY` writer - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_ACTIVITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_ACTIVITY_A, O>;
impl<'a, const O: u8> M_ACTIVITY_W<'a, O> {
    #[doc = "ACTIVITY interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_ACTIVITY_A::ENABLED)
    }
    #[doc = "ACTIVITY interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_ACTIVITY_A::DISABLED)
    }
}
#[doc = "Field `M_STOP_DET` reader - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_STOP_DET_R = crate::BitReader<M_STOP_DET_A>;
#[doc = "This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_STOP_DET_A {
    #[doc = "0: STOP_DET interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: STOP_DET interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_STOP_DET_A> for bool {
    #[inline(always)]
    fn from(variant: M_STOP_DET_A) -> Self {
        variant as u8 != 0
    }
}
impl M_STOP_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_STOP_DET_A {
        match self.bits {
            false => M_STOP_DET_A::ENABLED,
            true => M_STOP_DET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_STOP_DET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_STOP_DET_A::DISABLED
    }
}
#[doc = "Field `M_STOP_DET` writer - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_STOP_DET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_STOP_DET_A, O>;
impl<'a, const O: u8> M_STOP_DET_W<'a, O> {
    #[doc = "STOP_DET interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_STOP_DET_A::ENABLED)
    }
    #[doc = "STOP_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_STOP_DET_A::DISABLED)
    }
}
#[doc = "Field `M_START_DET` reader - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_START_DET_R = crate::BitReader<M_START_DET_A>;
#[doc = "This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_START_DET_A {
    #[doc = "0: START_DET interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: START_DET interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_START_DET_A> for bool {
    #[inline(always)]
    fn from(variant: M_START_DET_A) -> Self {
        variant as u8 != 0
    }
}
impl M_START_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_START_DET_A {
        match self.bits {
            false => M_START_DET_A::ENABLED,
            true => M_START_DET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_START_DET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_START_DET_A::DISABLED
    }
}
#[doc = "Field `M_START_DET` writer - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_START_DET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_START_DET_A, O>;
impl<'a, const O: u8> M_START_DET_W<'a, O> {
    #[doc = "START_DET interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_START_DET_A::ENABLED)
    }
    #[doc = "START_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_START_DET_A::DISABLED)
    }
}
#[doc = "Field `M_GEN_CALL` reader - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_GEN_CALL_R = crate::BitReader<M_GEN_CALL_A>;
#[doc = "This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_GEN_CALL_A {
    #[doc = "0: GEN_CALL interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: GEN_CALL interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_GEN_CALL_A> for bool {
    #[inline(always)]
    fn from(variant: M_GEN_CALL_A) -> Self {
        variant as u8 != 0
    }
}
impl M_GEN_CALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_GEN_CALL_A {
        match self.bits {
            false => M_GEN_CALL_A::ENABLED,
            true => M_GEN_CALL_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_GEN_CALL_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_GEN_CALL_A::DISABLED
    }
}
#[doc = "Field `M_GEN_CALL` writer - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_GEN_CALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_GEN_CALL_A, O>;
impl<'a, const O: u8> M_GEN_CALL_W<'a, O> {
    #[doc = "GEN_CALL interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_GEN_CALL_A::ENABLED)
    }
    #[doc = "GEN_CALL interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_GEN_CALL_A::DISABLED)
    }
}
#[doc = "Field `M_RESTART_DET` reader - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_RESTART_DET_R = crate::BitReader<M_RESTART_DET_A>;
#[doc = "This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_RESTART_DET_A {
    #[doc = "0: RESTART_DET interrupt is masked"]
    ENABLED = 0,
    #[doc = "1: RESTART_DET interrupt is unmasked"]
    DISABLED = 1,
}
impl From<M_RESTART_DET_A> for bool {
    #[inline(always)]
    fn from(variant: M_RESTART_DET_A) -> Self {
        variant as u8 != 0
    }
}
impl M_RESTART_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_RESTART_DET_A {
        match self.bits {
            false => M_RESTART_DET_A::ENABLED,
            true => M_RESTART_DET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RESTART_DET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RESTART_DET_A::DISABLED
    }
}
#[doc = "Field `M_RESTART_DET` writer - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_RESTART_DET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_INTR_MASK_SPEC, M_RESTART_DET_A, O>;
impl<'a, const O: u8> M_RESTART_DET_W<'a, O> {
    #[doc = "RESTART_DET interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(M_RESTART_DET_A::ENABLED)
    }
    #[doc = "RESTART_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(M_RESTART_DET_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_rx_under(&self) -> M_RX_UNDER_R {
        M_RX_UNDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_rx_over(&self) -> M_RX_OVER_R {
        M_RX_OVER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_rx_full(&self) -> M_RX_FULL_R {
        M_RX_FULL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_tx_over(&self) -> M_TX_OVER_R {
        M_TX_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_tx_empty(&self) -> M_TX_EMPTY_R {
        M_TX_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_rd_req(&self) -> M_RD_REQ_R {
        M_RD_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_tx_abrt(&self) -> M_TX_ABRT_R {
        M_TX_ABRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_rx_done(&self) -> M_RX_DONE_R {
        M_RX_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn m_activity(&self) -> M_ACTIVITY_R {
        M_ACTIVITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn m_stop_det(&self) -> M_STOP_DET_R {
        M_STOP_DET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn m_start_det(&self) -> M_START_DET_R {
        M_START_DET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    pub fn m_gen_call(&self) -> M_GEN_CALL_R {
        M_GEN_CALL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    pub fn m_restart_det(&self) -> M_RESTART_DET_R {
        M_RESTART_DET_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_under(&mut self) -> M_RX_UNDER_W<0> {
        M_RX_UNDER_W::new(self)
    }
    #[doc = "Bit 1 - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_over(&mut self) -> M_RX_OVER_W<1> {
        M_RX_OVER_W::new(self)
    }
    #[doc = "Bit 2 - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_full(&mut self) -> M_RX_FULL_W<2> {
        M_RX_FULL_W::new(self)
    }
    #[doc = "Bit 3 - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_tx_over(&mut self) -> M_TX_OVER_W<3> {
        M_TX_OVER_W::new(self)
    }
    #[doc = "Bit 4 - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_tx_empty(&mut self) -> M_TX_EMPTY_W<4> {
        M_TX_EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rd_req(&mut self) -> M_RD_REQ_W<5> {
        M_RD_REQ_W::new(self)
    }
    #[doc = "Bit 6 - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_tx_abrt(&mut self) -> M_TX_ABRT_W<6> {
        M_TX_ABRT_W::new(self)
    }
    #[doc = "Bit 7 - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_done(&mut self) -> M_RX_DONE_W<7> {
        M_RX_DONE_W::new(self)
    }
    #[doc = "Bit 8 - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_activity(&mut self) -> M_ACTIVITY_W<8> {
        M_ACTIVITY_W::new(self)
    }
    #[doc = "Bit 9 - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_stop_det(&mut self) -> M_STOP_DET_W<9> {
        M_STOP_DET_W::new(self)
    }
    #[doc = "Bit 10 - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_start_det(&mut self) -> M_START_DET_W<10> {
        M_START_DET_W::new(self)
    }
    #[doc = "Bit 11 - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_gen_call(&mut self) -> M_GEN_CALL_W<11> {
        M_GEN_CALL_W::new(self)
    }
    #[doc = "Bit 12 - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_restart_det(&mut self) -> M_RESTART_DET_W<12> {
        M_RESTART_DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_intr_mask](index.html) module"]
pub struct IC_INTR_MASK_SPEC;
impl crate::RegisterSpec for IC_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_intr_mask::R](R) reader structure"]
impl crate::Readable for IC_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_intr_mask::W](W) writer structure"]
impl crate::Writable for IC_INTR_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_INTR_MASK to value 0x08ff"]
impl crate::Resettable for IC_INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x08ff;
}
