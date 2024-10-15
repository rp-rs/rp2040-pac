#[doc = "Register `IC_INTR_MASK` reader"]
pub type R = crate::R<IC_INTR_MASK_SPEC>;
#[doc = "Register `IC_INTR_MASK` writer"]
pub type W = crate::W<IC_INTR_MASK_SPEC>;
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
#[doc = "Field `M_RX_UNDER` reader - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_UNDER_R = crate::BitReader<M_RX_UNDER_A>;
impl M_RX_UNDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RX_UNDER_A {
        match self.bits {
            false => M_RX_UNDER_A::ENABLED,
            true => M_RX_UNDER_A::DISABLED,
        }
    }
    #[doc = "RX_UNDER interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_UNDER_A::ENABLED
    }
    #[doc = "RX_UNDER interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_UNDER_A::DISABLED
    }
}
#[doc = "Field `M_RX_UNDER` writer - This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_UNDER_W<'a, REG> = crate::BitWriter<'a, REG, M_RX_UNDER_A>;
impl<'a, REG> M_RX_UNDER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX_UNDER interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_UNDER_A::ENABLED)
    }
    #[doc = "RX_UNDER interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_UNDER_A::DISABLED)
    }
}
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
#[doc = "Field `M_RX_OVER` reader - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_OVER_R = crate::BitReader<M_RX_OVER_A>;
impl M_RX_OVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RX_OVER_A {
        match self.bits {
            false => M_RX_OVER_A::ENABLED,
            true => M_RX_OVER_A::DISABLED,
        }
    }
    #[doc = "RX_OVER interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_OVER_A::ENABLED
    }
    #[doc = "RX_OVER interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_OVER_A::DISABLED
    }
}
#[doc = "Field `M_RX_OVER` writer - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_OVER_W<'a, REG> = crate::BitWriter<'a, REG, M_RX_OVER_A>;
impl<'a, REG> M_RX_OVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX_OVER interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_OVER_A::ENABLED)
    }
    #[doc = "RX_OVER interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_OVER_A::DISABLED)
    }
}
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
#[doc = "Field `M_RX_FULL` reader - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_FULL_R = crate::BitReader<M_RX_FULL_A>;
impl M_RX_FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RX_FULL_A {
        match self.bits {
            false => M_RX_FULL_A::ENABLED,
            true => M_RX_FULL_A::DISABLED,
        }
    }
    #[doc = "RX_FULL interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_FULL_A::ENABLED
    }
    #[doc = "RX_FULL interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_FULL_A::DISABLED
    }
}
#[doc = "Field `M_RX_FULL` writer - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_FULL_W<'a, REG> = crate::BitWriter<'a, REG, M_RX_FULL_A>;
impl<'a, REG> M_RX_FULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX_FULL interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_FULL_A::ENABLED)
    }
    #[doc = "RX_FULL interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_FULL_A::DISABLED)
    }
}
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
#[doc = "Field `M_TX_OVER` reader - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_OVER_R = crate::BitReader<M_TX_OVER_A>;
impl M_TX_OVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_TX_OVER_A {
        match self.bits {
            false => M_TX_OVER_A::ENABLED,
            true => M_TX_OVER_A::DISABLED,
        }
    }
    #[doc = "TX_OVER interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_TX_OVER_A::ENABLED
    }
    #[doc = "TX_OVER interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_TX_OVER_A::DISABLED
    }
}
#[doc = "Field `M_TX_OVER` writer - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_OVER_W<'a, REG> = crate::BitWriter<'a, REG, M_TX_OVER_A>;
impl<'a, REG> M_TX_OVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX_OVER interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_TX_OVER_A::ENABLED)
    }
    #[doc = "TX_OVER interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_TX_OVER_A::DISABLED)
    }
}
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
#[doc = "Field `M_TX_EMPTY` reader - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_EMPTY_R = crate::BitReader<M_TX_EMPTY_A>;
impl M_TX_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_TX_EMPTY_A {
        match self.bits {
            false => M_TX_EMPTY_A::ENABLED,
            true => M_TX_EMPTY_A::DISABLED,
        }
    }
    #[doc = "TX_EMPTY interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_TX_EMPTY_A::ENABLED
    }
    #[doc = "TX_EMPTY interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_TX_EMPTY_A::DISABLED
    }
}
#[doc = "Field `M_TX_EMPTY` writer - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG, M_TX_EMPTY_A>;
impl<'a, REG> M_TX_EMPTY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX_EMPTY interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_TX_EMPTY_A::ENABLED)
    }
    #[doc = "TX_EMPTY interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_TX_EMPTY_A::DISABLED)
    }
}
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
#[doc = "Field `M_RD_REQ` reader - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RD_REQ_R = crate::BitReader<M_RD_REQ_A>;
impl M_RD_REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RD_REQ_A {
        match self.bits {
            false => M_RD_REQ_A::ENABLED,
            true => M_RD_REQ_A::DISABLED,
        }
    }
    #[doc = "RD_REQ interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RD_REQ_A::ENABLED
    }
    #[doc = "RD_REQ interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RD_REQ_A::DISABLED
    }
}
#[doc = "Field `M_RD_REQ` writer - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RD_REQ_W<'a, REG> = crate::BitWriter<'a, REG, M_RD_REQ_A>;
impl<'a, REG> M_RD_REQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RD_REQ interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RD_REQ_A::ENABLED)
    }
    #[doc = "RD_REQ interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RD_REQ_A::DISABLED)
    }
}
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
#[doc = "Field `M_TX_ABRT` reader - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_ABRT_R = crate::BitReader<M_TX_ABRT_A>;
impl M_TX_ABRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_TX_ABRT_A {
        match self.bits {
            false => M_TX_ABRT_A::ENABLED,
            true => M_TX_ABRT_A::DISABLED,
        }
    }
    #[doc = "TX_ABORT interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_TX_ABRT_A::ENABLED
    }
    #[doc = "TX_ABORT interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_TX_ABRT_A::DISABLED
    }
}
#[doc = "Field `M_TX_ABRT` writer - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_TX_ABRT_W<'a, REG> = crate::BitWriter<'a, REG, M_TX_ABRT_A>;
impl<'a, REG> M_TX_ABRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX_ABORT interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_TX_ABRT_A::ENABLED)
    }
    #[doc = "TX_ABORT interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_TX_ABRT_A::DISABLED)
    }
}
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
#[doc = "Field `M_RX_DONE` reader - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_DONE_R = crate::BitReader<M_RX_DONE_A>;
impl M_RX_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RX_DONE_A {
        match self.bits {
            false => M_RX_DONE_A::ENABLED,
            true => M_RX_DONE_A::DISABLED,
        }
    }
    #[doc = "RX_DONE interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RX_DONE_A::ENABLED
    }
    #[doc = "RX_DONE interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RX_DONE_A::DISABLED
    }
}
#[doc = "Field `M_RX_DONE` writer - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_RX_DONE_W<'a, REG> = crate::BitWriter<'a, REG, M_RX_DONE_A>;
impl<'a, REG> M_RX_DONE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX_DONE interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_DONE_A::ENABLED)
    }
    #[doc = "RX_DONE interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RX_DONE_A::DISABLED)
    }
}
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
#[doc = "Field `M_ACTIVITY` reader - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_ACTIVITY_R = crate::BitReader<M_ACTIVITY_A>;
impl M_ACTIVITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_ACTIVITY_A {
        match self.bits {
            false => M_ACTIVITY_A::ENABLED,
            true => M_ACTIVITY_A::DISABLED,
        }
    }
    #[doc = "ACTIVITY interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_ACTIVITY_A::ENABLED
    }
    #[doc = "ACTIVITY interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_ACTIVITY_A::DISABLED
    }
}
#[doc = "Field `M_ACTIVITY` writer - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_ACTIVITY_W<'a, REG> = crate::BitWriter<'a, REG, M_ACTIVITY_A>;
impl<'a, REG> M_ACTIVITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACTIVITY interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_ACTIVITY_A::ENABLED)
    }
    #[doc = "ACTIVITY interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_ACTIVITY_A::DISABLED)
    }
}
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
#[doc = "Field `M_STOP_DET` reader - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_STOP_DET_R = crate::BitReader<M_STOP_DET_A>;
impl M_STOP_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_STOP_DET_A {
        match self.bits {
            false => M_STOP_DET_A::ENABLED,
            true => M_STOP_DET_A::DISABLED,
        }
    }
    #[doc = "STOP_DET interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_STOP_DET_A::ENABLED
    }
    #[doc = "STOP_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_STOP_DET_A::DISABLED
    }
}
#[doc = "Field `M_STOP_DET` writer - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_STOP_DET_W<'a, REG> = crate::BitWriter<'a, REG, M_STOP_DET_A>;
impl<'a, REG> M_STOP_DET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP_DET interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_STOP_DET_A::ENABLED)
    }
    #[doc = "STOP_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_STOP_DET_A::DISABLED)
    }
}
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
#[doc = "Field `M_START_DET` reader - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_START_DET_R = crate::BitReader<M_START_DET_A>;
impl M_START_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_START_DET_A {
        match self.bits {
            false => M_START_DET_A::ENABLED,
            true => M_START_DET_A::DISABLED,
        }
    }
    #[doc = "START_DET interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_START_DET_A::ENABLED
    }
    #[doc = "START_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_START_DET_A::DISABLED
    }
}
#[doc = "Field `M_START_DET` writer - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_START_DET_W<'a, REG> = crate::BitWriter<'a, REG, M_START_DET_A>;
impl<'a, REG> M_START_DET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "START_DET interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_START_DET_A::ENABLED)
    }
    #[doc = "START_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_START_DET_A::DISABLED)
    }
}
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
#[doc = "Field `M_GEN_CALL` reader - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_GEN_CALL_R = crate::BitReader<M_GEN_CALL_A>;
impl M_GEN_CALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_GEN_CALL_A {
        match self.bits {
            false => M_GEN_CALL_A::ENABLED,
            true => M_GEN_CALL_A::DISABLED,
        }
    }
    #[doc = "GEN_CALL interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_GEN_CALL_A::ENABLED
    }
    #[doc = "GEN_CALL interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_GEN_CALL_A::DISABLED
    }
}
#[doc = "Field `M_GEN_CALL` writer - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
pub type M_GEN_CALL_W<'a, REG> = crate::BitWriter<'a, REG, M_GEN_CALL_A>;
impl<'a, REG> M_GEN_CALL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GEN_CALL interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_GEN_CALL_A::ENABLED)
    }
    #[doc = "GEN_CALL interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_GEN_CALL_A::DISABLED)
    }
}
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
#[doc = "Field `M_RESTART_DET` reader - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_RESTART_DET_R = crate::BitReader<M_RESTART_DET_A>;
impl M_RESTART_DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M_RESTART_DET_A {
        match self.bits {
            false => M_RESTART_DET_A::ENABLED,
            true => M_RESTART_DET_A::DISABLED,
        }
    }
    #[doc = "RESTART_DET interrupt is masked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M_RESTART_DET_A::ENABLED
    }
    #[doc = "RESTART_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M_RESTART_DET_A::DISABLED
    }
}
#[doc = "Field `M_RESTART_DET` writer - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
pub type M_RESTART_DET_W<'a, REG> = crate::BitWriter<'a, REG, M_RESTART_DET_A>;
impl<'a, REG> M_RESTART_DET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESTART_DET interrupt is masked"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M_RESTART_DET_A::ENABLED)
    }
    #[doc = "RESTART_DET interrupt is unmasked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
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
    pub fn m_rx_under(&mut self) -> M_RX_UNDER_W<IC_INTR_MASK_SPEC> {
        M_RX_UNDER_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_over(&mut self) -> M_RX_OVER_W<IC_INTR_MASK_SPEC> {
        M_RX_OVER_W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_full(&mut self) -> M_RX_FULL_W<IC_INTR_MASK_SPEC> {
        M_RX_FULL_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_tx_over(&mut self) -> M_TX_OVER_W<IC_INTR_MASK_SPEC> {
        M_TX_OVER_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_tx_empty(&mut self) -> M_TX_EMPTY_W<IC_INTR_MASK_SPEC> {
        M_TX_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rd_req(&mut self) -> M_RD_REQ_W<IC_INTR_MASK_SPEC> {
        M_RD_REQ_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_tx_abrt(&mut self) -> M_TX_ABRT_W<IC_INTR_MASK_SPEC> {
        M_TX_ABRT_W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_rx_done(&mut self) -> M_RX_DONE_W<IC_INTR_MASK_SPEC> {
        M_RX_DONE_W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_activity(&mut self) -> M_ACTIVITY_W<IC_INTR_MASK_SPEC> {
        M_ACTIVITY_W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_stop_det(&mut self) -> M_STOP_DET_W<IC_INTR_MASK_SPEC> {
        M_STOP_DET_W::new(self, 9)
    }
    #[doc = "Bit 10 - This bit masks the R_START_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_start_det(&mut self) -> M_START_DET_W<IC_INTR_MASK_SPEC> {
        M_START_DET_W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register.  

 Reset value: 0x1"]
    #[inline(always)]
    #[must_use]
    pub fn m_gen_call(&mut self) -> M_GEN_CALL_W<IC_INTR_MASK_SPEC> {
        M_GEN_CALL_W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register.  

 Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn m_restart_det(&mut self) -> M_RESTART_DET_W<IC_INTR_MASK_SPEC> {
        M_RESTART_DET_W::new(self, 12)
    }
}
#[doc = "I2C Interrupt Mask Register.  

 These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt.  

You can [`read`](crate::Reg::read) this register and get [`ic_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_INTR_MASK_SPEC;
impl crate::RegisterSpec for IC_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_intr_mask::R`](R) reader structure"]
impl crate::Readable for IC_INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_intr_mask::W`](W) writer structure"]
impl crate::Writable for IC_INTR_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_INTR_MASK to value 0x08ff"]
impl crate::Resettable for IC_INTR_MASK_SPEC {
    const RESET_VALUE: u32 = 0x08ff;
}
