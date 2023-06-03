a = {1}
b = {2}

mk = "__add"

setmetatable(a, {["__add"] = function(x, y)
    print("meta A")
    return y + 5 end})
setmetatable(b, {["__add"] = function(x, y) return 4 end})

print(7 + a)