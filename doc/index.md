## 总览
`typed-db`整体的设计目的是解决大量的游戏配置数据从`定义类型化的数据结构`到`可视化配置和查询`到`高效和低占用的存储和读取`的整套解决方案。   
传统的或者说原始的小型配置系统一般是由`json`或者`xml`甚至是`excel`之类的承载，数据量大到一定程度有些可能会采用`sqlite`之类的小型数据库.这些选型都有着各自的问题，下面列出`typed-db`解决的这些传统存储方案的问题，这些特性基本都是通过类型化来提供的。  

1. 消除弱类型的错误配置  
2. 实现各种各样的筛选搜索  
3. 存储文件只需存储最终数据，无需存储字段名等数据描述信息(json,xml)，减少内存占用。  
4. 可以生成数据配置的UI。  
5. 可以按需从内存读取。  

## 组成  
因为是整套解决方案整个项目由多个子项目组成，分别为如下所示:  
### dbl (类型定义语言)
### 
###