# `update-right-status`

*Since: 20210314-114017-04b7cedd*

The `update-right-status` event is emitted periodically (based on the
interval specified by the [status_update_interval](../config/status_update_interval.md)
configuration value).

Its purpose is to allow you the chance to carry out some activity
and then ultimately call [window:set_right_status](../window/set_right_status.md).

The first event parameter is a [`window` object](../window/index.md) that
represents the gui window.

The second event parameter is a [`pane` object](../pane/index.md) that
represents the active pane in that window.

There is no defined return value for the event.

`wezterm` will ensure that only a single instance of this event is outstanding;
if the hook takes longer than the
[status_update_interval](../config/status_update_interval.md) to complete,
`wezterm` won't schedule another call until `status_update_interval`
milliseconds have elapsed since the last call completed.

Possible usage example is to implement a sort of status bar (from https://github.com/wez/wezterm/issues/500)

```lua
local wezterm = require 'wezterm';

wezterm.on("update-right-status", function(window, pane)
      -- demonstrates shelling out to get some external status.
      -- wezterm will parse escape sequences output by the
      -- child process and include them in the status area, too.
      local success, date, stderr = wezterm.run_child_process({"date"});

     -- However, if all you need is to format the date/time, then:
     date = wezterm.strftime("%Y-%m-%d %H:%M:%S");

      -- Make it italic and underlined
      window:set_right_status(wezterm.format({
        {Attribute={Underline="Single"}},
        {Attribute={Italic=true}},
        {Text="Hello "..date},
    }));
end);

return {}

```
