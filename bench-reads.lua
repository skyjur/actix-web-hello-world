f=io.open("data.txt","r")

request = function()
    line = f:read("*l")
    if(line == nil) then
      f.seek("set")
      line = f.read("*l")
    end
    wrk.method = "GET"
    wrk.path = "/" .. trim1(line)
    wrk.headers["Content-Type"] = "text/plain"
    return wrk.format(nil, wrk.path)
end

  -- response handler
  -- response = function(status, headers, body)
  -- end

function trim1(s)
    return s:gsub("^%s+", ""):gsub("%s+$", "")
end