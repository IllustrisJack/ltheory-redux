-- SocketType ------------------------------------------------------------------
local ffi = require('ffi')
local libphx = require('libphx.libphx')
local SocketType

do -- Global Symbol Table
  SocketType = {
    None = 0x0,
    UDP  = 0x1,
    TCP  = 0x2,
  }

  if onDef_SocketType then onDef_SocketType(SocketType, mt) end
  SocketType = setmetatable(SocketType, mt)
end

return SocketType
