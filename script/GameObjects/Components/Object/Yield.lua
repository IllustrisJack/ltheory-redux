local Entity = require('GameObjects.Entity')

-- NOTE : The 'size' of an entity's yield serves as a rate limiter. No more
--        than 'size' (energy-normalized) units of item may be extracted per
--        unit time from the Entity.
local Yield = class(function (self, item, size)
  self.item        = item
  self.size        = size
  self.currentSize = size
end)

--------------------------------------------------------------------------------

function Entity:addYield (item, size)
  assert(not self.yield)
  self.yield = Yield(item, size)
  self:register(Event.Debug, Entity.debugYield)
end

function Entity:debugYield (state)
  local ctx = state.context
  ctx:text("Yield")
  ctx:indent()
  ctx:text("Item: %s", self.yield.item:getName())
  ctx:text("Size: %d", self.yield.size)
  ctx:undent()
end

function Entity:getYield ()
  assert(self.yield)
  return self.yield
end

function Entity:getYieldSize ()
  assert(self.yield)
  return self.yield.currentSize
end

function Entity:hasYield ()
  return self.yield ~= nil
end

--------------------------------------------------------------------------------
