<template>
  <div class="guide-button-container">
    <v-btn
      variant="text"
      size="small"
      @click="restartGuide"
      class="guide-button"
      :title="tooltip"
    >
      <v-icon>mdi-help-circle-outline</v-icon>
      <v-tooltip
        activator="parent"
        location="bottom"
        :text="tooltip"
      ></v-tooltip>
    </v-btn>
  </div>
</template>

<script>
export default {
  name: 'GuideButton',
  props: {
    tooltip: {
      type: String,
      default: 'Show Getting Started Guide'
    }
  },
  methods: {
    restartGuide() {
      // Clear the guide completion flag
      localStorage.removeItem('servermint-guide-completed');
      
      // Emit event to restart guide
      this.$emit('restart-guide');
      
      // Show confirmation
      window.showInfo('Guide Restarted', 'The getting started guide will appear shortly.');
    }
  }
}
</script>

<style scoped>
.guide-button-container {
  display: inline-block;
}

.guide-button {
  color: #ccc;
  transition: color 0.2s ease;
}

.guide-button:hover {
  color: #4ade80;
}

.guide-button .v-icon {
  font-size: 20px;
}
</style> 