//base on reference/IS122_TDGW_Move_Binary_CV0.57_MTP_Test_20230926.pdf
options {
	StringPreFixLenType = u16;
	RepeatPreFixSizeType = u16;
	LittleEndian = false;
}

MetaData DataType {
	uint32 date `日期`,
	int64 price `价格`,
	int64 quantity `数量`,
	uint64 ntime `时间`,
}

root packet SseBinary {
    uint32 MsgType `消息类型`,
    uint64 MsgSeqNum `消息序列号`,
    uint32 MsgBodyLen `消息体长度`,
    match MsgType {
		33 : Heartbeat,
		40 : Logon,
		41 : Logout,
		58 : NewOrderSingle,
		61 : OrderCancel,
		32 : Confirm,
		59 : CancelReject,
		103 : Report,
		204 : OrderReject,
		209 : PlatformState,
		208 : ExecRptInfo,
		206 : ExecRptSync,
		207 : ExecRptSyncRsp,
		210 : ExecRptEndOfStream,
	}
    uint32 Checksum `校验和`,
}

packet Heartbeat {
}

packet Logon {
    char[32] SenderCompID `发送方代码`,
    char[32] TargetCompID `接收方代码`,
    uint16 HeartBtInt `心跳间隔，单位为秒`,
    char[8] PrtclVersion `接入协议版本号，格式为 aa.bb，如 0.30，1.00`,
    date TradeDate `交易日`,
    uint32 QSize `预留`
}

packet Logout {
    uint32 SessionStatus `注销状态码`,
    char[64] Text `文本描述`
}

packet NewOrderSingle {
    uint32 BizID `业务编号`,
    char[8] BizPbu `业务 PBU 编号，前 5 位有效`,
    char[10] ClOrdID `会员内部订单编号`,
    char[12] SecurityID `证券代码，前 6 位有效`,
    char[13] Account `证券账户，前 10 位有效`,
    uint8 OwnerType `订单所有者类型，暂不启用`,
    char Side `买卖方向 1=买 2=卖`,
    price Price `申报价格`,
    quantity OrderQty `申报数量`,
    char OrdType `订单类型 1=市转撤 2=限价 3=市转限 4=本方最优 5=对手方最优`,
    char TimeInForce `订单有效时间类型 0=当日有效`,
    ntime TransactTime `申报时间`,
    char[2] CreditTag `信用标签，用于现货竞价交易业务的信用交易，取值：XY=担保品买卖 RZ=融资交易 RQ=融券交易 PC=平仓交易 其他业务填写默认值，无意义`,
    char[8] ClearingFirm `结算会员代码，前 5 位有效`,
    char[8] BranchID `营业部代码，前 5 位有效`,
    char[32] UserInfo `用户私有信息，前 12 位有效`
}

packet OrderCancel {
    uint32 BizID `业务编号`,
    char[8] BizPbu `业务 PBU 编号`,
    char[10] ClOrdID `会员内部订单编号`,
    char[12] SecurityID `证券代码`,
    char[13] Account `证券账户，暂不启用`,
    uint8 OwnerType `订单所有者类型，暂不启用`,
    char Side `买卖方向，暂不启用`,
    char[10] OrigClOrdID `原始会员内部订单编号，指待撤原订单的 ClOrdID`,
    ntime TransactTime `申报时间`,
    char[8] BranchID `营业部代码，暂不启用`,
    char[32] UserInfo `用户私有信息，前 12 位有效`
}

packet Confirm {
    char[8] Pbu `登录或订阅 Pbu`,
    uint32 SetID `平台内分区号`,
    uint64 ReportIndex `执行报告编号 从 1 开始连续递增编号`,
    uint32 BizID `业务编号`,
    char ExecType `执行类型 0=订单申报成功 4=订单撤销成功 8=订单申报拒绝`,
    char[8] BizPbu `业务 PBU 编号`,
    char[10] ClOrdID `会员内部订单编号`,
    char[12] SecurityID `证券代码`,
    char[13] Account `证券账户`,
    uint8 OwnerType `订单所有者类型，暂不启用`,
    char Side `买卖方向`,
    price Price `申报价格`,
    quantity OrderQty `申报数量`,
    quantity LeavesQty `剩余数量`,
    quantity CxlQty `撤单数量`,
    char OrdType `订单类型`,
    char TimeInForce `订单有效时间类型 0=当日有效`,
    char OrdStatus `订单状态 0=新订单 4=已撤销 8=已拒绝`,
    char[2] CreditTag `信用标签`,
    char[10] OrigClOrdID `原始会员内部订单编号，仅撤单成功（ExecType=4）时有意义`,
    char[8] ClearingFirm `结算会员代码`,
    char[8] BranchID `营业部代码`,
    uint32 OrdRejReason `订单拒绝码，仅拒绝响应（ExecType=8）时有意义`,
    char[16] OrdCnfmID `交易所订单编号，仅订单申报成功（ExecType=0）有意义`,
    char[16] OrigOrdCnfmID `暂不启用`,
    date TradeDate `交易日期`,
    ntime TransactTime `回报时间`,
    char[32] UserInfo `用户私有信息，前 12 位有效`
}

