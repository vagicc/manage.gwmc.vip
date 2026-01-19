跟我买车-后台管理系统

代拍服务费 5698元一台，包过户
拍卖过户线上咨询：139元
拍卖过户线上线下咨询全程远程指导过户：869元

全程代拍，基础为三千
代拿过户：基础服务费两千。加天数396元一天。交通费。交车的拖车或者开车过高速，及油费
360元全程咨询。  没拍下，有三台车。或者奶牛奶牛一百元



http://www.postgres.cn/docs/13/app-pgdump.html
备份：
pg_dump -h 117.50.172.160 -U postgres -c -C -d gwmc -f gwmc.bf.2020.10.27.sql

-C 大写，会生成创建数据库本身并且恢复时直接恢复到此库
恢复：
先去创建数据库，再执行下行语句
psql -h 128.14.229.27 -U postgres -d gwmc_160 -f gwmc.bf.2020.10.27.sql

一般来说不用加-C与-c，用默认来备份就行

获取A股行情数据方法:https://javaforall.cn/127821.html
 取得实时的A股:https://q.10jqka.com.cn
涨跌分布
上涨：2475只 下跌：2864只
涨跌停
涨停：63只 跌停：53只
API：https://q.10jqka.com.cn/api.php?t=indexflash&
curl 'https://q.10jqka.com.cn/api.php?t=indexflash&' \
  -H 'sec-ch-ua: "Chromium";v="94", ";Not A Brand";v="99"' \
  -H 'Accept: */*' \
  -H 'Referer: https://q.10jqka.com.cn/' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.114 Safari/537.36' \
  -H 'sec-ch-ua-platform: "Linux"' \
  --compressed
  
