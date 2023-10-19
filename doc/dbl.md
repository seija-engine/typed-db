```haskell
--data define
data Info = {
    name:String,
    desc:String,
}

--enum define
data Quality  = White | Green | Blue | Red

--sum type
data TaskInfo = KillUnit { unitId:Long } | MoveToTarget { px:Int,py:Int }

--type var and meta info
[Info {name = "通用",desc = "" }]
data CommonInfo a = {
    [Info { name = "ID",desc = "唯一ID用于索引" }]
    Id:Long,
    name:String,
    value:a,
}

--type alias
data ItemDesc = {
    icon:String,
    quality:Quality
}

type ItemInfo = CommonInfo ItemDesc
type ItemTable = List ItemInfo
```