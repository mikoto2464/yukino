import {createVuetify} from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import {getFallbackThemeColors} from '../theme/backgroundPalette'

const initial = getFallbackThemeColors()

const vuetify = createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'light',
        themes: {
            light: {
                dark: false,
                colors: {...initial.light} as Record<string, string>
            },
            dark: {
                dark: true,
                colors: {...initial.dark} as Record<string, string>
            }
        }
    },
    defaults: {
        VCard: {
            rounded: 'xl',
            elevation: 0,
            variant: 'flat',
            border: true
        }
    }
})

export default vuetify
