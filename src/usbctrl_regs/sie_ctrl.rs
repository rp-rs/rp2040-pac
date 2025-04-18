#[doc = "Register `SIE_CTRL` reader"]
pub type R = crate::R<SIE_CTRL_SPEC>;
#[doc = "Register `SIE_CTRL` writer"]
pub type W = crate::W<SIE_CTRL_SPEC>;
#[doc = "Field `START_TRANS` writer - Host: Start transaction"]
pub type START_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_SETUP` reader - Host: Send Setup packet"]
pub type SEND_SETUP_R = crate::BitReader;
#[doc = "Field `SEND_SETUP` writer - Host: Send Setup packet"]
pub type SEND_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_DATA` reader - Host: Send transaction (OUT from host)"]
pub type SEND_DATA_R = crate::BitReader;
#[doc = "Field `SEND_DATA` writer - Host: Send transaction (OUT from host)"]
pub type SEND_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_DATA` reader - Host: Receive transaction (IN to host)"]
pub type RECEIVE_DATA_R = crate::BitReader;
#[doc = "Field `RECEIVE_DATA` writer - Host: Receive transaction (IN to host)"]
pub type RECEIVE_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_TRANS` writer - Host: Stop transaction"]
pub type STOP_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREAMBLE_EN` reader - Host: Preable enable for LS device on FS hub"]
pub type PREAMBLE_EN_R = crate::BitReader;
#[doc = "Field `PREAMBLE_EN` writer - Host: Preable enable for LS device on FS hub"]
pub type PREAMBLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_SYNC` reader - Host: Delay packet(s) until after SOF"]
pub type SOF_SYNC_R = crate::BitReader;
#[doc = "Field `SOF_SYNC` writer - Host: Delay packet(s) until after SOF"]
pub type SOF_SYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_EN` reader - Host: Enable SOF generation (for full speed bus)"]
pub type SOF_EN_R = crate::BitReader;
#[doc = "Field `SOF_EN` writer - Host: Enable SOF generation (for full speed bus)"]
pub type SOF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEP_ALIVE_EN` reader - Host: Enable keep alive packet (for low speed bus)"]
pub type KEEP_ALIVE_EN_R = crate::BitReader;
#[doc = "Field `KEEP_ALIVE_EN` writer - Host: Enable keep alive packet (for low speed bus)"]
pub type KEEP_ALIVE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_EN` reader - Host: Enable VBUS"]
pub type VBUS_EN_R = crate::BitReader;
#[doc = "Field `VBUS_EN` writer - Host: Enable VBUS"]
pub type VBUS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` writer - Device: Remote wakeup. Device can initiate its own resume after suspend."]
pub type RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_BUS` writer - Host: Reset bus"]
pub type RESET_BUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLDOWN_EN` reader - Host: Enable pull down resistors"]
pub type PULLDOWN_EN_R = crate::BitReader;
#[doc = "Field `PULLDOWN_EN` writer - Host: Enable pull down resistors"]
pub type PULLDOWN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PULLUP_EN` reader - Device: Enable pull up resistor"]
pub type PULLUP_EN_R = crate::BitReader;
#[doc = "Field `PULLUP_EN` writer - Device: Enable pull up resistor"]
pub type PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPU_OPT` reader - Device: Pull-up strength (0=1K2, 1=2k3)"]
pub type RPU_OPT_R = crate::BitReader;
#[doc = "Field `RPU_OPT` writer - Device: Pull-up strength (0=1K2, 1=2k3)"]
pub type RPU_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSCEIVER_PD` reader - Power down bus transceiver"]
pub type TRANSCEIVER_PD_R = crate::BitReader;
#[doc = "Field `TRANSCEIVER_PD` writer - Power down bus transceiver"]
pub type TRANSCEIVER_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECT_DM` reader - Direct control of DM"]
pub type DIRECT_DM_R = crate::BitReader;
#[doc = "Field `DIRECT_DM` writer - Direct control of DM"]
pub type DIRECT_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECT_DP` reader - Direct control of DP"]
pub type DIRECT_DP_R = crate::BitReader;
#[doc = "Field `DIRECT_DP` writer - Direct control of DP"]
pub type DIRECT_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECT_EN` reader - Direct bus drive enable"]
pub type DIRECT_EN_R = crate::BitReader;
#[doc = "Field `DIRECT_EN` writer - Direct bus drive enable"]
pub type DIRECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INT_NAK` reader - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
pub type EP0_INT_NAK_R = crate::BitReader;
#[doc = "Field `EP0_INT_NAK` writer - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
pub type EP0_INT_NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INT_2BUF` reader - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
pub type EP0_INT_2BUF_R = crate::BitReader;
#[doc = "Field `EP0_INT_2BUF` writer - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
pub type EP0_INT_2BUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INT_1BUF` reader - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
pub type EP0_INT_1BUF_R = crate::BitReader;
#[doc = "Field `EP0_INT_1BUF` writer - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
pub type EP0_INT_1BUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_DOUBLE_BUF` reader - Device: EP0 single buffered = 0, double buffered = 1"]
pub type EP0_DOUBLE_BUF_R = crate::BitReader;
#[doc = "Field `EP0_DOUBLE_BUF` writer - Device: EP0 single buffered = 0, double buffered = 1"]
pub type EP0_DOUBLE_BUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP0_INT_STALL` reader - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
pub type EP0_INT_STALL_R = crate::BitReader;
#[doc = "Field `EP0_INT_STALL` writer - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
pub type EP0_INT_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    pub fn start_trans(&mut self) -> START_TRANS_W<SIE_CTRL_SPEC> {
        START_TRANS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Host: Send Setup packet"]
    #[inline(always)]
    #[must_use]
    pub fn send_setup(&mut self) -> SEND_SETUP_W<SIE_CTRL_SPEC> {
        SEND_SETUP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Host: Send transaction (OUT from host)"]
    #[inline(always)]
    #[must_use]
    pub fn send_data(&mut self) -> SEND_DATA_W<SIE_CTRL_SPEC> {
        SEND_DATA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Host: Receive transaction (IN to host)"]
    #[inline(always)]
    #[must_use]
    pub fn receive_data(&mut self) -> RECEIVE_DATA_W<SIE_CTRL_SPEC> {
        RECEIVE_DATA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Host: Stop transaction"]
    #[inline(always)]
    #[must_use]
    pub fn stop_trans(&mut self) -> STOP_TRANS_W<SIE_CTRL_SPEC> {
        STOP_TRANS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    #[must_use]
    pub fn preamble_en(&mut self) -> PREAMBLE_EN_W<SIE_CTRL_SPEC> {
        PREAMBLE_EN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    #[must_use]
    pub fn sof_sync(&mut self) -> SOF_SYNC_W<SIE_CTRL_SPEC> {
        SOF_SYNC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    #[must_use]
    pub fn sof_en(&mut self) -> SOF_EN_W<SIE_CTRL_SPEC> {
        SOF_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    #[must_use]
    pub fn keep_alive_en(&mut self) -> KEEP_ALIVE_EN_W<SIE_CTRL_SPEC> {
        KEEP_ALIVE_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Host: Enable VBUS"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_en(&mut self) -> VBUS_EN_W<SIE_CTRL_SPEC> {
        VBUS_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<SIE_CTRL_SPEC> {
        RESUME_W::new(self, 12)
    }
    #[doc = "Bit 13 - Host: Reset bus"]
    #[inline(always)]
    #[must_use]
    pub fn reset_bus(&mut self) -> RESET_BUS_W<SIE_CTRL_SPEC> {
        RESET_BUS_W::new(self, 13)
    }
    #[doc = "Bit 15 - Host: Enable pull down resistors"]
    #[inline(always)]
    #[must_use]
    pub fn pulldown_en(&mut self) -> PULLDOWN_EN_W<SIE_CTRL_SPEC> {
        PULLDOWN_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Device: Enable pull up resistor"]
    #[inline(always)]
    #[must_use]
    pub fn pullup_en(&mut self) -> PULLUP_EN_W<SIE_CTRL_SPEC> {
        PULLUP_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    #[must_use]
    pub fn rpu_opt(&mut self) -> RPU_OPT_W<SIE_CTRL_SPEC> {
        RPU_OPT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Power down bus transceiver"]
    #[inline(always)]
    #[must_use]
    pub fn transceiver_pd(&mut self) -> TRANSCEIVER_PD_W<SIE_CTRL_SPEC> {
        TRANSCEIVER_PD_W::new(self, 18)
    }
    #[doc = "Bit 24 - Direct control of DM"]
    #[inline(always)]
    #[must_use]
    pub fn direct_dm(&mut self) -> DIRECT_DM_W<SIE_CTRL_SPEC> {
        DIRECT_DM_W::new(self, 24)
    }
    #[doc = "Bit 25 - Direct control of DP"]
    #[inline(always)]
    #[must_use]
    pub fn direct_dp(&mut self) -> DIRECT_DP_W<SIE_CTRL_SPEC> {
        DIRECT_DP_W::new(self, 25)
    }
    #[doc = "Bit 26 - Direct bus drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn direct_en(&mut self) -> DIRECT_EN_W<SIE_CTRL_SPEC> {
        DIRECT_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_nak(&mut self) -> EP0_INT_NAK_W<SIE_CTRL_SPEC> {
        EP0_INT_NAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_2buf(&mut self) -> EP0_INT_2BUF_W<SIE_CTRL_SPEC> {
        EP0_INT_2BUF_W::new(self, 28)
    }
    #[doc = "Bit 29 - Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_1buf(&mut self) -> EP0_INT_1BUF_W<SIE_CTRL_SPEC> {
        EP0_INT_1BUF_W::new(self, 29)
    }
    #[doc = "Bit 30 - Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_double_buf(&mut self) -> EP0_DOUBLE_BUF_W<SIE_CTRL_SPEC> {
        EP0_DOUBLE_BUF_W::new(self, 30)
    }
    #[doc = "Bit 31 - Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_int_stall(&mut self) -> EP0_INT_STALL_W<SIE_CTRL_SPEC> {
        EP0_INT_STALL_W::new(self, 31)
    }
}
#[doc = "SIE control register  

You can [`read`](crate::generic::Reg::read) this register and get [`sie_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sie_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIE_CTRL_SPEC;
impl crate::RegisterSpec for SIE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ctrl::R`](R) reader structure"]
impl crate::Readable for SIE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sie_ctrl::W`](W) writer structure"]
impl crate::Writable for SIE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIE_CTRL to value 0"]
impl crate::Resettable for SIE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