packet CancelReject {
    char[8] Pbu `登录或订阅 Pbu`,
    uint32 SetID `平台内分区号`,
    uint64 ReportIndex `执行报告编号`,
    uint32 BizID `业务编号`,
    char[8] BizPbu `业务 PBU 编号`,
    char[10] ClOrdID `会员内部订单编号`,
    char[12] SecurityID `证券代码`,
    char[10] OrigClOrdID `原始会员内部订单编号`,
    char[8] BranchID `营业部代码，暂不启用`,
    uint32 CxlRejReason `撤单拒绝码`,
    date TradeDate `交易日期`,
    ntime TransactTime `回报时间`,
    char[32] UserInfo `用户私有信息，前 12 位有效`
}

packet Report {
    char[8] Pbu `登录或订阅 Pbu`,
    uint32 SetID `平台内分区号`,
    uint64 ReportIndex `执行报告编号 从 1 开始连续递增编号`,
    uint32 BizID `业务编号`,
    char ExecType `执行类型 0=订单申报成功 4=订单撤销成功 8=订单申报拒绝`,
    char[8] BizPbu `业务 PBU 编号`,
    char[10] ClOrdID `会员内部订单编号`,
    char[12] SecurityID `证券代码`,
    char[13] Account `证券账户`,
    uint8 OwnerType `订单所有者类型，暂不启用`,
    char Side `买卖方向`,
    price Price `申报价格`,
    quantity OrderQty `申报数量`,
    quantity LeavesQty `剩余数量`,
    quantity CxlQty `撤单数量`,
    char OrdType `订单类型`,
    char TimeInForce `订单有效时间类型 0=当日有效`,
    char OrdStatus `订单状态 0=新订单 4=已撤销 8=已拒绝`,
    char[2] CreditTag `信用标签`,
    char[10] OrigClOrdID `原始会员内部订单编号，仅撤单成功（ExecType=4）时有意义`,
    char[8] ClearingFirm `结算会员代码`,
    char[8] BranchID `营业部代码`,
    uint32 OrdRejReason `订单拒绝码，仅拒绝响应（ExecType=8）时有意义`,
    char[16] OrdCnfmID `交易所订单编号，仅订单申报成功（ExecType=0）有意义`,
    char[16] OrigOrdCnfmID `暂不启用`,
    date TradeDate `交易日期`,
    ntime TransactTime `回报时间`,
    char[32] UserInfo `用户私有信息，前 12 位有效`
}

packet OrderReject {
    uint32 BizID `业务编号`,
    char[8] BizPbu `业务 PBU 编号`,
    char[10] ClOrdID `会员内部订单编号`,
    char[12] SecurityID `证券代码`,
    uint32 OrdRejReason `订单拒绝码`,
    date TradeDate `交易日期`,
    ntime TransactTime `回报时间`,
    char[32] UserInfo `用户私有信息，前 12 位有效`
}

packet PlatformState {
    uint16 PlatformID `平台标识 0=竞价平台`,
    uint16 PlatformState `平台状态 0=NotOpen未开放 1=PreOpen预开放 2=Open开放 3=Break暂停 4=Close关闭`
}

packet ExecRptInfo {
    uint16 PlatformID `平台标识 0=竞价平台`,
    repeat char[8] Pbu `登录或订阅 PBU`,//size uint16
    repeat uint32 SetID `平台内分区号`//size uint16
}

packet ExecRptSync {
    repeat SubExecRptSync {
		char[8] Pbu `登录或订阅 PBU`,
		uint32 SetID `平台内分区号`,
		uint64 BeginReportIndex `分区预期回报序号，暂不支持2^32及更大取值`
	}//size uint16
}

packet ExecRptSyncRsp {
    repeat SubExecRptSyncRsp {
		char[8] Pbu `登录或订阅 PBU`,
		uint32 SetID `平台内分区号`,
		uint64 BeginReportIndex `分区回报序号起点`,
		uint64 EndReportIndex `分区最大回报序号`,
		uint32 RejReason `拒绝码`,
		char[64] Text `描述`
	}
}

packet ExecRptEndOfStream {
    char[8] Pbu `登录或订阅 PBU`,
    uint32 SetID `分区`,
    uint64 EndReportIndex `执行报告流最大序号，本消息编入该分区执行报告编号序列`
}
