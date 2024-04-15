-- Trigger ---------------------------------------------------------------------
local Loader = {}

function Loader.declareType()
    ffi.cdef [[
        typedef struct Trigger {} Trigger;
    ]]

    return 1, 'Trigger'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local Trigger

    do -- C Definitions
        ffi.cdef [[
            void       Trigger_Free             (Trigger*);
            Trigger*   Trigger_CreateBox        (Vec3f const* halfExtents);
            void       Trigger_Attach           (Trigger*, RigidBody* parent, Vec3f const* offset);
            void       Trigger_Detach           (Trigger*, RigidBody* parent);
            void       Trigger_GetBoundingBox   (Trigger const*, Box3f* out);
            int        Trigger_GetContentsCount (Trigger*);
            RigidBody* Trigger_GetContents      (Trigger const*, int i);
            void       Trigger_SetCollisionMask (Trigger*, uint32 mask);
            void       Trigger_SetPos           (Trigger*, Vec3f const* pos);
            void       Trigger_SetPosLocal      (Trigger*, Vec3f const* pos);
            void       Trigger_GetPos           (Trigger const*, Vec3f* out);
            void       Trigger_GetPosLocal      (Trigger const*, Vec3f* out);
            RigidBody* Trigger_GetParent        (Trigger*);
        ]]
    end

    do -- Global Symbol Table
        Trigger = {
            ---@param half_extents Vec3f const*
            ---@return Trigger*
            CreateBox        = function(...)
                local instance = libphx.Trigger_CreateBox(...)
                return Core.ManagedObject(instance, libphx.Trigger_Free)
            end,
        }

        if onDef_Trigger then onDef_Trigger(Trigger, mt) end
        Trigger = setmetatable(Trigger, mt)
    end

    do -- Metatype for class instances
        local t  = ffi.typeof('Trigger')
        local mt = {
            __index = {
                ---@param parent RigidBody*
                ---@param offset Vec3f const*
                attach           = libphx.Trigger_Attach,
                ---@param parent RigidBody*
                detach           = libphx.Trigger_Detach,
                ---@param [out] Box3f
                getBoundingBox   = libphx.Trigger_GetBoundingBox,
                ---@return int
                getContentsCount = libphx.Trigger_GetContentsCount,
                -- Will only include the parent object when a compound is within the trigger.
                ---@param i int
                ---@return RigidBody*
                getContents      = libphx.Trigger_GetContents,
                ---@param mask uint32
                setCollisionMask = libphx.Trigger_SetCollisionMask,
                ---@param pos Vec3f const*
                setPos           = libphx.Trigger_SetPos,
                ---@param pos Vec3f const*
                setPosLocal      = libphx.Trigger_SetPosLocal,
                ---@param [out] Vec3f
                getPos           = libphx.Trigger_GetPos,
                ---@param [out] Vec3f
                getPosLocal      = libphx.Trigger_GetPosLocal,
                ---@return RigidBody*
                getParent        = libphx.Trigger_GetParent,
            },
        }

        if onDef_Trigger_t then onDef_Trigger_t(t, mt) end
        Trigger_t = ffi.metatype(t, mt)
    end

    return Trigger
end

return Loader
