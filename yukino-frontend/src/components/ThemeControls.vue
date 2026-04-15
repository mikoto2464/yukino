<template>
  <div class="d-flex align-center ga-2">
    <v-btn :icon="isDark ? 'mdi-weather-sunny' : 'mdi-weather-night'" variant="text" @click="toggleLightDarkFromTrigger"/>

    <v-menu location="bottom end">
      <template #activator="{ props }">
        <v-btn icon="mdi-theme-light-dark" v-bind="props" variant="text"/>
      </template>

      <v-card class="pa-2" min-width="240" rounded="lg">
        <v-list class="py-0" density="comfortable">
          <v-list-subheader>显示模式</v-list-subheader>
          <v-list-item
              v-for="item in modeOptions"
              :key="item.value"
              :prepend-icon="item.icon"
              :title="item.label"
              @click="setMode(item.value)"
          >
            <template #append>
              <v-icon v-if="state.mode === item.value" color="primary" icon="mdi-check"/>
            </template>
          </v-list-item>
        </v-list>
      </v-card>
    </v-menu>

    <v-menu location="bottom end" max-height="520">
      <template #activator="{ props }">
        <v-btn icon="mdi-palette" v-bind="props" variant="text"/>
      </template>

      <v-card class="pa-2" min-width="320" rounded="lg">
        <v-list class="py-0" density="comfortable">
          <v-list-subheader>背景图取色</v-list-subheader>

          <v-list-item prepend-icon="mdi-shuffle-variant" title="随机背景图" @click="setRandomBackground"/>

          <v-list-item
              v-for="item in backgroundOptions"
              :key="item.key"
              :title="item.label"
              @click="setBackground(item.key)"
          >
            <template #prepend>
              <v-avatar class="me-1 bg-surface" rounded="lg" size="30">
                <v-img :src="item.thumbUrl" cover/>
              </v-avatar>
            </template>
            <template #append>
              <v-icon v-if="state.background === item.key" color="primary" icon="mdi-check"/>
            </template>
          </v-list-item>
        </v-list>
      </v-card>
    </v-menu>
  </div>
</template>

<script lang="ts" setup>
import {computed} from 'vue'
import {useThemePreferences} from '../theme/themeEngine'

const {
  state,
  modeOptions,
  backgroundOptions,
  setMode,
  setBackground,
  setRandomBackground,
  toggleLightDark,
  isDark: getIsDark
} = useThemePreferences()
const isDark = computed(() => getIsDark())

function toggleLightDarkFromTrigger(event: MouseEvent) {
  const trigger = event.currentTarget as HTMLElement | null
  if (!trigger) {
    toggleLightDark()
    return
  }

  const rect = trigger.getBoundingClientRect()
  toggleLightDark({
    x: rect.left + rect.width / 2,
    y: rect.top + rect.height / 2
  })
}
</script>
