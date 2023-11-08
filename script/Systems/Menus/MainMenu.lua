local MainMenu = class(function(self) end)

local MusicPlayer = require('Systems.SFX.MusicPlayer')
local Bindings = require('States.ApplicationBindings')
local InitFiles = require('Systems.Files.InitFiles')

local mainMenuMusic = nil

local guiElements = {
    {
        name = "Choose Seed",
        elems = {
            -- TODO: replace with proper list
            { nil, 5022463494542550306ULL,  false }, -- KEEP black (good for testing dynamic lighting)
            { nil, 5012768293123392491ULL,  false }, -- KEEP red
            { nil, 4933876146649964811ULL,  false }, -- KEEP blue and milky white
            { nil, 2008422628673393673ULL,  false }, -- MAYBE orange-ish
            { nil, 5712598467986491931ULL,  false }, -- KEEP gold-yellow
            { nil, 8272263000674654607ULL,  false }, -- KEEP milky-white and light blue (really pretty)
            { nil, 14169804077813660835ULL, false }, -- KEEP bluish-green with a bright gold star
            { nil, 9806676695553338612ULL,  false }, -- KEEP violet
            { nil, 14600758714913275339ULL, false }, -- KEEP blue
            { nil, 11589761683708427350ULL, false }, -- KEEP bright green
            { nil, 3432712644463072838ULL,  false }, -- KEEP blue-red-orange
            { nil, 10630444862697458122ULL, false }, -- MAYBE "Hubble palette"
            { nil, 5199604093543988311ULL,  false }, -- KEEP even bluish-white with a bright yellow star
            { nil, 9471911754066691691ULL,  false }, -- KEEP completely dark with one small blue star
            { nil, 15887563511063255006ULL, false }, -- looks pretty cool
            { nil, 976665863517979971ULL,   false }  -- looks pretty cool too
            -- 17682038400513250095ULL
        }
    }
}

local guiSettings = {
    { false, nil, "Audio" },               -- checkbox for audio toggle
    { false, nil, "Fullscreen" },          -- checkbox for fullscreen toggle
    { 0,     nil, "Supersampling" },       -- value for enum of supersampling (anti-aliasing) mode
    { 1,     nil, "Nebula Brightness" },   -- value for brightness scale of background nebula
    { 0,     nil, "Cursor Style" },        -- value for enum of cursor style
    { 0,     nil, "HUD Style" },           -- value for enum of HUD style
    { false, nil, "Unique Ships" },        -- checkbox for unique ships toggle
    { 0,     nil, "Asteroid Fields" },     -- value for number of asteroid fields
    { 0,     nil, "Asteroids per Field" }, -- value for number of asteroids per field
    { 0,     nil, "Planets" },             -- value for number of planets
    { 0,     nil, "Stations" },            -- value for number of stations
    { 0,     nil, "AI Players" },          -- value for number of AI Players
    { 0,     nil, "EconNPCs" },            -- value for number of EconNPCs
    { 0,     nil, "EscortNPCs" },          -- value for number of EscortNPCs
    { 0,     nil, "Ship Size" },           -- value for hull type of player's ship
}

function MainMenu:OnInit()
    self.enabled = true
    self.inBackgroundMode = false
    self.dialogDisplayed = false
    self.seedDialogDisplayed = false
    self.settingsScreenDisplayed = false
    self.dt = 0
    self.lastActionDelta = 0
    self.returnToSplashDelta = 0
    GameState.player.currentControl = Enums.ControlModes.Background

    if not self.keepState then
        GameState:SetState(Enums.GameStates.Splashscreen)
        self.currentMode = Enums.MenuMode.Splashscreen
        self.keepState = false
    else
        GameState:SetState(Enums.GameStates.MainMenu)
    end
    Log.Info("Initialize MainMenu")
end

function MainMenu:ActionRegistered()
    self.lastActionDelta = self.dt
end

