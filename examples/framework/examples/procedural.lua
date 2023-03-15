realtime = 0.0

TINY_FONT_GLYPHIDX = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!?*^&()[]<>-+=/\\\"'`~:;,.%abcdefghijklmnopqrstuvwxyz"

--font_tiny = load_font("core/fonts/tiny_font.png", TINY_FONT_GLYPHIDX, 5, 5, -1)

TEX_BRICK = "TEX_BRICK"
TEX_BRICK_SIZE = 256

cook_time = 0.0

function _conf()
    set_resolution(640, 360)
    set_fullscreen()
    
end

function _init()
    create_brick_texture(true)
    
end

function _update(delta)
    realtime = realtime + delta
end

function _draw()
    local name = TEX_BRICK
    local size = TEX_BRICK_SIZE

    clear()
    time_before = timestamp()
    pimgmtx(TEX_BRICK, mouse_x(), mouse_y(), realtime * 0.25, 1.0, 1.0, 0.5, 0.5)
    time_after = timestamp()
    --pprint(font_tiny, "Cook time: " .. cook_time .. "ms", 0, 0)
    --pprint(font_tiny, "Draw time: " .. math.ceil((time_after - time_before) * 10000.0) / 10.0 .. "ms", 0, 6)
end

function create_brick_texture(use_noise)
    local name = TEX_BRICK
    local size = TEX_BRICK_SIZE
    
    image(name, size, size)
    
    local time_before = timestamp()

    local line_color = rgb(200, 200, 200)

    -- Background
    irectangle(name, true, 0, 0, size, size, hsv(0.0, 0.8, 0.75))

    -- White Lines Horizontal
    for i = 0, size, 64 do
        irectangle(name, true, 0, i, size, 8, line_color)
    end

    -- White Lines Vertical Row 1
    for i = 0, size, 64 do
        irectangle(name, true, i, 0, 8, 64, line_color)
    end

    -- White Lines Vertical Row 2
    for i = 32, size, 64 do
        irectangle(name, true, i, 64, 8, 64, line_color)
    end

    -- White Lines Vertical Row 3
    for i = 0, size, 64 do
        irectangle(name, true, i, 128, 8, 64, line_color)
    end

    -- White Lines Vertical Row 4
    for i = 32, size, 64 do
        irectangle(name, true, i, 192, 8, 64, line_color)
    end

    -- Scattered Horizontal Noise (Very expensive!!!)
    if use_noise then
        set_image_draw_mode_multiply(name)
        for y = 0, size, 1 do
            for x = 0, size, 1 do
                irectangle(name, true, x, y, rand_range(0.0, 5.0), 1, hsv(0.0, 0.0, rand_range(0.0, 0.2)))
            end
        end
        set_image_draw_mode_opaque(name)
    end
    

    local time_after = timestamp()

    -- Variance in cook time is mainly from the difference of random numbers used (Some spans are longer than others)
    cook_time = math.ceil((time_after - time_before) * 10000.0) / 10.0
    print("Texture cook time: " .. cook_time .. " ms")
end

function rand_range(min, max)
    local random = math.random
    return min + (max - min) * random()
end