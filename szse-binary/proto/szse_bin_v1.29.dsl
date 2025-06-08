//base on reference/深圳证券交易所Binary交易数据接口规范(Ver1.29).pdf

options {
	StringPreFixLenType = u32;
	RepeatPreFixSizeType = u32;
  LittileEndian = false;
}

MetaData DataType {
    Int64 Price `价格，N13(4)`,
    Int64 Qty `数量，N15(2)`
    Int64 DeliveryQty `数量，N15(2)`
    Int64 Amt `金额，N18(4)`
    Int64 SeqNum `消息序号`
    uInt16 Boolean `1=True/Yes,0=False/No`
    uInt32 Length `长度表示字节为单位的数据长度，正数`
    Int64 LocalTimestamp `本地时间戳，YYYYMMDDHHMMSSsss（毫秒），YYYY = 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (秒)，sss=000-999 (毫秒)。`
    Int64 LocalTimeStamp `本地时间戳，YYYYMMDDHHMMSSsss（毫秒），YYYY = 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (秒)，sss=000-999 (毫秒)。`
    uInt32 NumInGroup `重复数，表示重复组的个数，正数`
    uInt32 LocalMktDate `本地市场日期，格式 YYYYMMDD，YYYY =     0000-9999, MM = 01-12, DD = 01-31`
    char[8] SecurityID `证券代码`
    char[6] PBUID `交易单元代码`
    char[12] AccountID `证券账户`
    char[4] BranchID `营业部代码`
    char[6] MemberID `交易商代码`
    char[2] InvestorType `交易主体类型，01=自营，02=资管，03=机构经  纪，04=个人经纪`
    char[10] InvestorID `交易主体代码`
    char[8] TraderCode `交易员代码`
    char[120] InvestorName `客户名称，可包含中文字符，最长 120 个字节`
}

MetaData SessionField {
    uInt32 BodyLength `消息体长度`
    uInt32 MsgType `消息类型`
    char[20] SenderCompID `发送方代码`
    char[20] TargetCompID `接收方代码`
    char[16] Password `密码`
    char[200] Text `文本信息，可能包含中文字符，表示最多 200 个字节`
    Int32 HeartBtInt `心跳监测的时间间隔，为系统设定值，以秒为单位`
    char[32] DefaultApplVerID `通信版本号，填写为 n.xy，如通信版本号为 1.01 时，则字段应设置为 1.01。`
    Int32 SessionStatus `会话状态`
}