function MainMenu:OnUpdate(dt)
    if not self.dt or not dt then return end

    self.dt = self.dt + dt

    if self.enabled and self.currentMode == Enums.MenuMode.MainMenu and not MainMenu.inBackgroundMode then
        if self.lastActionDelta then
            self.returnToSplashDelta = self.lastActionDelta + Config.timeToResetToSplashscreen
        end

        if self.returnToSplashDelta ~= 0 and self.dt >= self.returnToSplashDelta then
            self:SetMenuMode(Enums.MenuMode.Splashscreen)
            self.lastActionDelta = 0
            self.returnToSplashDelta = 0
        end

        --Log.Debug("dt:".. self.dt)
        --Log.Debug("lastAction: " .. self.lastActionDelta)
        --Log.Debug("returnToSplashDelta: " .. self.returnToSplashDelta)
        --Log.Debug(Config.timeToResetToSplashscreen)
    else
        self.lastActionDelta = 0
        self.returnToSplashDelta = 0
    end
end

function MainMenu:Open()
    if not self.enabled then
        self:OnInit()
    end

    Log.Debug("MainMenu:Open: QueueTrack(true)")
    mainMenuMusic = MusicPlayer:QueueTrack(Config.audio.mainMenu, true)

    Log.Debug("Opening Main Menu.")
end

function MainMenu:Close(keepState)
    self.enabled = false
    self.keepState = keepState

    MusicPlayer:StopTrack(mainMenuMusic)

    Log.Debug("Closing Main Menu.")
end

function MainMenu:SetBackgroundMode(enabled)
    Log.Debug("Set Background Mode to: " .. tostring(enabled))
    self.inBackgroundMode = enabled
end

function MainMenu:SetMenuMode(mode)
    Log.Debug("Set Menu Mode to: " .. mode)
    self.currentMode = mode

    -- TODO: this can be improved
    if mode == Enums.MenuMode.Splashscreen then
        GameState:SetState(Enums.GameStates.Splashscreen)
    elseif mode == Enums.MenuMode.MainMenu then
        GameState:SetState(Enums.GameStates.MainMenu)
    elseif mode == Enums.MenuMode.Dialog then
        GameState:SetState(Enums.GameStates.InGame)
    end
end

function MainMenu:ShadowText(text, fontName, fontSize, shadowSize, r, g, b, a)
    Gui:beginStackContainer()
    Gui:textEx(Cache.Font(fontName, fontSize), text, 0.2, 0.2, 0.2, a)
    Gui:setMarginEx(shadowSize, shadowSize, 0.0, 0.0) -- make a shadow
    Gui:textEx(Cache.Font(fontName, fontSize), text, r, g, b, a)
    Gui:endContainer()
end

function MainMenu:ShowGui()
    -- Add Main Menu dialog
    local scalefactor = (LTheoryRedux.resX / 22) / 72
    local scalefactorMenuX = 352.8 / LTheoryRedux.resX
    local scalefactorMenuY = 549 / LTheoryRedux.resY

    Gui:beginStackContainer()
    Gui:setPadding(10.0, 10.0)

    Gui:beginVerticalContainer()
    Gui:setChildrenDocking(Docking.StretchHorizontal)

    Gui:beginVerticalContainer()
    self:ShadowText('LIMIT THEORY', 'RajdhaniSemiBold', 72 * scalefactor, 2.0, 0.9, 0.9, 0.9, 1.0)
    Gui:setDocking(Docking.Right)
    Gui:setMargin(20.0, 20.0)
    self:ShadowText('REDUX', 'RajdhaniSemiBold', 58 * scalefactor, 2.0, 0.9, 0.9, 0.9, 1.0)
    Gui:setDocking(Docking.Right)
    Gui:setMargin(20.0, 20.0)
    Gui:endContainer()
    Gui:setDocking(Docking.Top)

    -- Vertical filler
    Gui:beginStackContainer()
    Gui:endContainer()
    Gui:setDocking(Docking.StretchVertical)

    self:ShowMainMenuInner()

    Gui:beginHorizontalContainer()
    self:ShadowText(Config.gameVersion, 'RajdhaniSemiBold', 12 * scalefactor, 2.0, 0.9, 0.9, 0.9, 1.0)
    Gui:setDocking(Docking.Left)
    self:ShadowText('Resolution = ' .. LTheoryRedux.resX .. ' x ' .. LTheoryRedux.resY, 'RajdhaniSemiBold', 12 * scalefactor, 2.0, 0.9, 0.9, 0.9, 1.0)
    Gui:setDocking(Docking.Right)
    Gui:endContainer()
    Gui:setMarginEx(5.0, 10.0, 5.0, 10.0)

    -- Vertical filler
    Gui:beginStackContainer()
    Gui:endContainer()
    Gui:setDocking(Docking.StretchVertical)

    Gui:endContainer()
    Gui:setDocking(bit.bor(Docking.Left, Docking.StretchVertical))

    Gui:endContainer()
    Gui:setDocking(Docking.StretchAll)
