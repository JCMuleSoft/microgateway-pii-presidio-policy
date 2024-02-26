%dw 2.0
output text

fun toTable(properties: Object): String = do {
    fun table(properties: Object, prefix: String = ""): String = properties pluck ((value, key, index) -> do {
            var name = if(isEmpty(prefix)) key else "$(prefix).$(key)"
            var line = "|$(name)|$(value.'type' default '') |$(value.description default '')| $(value.'default'  default '')|"
            ---
            line ++
            if(value.properties?)
               "\n"++ table(value.properties, key)
            else ""
        }
     )
     joinBy  "\n"
     ---
     "|Name|Type|Description|Default Value|\n|---|---|---|---|\n$(table(properties))"
}
---
toTable(payload.spec.properties)