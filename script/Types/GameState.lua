GameState = {
  state                 = Enums.GameStates.Splashscreen, -- previously gamemode
  paused                = false,
  panelActive           = false, -- indicates whether MasterControl panel is enabled or not
}

GameState.input = {
  invertPitch           = false,
}

GameState.debug = {
  metricsEnabled        = Config.debug.metricsEnabled,
  instantJobs           = Config.debug.instantJobs,
  jobSpeed              = Config.debug.jobSpeed
}

GameState.render = {
  fullscreen            = Config.render.fullscreen,
  resX                  = Config.render.defaultResX,
  resY                  = Config.render.defaultResY,
  vsync                 = Config.render.vsync,
  zNear                 = Config.render.zNear, -- default: 0.1
  zFar                  = Config.render.zFar, -- default: 1e6
  thrusterLights        = Config.render.thrusterLights,
  pulseLights           = Config.render.pulseLights,
}

GameState.audio = {
  enabled               = Config.audio.enabled,
  fxVolume              = Config.audio.fxVolume,
  musicVolume           = Config.audio.musicVolume,
}

GameState.ui = {
  showTrackers          = Config.ui.showTrackers,
  controlBarHeight      = Config.ui.controlBarHeight,
  hudStyle              = Config.ui.hudStyle,
  cursorStyle           = Config.ui.cursorStyle,
  cursorX               = Config.ui.cursorX,
  cursorY               = Config.ui.cursorY,
  displaySensors        = Config.ui.displaySensors
}

GameState.player = {
  humanPlayer         = nil,
  humanPlayerName     = "[Human Player Name]",
  humanPlayerShipName = "[Human Player Ship Name]",

  currentControl      = Config.ui.defaultControl,
  playerMoving        = false,

  currentShip         = nil,
  weaponGroup         = 1,

  mapSystemPos        = Vec3f(0, 0, 0),
  mapSystemZoom       = 0.0001,

  autonavTimestamp    = nil,
}

GameState.world = {
  -- world related states here later (system state, ai, economy etc)
  currentSystem         = nil,
}

function GameState:SetState(state)
  self.state = state

  if self.state == Enums.GameStates.MainMenu then
    self.ui.defaultControl = "Background" -- enable game startup mode
  else
    self.ui.defaultControl = "Ship" -- enable flight mode
  end
end

function GameState:GetCurrentState()
  return self.state
end