MetaData BusinessField {
    Int32 AlertRatio `预警线，单位为百分比，定义为 N6(2)`
    char[3] ApplID `应用标识`
    Amt AccruedInterestAmt `借贷费用金额`
    Amt AllocSettlCurrAmt `现金结算场内支付金额`
    Int16 BasketID `篮子编号，从 1 开始编号，最大不超过 99`
    char BidPositionEffect `买开平仓标志，O 表示开仓，C 表示平仓`
    Price BidPx `买报价`
    Qty BidSize `买数量`
    uInt16 BidExecInstType `竞买成交方式，1=单一主体中标，2=多主体单一价格中标，3=多主体多重价格中标`
    uInt16 BidTransType `竞买业务类别，1=竞买预约申报，2=竞买发起申报，3=竞买应价申报`
    BranchID BranchID `营业部代码`
    uInt16 BusinessRejectReason `拒绝原因`
    char[10] BusinessRejectRefID `被拒绝消息对应的业务层 ID`
    char[50] BusinessRejectText `拒绝原因说明，可能包含中文字符，表示最多 50 个字节`
    char CashMargin `融资融券信用标识，1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓`
    Amt CashOrderQty `订单现金金额`
    char[2] ClearingFirm `结算机构代码，00=中国证券登记结算总公司；01=中国证券登记结算公司深圳分公司；09=自建 TA`
    char[10] ClOrdID `客户订单编号`
    char[8] ConfirmID `约定号`
    char[30] ContactInfo `联系方式，可能包含中文字符，表示最多 30 个字节`
    char[12] Contactor `联系人，可能包含中文字符，表示最多 12 个字节`
    char[6] ContractAccountCode `合约账户标识码`
    AccountID CounterpartyAccountID `对手方证券账户`
    BranchID CounterpartyBranchID `对手方营业部代码`
    char[8] CounterpartyConfirmID `对手方约定号`
    char[16] CounterpartyExecID `交易所赋予的执行编号，单个交易日内不重复`
    char[17] CounterpartyExecutingTrader `对手方交易账户`
    Qty CounterpartyLastQty `对手方数量`
    Price CounterpartyLastPx `对手方成交价格`
    InvestorID CounterpartyInvestorID `对手方交易主体代码`
    InvestorName CounterpartyInvestorName `对手方客户名称`
    InvestorType CounterpartyInvestorType `对手方交易主体类型`
    MemberID CounterpartyMemberID `对手方交易商代码`
    char[160] CounterpartyMemo `对手方备注`
    PBUID CounterpartyPBUID `对手方交易单元`
    TraderCode CounterpartyTraderCode `对手方交易员代码`
    uInt8 CoveredOrUncovered `备兑标签，衍生品交易填写，0=Covered，表示备兑仓；1=UnCovered，表示普通仓`
    Qty CumQty `累计执行数量`
    uInt16 CxlRejReason `撤单拒绝原因代码，正数`
    AccountID DeductionAccountID `用于扣划证券的证券账户`
    PBUID DeductionPBU `用于扣划证券的交易单元`
    Qty DeliverQty `证券交付数量`
    char DeliverySide `方向，1=质押，2=解押`
    char DesignationInstruction `注册指令，3 表示转托管`
    uInt8 DesignationTransType `注册指令类型，1=New，表示新注册请求；3=Cancel，表示注册撤单`
    AccountID DisposalAccountID `待处置券存放账户`
    uInt8 DisposalFlag `处置标识，0=不同意，1=同意`
    PBUID DisposalPBU `待处置券存放交易单元`
    char[16] ExecID `交易所赋予的执行编号，单个交易日内不重复`
    char ExecType `执行报告类型，0=New，表示新订单；4=Cancelled，表示已撤销；8=Reject，表示已拒绝；F=Trade，表示已成交`
    char[17] ExecutingTrader `本方交易账户`
    uInt16 ExpirationDays `期限，单位为天数`
    uInt8 ExpirationExecInst `到期续做方式，1=到期自动续做；2=到期手动续做`
    uInt8 ExpirationType `期限类型，1=固定期限`
    LocalTimestamp ExpireTime `询价请求失效时间`
    char[6] FundPBUID `结算账号`
    uInt16 GracePeriod `申报宽限期、违约宽限期，单位为天数`
    Price HighLimitPrice `价格上限`
    char[] IMCRejectText `国际市场互联对方市场拒绝原因说明，变长字符串，长度由 IMCRejectTextLen 字段指示`
    Length IMCRejectTextLen `指示 IMCRejectText 字段长度`
    char[8] InsufficientSecurityID `申购赎回不足成份股证券代码`
    char[2] InvestmentType `资金用途类型，01=生产经营，02=补充流动资金，03=股权类投资，04=债权类投资，05=不动产投资，06=偿还债务，07=个人消费，99=其他`
    char[10] IOIID `客户意向申报编号`
    Qty IOIQty `意向申报数量`
    char[10] IOIRefID `原意向申报 IOIID，撤销意向申报时使用`
    char IOITransType `意向申报类型，N=意向申报，C=意向申报撤单`
    Price LastParPx `成交价格二`
    Price LastPx `成交价格`
    Qty LastQty `成交数量`
    Qty LeavesQty `订单剩余数量`
    Qty LegOrderQty `单腿数量`
    char[8] LegSecurityID `单腿证券代码`
    char[4] LegSecurityIDSource `单腿证券代码源`
    char LegSide `单腿方向`
    AccountID LenderAccountID `出借券证券账户`
    PBUID LenderPBU `出借券交易单元`
    char LotType `订单数量类型，1=零股订单（Odd Lot），2=整手订单（Round Lot）`
    Price LowLimitPrice `价格下限`
    Amt MarginAmount `保证金金额`
    uInt8 MarginItem `保证金条目类别，1=保证金可用余额，2=保证金总金额`
    char[8] MarketID `市场代码`
    char[8] MarketSegmentID `市场板块代码`
    LocalMktDate MaturityDate `到期日`
    Qty MaxFloor `可显示数量`
    uInt16 MaxPriceLevels `最多成交价位数，正数，0 表示不限制成交价位数`
    char[160] Memo `备注，可包含中文字符，最长160个字节`
    char[] MemoEx `扩展备注，变长字符串，长度由 MemoExLen 字段指示，可包含中文字符`
    Length MemoExLen `扩展备注长度，指示 MemoEx 字段长度`
    Qty MinQty `最低成交数量`
    NumInGroup NoBaskets `质押篮子个数`
    NumInGroup NoCounterparty `对手方个数`
    NumInGroup NoCounterpartyPBU `对手方交易单元个数`
    NumInGroup NoLegs `多腿订单包含的单腿数`
    NumInGroup NoMarginItems `保证金条目个数`
    NumInGroup NoPartitions `分区个数`
    NumInGroup NoPartyIDs `参与方个数`
    NumInGroup NoQuote `报价消息个数`
    NumInGroup NoSecurity `证券只数`
    AccountID OfferAccountID `卖方证券账户`
    BranchID OfferBranchID `卖方营业部代码`
    PBUID OfferPBUID `卖方交易单元`
    char OfferPositionEffect `卖开平仓标志，O=开仓，C=平仓`
    Price OfferPx `卖报价`
    Qty OfferSize `卖数量`
    char[16] OrderID `交易所订单编号，交易所赋予的订单编号，跨交易日不重复`
    Qty OrderQty `订单数量`
    char[4] OrderRestrictions `订单限定，1=程序化交易，E=算法交易，最多可同时填写四种限定类型，如没有限定类型则填空`
    uInt16 OrdRejReason `订单拒绝原因代码，正数`
    char OrdStatus `订单状态，0=New（新订单），1=Partially filled（部分成交），2=Filled（全部成交），4=Cancelled（已撤销），8=Reject（已拒绝）`
    char OrdType `订单类型，1=市价，2=限价，U=本方最优`
    char[10] OrigClOrdID `原订单 ClOrdID`
    PBUID OrigSubmittingPBUID `初始交易申报交易单元`
    LocalMktDate OrigTradeDate `原成交申报日期`
    char[16] OrigTradeID `初始成交申报的 TradeID`
    char[10] OrigTradeReportID `初始交易客户成交申报编号`
    uInt16 OwnerType `所有者类型，正数，1=个人投资者发起，101=交易所发起，102=会员发起，103=机构投资者发起，104=自营交易发起，105=流动性服务提供商发起，106=结算结构发起`
    Int32 PartitionNo `平台分区号`
    uInt16 PlatformID `平台号，1=现货集中竞价交易平台，2=综合金融服务平台，3=非交易处理平台，4=衍生品集中竞价交易平台，5=国际市场互联平台，6=固定收益交易平台`
    uInt16 PlatformState `平台状态，0=PreOpen（未开放），1=OpenUpComing（即将开放），2=Open（开放），3=Halt（暂停开放），4=Close（关闭）`
    uInt16 PledgeeType `质权人类型`
    char PositionEffect `开平仓标识，O=开仓，C=平仓`
    uInt8 PreTradeAnonymity `是否匿名，0=显名，1=匿名`
    Price Price `价格`
    uInt8 PriceType `价格类型，1=百分比，2=每交易单位，3=固定数量`
    uInt8 PrivateQuote `私有报价，0=私有报价（Private Quote），1=公开报价（Public Quote）`
    char[10] QuoteID `报价消息编号`
    char[10] QuoteMsgID `客户报价消息编号`
    Price QuotePrice `报价价格`
    uInt16 QuotePriceType `报价价格类型，预留，1=百分比（Percent of par），2=每股（Per Share）`
    Qty QuoteQty `报价数量`
    uInt16 QuoteRejectReason `意向申报、报价拒绝原因代码，正数`
    char[50] QuoteRejectText `报价拒绝原因说明`
    char[10] QuoteReqID `报价请求 ID`
    uInt16 QuoteRequestRejectReason `询价请求拒绝原因代码`
    char[50] QuoteRequestRejectText `询价请求拒绝原因说明`
    uInt8 QuoteRequestStatus `询价请求状态，0=Accepted（已接受），4=Cancelled（已撤销），5=Rejected（已拒绝），7=Expired（已超时），8=Filled（已成交）`
    Int16 QuoteRequestTransType `询价请求事务类型，0-New（新订单），1-Cancel（撤销）`
    uInt16 QuoteRequestType `询价请求类型，101=Submit（提交），102=Alleged（转发）`
    char[16] QuoteRespID `交易所意向申报编号或报价回复消息编号，交易所赋予的意向申报编号，跨交易日不重复`
    uInt8 QuoteRespType `对于报价回复消息，为报价回复类型，取值为：1=Hit/Lift（接受），2=Counter（重报），6=Pass（拒绝）；对于意向申报，为意向申报响应类型，取值为：2=意向申报响应`
    uInt8 QuoteStatus `报价状态，0=Accepted（接受），4=Cancelled（已撤销），5=Rejected（拒绝），7=Expired（已超时）`
    uInt8 QuoteType `报价类型，1=Tradeable（表示可交易的报价）`
    uInt32 RefMsgType `被拒绝的消息类型`
    SeqNum RefSeqNum `引用的消息序号`
    char[16] RejectText `拒绝原因说明，可能包含中文字符，最多16个字节`
    SeqNum ReportIndex `回报记录号，从1开始编号`
    PBUID ReportingPBUID `回报交易单元`
    char[50] TradeReportRejectText `成交申报拒绝原因说明`
    char[16] RptExecID `成交申报执行编号`
    char[16] SecondaryExecID `第二执行编号`
    char[16] SecondaryOrderID `第二交易所订单编号`
    char[8] SecurityID `证券代码`
    char[4] SecurityIDSource `证券代码源，102=深圳证券交易所`
    char[8] SecurityType `证券类别，MLEG=组合`
    char[8] SecuritySubType `证券子类别`
    Amt SettlCurrAmt `结算金额`
    Int32 SettlementRatio `平仓线，单位为百分比，定义为N6(2)`
    uInt8 SettlPeriod `结算周期，0=T+0，1=T+1，2=T+2，3=T+3`
    uInt16 SettlType `结算方式，0=不指定，103=多边净额，104=逐笔全额`
    char[2] ShareProperty `股份性质，00-无限售流通股，01-首发后限售股，05-首发前限售股，07-首发后可出借限售股`
    char Side `买卖方向，1=买，2=卖，G=借入，F=出借，D=申购，E=赎回`
    Price StopPx `止损价格`
    InvestorID SubmittingInvestorID `申报方交易主体代码`
    InvestorName SubmittingInvestorName `申报方客户名称`
    InvestorType SubmittingInvestorType `申报方交易主体类型`
    MemberID SubmittingMemberID `申报交易商代码`
    PBUID SubmittingPBUID `申报交易单元`
    TraderCode SubmittingTraderCode `申报方交易员代码`
    Amt SubstCash `执行报告现金替代金额或成交申报现金结算金额`
    char[] SupplementaryAgreement `补充约定，变长字符串，长度由SupplementaryAgreementLen字段指示，可包含中文字符，最长600个字节`
    Length SupplementaryAgreementLen `补充约定长度指示，SupplementaryAgreement字段长度，必须小于等于600`
    char[6] Tenderer `要约收购参与人编码`
    char TimeInForce `订单有效时期类型，0=当日有效（Day），3=即时成交或取消（IOC），9=At Crossing（港股通竞价限价盘）`
    uInt16 TimeToExpiration `期限，单位为天数，正数`
    LocalMktDate TradeDate `开始日期`
    char TradeHandlingInstr `成交申报模式，0=Trade Confirm（成交确认），1=Two-Party Report（成交报告），2=One-Party Report For Matching（协议配对），3=One-Party Report for Pass Through（成交请求），5=Two Party Report for Claim（双边确认成交报告）`
    char[16] TradeID `交易所成交申报编号，交易所赋予的成交申报编号，跨交易日不重复`
    char[10] TradeReportID `客户成交申报编号`
    char[10] TradeReportRefID `原客户成交申报编号`
    uInt16 TradeReportRejectReason `成交申报拒绝原因代码`
    uInt8 TradeReportTransType `成交申报事务类别，0=New（新申报），1=Cancel（撤销申报），2=Replace（响应）`
    uInt8 TradeReportType `成交申报消息类型，0=Submit（提交成交申报），1=Alleged（转发成交申报），2=Accept（接受成交申报），3=Decline（拒绝成交申报）`
    char[4] TradingSessionID `交易会话 ID`
    char[4] TradingSessionSubID `交易会话子 ID`
    LocalTimeStamp TradSesEndTime `交易会话结束时间`
    LocalTimeStamp TradSesStartTime `交易会话开始时间`
    uInt16 TradSesStatus `交易会话状态`
    LocalTimestamp TransactTime `订单或执行的创建时间`
    PBUID TransfereePBUID `转入交易单元代码`
    uInt8 TrdAckStatus `成交申报响应状态，0=Accepted（接受），1=Rejected（拒绝）`
    char[16] TrdMatchID `交易编号，跨交易日唯一`
    uInt8 TrdRptStatus `成交申报状态，0=Accepted（接受），1=Rejected（拒绝），2=Cancelled（已撤销），100=Unmatched（尚未匹配），101=Matched（已匹配）`
    uInt16 TrdSubType `成交申报业务子类别`
    uInt16 TrdType `成交申报类型，正数，如1001=股票质押式回购初始交易，1002=股票质押式回购提前购回`
    Amt UnderlyingCashOrderQty `基础证券券面总额`
    char[8] UnderlyingSecurityID `基础证券代码`
    char[4] UnderlyingSecurityIDSource `基础证券代码源，102=深圳证券交易所`
    char[2] UnderlyingShareProperty `基础证券股份性质，00-无限售流通股，01-首发后限售股`
    char[8] UserInfo `用户私有信息，仅可包含ASCII可显示字符`
    uInt32 ValidationCode `密码激活校验号，正数`
    LocalTimestamp ValidUntilTime `报价有效时间`
    char VotingPreference `投票意向，1代表同意，2代表反对，3代表弃权`
    uInt16 VotingProposal `投票议案号，正数，1-99表示具体议案号，100表示总议案`
    uInt16 VotingSubProposal `投票子议案号或选举候选人，正数，0表示所有子议案`
}

