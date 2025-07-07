# tshark 基本命令

tshark -X lua_script:proto/sse_binary_01.lua -i any -f "tcp port 8080" -V
sudo tshark -X lua_script:proto/sse_binary.lua -i any -f "tcp port 8080" -V
sudo tshark -i any -f "tcp port 8080" -w 8080.pcap

# 常用参数说明：

# -X lua_script:<file> 加载 Lua 脚本

# -i <interface> 指定网卡

# -f <capture filter> 抓包过滤表达式

# -V 显示详细解析结果

# -r <file> 读取已有 pcap 文件

# -w <file> 保存抓包结果

# -Y <display filter> 显示过滤

# tcpdump 基本命令

sudo tcpdump -i any port 8080 -w capture.pcap
