-- taken from example in https://www.lua.org/manual/5.4/manual.html#3.5

x = 10                -- global variable
do                    -- new block
  local x = x         -- new 'x', with value 10
  print("First local x is " .. x)--   print(x)            --> 10
  x = x+1
  do                  -- another block
    local x = x+1     -- another 'x'
    print("Second local x is " .. x)          --> 12
  end
  print("First local x is now " .. x)            --> 11
end
print("Global x is " .. x)            --> 10  (the global one)