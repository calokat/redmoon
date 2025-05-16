t = {2, [9] = 4, 6, 8, [7] = function(s) print(s .. "Batman!") end}
print(t[3])

t[7]("Holy bank robbery, ")

f = t[7]


f("I am vengeance. I am the night. I am ")

x = "target"

t2 = {[x] = "John Connor"}
print(t2["target"])
