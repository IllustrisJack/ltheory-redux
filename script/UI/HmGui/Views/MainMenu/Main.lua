---@type UIView
local MainView = UICore.View {
    name = "Main"
}

---@type UIRouter
local UIRouter = require("UI.HmGui.UICore.UIRouter")
--local MusicPlayer = require('Systems.SFX.MusicPlayer')

local logo = Tex2D.Load("./res/images/LTR-logo-name.png")

local function distance(x1, y1, x2, y2)
    return math.sqrt((x2 - x1) ^ 2 + (y2 - y1) ^ 2)
end

local lastMousePos = Vec2f(0, 0)
local lastMoved = TimeStamp.Now()
local minDistance = 50 -- pixel
local timeTillBackground = 30

function MainView:onInput()
    local mousePos = InputInstance:mouse():position()

    if distance(mousePos.x, mousePos.y, lastMousePos.x, lastMousePos.y) > minDistance then
        lastMoved = TimeStamp.Now()
        lastMousePos = mousePos
    end

    if lastMoved:getElapsed() >= timeTillBackground then
        UIRouter:getCurrentPage():setView("Background")
    end
end

function MainView:onUpdate(dt) end

function MainView:onViewOpen(isPageOpen)
    if isPageOpen then
        --MusicPlayer:QueueTrack(GameState.audio.menuTheme, true)
    end

    lastMoved = TimeStamp.Now()
end

function MainView:onViewClose(isPageClose)
    if isPageClose then
        --MusicPlayer:ClearQueue()
    end
end

local function getButtonWidth()
    return GameState.render.resX / 1600 * 200
end

local function getButtonHeight()
    return GameState.render.resY / 900 * 40
end

local function getLayoutContainerWidthPercentage() --todo: needs replacement with a more sophisticated layout system
    return GameState.render.resX / 1600 * 170 * 2 / GameState.render.resX
end

local function getRemainingWidthPercentage()
    return 1 - getLayoutContainerWidthPercentage()
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

local function switchToCreditsView()
    UIRouter:getCurrentPage():setView("Credits")
end

local menuGrid = UILayout.Grid {
    align = { AlignHorizontal.Stretch, AlignVertical.Stretch },
    padding = { 125, 0 },
    margin = { 0, 0 },
    stackDirection = Enums.UI.StackDirection.Horizontal,
    contents = {
        UILayout.Grid {
            align = { AlignHorizontal.Stretch, AlignVertical.Stretch },
            padding = { 0, 0 },
            margin = { 0, 0 },
            widthInLayout = getLayoutContainerWidthPercentage,
            stackDirection = Enums.UI.StackDirection.Vertical,
            contents = {
                UIComponent.Container {
                    align = { AlignHorizontal.Stretch, AlignVertical.Top },
                    padding = { 0, 0 },
                    margin = { 0, 0 },
                    stackDirection = Enums.UI.StackDirection.Vertical,
                    heightInLayout = 3 / 4,
                    color = {
                        background = Color(0.1, 0.1, 0.1, 0.2)
                    },
                    contents = {
                        UIComponent.RawInput { fn = function()
                            Gui:beginVerticalContainer()
                            Gui:setBorder(0.0001, Color(1.0, 1.0, 1.0, 1)) --! using border as theres currently no other way
                            Gui:image(logo)
                            Gui:setPercentSize(100, 50)                    --! needs proper resolution scaling later
                            Gui:endContainer()
                            Gui:setPaddingTop(50)
                            Gui:setPaddingRight(10)
                            Gui:setPaddingLeft(10)
                            Gui:setPercentSize(100, 55)
                        end },
                        UIComponent.Button_MainMenu {
                            title = "Play",
                            width = getButtonWidth,
                            height = getButtonHeight,
                            callback = switchToPlayView,
                            align = { AlignHorizontal.Center, AlignVertical.Center }
                        },
                        UIComponent.Button_MainMenu {
                            title = "Settings",
                            width = getButtonWidth,
                            height = getButtonHeight,
                            align = { AlignHorizontal.Center, AlignVertical.Center },
                            callback = switchToSettingsView
                        },
                        UIComponent.Button_MainMenu {
                            title = "Credits",
                            width = getButtonWidth,
                            height = getButtonHeight,
                            callback = switchToCreditsView,
                            align = { AlignHorizontal.Center, AlignVertical.Center }
                        },
                        UIComponent.Button_MainMenu {
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
                    childrenAlign = { AlignHorizontal.Center, AlignVertical.Bottom },
                    padding = { 0, 30 },
                    margin = { 0, 0 },
                    heightInLayout = 1 / 4,
                    stackDirection = Enums.UI.StackDirection.Vertical,
                    color = {
                        background = Color(0.1, 0.1, 0.1, 0.2)
                    },
                    contents = {
                        UIComponent.Text {
                            text = Config.gameVersion,
                            align = { AlignHorizontal.Center, AlignVertical.Bottom }
                        }
                    }
                }
            }
        },
        UIComponent.Container {
            align = { AlignHorizontal.Stretch, AlignVertical.Stretch },
            childrenAlign = { AlignHorizontal.Center, AlignVertical.Center },
            padding = { 0, 0 },
            margin = { 0, 0 },
            widthInLayout = getRemainingWidthPercentage,
            stackDirection = Enums.UI.StackDirection.Vertical,
            contents = {
                UIComponent.RawInput { fn = function() end }
            }
        }
    }
}

MainView:addContent(menuGrid)

return MainView
