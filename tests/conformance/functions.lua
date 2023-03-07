function y(s, t, u)
    print(s)
    print(u == nil)
    if false then
        return "lucky", 7
    end
    print("Why " .. s .. "? Because it's " .. t)
    return 5, 8, s
end

f, g, n = y("Rust", [[Fast,
Efficient,
safe,
and friendly!!!
    ]])

print("f is " .. f)
print("g is " .. g)
print(n)

print"Lua works really well"
print[[Lua, you are
so
GREAT!!]]

function get_third(t)
    return t[3]
end

print(get_third{1,2, function(s) print(s .. "is fun") end}"Rust ")