root packet SzseBinary {
    MsgType,
    BodyLength,
    match MsgType {
        1 : Logon,
        2 : Logout,
        3 : Heartbeat,
        4 : BusinessReject,
        5 : ReportSynchronization,
        6 : PlatformStateInfo,
        7 : ReportFinished,
        9 : PlatformPartition,
        10: TradingSessionStatus,
        [
            100101,100201,100301,100401,100501,
            100601,100701,101201,101301,101401,
            101501,101601,101701,101801,101901,
            102301,102701,102701,102801,102801,
            102901,102901,103101,103101,106301,
            103301,103501,103701,104101,104128,
            104701
        ] : NewOrder,
        [
            200102,200202,200302,200402,200502,
            200602,200702,201202,201302,201402,
            201502,201602,201702,201802,201902,
            202202,202302,202702,202702,202802,
            202802,202902,202902,203102,203102,
            206302,203302,203302,203502,203502,
            203702,204102,204129,204702
        ] : ExecutionConfirm,
        [
            200115,200215,200315,200415,200515,
            200615,200715,206315,203715,204115,
            204130
        ] : ExecutionReport,
        190007 : OrderCancelRequest,
        290008 : CancelReject,
    },
    int32 Checksum,
}

packet Logon {
    SenderCompID `发送方代码`,
    TargetCompID `接收方代码`,
    HeartBtInt `心跳间隔，单位为秒。订单管理系统登陆时提供给交易网关`,
    Password `密码`,
    DefaultApplVerID `二进制协议版本，即通信版本号`,
}

