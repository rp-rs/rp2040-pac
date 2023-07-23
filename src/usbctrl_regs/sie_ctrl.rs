#[doc = "Register `SIE_CTRL` reader"]
pub struct R(crate::R<SIE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_CTRL` writer"]
pub struct W(crate::W<SIE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_CTRL_SPEC>;
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
impl From<crate::W<SIE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_TRANS` reader - Host: Start transaction"]
pub type START_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `START_TRANS` writer - Host: Start transaction"]
pub type START_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `SEND_SETUP` reader - Host: Send Setup packet"]
pub type SEND_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SEND_SETUP` writer - Host: Send Setup packet"]
pub type SEND_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `SEND_DATA` reader - Host: Send transaction (OUT from host)"]
pub type SEND_DATA_R = crate::BitReader<bool>;
#[doc = "Field `SEND_DATA` writer - Host: Send transaction (OUT from host)"]
pub type SEND_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `RECEIVE_DATA` reader - Host: Receive transaction (IN to host)"]
pub type RECEIVE_DATA_R = crate::BitReader<bool>;
#[doc = "Field `RECEIVE_DATA` writer - Host: Receive transaction (IN to host)"]
pub type RECEIVE_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `STOP_TRANS` reader - Host: Stop transaction"]
pub type STOP_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `STOP_TRANS` writer - Host: Stop transaction"]
pub type STOP_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `PREAMBLE_EN` reader - Host: Preable enable for LS device on FS hub"]
pub type PREAMBLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREAMBLE_EN` writer - Host: Preable enable for LS device on FS hub"]
pub type PREAMBLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `SOF_SYNC` reader - Host: Delay packet(s) until after SOF"]
pub type SOF_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SOF_SYNC` writer - Host: Delay packet(s) until after SOF"]
pub type SOF_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `SOF_EN` reader - Host: Enable SOF generation (for full speed bus)"]
pub type SOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `SOF_EN` writer - Host: Enable SOF generation (for full speed bus)"]
pub type SOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `KEEP_ALIVE_EN` reader - Host: Enable keep alive packet (for low speed bus)"]
pub type KEEP_ALIVE_EN_R = crate::BitReader<bool>;
#[doc = "Field `KEEP_ALIVE_EN` writer - Host: Enable keep alive packet (for low speed bus)"]
pub type KEEP_ALIVE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `VBUS_EN` reader - Host: Enable VBUS"]
pub type VBUS_EN_R = crate::BitReader<bool>;
#[doc = "Field `VBUS_EN` writer - Host: Enable VBUS"]
pub type VBUS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - Device: Remote wakeup. Device can initiate its own resume after suspend."]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Device: Remote wakeup. Device can initiate its own resume after suspend."]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `RESET_BUS` reader - Host: Reset bus"]
pub type RESET_BUS_R = crate::BitReader<bool>;
#[doc = "Field `RESET_BUS` writer - Host: Reset bus"]
pub type RESET_BUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `PULLDOWN_EN` reader - Host: Enable pull down resistors"]
pub type PULLDOWN_EN_R = crate::BitReader<bool>;
#[doc = "Field `PULLDOWN_EN` writer - Host: Enable pull down resistors"]
pub type PULLDOWN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `PULLUP_EN` reader - Device: Enable pull up resistor"]
pub type PULLUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PULLUP_EN` writer - Device: Enable pull up resistor"]
pub type PULLUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `RPU_OPT` reader - Device: Pull-up strength (0=1K2, 1=2k3)"]
pub type RPU_OPT_R = crate::BitReader<bool>;
#[doc = "Field `RPU_OPT` writer - Device: Pull-up strength (0=1K2, 1=2k3)"]
pub type RPU_OPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `TRANSCEIVER_PD` reader - Power down bus transceiver"]
pub type TRANSCEIVER_PD_R = crate::BitReader<bool>;
#[doc = "Field `TRANSCEIVER_PD` writer - Power down bus transceiver"]
pub type TRANSCEIVER_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `DIRECT_DM` reader - Direct control of DM"]
pub type DIRECT_DM_R = crate::BitReader<bool>;
#[doc = "Field `DIRECT_DM` writer - Direct control of DM"]
pub type DIRECT_DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `DIRECT_DP` reader - Direct control of DP"]
pub type DIRECT_DP_R = crate::BitReader<bool>;
#[doc = "Field `DIRECT_DP` writer - Direct control of DP"]
pub type DIRECT_DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `DIRECT_EN` reader - Direct bus drive enable"]
pub type DIRECT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIRECT_EN` writer - Direct bus drive enable"]
pub type DIRECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `EP0_INT_NAK` reader - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
pub type EP0_INT_NAK_R = crate::BitReader<bool>;
#[doc = "Field `EP0_INT_NAK` writer - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
pub type EP0_INT_NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `EP0_INT_2BUF` reader - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
pub type EP0_INT_2BUF_R = crate::BitReader<bool>;
#[doc = "Field `EP0_INT_2BUF` writer - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
pub type EP0_INT_2BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `EP0_INT_1BUF` reader - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
pub type EP0_INT_1BUF_R = crate::BitReader<bool>;
#[doc = "Field `EP0_INT_1BUF` writer - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
pub type EP0_INT_1BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `EP0_DOUBLE_BUF` reader - Device: EP0 single buffered = 0, double buffered = 1"]
pub type EP0_DOUBLE_BUF_R = crate::BitReader<bool>;
#[doc = "Field `EP0_DOUBLE_BUF` writer - Device: EP0 single buffered = 0, double buffered = 1"]
pub type EP0_DOUBLE_BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
#[doc = "Field `EP0_INT_STALL` reader - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
pub type EP0_INT_STALL_R = crate::BitReader<bool>;
#[doc = "Field `EP0_INT_STALL` writer - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
pub type EP0_INT_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Host: Start transaction"]
    #[inline(always)]
    pub fn start_trans(&self) -> START_TRANS_R {
        START_TRANS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    pub fn send_setup(&self) -> SEND_SETUP_R {
        SEND_SETUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub fn send_data(&self) -> SEND_DATA_R {
        SEND_DATA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub fn receive_data(&self) -> RECEIVE_DATA_R {
        RECEIVE_DATA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    pub fn stop_trans(&self) -> STOP_TRANS_R {
        STOP_TRANS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub fn preamble_en(&self) -> PREAMBLE_EN_R {
        PREAMBLE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub fn sof_sync(&self) -> SOF_SYNC_R {
        SOF_SYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub fn sof_en(&self) -> SOF_EN_R {
        SOF_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub fn keep_alive_en(&self) -> KEEP_ALIVE_EN_R {
        KEEP_ALIVE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    pub fn vbus_en(&self) -> VBUS_EN_R {
        VBUS_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    pub fn reset_bus(&self) -> RESET_BUS_R {
        RESET_BUS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    pub fn pulldown_en(&self) -> PULLDOWN_EN_R {
        PULLDOWN_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    pub fn pullup_en(&self) -> PULLUP_EN_R {
        PULLUP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub fn rpu_opt(&self) -> RPU_OPT_R {
        RPU_OPT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    pub fn transceiver_pd(&self) -> TRANSCEIVER_PD_R {
        TRANSCEIVER_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    pub fn direct_dm(&self) -> DIRECT_DM_R {
        DIRECT_DM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    pub fn direct_dp(&self) -> DIRECT_DP_R {
        DIRECT_DP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    pub fn direct_en(&self) -> DIRECT_EN_R {
        DIRECT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub fn ep0_int_nak(&self) -> EP0_INT_NAK_R {
        EP0_INT_NAK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_2buf(&self) -> EP0_INT_2BUF_R {
        EP0_INT_2BUF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub fn ep0_int_1buf(&self) -> EP0_INT_1BUF_R {
        EP0_INT_1BUF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub fn ep0_double_buf(&self) -> EP0_DOUBLE_BUF_R {
        EP0_DOUBLE_BUF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub fn ep0_int_stall(&self) -> EP0_INT_STALL_R {
        EP0_INT_STALL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host: Start transaction"]
    #[inline(always)]
    #[must_use]
    pub fn start_trans(&mut self) -> START_TRANS_W<0> {
        START_TRANS_W::new(self)
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    #[must_use]
    pub fn send_setup(&mut self) -> SEND_SETUP_W<1> {
        SEND_SETUP_W::new(self)
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    #[must_use]
    pub fn send_data(&mut self) -> SEND_DATA_W<2> {
        SEND_DATA_W::new(self)
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    #[must_use]
    pub fn receive_data(&mut self) -> RECEIVE_DATA_W<3> {
        RECEIVE_DATA_W::new(self)
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    #[must_use]
    pub fn stop_trans(&mut self) -> STOP_TRANS_W<4> {
        STOP_TRANS_W::new(self)
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    #[must_use]
    pub fn preamble_en(&mut self) -> PREAMBLE_EN_W<6> {
        PREAMBLE_EN_W::new(self)
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof_sync(&mut self) -> SOF_SYNC_W<8> {
        SOF_SYNC_W::new(self)
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    #[must_use]
    pub fn sof_en(&mut self) -> SOF_EN_W<9> {
        SOF_EN_W::new(self)
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    #[must_use]
    pub fn keep_alive_en(&mut self) -> KEEP_ALIVE_EN_W<10> {
        KEEP_ALIVE_EN_W::new(self)
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_en(&mut self) -> VBUS_EN_W<11> {
        VBUS_EN_W::new(self)
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<12> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    #[must_use]
    pub fn reset_bus(&mut self) -> RESET_BUS_W<13> {
        RESET_BUS_W::new(self)
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    #[must_use]
    pub fn pulldown_en(&mut self) -> PULLDOWN_EN_W<15> {
        PULLDOWN_EN_W::new(self)
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    #[must_use]
    pub fn pullup_en(&mut self) -> PULLUP_EN_W<16> {
        PULLUP_EN_W::new(self)
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    #[must_use]
    pub fn rpu_opt(&mut self) -> RPU_OPT_W<17> {
        RPU_OPT_W::new(self)
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    #[must_use]
    pub fn transceiver_pd(&mut self) -> TRANSCEIVER_PD_W<18> {
        TRANSCEIVER_PD_W::new(self)
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    #[must_use]
    pub fn direct_dm(&mut self) -> DIRECT_DM_W<24> {
        DIRECT_DM_W::new(self)
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    #[must_use]
    pub fn direct_dp(&mut self) -> DIRECT_DP_W<25> {
        DIRECT_DP_W::new(self)
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn direct_en(&mut self) -> DIRECT_EN_W<26> {
        DIRECT_EN_W::new(self)
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_nak(&mut self) -> EP0_INT_NAK_W<27> {
        EP0_INT_NAK_W::new(self)
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_2buf(&mut self) -> EP0_INT_2BUF_W<28> {
        EP0_INT_2BUF_W::new(self)
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_1buf(&mut self) -> EP0_INT_1BUF_W<29> {
        EP0_INT_1BUF_W::new(self)
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_double_buf(&mut self) -> EP0_DOUBLE_BUF_W<30> {
        EP0_DOUBLE_BUF_W::new(self)
    }
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_stall(&mut self) -> EP0_INT_STALL_W<31> {
        EP0_INT_STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIE control register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sie_ctrl](index.html) module"]
pub struct SIE_CTRL_SPEC;
impl crate::RegisterSpec for SIE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ctrl::R](R) reader structure"]
impl crate::Readable for SIE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ctrl::W](W) writer structure"]
impl crate::Writable for SIE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIE_CTRL to value 0"]
impl crate::Resettable for SIE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
