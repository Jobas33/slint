/* LICENSE BEGIN

    This file is part of the Sixty FPS Project

    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only

LICENSE END */
#pragma once

#include "sixtyfps.h"

namespace sixtyfps::testing {
inline void mock_elapsed_time(int64_t time_in_ms)
{
    internal::sixtyfps_mock_elapsed_time(time_in_ms);
}
template<typename Component>
inline void send_mouse_click(Component &component, float x, float y) {
    internal::sixtyfps_send_mouse_click({&Component::component_type, &component}, x, y);
}
} // namespace sixtyfps