packet Logout {
    SessionStatus `退出时的会话状态，取值：0 = 会话活跃, 1 = 会话口令已更改, 2 = 将过期的会话口令, 3 = 新会话口令不符合规范, 4 = 会话退登完成, 5 = 不合法的用户名或口令, 6 = 账户锁定, 7 = 当前时间不允许登录, 8 = 口令过期, 9 = 收到的 MsgSeqNum(34)太小, 10 = 收到的 NextExpectedMsgSeqNum(789)太大, 101 = 其他, 102 = 无效消息`,
    Text `文本，注销原因的进一步补充说明`
}

packet Heartbeat {
}

packet NewOrder {
    ApplID `应用标识`,
    SubmittingPBUID `申报交易单元`,
    SecurityID `证券代码`,
    SecurityIDSource `证券代码源`,
    OwnerType `订单所有者类型`,
    ClearingFirm `结算机构代码`,
    TransactTime `委托时间`,
    UserInfo `用户私有信息`,
    ClOrdID `客户订单编号`,
    AccountID `证券账户`,
    BranchID `营业部代码`,
    OrderRestrictions `订单限定`,
    Side `买卖方向`,
    OrdType `订单类别`,
    OrderQty `订单数量`,
    Price `价格`,
    //010,020,030,040,051,052,060,061,070,120,130,140,150,151,152,160,170,180,181,190,191,230,270,271,280,281,290,291,310,311,630,330,331,350,351,370,410,417,470
    match ApplID {
        "010" : Extend100101,
        "020" : Extend100201,
        "030" : Extend100301,
        //"040" : Extend100401,
        ["051", "052"] : Extend100501,
        ["060", "061"] : Extend100601,
        "070" : Extend100701,
        // 120 101201
        // 130 101301
        // 140 101401
        ["150", "151", "152"] : Extend101501,
        "160" : Extend101601,
        "170" : Extend101701,
        ["180", "181"] : Extend101801,
        //["190", "191"] : Extend101901,
        //"230" : Extend102301,
        ["270", "271"] : Extend102701,
        ["280", "281"] : Extend102801,
        ["290", "291"] : Extend102901,
        //310 311 103101
        "630" : Extend106301,
        //330 331 103301
        ["350","351"] : Extend103501,
        "370" : Extend103701,
        "410" : Extend104101,
        "417" : Extend104128,
        "470" : Extend104701,
    }  `各业务扩展字段`
}

