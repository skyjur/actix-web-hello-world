f=io.open("data.txt","r")

request = function()
    line = f:read("*l")
    if(line == nil) then
      f.seek("set", 0)
      line = f.read("*l")
    end
    wrk.method = "POST"
    wrk.path = "/" .. trim1(line)
    wrk.body = line
    -- print("Path: " .. wrk.path)
    -- print("Body: " .. wrk.body)
    wrk.headers["Content-Type"] = "text/plain"
    return wrk.format(nil, wrk.path)
end

  -- response handler
  -- response = function(status, headers, body)
  -- end

function trim1(s)
    return s:gsub("^%s+", ""):gsub("%s+$", "")
end