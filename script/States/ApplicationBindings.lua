local Control = require('Systems.Controls.Control')

local self = {
  Escape           = Button.Keyboard.Escape,
  Pause            = Button.Keyboard.Space,
  ToggleSound      = Button.Keyboard.Return,
  SystemMap        = Button.Keyboard.Tab,
  Reload           = Button.Keyboard.F5,
  ProfilerToggle   = Button.Keyboard.F10,
  ToggleFullscreen = Button.Keyboard.F11,
  Screenshot       = Button.Keyboard.F12,
  NewBackground    = Button.Keyboard.B,
  TimeAccel        = Button.Keyboard.H,
  ToggleMetrics    = Button.Keyboard.K,
  MoveTo           = Button.Keyboard.M,
  ToggleHUD        = Button.Keyboard.V,
  ToggleWireframe  = Button.Keyboard.W, -- does nothing
  ScoreNebulaBad   = Button.Keyboard.Minus, -- does nothing
  ScoreNebulaGood  = Button.Keyboard.Equals, -- does nothing
  Exit             = Button.System.Exit, -- Modifier.Ctrl + Button.W or Modifier.Alt + Button.Q

  All = Control.Or(
    Control.Key(Button.Keyboard.A         ),
    Control.Key(Button.Keyboard.B         ),
    Control.Key(Button.Keyboard.C         ),
    Control.Key(Button.Keyboard.D         ),
    Control.Key(Button.Keyboard.E         ),
    Control.Key(Button.Keyboard.F         ),
    Control.Key(Button.Keyboard.G         ),
    Control.Key(Button.Keyboard.I         ),
    Control.Key(Button.Keyboard.J         ),
    Control.Key(Button.Keyboard.K         ),
    Control.Key(Button.Keyboard.L         ),
    Control.Key(Button.Keyboard.M         ),
    Control.Key(Button.Keyboard.N         ),
    Control.Key(Button.Keyboard.O         ),
    Control.Key(Button.Keyboard.P         ),
    Control.Key(Button.Keyboard.Q         ),
    Control.Key(Button.Keyboard.R         ),
    Control.Key(Button.Keyboard.S         ),
    Control.Key(Button.Keyboard.T         ),
    Control.Key(Button.Keyboard.U         ),
    Control.Key(Button.Keyboard.V         ),
    Control.Key(Button.Keyboard.W         ),
    Control.Key(Button.Keyboard.X         ),
    Control.Key(Button.Keyboard.Y         ),
    Control.Key(Button.Keyboard.Z         ),
    Control.Key(Button.Keyboard.N0        ),
    Control.Key(Button.Keyboard.N1        ),
    Control.Key(Button.Keyboard.N2        ),
    Control.Key(Button.Keyboard.N3        ),
    Control.Key(Button.Keyboard.N4        ),
    Control.Key(Button.Keyboard.N5        ),
    Control.Key(Button.Keyboard.N6        ),
    Control.Key(Button.Keyboard.N7        ),
    Control.Key(Button.Keyboard.N8        ),
    Control.Key(Button.Keyboard.N9        ),
    Control.Key(Button.Keyboard.KP0       ),
    Control.Key(Button.Keyboard.KP1       ),
    Control.Key(Button.Keyboard.KP2       ),
    Control.Key(Button.Keyboard.KP3       ),
    Control.Key(Button.Keyboard.KP4       ),
    Control.Key(Button.Keyboard.KP5       ),
    Control.Key(Button.Keyboard.KP6       ),
    Control.Key(Button.Keyboard.KP7       ),
    Control.Key(Button.Keyboard.KP8       ),
    Control.Key(Button.Keyboard.KP9       ),
    Control.Key(Button.Keyboard.KPNumLock ),
    Control.Key(Button.Keyboard.KPDivide  ),
    Control.Key(Button.Keyboard.KPMultiply),
    Control.Key(Button.Keyboard.KPSubtract),
    Control.Key(Button.Keyboard.KPAdd     ),
    Control.Key(Button.Keyboard.KPEnter   ),
    Control.Key(Button.Keyboard.KPDecimal ),
    Control.Key(Button.Keyboard.Backspace ),
    Control.Key(Button.Keyboard.Escape    ),
    Control.Key(Button.Keyboard.Return    ),
    Control.Key(Button.Keyboard.Space     ),
    Control.Key(Button.Keyboard.Tab       ),
    Control.Key(Button.Keyboard.Backtick  ),
    Control.Key(Button.Keyboard.CapsLock  ),
    Control.Key(Button.Keyboard.Minus     ),
    Control.Key(Button.Keyboard.Equals    ),
    Control.Key(Button.Keyboard.LBracket  ),
    Control.Key(Button.Keyboard.RBracket  ),
    Control.Key(Button.Keyboard.Backslash ),
    Control.Key(Button.Keyboard.Semicolon ),
    Control.Key(Button.Keyboard.Apostrophe),
    Control.Key(Button.Keyboard.Comma     ),
    Control.Key(Button.Keyboard.Period    ),
    Control.Key(Button.Keyboard.Slash     ),
    Control.Key(Button.Keyboard.ScrollLock),
    Control.Key(Button.Keyboard.Pause     ),
    Control.Key(Button.Keyboard.Insert    ),
    Control.Key(Button.Keyboard.Delete    ),
    Control.Key(Button.Keyboard.Home      ),
    Control.Key(Button.Keyboard.End       ),
    Control.Key(Button.Keyboard.PageUp    ),
    Control.Key(Button.Keyboard.PageDown  ),
    Control.Key(Button.Keyboard.Right     ),
    Control.Key(Button.Keyboard.Left      ),
    Control.Key(Button.Keyboard.Down      ),
    Control.Key(Button.Keyboard.Up        ),
    Control.Key(Button.Mouse.Left         ),
    Control.Key(Button.Mouse.Middle       ),
    Control.Key(Button.Mouse.Right        ),
    Control.Key(Button.Mouse.X1           ),
    Control.Key(Button.Mouse.X2           ),
    Control.Key(Button.Gamepad.A          ),
    Control.Key(Button.Gamepad.B          ),
    Control.Key(Button.Gamepad.X          ),
    Control.Key(Button.Gamepad.Y          ),
    Control.Key(Button.Gamepad.Back       ),
    Control.Key(Button.Gamepad.Guide      ),
    Control.Key(Button.Gamepad.Start      ),
    Control.Key(Button.Gamepad.LStick     ),
    Control.Key(Button.Gamepad.RStick     ),
    Control.Key(Button.Gamepad.LBumper    ),
    Control.Key(Button.Gamepad.RBumper    ),
    Control.Key(Button.Gamepad.Up         ),
    Control.Key(Button.Gamepad.Down       ),
    Control.Key(Button.Gamepad.Left       ),
    Control.Key(Button.Gamepad.Right      ),
    Control.Key(Button.Gamepad.LTrigger   ),
    Control.Key(Button.Gamepad.RTrigger   ))
    :delta(),

}

return self
