import {reactive, readonly} from 'vue'
import type {ThemeInstance} from 'vuetify/lib/framework.mjs'
import {BACKGROUND_OPTIONS, extractThemeFromBackground, getFallbackThemeColors} from './backgroundPalette'

export type ThemeMode = 'system' | 'light' | 'dark'
interface ThemeTransitionOrigin {
    x: number
    y: number
}

const STORAGE_MODE_KEY = 'yukino.theme.mode'
const STORAGE_BACKGROUND_KEY = 'yukino.theme.background'
const STORAGE_CACHE_KEY = 'yukino.theme.cached'

interface ThemeState {
    mode: ThemeMode
    background: string
    ready: boolean
}

const defaultBackground = BACKGROUND_OPTIONS[0]?.key ?? ''

const state = reactive<ThemeState>({
    mode: 'system',
    background: defaultBackground,
    ready: false
})

let themeInstance: ThemeInstance | null = null
let mediaQuery: MediaQueryList | null = null
let initOnce: Promise<void> | null = null
let applyToken = 0

function getSystemThemeName(): 'light' | 'dark' {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

function getEffectiveThemeName(): 'light' | 'dark' {
    return state.mode === 'system' ? getSystemThemeName() : state.mode
}

function readStorage() {
    const mode = localStorage.getItem(STORAGE_MODE_KEY)
    const background = localStorage.getItem(STORAGE_BACKGROUND_KEY)

    if (mode === 'system' || mode === 'light' || mode === 'dark') {
        state.mode = mode
    }

    if (background && BACKGROUND_OPTIONS.some((item) => item.key === background)) {
        state.background = background
    }
}

function writeStorage() {
    localStorage.setItem(STORAGE_MODE_KEY, state.mode)
    localStorage.setItem(STORAGE_BACKGROUND_KEY, state.background)
}

function writeCachedColors(backgroundKey: string, colors: unknown) {
    localStorage.setItem(`${STORAGE_CACHE_KEY}.${backgroundKey}`, JSON.stringify(colors))
}

function readCachedColors(backgroundKey: string) {
    const raw = localStorage.getItem(`${STORAGE_CACHE_KEY}.${backgroundKey}`)
    if (!raw) {
        return null
    }

    try {
        return JSON.parse(raw) as ReturnType<typeof getFallbackThemeColors>
    } catch {
        return null
    }
}

function applyBackgroundStyle(dark: boolean) {
    const current = BACKGROUND_OPTIONS.find((item) => item.key === state.background)
    if (!current) {
        return
    }

    document.documentElement.style.setProperty('--yukino-bg-image', `url("${current.url}")`)
    document.documentElement.style.setProperty('--yukino-bg-opacity', dark ? '0.24' : '0.38')
    document.documentElement.style.setProperty('--yukino-bg-mask-opacity', dark ? '0.62' : '0.52')
}

async function applyTheme(recompute = false) {
    if (!themeInstance) {
        return
    }

    const token = ++applyToken
    const current = BACKGROUND_OPTIONS.find((item) => item.key === state.background)
    if (!current) {
        return
    }

    let colors = readCachedColors(current.key) ?? getFallbackThemeColors()

    if (recompute || !readCachedColors(current.key)) {
        try {
            colors = await extractThemeFromBackground(current.url)
            writeCachedColors(current.key, colors)
        } catch {
            colors = readCachedColors(current.key) ?? getFallbackThemeColors()
        }
    }

    if (token !== applyToken || !themeInstance) {
        return
    }

    Object.assign(themeInstance.themes.value.light.colors, colors.light)
    Object.assign(themeInstance.themes.value.dark.colors, colors.dark)

    const dark = getEffectiveThemeName() === 'dark'
    await applyThemeWithTransition(dark)
}

async function applyThemeWithTransition(dark: boolean, origin?: ThemeTransitionOrigin) {
    if (!themeInstance) {
        return
    }

    const apply = () => {
        themeInstance?.change(dark ? 'dark' : 'light')
        applyBackgroundStyle(dark)
    }

    if (!origin || window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
        apply()
        return
    }

    const doc = document as Document & {
        startViewTransition?: (callback: () => void) => {ready: Promise<void>}
    }

    if (!doc.startViewTransition) {
        apply()
        return
    }

    const endRadius = Math.hypot(
        Math.max(origin.x, window.innerWidth - origin.x),
        Math.max(origin.y, window.innerHeight - origin.y)
    )

    const transition = doc.startViewTransition(() => {
        apply()
    })

    try {
        await transition.ready
        document.documentElement.animate(
            {
                clipPath: [
                    `circle(0px at ${origin.x}px ${origin.y}px)`,
                    `circle(${endRadius}px at ${origin.x}px ${origin.y}px)`
                ]
            },
            {
                duration: 520,
                easing: 'cubic-bezier(0.22, 0.61, 0.36, 1)',
                pseudoElement: '::view-transition-new(root)'
            }
        )
    } catch {
        // fallback: theme has already been applied
    }
}

function randomBackgroundKey() {
    const options = BACKGROUND_OPTIONS.map((item) => item.key)
    if (options.length <= 1) {
        return options[0] ?? defaultBackground
    }

    const pool = options.filter((key) => key !== state.background)
    return pool[Math.floor(Math.random() * pool.length)]
}

function handleSystemThemeChange() {
    if (state.mode === 'system') {
        void applyTheme(false)
    }
}

export async function initThemeEngine(theme: ThemeInstance) {
    if (initOnce) {
        return initOnce
    }

    themeInstance = theme
    readStorage()

    initOnce = (async () => {
        await applyTheme(true)
        state.ready = true

        mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
        mediaQuery.addEventListener('change', handleSystemThemeChange)
    })()

    return initOnce
}

export function useThemePreferences() {
    function setMode(mode: ThemeMode, origin?: ThemeTransitionOrigin) {
        state.mode = mode
        writeStorage()
        if (origin) {
            const dark = getEffectiveThemeName() === 'dark'
            void applyThemeWithTransition(dark, origin)
            return
        }
        void applyTheme(false)
    }

    function setBackground(backgroundKey: string) {
        if (!BACKGROUND_OPTIONS.some((item) => item.key === backgroundKey)) {
            return
        }

        state.background = backgroundKey
        writeStorage()
        void applyTheme(true)
    }

    function setRandomBackground() {
        setBackground(randomBackgroundKey())
    }

    function toggleLightDark(origin?: ThemeTransitionOrigin) {
        setMode(getEffectiveThemeName() === 'dark' ? 'light' : 'dark', origin)
    }

    return {
        state: readonly(state),
        modeOptions: [
            {value: 'system' as const, label: '跟随系统', icon: 'mdi-monitor'},
            {value: 'light' as const, label: '浅色模式', icon: 'mdi-white-balance-sunny'},
            {value: 'dark' as const, label: '深色模式', icon: 'mdi-weather-night'}
        ],
        backgroundOptions: BACKGROUND_OPTIONS,
        setMode,
        setBackground,
        setRandomBackground,
        toggleLightDark,
        isDark: () => getEffectiveThemeName() === 'dark'
    }
}
