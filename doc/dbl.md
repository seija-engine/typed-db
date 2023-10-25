```Scala
class Info {
    name:String,
    desc:String
}

enum Quality {
    case White,
    case Green,
    case Blue,
    case Red
}

enum TaskInfo {
    case KillUnit(
        info:KillUnitInfo
    )
    case MoveToTarget(
        info:MoveToTargetInfo
    )
}

@Info("通用","")
class CommonInfo[T] {
    Id:Long,
    name:String,
    value:T
}

class ItemDesc {
    icon:String,
    quality:Quality
}

type ItemInfo = CommonInfo[ItemDesc]
type ItemTable = Array[ItemInfo]
```