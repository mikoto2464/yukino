<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row>
      <v-col cols="12" lg="6" md="8">
        <v-card class="pa-6" elevation="2" rounded="xl">
          <h1 class="text-h5 font-weight-bold mb-2 text-primary">管理员页面</h1>
          <p class="text-medium-emphasis mb-6">该页面仅允许管理员访问。</p>

          <v-btn :to="{ name: 'dashboard' }" color="primary" variant="tonal">返回控制台</v-btn>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>
