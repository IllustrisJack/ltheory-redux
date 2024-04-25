---@type UIView
local MainView = UICore.View {
    name = "Main"
}

---@type UIRouter
local UIRouter = require("UI.HmGui.UICore.UIRouter")

local logo = Tex2D.Load("./res/images/LTR-logo-name.png")

function MainView:onInput() end
function MainView:onUpdate(dt) end

local function getButtonWidth()
    return GameState.render.resX / 1600 * 200
end

local function getButtonHeight()
    return GameState.render.resY / 900 * 40
end

local function switchToPlayView()
    UIRouter:getCurrentPage():setView("Play")
end

local function switchToSettingsView()
    UIRouter:getCurrentPage():setView("Settings")
end

local function switchToBackgroundView()
    UIRouter:getCurrentPage():setView("Background")
end

local menuGrid = UILayout.Grid {
    align = { AlignHorizontal.Stretch, AlignVertical.Stretch },
    padding = { 50, 50 },
    margin = { 0, 0 },
    stackDirection = Enums.UI.StackDirection.Horizontal,
    showGrid = false,
    contents = {
        UIComponent.Container {
            align = { AlignHorizontal.Stretch, AlignVertical.Center },
            padding = { 50, 10 },
            margin = { 0, 0 },
            stackDirection = Enums.UI.StackDirection.Vertical,
            contents = {
                UIComponent.Button {
                    title = "Play",
                    width = getButtonWidth,
                    height = getButtonHeight,
                    callback = switchToPlayView,
                    align = { AlignHorizontal.Center, AlignVertical.Center }
                },
                UIComponent.Button {
                    title = "Settings",
                    width = getButtonWidth,
                    height = getButtonHeight,
                    align = { AlignHorizontal.Center, AlignVertical.Center },
                    callback = switchToSettingsView
                },
                UIComponent.Button {
                    title = "Background Mode",
                    width = getButtonWidth,
                    height = getButtonHeight,
                    callback = switchToBackgroundView,
                    align = { AlignHorizontal.Center, AlignVertical.Center }
                },
                UIComponent.Button {
                    title = "Exit",
                    width = getButtonWidth,
                    height = getButtonHeight,
                    callback = function() EngineInstance:exit() end,
                    align = { AlignHorizontal.Center, AlignVertical.Center }
                }
            }
        },
        UIComponent.Container {
            align = { AlignHorizontal.Stretch, AlignVertical.Stretch },
            childrenAlign = { AlignHorizontal.Center, AlignVertical.Center },
            padding = { 0, 0 },
            margin = { 0, 0 },
            stackDirection = Enums.UI.StackDirection.Vertical,
            contents = {
                UIComponent.RawInput { fn = function()
                    Gui:image(logo)
                    Gui:setPercentSize(100, 20)
                    Gui:setAlignment(AlignHorizontal.Center, AlignVertical.Center)
                end }
            }
        }
    }
}

MainView:addContent(menuGrid)

return MainView