end

function MainMenu:ShowMainMenuInner()
    -- Add Main Menu items
    local scalefactor = (LTheoryRedux.resX / 24) / 72

    Gui:beginVerticalContainer()
    Gui:setChildrenDocking(Docking.StretchHorizontal)

    Gui:pushTextColor(0.9, 0.9, 0.9, 1.0)
    Gui:pushFont(Cache.Font('RajdhaniSemiBold', 36 * scalefactor))

    if Gui:button("NEW GAME") then
        self:ShowSeedDialog()
    end

    if Gui:button("LOAD GAME") then
        self:ShowSeedDialog()
    end

    if Gui:button("SETTINGS") then
        self:ShowSettingsScreen()
    end

    if Gui:button("CREDITS") then
    end

    if Gui:button("BACKGROUND") then
        self:SetBackgroundMode(true)
    end

    if Gui:button("EXIT GAME") then
        LTheoryRedux:exitGame()
    end

    Gui:popStyle(2)
    Gui:endContainer()
end

function MainMenu:ShowSeedDialog()
    -- Add new star system seed selection dialog menu
    self.dialogDisplayed = false
    self.seedDialogDisplayed = true

    Gui:beginWindow(guiElements.name, InputInstance)
    Gui:textEx(Cache.Font('Iceland', 42), 'Choose Seed', 0.3, 0.6, 1.0, 1.0)
    Gui:setAlign(0.5, 0.5)
    Gui:setSpacing(16)
    self:ShowSeedDialogInner()
    Gui:endWindow()
    Gui:setAlign(0.5, 0.5)
end

function MainMenu:ShowSeedDialogInner()
    -- Add new star system seed selection dialog menu items
    Gui:beginVerticalContainer()
    Gui:pushTextColor(1.0, 1.0, 1.0, 1.0)
    Gui:pushFont(Cache.Font('Exo2', 26))

    -- Loop through saved seeds (hardcoded for now) and display as checkboxes
    for i = 1, #guiElements[1]["elems"] do
        -- Create the new checkbox and save a reference to its current state (T/F)
        guiElements[1]["elems"][i][3] = Gui:checkbox(tostring(guiElements[1]["elems"][i][2]),
            guiElements[1]["elems"][i][3])

        if guiElements[1]["elems"][i][3] then
            -- Checkbox was selected
            -- Reset all other checkboxes (so that these checkboxes will work like radio buttons, where only one can be active)
            for j = 1, #guiElements[1]["elems"] do
                if j ~= i then
                    guiElements[1]["elems"][j][3] = false
                end
            end
            -- Save the star system seed associated with it
            LTheoryRedux.seed = guiElements[1]["elems"][i][2]
        end
        Gui:setSpacing(8)
    end
    Gui:setSpacing(16)

    Gui:beginHorizontalContainer()
    Gui:pushTextColor(1.0, 1.0, 1.0, 1.0)
    Gui:pushFont(Cache.Font('Exo2Bold', 28))

    if Gui:button("Cancel") then
        if GameState:GetCurrentState() == Enums.GameStates.InGame then
            self.dialogDisplayed = true
        end
        self.seedDialogDisplayed = false
    end

    Gui:setSpacing(16)

    if Gui:button("Random Seed") then
        LTheoryRedux:generateNewSeed()
        self.seedDialogDisplayed = false

        for i = 1, #guiElements[1]["elems"] do -- reset all seed selection checkboxes
            guiElements[1]["elems"][i][3] = false
        end

        self:SetMenuMode(Enums.MenuMode.Dialog)
        GameState:Unpause()
        InputInstance:setCursorVisible(false)
        LTheoryRedux:createStarSystem()
    end

    Gui:setSpacing(16)

    if Gui:button("Use Seed") then
        self.seedDialogDisplayed = false

        for i = 1, #guiElements[1]["elems"] do -- reset all seed selection checkboxes
            guiElements[1]["elems"][i][3] = false
        end

        self:SetMenuMode(Enums.MenuMode.Dialog)
        GameState:Unpause()
        GameState.player.currentControl = Enums.ControlModes.Ship
        InputInstance:setCursorVisible(false)
        LTheoryRedux:createStarSystem()
    end

    Gui:popStyle(2)
    Gui:endContainer()
    Gui:setAlign(0.5, 0.5)
    Gui:popStyle(2)
    Gui:endContainer()
