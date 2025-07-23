<template>
  <div class="toast-container">
    <TransitionGroup name="toast" tag="div" class="toast-list">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="toast"
        :class="toast.type"
        @click="removeToast(toast.id)"
      >
        <div class="toast-content">
          <div class="toast-icon">
            <v-icon v-if="toast.type === 'success'" color="white">mdi-check-circle</v-icon>
            <v-icon v-else-if="toast.type === 'error'" color="white">mdi-alert-circle</v-icon>
            <v-icon v-else-if="toast.type === 'warning'" color="white">mdi-alert</v-icon>
            <v-icon v-else color="white">mdi-information</v-icon>
          </div>
          <div class="toast-text">
            <div class="toast-title">{{ toast.title }}</div>
            <div v-if="toast.message" class="toast-message">{{ toast.message }}</div>
          </div>
          <div class="toast-close">
            <v-icon size="small" color="white" @click.stop="removeToast(toast.id)">mdi-close</v-icon>
          </div>
        </div>
        <div class="toast-progress" :style="{ width: `${toast.progress}%` }"></div>
      </div>
    </TransitionGroup>
  </div>
</template>

<script>
export default {
  name: 'ToastNotification',
  data() {
    return {
      toasts: [],
      nextId: 1
    }
  },
  mounted() {
    // Make the toast system globally available
    window.showToast = this.showToast;
    window.showSuccess = this.showSuccess;
    window.showError = this.showError;
    window.showWarning = this.showWarning;
    window.showInfo = this.showInfo;
  },
  beforeUnmount() {
    // Clean up global references
    delete window.showToast;
    delete window.showSuccess;
    delete window.showError;
    delete window.showWarning;
    delete window.showInfo;
  },
  methods: {
    showToast(type, title, message = '', duration = 5000) {
      const toast = {
        id: this.nextId++,
        type,
        title,
        message,
        progress: 100,
        duration
      };
      
      this.toasts.push(toast);
      
      // Auto-remove after duration
      if (duration > 0) {
        const startTime = Date.now();
        const progressInterval = setInterval(() => {
          const elapsed = Date.now() - startTime;
          const remaining = Math.max(0, duration - elapsed);
          toast.progress = (remaining / duration) * 100;
          
          if (remaining <= 0) {
            clearInterval(progressInterval);
            this.removeToast(toast.id);
          }
        }, 50);
      }
      
      return toast.id;
    },
    
    showSuccess(title, message = '', duration = 5000) {
      return this.showToast('success', title, message, duration);
    },
    
    showError(title, message = '', duration = 7000) {
      return this.showToast('error', title, message, duration);
    },
    
    showWarning(title, message = '', duration = 6000) {
      return this.showToast('warning', title, message, duration);
    },
    
    showInfo(title, message = '', duration = 5000) {
      return this.showToast('info', title, message, duration);
    },
    
    removeToast(id) {
      const index = this.toasts.findIndex(toast => toast.id === id);
      if (index !== -1) {
        this.toasts.splice(index, 1);
      }
    }
  }
}
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  pointer-events: none;
}

.toast-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  pointer-events: none;
}

.toast {
  background: rgba(30, 30, 30, 0.95);
  border: 1px solid rgba(74, 222, 128, 0.2);
  border-radius: 12px;
  padding: 16px;
  min-width: 320px;
  max-width: 400px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(12px);
  position: relative;
  overflow: hidden;
  pointer-events: auto;
  cursor: pointer;
  transition: all 0.3s ease;
}

.toast:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
}

.toast.success {
  border-color: rgba(74, 222, 128, 0.3);
  background: linear-gradient(135deg, rgba(74, 222, 128, 0.1) 0%, rgba(30, 30, 30, 0.95) 100%);
}

.toast.error {
  border-color: rgba(239, 68, 68, 0.3);
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(30, 30, 30, 0.95) 100%);
}

.toast.warning {
  border-color: rgba(245, 158, 11, 0.3);
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.1) 0%, rgba(30, 30, 30, 0.95) 100%);
}

.toast.info {
  border-color: rgba(59, 130, 246, 0.3);
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(30, 30, 30, 0.95) 100%);
}

.toast-content {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.toast-icon {
  flex-shrink: 0;
  margin-top: 2px;
}

.toast-text {
  flex-grow: 1;
  min-width: 0;
}

.toast-title {
  font-weight: 600;
  font-size: 14px;
  color: white;
  margin-bottom: 4px;
  line-height: 1.3;
}

.toast-message {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.8);
  line-height: 1.4;
  word-wrap: break-word;
}

.toast-close {
  flex-shrink: 0;
  opacity: 0.7;
  transition: opacity 0.2s ease;
  cursor: pointer;
  padding: 2px;
  border-radius: 4px;
}

.toast-close:hover {
  opacity: 1;
  background: rgba(255, 255, 255, 0.1);
}

.toast-progress {
  position: absolute;
  bottom: 0;
  left: 0;
  height: 3px;
  background: linear-gradient(90deg, rgba(74, 222, 128, 0.8) 0%, rgba(74, 222, 128, 0.4) 100%);
  transition: width 0.05s linear;
}

.toast.error .toast-progress {
  background: linear-gradient(90deg, rgba(239, 68, 68, 0.8) 0%, rgba(239, 68, 68, 0.4) 100%);
}

.toast.warning .toast-progress {
  background: linear-gradient(90deg, rgba(245, 158, 11, 0.8) 0%, rgba(245, 158, 11, 0.4) 100%);
}

.toast.info .toast-progress {
  background: linear-gradient(90deg, rgba(59, 130, 246, 0.8) 0%, rgba(59, 130, 246, 0.4) 100%);
}

/* Toast animations */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.9);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.9);
}

.toast-move {
  transition: transform 0.3s ease;
}

/* Responsive design */
@media (max-width: 768px) {
  .toast-container {
    top: 10px;
    right: 10px;
    left: 10px;
  }
  
  .toast {
    min-width: auto;
    max-width: none;
  }
}
</style> 