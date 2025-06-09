options {
    StringPreFixLenType = u16;
	RepeatPreFixSizeType = u16;
    LittleEndian = true;
}

root packet SampleBinary {
    uint16 MsgType `消息类型`,
    u16 BodyLength `消息体长度`,
    match MsgType {
		4 : RiskControlRequest,
		5 : RiskControlResponse,
	}
}


packet RiskControlRequest {
    string UniqueOrderId `唯一订单号`,
    char[16] ClOrdID `客户订单号`,
    char[3] MarketID `市场id`,
    char[12] SecurityID `证券代码`,
    char Side `买卖方向`,
    char OrderType `订单类型`,
    u64 Price `价格`,
    u32 Qty `数量`,
    repeat string ExtraInfo `附加信息`,
    SubOrder {
  		char[16] ClOrdID `子订单号`,
  		u64 Price `子订单价格`,
  		u32 Qty `子订单数量`,
	}
}

packet RiskControlResponse {
    string UniqueOrderId `唯一订单号`,
    i32 Status `状态`,
    string Msg `结果信息`,
}

packet Detail {
    string RuleName `规则名称`,
    u16 Code `原因代码`,
}
