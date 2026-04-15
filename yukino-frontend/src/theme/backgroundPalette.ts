import colors from 'vuetify/util/colors'

export interface BackgroundOption {
    key: string
    label: string
    url: string
    thumbUrl: string
}

export interface ThemePalette {
    primary: string
    secondary: string
    info: string
    success: string
}

export interface ThemeColors {
    light: ThemePalette & {
        background: string
        surface: string
        error: string
        'on-background': string
        'on-surface': string
        'on-primary': string
    }
    dark: ThemePalette & {
        background: string
        surface: string
        error: string
        'on-background': string
        'on-surface': string
        'on-primary': string
    }
}

const FALLBACK: ThemeColors = {
    light: {
        background: colors.blue.lighten5,
        surface: colors.shades.white,
        error: colors.red.darken1,
        primary: colors.blueGrey.base,
        secondary: colors.indigo.lighten1,
        info: colors.lightBlue.base,
        success: colors.teal.base,
        'on-background': colors.blueGrey.darken4,
        'on-surface': colors.blueGrey.darken4,
        'on-primary': colors.blueGrey.darken4
    },
    dark: {
        background: colors.blueGrey.darken4,
        surface: colors.blueGrey.darken3,
        error: colors.red.lighten3,
        primary: colors.blueGrey.lighten2,
        secondary: colors.indigo.lighten2,
        info: colors.lightBlue.lighten2,
        success: colors.teal.lighten2,
        'on-background': colors.blueGrey.lighten4,
        'on-surface': colors.blueGrey.lighten4,
        'on-primary': colors.blueGrey.darken4
    }
}

const names = [
    '119391638_p0.png',
    '119812287_p0.png',
    '121052377_p0.png',
    '122128866_p0.png',
    '124263276_p0.png',
    '125245434_p0.png',
    '127133721_p0.png',
    '127441961_p0.png',
    '128560882_p0.png',
    '130143139_p0.png',
    '136261196_p0.png',
    '137874580_p0.png',
    '138900181_p0.png',
    '139030627_p0.png'
]

export const BACKGROUND_OPTIONS: BackgroundOption[] = names.map((name, index) => {
    const base = name.replace('.png', '')
    return {
        key: name,
        label: `背景 ${index + 1}`,
        url: `/background/${name}`,
        thumbUrl: `/background/thumbs/${base}.jpg`
    }
})

interface Hsl {
    h: number
    s: number
    l: number
}

const cache = new Map<string, ThemeColors>()

function clamp(value: number, min: number, max: number) {
    return Math.min(max, Math.max(min, value))
}

function rgbToHsl(r: number, g: number, b: number): Hsl {
    const rn = r / 255
    const gn = g / 255
    const bn = b / 255
    const max = Math.max(rn, gn, bn)
    const min = Math.min(rn, gn, bn)
    const d = max - min

    let h = 0
    const l = (max + min) / 2
    const s = d === 0 ? 0 : d / (1 - Math.abs(2 * l - 1))

    if (d !== 0) {
        if (max === rn) {
            h = ((gn - bn) / d) % 6
        } else if (max === gn) {
            h = (bn - rn) / d + 2
        } else {
            h = (rn - gn) / d + 4
        }

        h = Math.round(h * 60)
        if (h < 0) {
            h += 360
        }
    }

    return {h, s: s * 100, l: l * 100}
}

function hslToHex(h: number, s: number, l: number) {
    const hs = clamp(h, 0, 360)
    const ss = clamp(s, 0, 100) / 100
    const ll = clamp(l, 0, 100) / 100

    const c = (1 - Math.abs(2 * ll - 1)) * ss
    const x = c * (1 - Math.abs(((hs / 60) % 2) - 1))
    const m = ll - c / 2

    let r = 0
    let g = 0
    let b = 0

    if (hs < 60) {
        r = c
        g = x
    } else if (hs < 120) {
        r = x
        g = c
    } else if (hs < 180) {
        g = c
        b = x
    } else if (hs < 240) {
        g = x
        b = c
    } else if (hs < 300) {
        r = x
        b = c
    } else {
        r = c
        b = x
    }

    const toHex = (value: number) => Math.round((value + m) * 255).toString(16).padStart(2, '0')
    return `#${toHex(r)}${toHex(g)}${toHex(b)}`
}

