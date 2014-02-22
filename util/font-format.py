# A very simple tool for converting from a common bitmap font format to just a
# plain mono-width file.
#
# The MIT License (MIT)
#
# Copyright (c) 2014 Michael Chesser
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

from gimpfu import *

def format_font(image, drawable, glyph_width, glyph_height, num_glyphs):
    pdb.gimp_image_convert_rgb(image)
    new_width = glyph_width * num_glyphs
    new_img = pdb.gimp_image_new(new_width, glyph_height, pdb.gimp_image_base_type(image))
    layer_type = pdb.gimp_drawable_type(drawable)
    layer = pdb.gimp_layer_new(new_img, new_width, glyph_height, layer_type, "glyphs", 100, 0)
    new_img.add_layer(layer, 0)
    
    for i in range(num_glyphs):
        for x in range(i*(glyph_width+1)+1, (i+1)*(glyph_width+1)):
            for y in range(0, glyph_height):
                n_channels, pixel = pdb.gimp_drawable_get_pixel(drawable, x, y)
                layer.set_pixel(x-i-1, y, pixel)

    disp1 = gimp.Display(new_img)

register(
    "format-font",
    "Converts font from Bitmap Font Writer format to a more simple form",
    "Converts font from Bitmap Font Writer format to a more simple form",
    "Michael Chesser",
    "The MIT License (MIT)",
    "2014",
    "<Image>/Image/Format Font...",
    "",
    [
        (PF_INT, "glyph_width", "Glyph Width",""),
        (PF_INT, "glyph_height", "Glyph Height", ""),
        (PF_INT, "num_glyphs", "Number of Glyphs", ""),
    ],
    [],
    format_font)
main()
