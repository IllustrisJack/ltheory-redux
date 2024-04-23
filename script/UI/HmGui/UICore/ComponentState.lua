local ComponentState = {}
ComponentState.__index = ComponentState

local meta = {
    __call = function(cls, ...)
        return cls:new(...)
    end
}

---@class UIComponentState
---@class UIComponentStateConstructor

---returns a text object
---@param args UIComponentStateConstructor
---@return UIComponentState|nil
function ComponentState:new(args)
    if not args then
        return
    end

    local newState = {}
    for index, arg in pairs(args) do
        newState[index] = function()
            if type(arg) == "function" then
                return arg()
            else
                return arg
            end
        end
    end

    return newState
end

setmetatable(ComponentState, meta)

-- Add to global UIComponent table
---@type UIComponentStateConstructor
UICore.ComponentState = ComponentState

return ComponentState
