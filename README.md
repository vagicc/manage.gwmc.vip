跟我买车-后台管理系统

代拍服务费 5000元一台，包过户
拍卖过户线上咨询：100元
拍卖过户线上线下咨询全程远程指导过户：800元


http://www.postgres.cn/docs/13/app-pgdump.html
备份：
pg_dump -h 117.50.172.160 -U postgres -c -C -d gwmc -f gwmc.bf.2020.10.27.sql

-C 大写，会生成创建数据库本身并且恢复时直接恢复到此库
恢复：
先去创建数据库，再执行下行语句
psql -h 128.14.229.27 -U postgres -d gwmc_160 -f gwmc.bf.2020.10.27.sql

一般来说不用加-C与-c，用默认来备份就行

获取A股行情数据方法:https://javaforall.cn/127821.html
 