packet Extend100101 {
    StopPx `止损价`
    MinQty `最低成交数量`
    MaxPriceLevels `最多成交价位数，0 表示不限制成交价位数`
    TimeInForce `订单有效时间类型`
    CashMargin `信用标识，1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓`
}

packet Extend100201 {
    StopPx `止损价`
    MinQty `最低成交数量`
    MaxPriceLevels `最多成交价位数，0 表示不限制成交价位数`
    TimeInForce `订单有效时间类型`
}
packet Extend100301 {
    StopPx `止损价`
    MinQty `最低成交数量`
    MaxPriceLevels `最多成交价位数，0 表示不限制成交价位数`
    TimeInForce `订单有效时间类型`
}

packet Extend101401 {
    StopPx `止损价`
    MinQty `最低成交数量`
    MaxPriceLevels `最多成交价位数，0 表示不限制成交价位数`
    TimeInForce `订单有效时间类型`
    PositionEffect `平仓标识（‘O’：开仓，‘C’：平仓）`
    CoveredOrUncovered `备兑标签（0=Covered，表示备兑；1=UnCovered，表示非备兑）`
    ContractAccountCode `合约账户标识码`
    SecondaryOrderID `第二交易所订单编号，组合策略单边平仓时填组合策略对应的组合流水号（构建组合申报时返回的执行报告中的 OrderID），其余填全空格`
}

packet Extend100501 {
    ConfirmID `约定号（点击成交申报填写）`
    CashMargin `信用标识（1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓）`
}


packet Extend100601 {
    CashMargin `信用标识（1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓）`
}

packet Extend100701 {
    ExpirationDays `期限，单位为天数`
    ExpirationType `期限类型（1=固定期限）`
    ShareProperty `股份性质（00=无限售流通股；07=首发后可出借限售股。若使用首发后可出借限售股进行出借，则填07，否则填00）`
}

packet Extend101501 {
    ShareProperty `股份性质（00=无限售流通股；01=首发后限售股。定向可转债转股申报使用限售部分转股时填‘01’，其他都填‘00’）`
}

packet Extend101601 {
    ContractAccountCode `合约账户标识码`
}

packet Extend101701 {
    CashOrderQty `申购金额（LOF 为现金申购、份额赎回。申购使用 CashOrderQty，金额为1元的整数倍，OrderQty 填 0；赎回时使用 OrderQty）`
}

packet Extend101801 {
    Tenderer `收购人编码`
}

packet Extend102701 {
    DisposalPBU `划入待处置券的交易单元`
    DisposalAccountID `划入待处置券的证券账户`
}

packet Extend102801 {
    LenderPBU `出借券交易单元`
    LenderAccountID `出借券证券账户`
}

packet Extend102901 {
    DeductionPBU `用于扣划证券的交易单元`
    DeductionAccountID `用于扣划证券的证券账户`
}

packet Extend106301 {
    StopPx `止损价，预留，固定填 0`
    MinQty `最低成交数量，预留，固定填 0`
    MaxPriceLevels `最多成交价位数，预留，固定填 0`
    TimeInForce `订单有效时间类型（0=增强限价盘，Day；9=竞价限价盘，At Crossing）`
    LotType `订单数量类型（1=零股订单，Odd Lot；2=整手订单，Round Lot）`
}

packet Extend103501 {
    ContractAccountCode `合约账户标识码`
}

packet Extend103701 {
    CashMargin `信用标识（1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓）`
}

