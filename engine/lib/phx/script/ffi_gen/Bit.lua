-- Bit -------------------------------------------------------------------------
local Loader = {}

function Loader.declareType()
    return 0, 'Bit'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local Bit

    do -- C Definitions
        ffi.cdef [[
            uint32 Bit_And32 (uint32 x, uint32 y);
            uint32 Bit_Or32  (uint32 x, uint32 y);
            uint32 Bit_Xor32 (uint32 x, uint32 y);
            bool   Bit_Has32 (uint32 x, uint32 y);
            uint64 Bit_And64 (uint64 x, uint64 y);
            uint64 Bit_Or64  (uint64 x, uint64 y);
            uint64 Bit_Xor64 (uint64 x, uint64 y);
            bool   Bit_Has64 (uint64 x, uint64 y);
        ]]
    end

    do -- Global Symbol Table
        Bit = {
            ---@param x uint32
            ---@param y uint32
            ---@return uint32
            And32 = libphx.Bit_And32,
            ---@param x uint32
            ---@param y uint32
            ---@return uint32
            Or32  = libphx.Bit_Or32,
            ---@param x uint32
            ---@param y uint32
            ---@return uint32
            Xor32 = libphx.Bit_Xor32,
            ---@param x uint32
            ---@param y uint32
            ---@return bool
            Has32 = libphx.Bit_Has32,
            ---@param x uint64
            ---@param y uint64
            ---@return uint64
            And64 = libphx.Bit_And64,
            ---@param x uint64
            ---@param y uint64
            ---@return uint64
            Or64  = libphx.Bit_Or64,
            ---@param x uint64
            ---@param y uint64
            ---@return uint64
            Xor64 = libphx.Bit_Xor64,
            ---@param x uint64
            ---@param y uint64
            ---@return bool
            Has64 = libphx.Bit_Has64,
        }

        if onDef_Bit then onDef_Bit(Bit, mt) end
        Bit = setmetatable(Bit, mt)
    end

    return Bit
end

return Loader
