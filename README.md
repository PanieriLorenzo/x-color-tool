# x-color-tool

A small tool for adjusting color settings in Xorg. Requires XRandR to be installed in order to work. Some settings may only work on some GPUs (tested on AMD Radeon).

This repository repurposes some code from [libvibrant]() and [linux gpu drivers]().

## Caveats

- gamma is often reset by blue-light filters like the one built into Gnome and redshift.
- you cannot combine multiple CTM options at once, without them esetting one another.
- there's no error handling, so if you make a mistake, anything could happen!

## Roadmap

- [x] set basic XRandR settings (brightness, gamma)
- [x] float CTM to obscure XRandR 32-bit int conversion
- [x] set saturation with CTM
- [x] set individual RGB brightnesses with CTM
- [ ] set color temperature with CTM
- [ ] set tint with CTM
- [ ] set hue with CTM
- [ ] combine multiple CTM filters with matrix multiplications
- [ ] inspect active configuration
- [ ] more robust CLI
- [ ] set gamma and degamma LUTs
- [ ] set lift with gamma LUT
- [ ] set contrast with gamma LUT
- [ ] support for Wayland
- [ ] interface directly with gpu firmware?