packet Extend104101 {
    StopPx `止损价`
    MinQty `最低成交数量`
    MaxPriceLevels `最多成交价位数，0 表示不限制`
    TimeInForce `订单有效时间类型`
    CashMargin `信用标识（1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓）`
}

packet Extend104128 {
    MemberID `本方交易商代码`
    InvestorType `本方交易主体类型`
    InvestorID `本方交易主体代码`
    InvestorName `本方客户名称`
    TraderCode `本方交易员代码`
    SecondaryOrderID `第二交易所订单编号（竞买业务类别为2或3时，填写竞买场次编号）`
    BidTransType `竞买业务类别（1=竞买预约申报；2=竞买发起申报；3=竞买应价申报）`
    BidExecInstType `竞买成交方式（1=单一主体中标；2=多主体单一价格中标；3=多主体多重价格中标）`
    LowLimitPrice `价格下限`
    HighLimitPrice `价格上限`
    MinQty `最低成交数量`
    TradeDate `交易日期`
    SettlType `结算方式，预留`
    SettlPeriod `结算周期，预留`
    PreTradeAnonymity `是否匿名（0=显名；1=匿名）`
    CashMargin `信用标识，预留（1=Cash，普通交易；2=Open，融资融券开仓；3=Close，融资融券平仓）`
    Memo `备注`
}

packet Extend104701 {
    SecondaryOrderID `中央国债登记结算有限责任公司生成的唯一的业务标识号，由投资者在委托申报时填写`
}

packet ExecutionConfirm {
    PartitionNo `平台分区号`,
    ReportIndex `回报记录号`,
    ApplID `应用标识`,
    ReportingPBUID `回报交易单元`,
    SubmittingPBUID `申报交易单元`,
    SecurityID `证券代码`,
    SecurityIDSource `证券代码源`,
    OwnerType `订单所有者类型`,
    ClearingFirm `结算机构代码`,
    TransactTime `回报时间`,
    UserInfo `用户私有信息`,
    OrderID `交易所订单编号`,
    ClOrdID `客户订单编号`,
    QuoteMsgID `报价消息编号`,
    OrigClOrdID `原始订单客户订单编号`,
    ExecID `执行编号`,
    ExecType `执行类型`,
    OrdStatus `订单状态`,
    OrdRejReason `撤单/拒绝原因代码`,
    LeavesQty `订单剩余数量`,
    CumQty `累计执行数量`,
    Side `买卖方向`,
    OrdType `订单类别`,
    OrderQty `订单数量`,
    Price `价格`,
    AccountID `证券账户`,
    BranchID `营业部代码`,
    OrderRestrictions `订单限定`,
    match ApplID {
        "010" : Extend200102,
        "020" : Extend200202,
        "030" : Extend200302,
        //"040" : Extend200402,
        ["051", "052"] : Extend200502,
        ["060", "061"] : Extend200602,
        "070" : Extend200702,
        // 120 Extend2012012
        // 130 Extend2013012
        // 140 Extend2014012
        ["150", "151", "152"] : Extend201502,
        "160" : Extend201602,
        "170" : Extend201702,
        ["180", "181"] : Extend201802,
        //["190", "191"] : Extend201902,
        //"220" : Extend202202,
        //"230" : Extend202302,
        ["270", "271"] : Extend202702,
        ["280", "281"] : Extend202802,
        ["290", "291"] : Extend202902,
        //310 311 203102
        "630" : Extend206302,
        //330 331 Extend203302
        ["350","351"] : Extend203502,
        "370" : Extend203702,
        "410" : Extend204102,
        "417" : Extend204129,
        "470" : Extend204702,
    } `各业务扩展字段`
}

packet Extend200102 {
  StopPx `止损价`
  MinQty `最低成交数量`
  MaxPriceLevels `最多成交价位数（0 表示不限制）`
  TimeInForce `订单有效时间类型`
  CashMargin `信用标识`
}

packet Extend200202 {
  StopPx `止损价`
  MinQty `最低成交数量`
  MaxPriceLevels `最多成交价位数（0 表示不限制）`
  TimeInForce `订单有效时间类型`
}
packet Extend200302 {
  StopPx `止损价`
  MinQty `最低成交数量`
  MaxPriceLevels `最多成交价位数（0 表示不限制）`
  TimeInForce `订单有效时间类型`
}

packet Extend200402 {
  StopPx `止损价`
  MinQty `最低成交数量`
  MaxPriceLevels `最多成交价位数（0 表示不限制）`
  TimeInForce `订单有效时间类型`
  PositionEffect `平仓标识（‘O’：开仓，‘C’：平仓）`
  CoveredOrUncovered `备兑标签（0=Covered，1=UnCovered）`
  ContractAccountCode `合约账户标识码`
  SecondaryOrderID `第二交易所订单编号`
}

packet Extend200502 {
  ConfirmID `约定号`
  CashMargin `信用标识`
}

packet Extend200602 {
  CashMargin `信用标识`
}

packet Extend200702 {
  ExpirationDays `期限，单位为天数`
  ExpirationType `期限类型`
  ShareProperty `股份性质`
}

packet Extend201202 {
  InsufficientSecurityID `申赎不足成份股证券代码`
  NoSecurity `成份股记录数`
  UnderlyingSecurityID `成份股证券代码`
  UnderlyingSecurityIDSource `证券代码源`
  DeliveryQty `股份交付数量`
  SubstCash `现金替代金额`
}

