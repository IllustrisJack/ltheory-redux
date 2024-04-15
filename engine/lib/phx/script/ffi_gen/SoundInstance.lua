-- SoundInstance ---------------------------------------------------------------
local Loader = {}

function Loader.declareType()
    ffi.cdef [[
        typedef struct SoundInstance {} SoundInstance;
    ]]

    return 1, 'SoundInstance'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local SoundInstance

    do -- C Definitions
        ffi.cdef [[
            void   SoundInstance_Free            (SoundInstance*);
            bool   SoundInstance_IsPlaying       (SoundInstance const*);
            bool   SoundInstance_IsPaused        (SoundInstance const*);
            bool   SoundInstance_IsStopped       (SoundInstance const*);
            double SoundInstance_GetVolume       (SoundInstance const*);
            void   SoundInstance_SetVolume       (SoundInstance*, double volume, uint64 fadeMillis);
            void   SoundInstance_Pause           (SoundInstance*, uint64 fadeMillis);
            void   SoundInstance_Resume          (SoundInstance*, uint64 fadeMillis);
            void   SoundInstance_Stop            (SoundInstance*, uint64 fadeMillis);
            void   SoundInstance_FreeEmitter     (SoundInstance*);
            void   SoundInstance_SetPlayPos      (SoundInstance*, double position);
            void   SoundInstance_MovePlayPos     (SoundInstance*, double offset);
            void   SoundInstance_SetEmitterPos   (SoundInstance*, Vec3f const* position);
            Vec3f  SoundInstance_EmitterPos      (SoundInstance const*);
            float  SoundInstance_EmitterDistance (SoundInstance const*, Vec3f const* listenerPos);
        ]]
    end

    do -- Global Symbol Table
        SoundInstance = {}

        if onDef_SoundInstance then onDef_SoundInstance(SoundInstance, mt) end
        SoundInstance = setmetatable(SoundInstance, mt)
    end

    do -- Metatype for class instances
        local t  = ffi.typeof('SoundInstance')
        local mt = {
            __index = {
                ---@return bool
                isPlaying       = libphx.SoundInstance_IsPlaying,
                ---@return bool
                isPaused        = libphx.SoundInstance_IsPaused,
                ---@return bool
                isStopped       = libphx.SoundInstance_IsStopped,
                ---@return double
                getVolume       = libphx.SoundInstance_GetVolume,
                ---@param volume double
                ---@param fade_millis uint64
                setVolume       = libphx.SoundInstance_SetVolume,
                ---@param fade_millis uint64
                pause           = libphx.SoundInstance_Pause,
                ---@param fade_millis uint64
                resume          = libphx.SoundInstance_Resume,
                ---@param fade_millis uint64
                stop            = libphx.SoundInstance_Stop,
                freeEmitter     = libphx.SoundInstance_FreeEmitter,
                ---@param position double
                setPlayPos      = libphx.SoundInstance_SetPlayPos,
                ---@param offset double
                movePlayPos     = libphx.SoundInstance_MovePlayPos,
                ---@param position Vec3f const*
                setEmitterPos   = libphx.SoundInstance_SetEmitterPos,
                ---@return Vec3f
                emitterPos      = libphx.SoundInstance_EmitterPos,
                ---@param listener_pos Vec3f const*
                ---@return float
                emitterDistance = libphx.SoundInstance_EmitterDistance,
            },
        }

        if onDef_SoundInstance_t then onDef_SoundInstance_t(t, mt) end
        SoundInstance_t = ffi.metatype(t, mt)
    end

    return SoundInstance
end

return Loader
