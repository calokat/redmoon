function outer(outerParam)
    local another = "another"
    function inner()
      print(another .. " " .. outerParam);
    end
    return inner;
end
  
closure = outer("Hi 5");
closure();