function makeMonetPalette(seed: Hsl): ThemeColors {
    const sat = clamp(seed.s * 0.32 + 6, 8, 24)
    const mid = clamp(seed.l * 0.64 + 16, 32, 48)

    return {
        light: {
            background: hslToHex(seed.h, 18, 96),
            surface: hslToHex(seed.h, 12, 99),
            error: colors.red.darken1,
            primary: hslToHex(seed.h, sat, mid),
            secondary: hslToHex((seed.h + 26) % 360, clamp(sat * 0.88, 8, 20), clamp(mid + 7, 40, 58)),
            info: hslToHex((seed.h + 338) % 360, clamp(sat * 0.92, 8, 22), clamp(mid + 5, 38, 56)),
            success: hslToHex(154, 20, 44),
            'on-background': hslToHex((seed.h + 8) % 360, 16, 22),
            'on-surface': hslToHex((seed.h + 8) % 360, 14, 24),
            'on-primary': hslToHex((seed.h + 10) % 360, 18, 16)
        },
        dark: {
            background: hslToHex(seed.h, 10, 11),
            surface: hslToHex(seed.h, 8, 16),
            error: colors.red.lighten3,
            primary: hslToHex(seed.h, clamp(sat + 8, 16, 30), 74),
            secondary: hslToHex((seed.h + 26) % 360, clamp(sat + 6, 14, 26), 70),
            info: hslToHex((seed.h + 338) % 360, clamp(sat + 7, 14, 28), 72),
            success: hslToHex(156, 22, 70),
            'on-background': hslToHex((seed.h + 10) % 360, 12, 88),
            'on-surface': hslToHex((seed.h + 10) % 360, 10, 90),
            'on-primary': hslToHex((seed.h + 8) % 360, 14, 20)
        }
    }
}

function extractSeed(pixels: Uint8ClampedArray): Hsl {
    let r = 0
    let g = 0
    let b = 0
    let count = 0

    for (let i = 0; i < pixels.length; i += 16) {
        const alpha = pixels[i + 3]
        if (alpha < 180) {
            continue
        }

        r += pixels[i]
        g += pixels[i + 1]
        b += pixels[i + 2]
        count += 1
    }

    if (count === 0) {
        return {h: 210, s: 14, l: 48}
    }

    return rgbToHsl(Math.round(r / count), Math.round(g / count), Math.round(b / count))
}

async function loadImage(url: string) {
    return new Promise<HTMLImageElement>((resolve, reject) => {
        const image = new Image()
        image.crossOrigin = 'anonymous'
        image.onload = () => resolve(image)
        image.onerror = () => reject(new Error(`failed to load ${url}`))
        image.src = url
    })
}

export async function extractThemeFromBackground(url: string): Promise<ThemeColors> {
    const fromCache = cache.get(url)
    if (fromCache) {
        return fromCache
    }

    const image = await loadImage(url)
    const canvas = document.createElement('canvas')
    const ctx = canvas.getContext('2d', {willReadFrequently: true})

    if (!ctx) {
        return FALLBACK
    }

    canvas.width = 56
    canvas.height = 56
    ctx.drawImage(image, 0, 0, canvas.width, canvas.height)

    const pixels = ctx.getImageData(0, 0, canvas.width, canvas.height).data
    const seed = extractSeed(pixels)
    const palette = makeMonetPalette(seed)
    cache.set(url, palette)

    return palette
}

export function getFallbackThemeColors() {
    return FALLBACK
}