end

function MainMenu:ShowSettingsScreen()
    -- Add new star system seed selection dialog menu
    if GameState:GetCurrentState() == Enums.GameStates.InGame then
        self.dialogDisplayed = false
    end
    self.settingsScreenDisplayed = true

    Gui:beginWindow(guiElements.name, InputInstance)
    Gui:textEx(Cache.Font('Iceland', 42), 'Settings', 0.3, 0.6, 1.0, 1.0)
    Gui:setAlign(0.5, 0.5)
    Gui:rect(1.0, 1.0, 0.3, 0.6, 1.0, 1.0)
    Gui:setStretch(1.0, 0.5)
    Gui:setSpacing(16)
    self:ShowSettingsScreenInner()
    Gui:endWindow()
    Gui:setAlign(0.5, 0.5)
end

function MainMenu:ShowSettingsScreenInner()
    -- Add new star system seed selection dialog menu items
    Gui:beginVerticalContainer()
    Gui:pushTextColor(1.0, 1.0, 1.0, 1.0)
    Gui:pushFont(Cache.Font('Exo2', 24))

    -- Show Settings options
    Gui:beginVerticalContainer()

    Gui:textEx(Cache.Font('Exo2', 24), "--- Audio ---", 0.3, 0.6, 1.0, 1.0)
    Gui:setStretch(0.0, 0.0)
    Gui:setAlign(0.5, 0.5)

    guiSettings[1][1] = GameState.audio.soundEnabled
    if guiSettings[1][2] == nil then
        guiSettings[1][2] = GameState.audio.soundEnabled
    end
    guiSettings[1][1] = Gui:checkbox(guiSettings[1][3], guiSettings[1][1])
    if guiSettings[1][1] then
        -- Checkbox was selected
        if not GameState.audio.soundEnabled then
            LTheoryRedux:SoundOn()
        end
    else
        if GameState.audio.soundEnabled then
            LTheoryRedux:SoundOff()
        end
    end

    Gui:setSpacing(16)
    Gui:textEx(Cache.Font('Exo2', 24), "--- Graphics ---", 0.3, 0.6, 1.0, 1.0)
    Gui:setStretch(0.0, 0.0)
    Gui:setAlign(0.5, 0.5)

    guiSettings[2][1] = GameState.render.fullscreen
    if guiSettings[2][2] == nil then
        guiSettings[2][2] = GameState.render.fullscreen
    end
    guiSettings[2][1] = Gui:checkbox(guiSettings[2][3], guiSettings[2][1])
    if guiSettings[2][1] then
        -- Checkbox was selected
        if not GameState.render.fullscreen then
            LTheoryRedux:SetFullscreen(true)
        end
    else
        if GameState.render.fullscreen then
            LTheoryRedux:SetFullscreen(false)
        end
    end

    Gui:setSpacing(8)
    Gui:beginHorizontalContainer()
    Gui:textEx(Cache.Font('Exo2', 24), guiSettings[3][3], 1.0, 1.0, 1.0, 1.0)
    Gui:setStretch(1.0, 0.0)
    Gui:beginHorizontalContainer()
    if guiSettings[3][2] == nil then
        guiSettings[3][1] = Settings.get('render.superSample')
        guiSettings[3][2] = Settings.get('render.superSample')
    end
    if Gui:button("-") and guiSettings[3][1] > 1 then
        guiSettings[3][1] = guiSettings[3][1] - 1
        Settings.set('render.superSample', guiSettings[3][1])
    end
    Gui:textEx(Cache.Font("Ubuntu", 20), Settings.getEnumValName('render.superSample', guiSettings[3][1]), 0.3, 1.0,
        0.4, 1.0)
    if Gui:button("+") and guiSettings[3][1] < 3 then
        guiSettings[3][1] = guiSettings[3][1] + 1
        Settings.set('render.superSample', guiSettings[3][1])
    end
    Gui:endContainer()
    Gui:endContainer()
    Gui:setStretch(1.0, 0.0)

    Gui:setSpacing(8)
    Gui:beginHorizontalContainer()
    Gui:textEx(Cache.Font('Exo2', 24), guiSettings[4][3], 1.0, 1.0, 1.0, 1.0)
    Gui:setStretch(1.0, 0.0)
    Gui:beginHorizontalContainer()
    if guiSettings[4][2] == nil then
        guiSettings[4][1] = GameState.gen.nebulaBrightnessScale
        guiSettings[4][2] = GameState.gen.nebulaBrightnessScale
    end
    if Gui:button("-") and guiSettings[4][1] > 0.25 then
        guiSettings[4][1] = guiSettings[4][1] - 0.25
    end
    Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[4][1]), 0.3, 1.0, 0.4, 1.0)
    if Gui:button("+") and guiSettings[4][1] < 10 then
        guiSettings[4][1] = guiSettings[4][1] + 0.25
    end
    Gui:endContainer()
    Gui:endContainer()
    Gui:setStretch(1.0, 0.0)

    Gui:setSpacing(16)
    Gui:textEx(Cache.Font('Exo2', 24), "--- Interface ---", 0.3, 0.6, 1.0, 1.0)
    Gui:setStretch(0.0, 0.0)
    Gui:setAlign(0.5, 0.5)

    Gui:setSpacing(8)
    Gui:beginHorizontalContainer()
    Gui:textEx(Cache.Font('Exo2', 24), guiSettings[5][3], 1.0, 1.0, 1.0, 1.0)
    Gui:setStretch(1.0, 0.0)
    Gui:beginHorizontalContainer()
    if guiSettings[5][2] == nil then
        guiSettings[5][1] = GameState.ui.cursorStyle
        guiSettings[5][2] = GameState.ui.cursorStyle
    end
    if Gui:button("-") and guiSettings[5][1] > 1 then
        guiSettings[5][1] = guiSettings[5][1] - 1
        LTheoryRedux:setCursor(Enums.CursorFilenames[guiSettings[5][1]], GameState.ui.cursorX, GameState.ui.cursorY)
    end
    Gui:textEx(Cache.Font("Ubuntu", 20), Enums.CursorStyleNames[guiSettings[5][1]], 0.3, 1.0, 0.4, 1.0)
    if Gui:button("+") and guiSettings[5][1] < Enums.CursorStyleCount then
        guiSettings[5][1] = guiSettings[5][1] + 1
        LTheoryRedux:setCursor(Enums.CursorFilenames[guiSettings[5][1]], GameState.ui.cursorX, GameState.ui.cursorY)
    end
    Gui:endContainer()
    Gui:endContainer()
    Gui:setStretch(1.0, 0.0)

    Gui:setSpacing(8)
    Gui:beginHorizontalContainer()
    Gui:textEx(Cache.Font('Exo2', 24), guiSettings[6][3], 1.0, 1.0, 1.0, 1.0)
    Gui:setStretch(1.0, 0.0)
    Gui:beginHorizontalContainer()
    if guiSettings[6][2] == nil then
        guiSettings[6][1] = GameState.ui.hudStyle
        guiSettings[6][2] = GameState.ui.hudStyle
    end
    if Gui:button("-") and guiSettings[6][1] > 1 then
        guiSettings[6][1] = guiSettings[6][1] - 1
    end
    Gui:textEx(Cache.Font("Ubuntu", 20), Enums.HudStyleNames[guiSettings[6][1]], 0.3, 1.0, 0.4, 1.0)
    if Gui:button("+") and guiSettings[6][1] < Enums.HudStyleCount then
        guiSettings[6][1] = guiSettings[6][1] + 1
    end
    Gui:endContainer()
    Gui:endContainer()
    Gui:setStretch(1.0, 0.0)

    if MainMenu.currentMode ~= Enums.MenuMode.Dialog then
        -- Don't display game generation settings when viewing Settings in Flight mode
        Gui:setSpacing(16)
        Gui:textEx(Cache.Font('Exo2', 24), "--- Generation ---", 0.3, 0.6, 1.0, 1.0)
        Gui:setStretch(0.0, 0.0)
        Gui:setAlign(0.5, 0.5)

        Gui:setSpacing(8)
        guiSettings[7][1] = GameState.gen.uniqueShips
        if guiSettings[7][2] == nil then
            guiSettings[7][2] = GameState.gen.uniqueShips
        end
        guiSettings[7][1] = Gui:checkbox(guiSettings[7][3], guiSettings[7][1])
        if guiSettings[7][1] then
            -- Checkbox was selected
            if not GameState.gen.uniqueShips then
                GameState.gen.uniqueShips = true
            end
        else
            if GameState.gen.uniqueShips then
                GameState.gen.uniqueShips = false
            end
        end

        -- NOTE: Although it's possible to factor these latter values down into one set of updates and one loop (which I tried),
        --       the things we can tweak in Settings *will* change. This section of code should not be prematurely optimized.

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[8][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[8][2] == nil then
            guiSettings[8][1] = GameState.gen.nFields
            guiSettings[8][2] = GameState.gen.nFields
        end
        if Gui:button("-") and guiSettings[8][1] > 0 then
            guiSettings[8][1] = guiSettings[8][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[8][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[8][1] < 20 then
            guiSettings[8][1] = guiSettings[8][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[9][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[9][2] == nil then
            guiSettings[9][1] = GameState.gen.nAsteroids
            guiSettings[9][2] = GameState.gen.nAsteroids
        end
        if Gui:button("-") and guiSettings[9][1] > 1 then
            guiSettings[9][1] = guiSettings[9][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[9][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[9][1] < 200 then
            guiSettings[9][1] = guiSettings[9][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[10][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[10][2] == nil then
            guiSettings[10][1] = GameState.gen.nPlanets
            guiSettings[10][2] = GameState.gen.nPlanets
        end
        if Gui:button("-") and guiSettings[10][1] > 0 then
            guiSettings[10][1] = guiSettings[10][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[10][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[10][1] < 1 then
            guiSettings[10][1] = guiSettings[10][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[11][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[11][2] == nil then
            guiSettings[11][1] = GameState.gen.nStations
            guiSettings[11][2] = GameState.gen.nStations
        end
        if Gui:button("-") and guiSettings[11][1] > 0 then
            guiSettings[11][1] = guiSettings[11][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[11][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[11][1] < 50 then
            guiSettings[11][1] = guiSettings[11][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[12][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[12][2] == nil then
            guiSettings[12][1] = GameState.gen.nAIPlayers
            guiSettings[12][2] = GameState.gen.nAIPlayers
        end
        if Gui:button("-") and guiSettings[12][1] > 0 then
            guiSettings[12][1] = guiSettings[12][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[12][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[12][1] < 20 then
            guiSettings[12][1] = guiSettings[12][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[13][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[13][2] == nil then
            guiSettings[13][1] = GameState.gen.nEconNPCs
            guiSettings[13][2] = GameState.gen.nEconNPCs
        end
        if Gui:button("-") and guiSettings[13][1] > 0 then
            guiSettings[13][1] = guiSettings[13][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[13][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[13][1] < 100 then
            guiSettings[13][1] = guiSettings[13][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[14][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[14][2] == nil then
            guiSettings[14][1] = GameState.gen.nEscortNPCs
            guiSettings[14][2] = GameState.gen.nEscortNPCs
        end
        if Gui:button("-") and guiSettings[14][1] > 0 then
            guiSettings[14][1] = guiSettings[14][1] - 1
        end
        Gui:textEx(Cache.Font("Ubuntu", 20), tostring(guiSettings[14][1]), 0.3, 1.0, 0.4, 1.0)
        if Gui:button("+") and guiSettings[14][1] < 50 then
            guiSettings[14][1] = guiSettings[14][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)

        Gui:setSpacing(8)
        Gui:beginHorizontalContainer()
        Gui:textEx(Cache.Font('Exo2', 24), guiSettings[15][3], 1.0, 1.0, 1.0, 1.0)
        Gui:setStretch(1.0, 0.0)
        Gui:beginHorizontalContainer()
        if guiSettings[15][2] == nil then
            guiSettings[15][1] = GameState.player.shipHull
            guiSettings[15][2] = GameState.player.shipHull
        end
        if Gui:button("-") and guiSettings[15][1] > Enums.ShipHulls.Solo then
            guiSettings[15][1] = guiSettings[15][1] - 1
        end
        local hullSizeName = Config:getObjectInfo("ship_subtypes", 3 + (guiSettings[15][1] - 1))
        Gui:textEx(Cache.Font("Ubuntu", 20), hullSizeName, 0.3, 1.0, 0.4, 1.0)

        if Gui:button("+") and guiSettings[15][1] < Enums.ShipHulls.VeryLarge then
            guiSettings[15][1] = guiSettings[15][1] + 1
        end
        Gui:endContainer()
        Gui:endContainer()
        Gui:setStretch(1.0, 0.0)
    end

    Gui:endContainer()

    -- Show Settings control buttons
    Gui:setSpacing(16)
    Gui:beginHorizontalContainer()
    Gui:pushTextColor(1.0, 1.0, 1.0, 1.0)
    Gui:pushFont(Cache.Font('Exo2Bold', 28))

    if Gui:button("Cancel") then
        -- Revert to the pre-Settings values of each setting
        if guiSettings[1][2] then
            LTheoryRedux:SoundOn()
        else
            LTheoryRedux:SoundOff()
        end

        LTheoryRedux:SetFullscreen(guiSettings[2][2])

        Settings.set('render.superSample', guiSettings[3][2])

        GameState.gen.nebulaBrightnessScale = guiSettings[4][2]

        GameState.ui.cursorStyle = guiSettings[5][2]
        LTheoryRedux:setCursor(Enums.CursorFilenames[GameState.ui.cursorStyle], GameState.ui.cursorX,
            GameState.ui.cursorY)

        GameState.ui.hudStyle = guiSettings[6][2]

        if MainMenu.currentMode ~= Enums.MenuMode.Dialog then
            if guiSettings[7][2] then
                GameState.gen.uniqueShips = true
            else
                GameState.gen.uniqueShips = false
            end

            GameState.gen.nFields     = guiSettings[8][2]
            GameState.gen.nAsteroids  = guiSettings[9][2]
            GameState.gen.nPlanets    = guiSettings[10][2]
            GameState.gen.nStations   = guiSettings[11][2]
            GameState.gen.nAIPlayers  = guiSettings[12][2]
            GameState.gen.nEconNPCs   = guiSettings[13][2]
            GameState.gen.nEscortNPCs = guiSettings[14][2]
            GameState.player.shipHull = guiSettings[15][2]
        end

        for i = 1, #guiSettings do
            guiSettings[i][2] = nil
        end

        if GameState:GetCurrentState() == Enums.GameStates.InGame then
            self.dialogDisplayed = true
        end
        self.settingsScreenDisplayed = false

        if MainMenu.currentMode == Enums.MenuMode.Dialog then
            LTheoryRedux:freezeTurrets()
            InputInstance:setCursorVisible(true)
        end
    end

    Gui:setSpacing(16)

    if Gui:button("Use") then
        -- Return to the game using the selected values of each setting
        if GameState:GetCurrentState() == Enums.GameStates.InGame then
            self.dialogDisplayed = true
        end
        self.settingsScreenDisplayed = false

        GameState.gen.nebulaBrightnessScale = guiSettings[4][1]

        GameState.ui.cursorStyle = guiSettings[5][1]
        GameState.ui.hudStyle = guiSettings[6][1]

        if MainMenu.currentMode ~= Enums.MenuMode.Dialog then
            GameState.gen.nFields     = guiSettings[8][1]
            GameState.gen.nAsteroids  = guiSettings[9][1]
            GameState.gen.nPlanets    = guiSettings[10][1]
            GameState.gen.nStations   = guiSettings[11][1]
            GameState.gen.nAIPlayers  = guiSettings[12][1]
            GameState.gen.nEconNPCs   = guiSettings[13][1]
            GameState.gen.nEscortNPCs = guiSettings[14][1]
            GameState.player.shipHull = guiSettings[15][1]
        end

        for i = 1, #guiSettings do
            guiSettings[i][2] = nil
        end

        if MainMenu.currentMode == Enums.MenuMode.Dialog then
            LTheoryRedux:freezeTurrets()
            InputInstance:setCursorVisible(true)
        end

        -- Write player-specific game variables to preserve them across gameplay sessions
        InitFiles:writeUserInits()
    end

    Gui:popStyle(2)
    Gui:endContainer()

    Gui:setAlign(0.5, 0.5)
    Gui:popStyle(2)
    Gui:endContainer()
end

function MainMenu:ShowFlightDialog()
    -- Add Flight Mode dialog menu
    Gui:beginWindow("Flight Mode", InputInstance)
    Gui:textEx(Cache.Font('Iceland', 36), 'Flight Mode Controls', 0.3, 0.6, 1.0, 1.0)
    Gui:setAlign(0.5, 0.5)
    Gui:setSpacing(16)
    self:ShowFlightDialogInner()
    Gui:endWindow()
    Gui:setAlign(0.5, 0.5)
end

function MainMenu:ShowFlightDialogInner()
    -- Add Flight Mode dialog menu items
    Gui:beginVerticalContainer()
    Gui:pushTextColor(1.0, 1.0, 1.0, 1.0)
    Gui:pushFont(Cache.Font('Exo2Bold', 26))

    if GameState.player.currentShip ~= nil and not GameState.player.currentShip:isDestroyed() then
        if Gui:button("Return to Game") then
            LTheoryRedux:freezeTurrets()
            GameState:SetState(Enums.GameStates.InGame)
            GameState:Unpause()
            GameState.panelActive = false
            self.dialogDisplayed = false

            if GameState.player.currentControl == Enums.ControlModes.Ship then
                InputInstance:setCursorVisible(false)
            end
        end
    end

    if GameState.player.currentShip ~= nil and not GameState.player.currentShip:isDestroyed() then
        Gui:setSpacing(8)

        if Gui:button("Save Game") then
            -- TODO: Save game state here
            LTheoryRedux:freezeTurrets()
            GameState:Unpause()
            GameState.panelActive = false
            InputInstance:setCursorVisible(false)
        end
    end
    Gui:setSpacing(8)

    if Gui:button("Load Game") then
        -- TODO: Show Load Game menu once that's been implemented
        -- NOTE: For now, just pop up a Seed Menu dialog for creating a new star system
        self:ShowSeedDialog()
    end
    Gui:setSpacing(8)

    if Gui:button("Game Settings") then
        -- Show Game Settings menu
        self:ShowSettingsScreen()
        GameState:Pause()
        InputInstance:setCursorVisible(true)
    end
    Gui:setSpacing(8)

    if Gui:button("Exit to Main Menu") then
        GameState:SetState(Enums.GameStates.MainMenu)        -- switch to Startup Mode
        LTheoryRedux:seedStarsystem(Enums.MenuMode.MainMenu) -- use random seed for new background star system and display it in Main Menu mode
        GameState:Unpause()
    end
    Gui:setSpacing(8)

    if Gui:button("Exit Game") then
        LTheoryRedux:exitGame()
    end
    Gui:popStyle(2)
    Gui:endContainer()
end

function MainMenu:utf8(decimal)
    local bytemarkers = { { 0x7FF, 192 }, { 0xFFFF, 224 }, { 0x1FFFFF, 240 } }
    if decimal < 128 then return string.char(decimal) end
    local charbytes = {}
    for bytes, vals in ipairs(bytemarkers) do
        if decimal <= vals[1] then
            for b = bytes + 1, 2, -1 do
                local mod = decimal % 64
                decimal = (decimal - mod) / 64
                charbytes[b] = string.char(128 + mod)
            end
            charbytes[1] = string.char(vals[2] + decimal)
            break
        end
    end
    return table.concat(charbytes)
end

return MainMenu
