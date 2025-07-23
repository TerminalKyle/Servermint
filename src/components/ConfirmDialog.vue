<template>
  <v-dialog v-model="show" max-width="500" persistent>
    <v-card class="confirm-dialog">
      <!-- Header -->
      <v-card-title class="d-flex align-center pa-4">
        <v-avatar size="40" color="error" class="mr-3">
          <v-icon color="white" size="24">mdi-alert-circle</v-icon>
        </v-avatar>
        <div>
          <div class="text-h6 font-weight-bold">{{ title }}</div>
          <div class="text-caption text-medium-emphasis">{{ subtitle }}</div>
        </div>
      </v-card-title>

      <!-- Content -->
      <v-card-text class="pa-4 pt-0">
        <div class="text-body-1 mb-3">{{ message }}</div>
        
        <!-- Warning details -->
        <v-alert
          v-if="warningDetails"
          type="warning"
          variant="tonal"
          class="mb-3"
          density="compact"
        >
          <template #prepend>
            <v-icon>mdi-information</v-icon>
          </template>
          <div class="text-body-2">
            <div class="font-weight-medium mb-1">This action will:</div>
            <ul class="ma-0 pa-0 pl-3">
              <li v-for="(detail, index) in warningDetails" :key="index">
                {{ detail }}
              </li>
            </ul>
          </div>
        </v-alert>

        <!-- Confirmation input -->
        <div v-if="requireConfirmation" class="mt-4">
          <v-text-field
            v-model="confirmationText"
            :label="confirmationLabel"
            variant="outlined"
            density="comfortable"
            :placeholder="confirmationPlaceholder"
            :error-messages="confirmationError"
            @keydown.enter="handleConfirm"
          ></v-text-field>
        </div>
      </v-card-text>

      <!-- Actions -->
      <v-card-actions class="pa-4 pt-0">
        <v-spacer></v-spacer>
        
        <v-btn
          variant="text"
          @click="handleCancel"
          :disabled="loading"
        >
          {{ cancelText }}
        </v-btn>
        
        <v-btn
          :color="confirmColor"
          :variant="confirmVariant"
          @click="handleConfirm"
          :loading="loading"
          :disabled="requireConfirmation && confirmationText !== confirmationPlaceholder"
        >
          <v-icon v-if="confirmIcon" class="mr-2">{{ confirmIcon }}</v-icon>
          {{ confirmText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
export default {
  name: 'ConfirmDialog',
  props: {
    modelValue: {
      type: Boolean,
      default: false
    },
    title: {
      type: String,
      default: 'Confirm Action'
    },
    subtitle: {
      type: String,
      default: ''
    },
    message: {
      type: String,
      default: 'Are you sure you want to proceed?'
    },
    warningDetails: {
      type: Array,
      default: () => []
    },
    confirmText: {
      type: String,
      default: 'Confirm'
    },
    cancelText: {
      type: String,
      default: 'Cancel'
    },
    confirmColor: {
      type: String,
      default: 'error'
    },
    confirmVariant: {
      type: String,
      default: 'elevated'
    },
    confirmIcon: {
      type: String,
      default: ''
    },
    requireConfirmation: {
      type: Boolean,
      default: false
    },
    confirmationLabel: {
      type: String,
      default: 'Type to confirm'
    },
    confirmationPlaceholder: {
      type: String,
      default: 'Type "DELETE" to confirm'
    },
    loading: {
      type: Boolean,
      default: false
    }
  },
  emits: ['update:modelValue', 'confirm', 'cancel'],
  data() {
    return {
      confirmationText: '',
      confirmationError: ''
    }
  },
  computed: {
    show: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', value)
      }
    }
  },
  watch: {
    show(newValue) {
      if (newValue) {
        this.confirmationText = ''
        this.confirmationError = ''
      }
    }
  },
  methods: {
    handleConfirm() {
      if (this.requireConfirmation && this.confirmationText !== this.confirmationPlaceholder) {
        this.confirmationError = `Please type "${this.confirmationPlaceholder}" to confirm`
        return
      }
      
      this.$emit('confirm')
    },
    handleCancel() {
      this.$emit('cancel')
      this.show = false
    }
  }
}
</script>

<style scoped>
.confirm-dialog {
  border-radius: 12px;
}

.confirm-dialog :deep(.v-card-title) {
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.confirm-dialog :deep(.v-card-actions) {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}
</style> 