packet Extend201502 {
  ShareProperty `股份性质`
}

packet Extend201602 {
  ContractAccountCode `合约账户标识码`
}

packet Extend201702 {
  CashOrderQty `申购金额`
}

packet Extend201802 {
  Tenderer `收购人编码`
}

packet Extend202702 {
  DisposalPBU `划入待处置券的交易单元`
  DisposalAccountID `划入待处置券的证券账户`
}

packet Extend202802 {
  LenderPBU `出借券交易单元`
  LenderAccountID `出借券证券账户`
}

packet Extend202902 {
  DeductionPBU `用于扣划证券的交易单元`
  DeductionAccountID `用于扣划证券的证券账户`
}

packet Extend203102 {
  InsufficientSecurityID `合并不足子基金代码`
  NoSecurity `子基金记录数`
  UnderlyingSecurityID `子基金证券代码`
  UnderlyingSecurityIDSource `证券代码源`
  DeliveryQty `子基金交付数量`
}

packet Extend206302 {
  RejectText `拒绝原因说明（填写联交所拒绝原因代码）`
  StopPx `止损价，预留，固定填 0`
  MinQty `最低成交数量，预留，固定填 0`
  MaxPriceLevels `最多成交价位数`
  TimeInForce `订单有效时间类型`
  LotType `订单数量类型`
  IMCRejectTextLen `联交所拒绝原因说明长度`
  IMCRejectText `联交所拒绝原因说明`
}

packet Extend203502 {
  ContractAccountCode `合约账户标识码`
}

packet Extend203702 {
  CashMargin `信用标识`
}

packet Extend204102 {
  StopPx `止损价`
  MinQty `最低成交数量`
  MaxPriceLevels `最多成交价位数（0 表示不限制）`
  TimeInForce `订单有效时间类型`
  CashMargin `信用标识`
}

packet Extend204129 {
  MemberID `本方交易商代码`
  InvestorType `本方交易主体类型`
  InvestorID `本方交易主体代码`
  InvestorName `本方客户名称`
  TraderCode `本方交易员代码`
  SecondaryOrderID `第二交易所订单编号`
  BidTransType `竞买业务类别`
  BidExecInstType `竞买成交方式`
  LowLimitPrice `价格下限`
  HighLimitPrice `价格上限`
  MinQty `最低成交数量`
  TradeDate `交易日期`
  SettlType `结算方式，预留`
  SettlPeriod `结算周期，预留`
  PreTradeAnonymity `是否匿名`
  CashMargin `信用标识，预留`
  Memo `备注`
}

packet Extend204702 {
  SecondaryOrderID `中央国债登记结算有限责任公司生成的唯一的业务标识号`
}


packet ExecutionReport {
    PartitionNo `平台分区号`,
    ReportIndex `回报记录号`,
    ApplID `应用标识`,
    ReportingPBUID `回报交易单元`,
    SubmittingPBUID `申报交易单元`,
    SecurityID `证券代码`,
    SecurityIDSource `证券代码源`,
    OwnerType `订单所有者类型`,
    ClearingFirm `结算机构代码`,
    TransactTime `回报时间`,
    UserInfo `用户私有信息`,
    OrderID `交易所订单编号`,
    ClOrdID `客户订单编号`,
    QuoteMsgID `报价消息编号`,
    ExecID `执行编号`,
    ExecType `执行类型`,
    OrdStatus `订单状态`,
    LastPx `成交价`,
    LastQty `成交数量`,
    LeavesQty `订单剩余数量`,
    CumQty `累计执行数量`,
    Side `买卖方向`,
    AccountID `证券账户`,
    BranchID `营业部代码`,
    match ApplID {
        "010" : Extend200115,
        "020" : Extend200215,
        "030" : Extend200315,
        //"040" : Extend200415,
        ["051", "052", "056", "057"] : Extend200515,
        ["060", "061"] : Extend200615,
        "070" : Extend200715,
        "630" : Extend206315,
        "370" : Extend203715,
        ["410", "412", "413", "415", "416"] : Extend204115,
        "417" : Extend204130,
        "470" : Extend204715,
    } `各业务扩展字段`
}

packet Extend200115 {
  CashMargin `信用标识`
}

packet Extend200215 {
  MaturityDate `到期日`
}
packet Extend200315 {
  MaturityDate `到期日`
}

packet Extend200415 {
  PositionEffect `平仓标识（‘O’：开仓 ‘C’：平仓）`
  CoveredOrUncovered `备兑标签（0=Covered，表示备兑；1=UnCovered，表示非备兑）`
  ContractAccountCode `合约账户标识码`
  SecondaryOrderID `第二交易所订单编号`
}

packet Extend200515 {
  ConfirmID `约定号`
  CashMargin `信用标识`
}

packet Extend200615 {
  CashMargin `信用标识`
}

packet Extend206315 {
  CashMargin `信用标识`
}

packet Extend200715 {
  ExpirationDays `期限，单位为天数`
  ExpirationType `期限类型`
  MaturityDate `到期日`
  ShareProperty `股份性质`
}
packet Extend204715 {
  ExpirationDays `期限，单位为天数`
  ExpirationType `期限类型`
  MaturityDate `到期日`
  ShareProperty `股份性质`
}

