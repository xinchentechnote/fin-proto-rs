-- sse_binary_heartbeat_logon.lua
-- 专用于解析SSE Binary协议中的Heartbeat和Logon消息

local sse_proto = Proto("SSE", "SSE Binary Protocol (Heartbeat & Logon)")

-- 字段定义 (大端序)
local fields = {
    -- 公共头字段
    msg_type       = ProtoField.uint32("sse.header.msg_type", "MessageType", base.DEC),
    msg_seq_num    = ProtoField.bytes("sse.header.msg_seq_num", "MsgSeqNum"), -- 8字节使用bytes类型
    msg_body_len   = ProtoField.uint32("sse.header.msg_body_len", "MsgBodyLen", base.DEC),

    -- Logon特有字段
    sender_comp_id = ProtoField.string("sse.logon.sender", "SenderCompID"),
    target_comp_id = ProtoField.string("sse.logon.target", "TargetCompID"),
    heart_bt_int   = ProtoField.uint16("sse.logon.hb_interval", "HeartbeatInt", base.DEC),
    prtcl_version  = ProtoField.string("sse.logon.version", "ProtocolVersion"),
    trade_date     = ProtoField.uint32("sse.logon.date", "TradeDate", base.DEC),
    q_size         = ProtoField.uint32("sse.logon.qsize", "QSize", base.DEC),

    checksum       = ProtoField.uint32("sse.checksum", "Checksum", base.HEX)
}

-- 注册所有字段
for _, field in pairs(fields) do
    sse_proto.fields[field] = field
end

-- 自定义8字节大端序解析
local function parse_be_uint64(buf, offset)
    local bytes = buf(offset, 8):bytes()
    return bytes:get_index(0) * 0x100000000000000 +
        bytes:get_index(1) * 0x1000000000000 +
        bytes:get_index(2) * 0x10000000000 +
        bytes:get_index(3) * 0x100000000 +
        bytes:get_index(4) * 0x1000000 +
        bytes:get_index(5) * 0x10000 +
        bytes:get_index(6) * 0x100 +
        bytes:get_index(7)
end

-- 解析Heartbeat消息
local function dissect_heartbeat(buf, tree, offset)
    local subtree = tree:add(sse_proto, buf(offset, 0), "Heartbeat")
    subtree:append_text(" (No Body)")
    return offset -- Heartbeat无消息体
end

-- 解析Logon消息
local function dissect_logon(buf, tree, offset)
    local subtree = tree:add(sse_proto, buf(offset, 82), "Logon")

    -- 解析固定长度字段
    subtree:add(fields.sender_comp_id, buf(offset, 32))
    offset = offset + 32

    subtree:add(fields.target_comp_id, buf(offset, 32))
    offset = offset + 32

    subtree:add(fields.heart_bt_int, buf(offset, 2))
    offset = offset + 2

    subtree:add(fields.prtcl_version, buf(offset, 8))
    offset = offset + 8

    subtree:add(fields.trade_date, buf(offset, 4))
    offset = offset + 4

    subtree:add(fields.q_size, buf(offset, 4))
    offset = offset + 4

    return offset
end

-- 主解析函数
function sse_proto.dissector(buf, pinfo, tree)
    local length = buf:len()
    if length < 16 then return end -- 最小包头长度检查

    pinfo.cols.protocol = "SSE"
    local offset = 0

    -- 解析包头
    local msg_type = buf(offset, 4):uint()
    tree:add(fields.msg_type, buf(offset, 4))
    offset = offset + 4

    local seq_num = parse_be_uint64(buf, offset)
    local seq_item = tree:add(fields.msg_seq_num, buf(offset, 8))
    seq_item:append_text(" [" .. seq_num .. "]") -- 显示可读序列号
    offset = offset + 8

    tree:add(fields.msg_body_len, buf(offset, 4))
    offset = offset + 4

    -- 根据消息类型分发处理
    if msg_type == 33 then -- Heartbeat
        dissect_heartbeat(buf, tree, offset)
        pinfo.cols.info:set("HEARTBEAT")
    elseif msg_type == 40 then -- Logon
        dissect_logon(buf, tree, offset)
        pinfo.cols.info:set("LOGON")
    end

    -- 解析校验和 (最后4字节)
    if length >= offset + 4 then
        tree:add(fields.checksum, buf(length - 4, 4))
    end
end

-- 注册到TCP端口
local tcp_table = DissectorTable.get("tcp.port")
tcp_table:add(8080, sse_proto)
