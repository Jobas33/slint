/* LICENSE BEGIN

    This file is part of the Sixty FPS Project

    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only

LICENSE END */
#pragma once
#include "sixtyfps_sharedarray_internal.h"

namespace sixtyfps {

template<typename T>
struct SharedArray
{
    SharedArray()
    {
        internal::sixtyfps_shared_array_new_null(reinterpret_cast<SharedArray<uint8_t> *>(this));
    }

    SharedArray(const SharedArray &other)
    {
        internal::sixtyfps_shared_array_clone(
                reinterpret_cast<SharedArray<uint8_t> *>(this),
                reinterpret_cast<const SharedArray<uint8_t> *>(&other));
    }
    ~SharedArray()
    {
        internal::sixtyfps_shared_array_drop(reinterpret_cast<SharedArray<uint8_t> *>(this));
    }
    SharedArray &operator=(const SharedArray &other)
    {
        internal::sixtyfps_shared_array_drop(reinterpret_cast<SharedArray<uint8_t> *>(this));
        internal::sixtyfps_shared_array_clone(
                reinterpret_cast<SharedArray<uint8_t> *>(this),
                reinterpret_cast<const SharedArray<uint8_t> *>(&other));
        return *this;
    }
    SharedArray &operator=(SharedArray &&other)
    {
        std::swap(inner, other.inner);
        return *this;
    }

private:
    void *inner; // opaque
};
}