packet Extend203715 {
  CashMargin `信用标识`
}

packet Extend204115 {
  CashMargin `信用标识`
  SettlType `结算方式`
  SettlPeriod `结算周期`
  CounterpartyMemberID `对手方交易商代码`
  CounterpartyInvestorType `对手方交易主体类型`
  CounterpartyInvestorID `对手方交易主体代码`
  CounterpartyInvestorName `对手方客户名称`
  CounterpartyTraderCode `对手方交易员代码`
}

packet Extend204130 {
  MemberID `本方交易商代码`
  InvestorType `本方交易主体类型`
  InvestorID `本方交易主体代码`
  InvestorName `本方客户名称`
  TraderCode `本方交易员代码`
  CounterpartyMemberID `对手方交易商代码`
  CounterpartyInvestorType `对手方交易主体类型`
  CounterpartyInvestorID `对手方交易主体代码`
  CounterpartyInvestorName `对手方客户名称`
  CounterpartyTraderCode `对手方交易员代码`
  SecondaryOrderID `第二交易所订单编号`
  BidTransType `竞买业务类别`
  BidExecInstType `竞买成交方式`
  SettlType `结算方式`
  SettlPeriod `结算周期`
  CashMargin `信用标识`
  Memo `备注`
}


packet OrderCancelRequest {
    ApplID `应用标识`,
    SubmittingPBUID `申报交易单元`,
    SecurityID `证券代码`,
    SecurityIDSource `证券代码源`,
    OwnerType `订单所有者类型`,
    ClearingFirm `结算机构代码`,
    TransactTime `委托时间`,
    UserInfo `用户私有信息`,
    ClOrdID `客户订单编号`,
    OrigClOrdID `原始订单客户订单编号`,
    Side `原始订单买卖方向`,
    OrderID `原始订单交易所订单编号`,
    OrderQty `原始订单数量`
}

packet CancelReject {
    PartitionNo `平台分区号`,
    ReportIndex `回报记录号`,
    ApplID `应用标识`,
    ReportingPBUID `回报交易单元`,
    SubmittingPBUID `申报交易单元`,
    SecurityID `证券代码`,
    SecurityIDSource `证券代码源`,
    OwnerType `订单所有者类型`,
    ClearingFirm `结算机构代码`,
    TransactTime `回报时间`,
    UserInfo `用户私有信息`,
    ClOrdID `客户订单编号`,
    OrigClOrdID `原始订单客户订单编号`,
    Side `原始订单买卖方向`,
    OrdStatus `原始订单当前状态`,
    CxlRejReason `拒绝原因代码`,
    RejectText `拒绝原因说明`,
    OrderID `原始订单交易所订单编号`
}

packet BusinessReject {
    ApplID `应用标识`
    TransactTime `回报时间`
    SubmittingPBUID `申报交易单元`
    SecurityID `证券代码`
    SecurityIDSource `证券代码源`
    RefSeqNum `被拒绝消息的消息序号`
    RefMsgType `被拒绝的消息类型`
    BusinessRejectRefID `被拒绝消息对应的业务层 ID`
    BusinessRejectReason `拒绝原因`
    BusinessRejectText `拒绝原因说明`
}


packet ReportSynchronization {
  NoPartitions `平台分区数量`
  repeat PartitionReport
}

packet PartitionReport {
  PartitionNo `OMS 期望接收的平台分区号（分区号须存在且不重复）`
  ReportIndex `对应分区下 OMS 期望接收的下一条回报的记录号（从1开始）`
}

packet PlatformStateInfo {
  PlatformID `平台号（1=现货集中竞价交易平台 2=综合金融服务平台 3=非交易处理平台 4=衍生品集中竞价交易平台 5=国际市场互联平台 6=固定收益交易平台）`
  PlatformState `平台状态（0=PreOpen，未开放 1=OpenUpComing，即将开放 2=Open，开放 3=Halt，暂停开放 4=Close，关闭）`
}

packet ReportFinished {
  PartitionNo `平台分区号`
  ReportIndex `回报记录号（在本分区各类回报消息中的连续编号）`
  PlatformID `平台号（1=现货集中竞价交易平台 2=综合金融服务平台 3=非交易处理平台 4=衍生品集中竞价交易平台 5=国际市场互联平台 6=固定收益交易平台）`
}

packet PlatformInfo {
  PlatformID `平台号（1=现货集中竞价交易平台 2=综合金融服务平台 3=非交易处理平台 4=衍生品集中竞价交易平台 5=国际市场互联平台 6=固定收益交易平台）`
  NoPartitions `平台分区数量`
  repeat PlatformPartition
}

packet PlatformPartition {
  PartitionNo `平台分区号（不一定连续）`
}

packet TradingSessionStatus {
  MarketID `市场代码（预留）`
  MarketSegmentID `市场板块代码（第一位表示平台号：1=现货集中竞价交易平台，2=综合金融服务平台，3=非交易处理平台，4=衍生品集中竞价交易平台，5=国际市场互联平台，6=固定收益交易平台）`
  TradingSessionID `交易会话 ID（预留）`
  TradingSessionSubID `交易会话子 ID（为数值型字符串）`
  TradSesStatus `交易会话状态（预留）`
  TradSesStartTime `交易会话起始时间`
  TradSesEndTime `交易会话结束时间`
}
