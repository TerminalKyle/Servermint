<template>
  <div v-if="isActive" class="guide-overlay">
    <!-- Backdrop -->
    <div class="guide-backdrop" @click="closeGuide"></div>
    
    <!-- Current step tooltip -->
    <div 
      v-if="currentStep"
      class="guide-tooltip"
      :class="currentStep.position || 'bottom'"
      :style="tooltipStyle"
    >
      <div class="guide-header">
        <div class="guide-step-indicator">
          Step {{ currentStepIndex + 1 }} of {{ steps.length }}
        </div>
        <v-btn 
          icon="mdi-close" 
          variant="text" 
          size="small" 
          @click="closeGuide"
          class="guide-close-btn"
        ></v-btn>
      </div>
      
      <div class="guide-content">
        <h3 class="guide-title">{{ currentStep.title }}</h3>
        <p class="guide-description">{{ currentStep.description }}</p>
        
        <div v-if="currentStep.hint" class="guide-hint">
          <v-icon size="small" color="primary">mdi-lightbulb-outline</v-icon>
          <span>{{ currentStep.hint }}</span>
        </div>
      </div>
      
      <div class="guide-actions">
        <v-btn 
          v-if="currentStepIndex > 0"
          variant="outlined" 
          size="small"
          @click="previousStep"
        >
          Previous
        </v-btn>
        
        <v-btn 
          v-if="currentStepIndex < steps.length - 1"
          color="primary" 
          size="small"
          @click="nextStep"
        >
          Next
        </v-btn>
        
        <v-btn 
          v-else
          color="primary" 
          size="small"
          @click="finishGuide"
        >
          Get Started!
        </v-btn>
      </div>
    </div>
    
    <!-- Skip button -->
    <div class="guide-skip">
      <v-btn 
        variant="text" 
        size="small" 
        @click="closeGuide"
        class="guide-skip-btn"
      >
        Skip Guide
      </v-btn>
    </div>
  </div>
</template>

<script>
export default {
  name: 'GuideOverlay',
  data() {
    return {
      isActive: false,
      currentStepIndex: 0,
      resizeTimeout: null,
      steps: [
        {
          title: 'Welcome to ServerMint!',
          description: 'Let\'s take a quick tour to help you get started with managing your Minecraft servers.',
          position: 'center',
          hint: 'This guide will show you the key features and help you create your first server.'
        },
        {
          title: 'Add Your First Server',
          description: 'Click the "ADD SERVER" button to create your first Minecraft server instance.',
          position: 'bottom',
          target: '.add-server-btn',
          hint: 'You can create custom servers or import existing ones from other launchers.'
        },
        {
          title: 'Server Management',
          description: 'Once you have servers, you can start, stop, and manage them from this dashboard.',
          position: 'top',
          target: '.server-card',
          hint: 'Each server card shows the server status and quick action buttons.'
        },
        {
          title: 'Server Settings',
          description: 'Click the settings icon on any server to configure memory, ports, and other options.',
          position: 'left',
          target: '.server-settings-btn',
          hint: 'You can adjust memory allocation, server ports, and advanced settings here.'
        },
        {
          title: 'File Management',
          description: 'Access your server files, plugins, and worlds through the file manager.',
          position: 'right',
          target: '.file-manager-btn',
          hint: 'Upload plugins, manage worlds, and edit server configuration files.'
        },
        {
          title: 'Console & Logs',
          description: 'Monitor your server in real-time and send commands through the console.',
          position: 'bottom',
          target: '.console-btn',
          hint: 'View live logs, send commands, and monitor server performance.'
        },
        {
          title: 'Backup & Export',
          description: 'Create backups of your servers and export them for sharing or migration.',
          position: 'top',
          target: '.backup-btn',
          hint: 'Regular backups ensure you never lose your server data.'
        },
        {
          title: 'You\'re All Set!',
          description: 'You now know the basics of ServerMint. Start by creating your first server!',
          position: 'center',
          hint: 'Need help? Check the documentation or ask in our community.'
        }
      ]
    }
  },
  computed: {
    currentStep() {
      return this.steps[this.currentStepIndex];
    },
             tooltipStyle() {
      if (!this.currentStep) {
        return {};
      }
      
      if (this.currentStep.position === 'center') {
        return {
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)'
        };
      }
      
      // If no target is specified, center it
      if (!this.currentStep.target) {
        return {
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)'
        };
      }
       
       const target = document.querySelector(this.currentStep.target);
       if (!target) {
         // Fallback to center if target not found
         return {
           top: '50%',
           left: '50%',
           transform: 'translate(-50%, -50%)'
         };
       }
       
       const rect = target.getBoundingClientRect();
       const position = this.currentStep.position || 'bottom';
       
       // Tooltip dimensions (approximate)
       const tooltipWidth = 400;
       const tooltipHeight = 250;
       const margin = 60; // Safe margin from screen edges
       
       let top, left;
       
               switch (position) {
          case 'top':
            top = rect.top - tooltipHeight - 20;
            left = rect.left + rect.width / 2;
            break;
          case 'bottom':
            top = rect.bottom + 20;
            left = rect.left + rect.width / 2;
            break;
          case 'left':
            top = rect.top + rect.height / 2;
            left = rect.left - tooltipWidth - 20;
            break;
          case 'right':
            top = rect.top + rect.height / 2;
            left = rect.right + 20;
            break;
          case 'center':
            // For center position, always center it regardless of target
            top = window.innerHeight / 2;
            left = window.innerWidth / 2;
            break;
          default:
            top = rect.bottom + 20;
            left = rect.left + rect.width / 2;
                }
        
        // Ensure the tooltip is always within viewport bounds with safe margins
        let finalTop = Math.max(margin, Math.min(window.innerHeight - margin - tooltipHeight, top));
        let finalLeft = Math.max(margin + tooltipWidth / 2, Math.min(window.innerWidth - margin - tooltipWidth / 2, left));
        
        // Check if tooltip would be cut off
        const wouldBeCutOff = finalTop < margin || 
                             finalTop > window.innerHeight - margin - tooltipHeight ||
                             finalLeft < margin + tooltipWidth / 2 || 
                             finalLeft > window.innerWidth - margin - tooltipWidth / 2;
        
        // If the tooltip would be cut off, try alternative positions
        if (wouldBeCutOff) {
          // Try positioning above the target
          if (rect.top > tooltipHeight + margin) {
            finalTop = rect.top - tooltipHeight - 20;
            finalLeft = Math.max(margin + tooltipWidth / 2, Math.min(window.innerWidth - margin - tooltipWidth / 2, rect.left + rect.width / 2));
          }
          // Try positioning to the left
          else if (rect.left > tooltipWidth + margin) {
            finalTop = Math.max(margin, Math.min(window.innerHeight - margin - tooltipHeight, rect.top + rect.height / 2));
            finalLeft = rect.left - tooltipWidth - 20;
          }
          // Try positioning to the right
          else if (rect.right + tooltipWidth + margin < window.innerWidth) {
            finalTop = Math.max(margin, Math.min(window.innerHeight - margin - tooltipHeight, rect.top + rect.height / 2));
            finalLeft = rect.right + 20;
          }
          // If all else fails, center it
          else {
            finalTop = window.innerHeight / 2;
            finalLeft = window.innerWidth / 2;
          }
        }
        
        // Final bounds check
        finalTop = Math.max(margin, Math.min(window.innerHeight - margin - tooltipHeight, finalTop));
        finalLeft = Math.max(margin + tooltipWidth / 2, Math.min(window.innerWidth - margin - tooltipWidth / 2, finalLeft));
       
       return {
         top: `${finalTop}px`,
         left: `${finalLeft}px`,
         transform: 'translate(-50%, -50%)'
       };
     }
  },
  methods: {
    startGuide() {
      this.isActive = true;
      this.currentStepIndex = 0;
      this.highlightCurrentTarget();
    },
    
    nextStep() {
      if (this.currentStepIndex < this.steps.length - 1) {
        this.currentStepIndex++;
        this.highlightCurrentTarget();
      }
    },
    
    previousStep() {
      if (this.currentStepIndex > 0) {
        this.currentStepIndex--;
        this.highlightCurrentTarget();
      }
    },
    
    closeGuide() {
      this.isActive = false;
      this.removeHighlights();
      // Mark guide as completed
      localStorage.setItem('servermint-guide-completed', 'true');
    },
    
    finishGuide() {
      this.closeGuide();
      // Show success message
      window.showSuccess('Welcome to ServerMint!', 'You\'re all set to start managing your Minecraft servers.');
    },
    
         highlightCurrentTarget() {
       this.removeHighlights();
       
       if (this.currentStep && this.currentStep.target) {
         const target = document.querySelector(this.currentStep.target);
         if (target) {
           target.classList.add('guide-highlight');
           // Force position update after highlighting
           this.$nextTick(() => {
             this.updateTooltipPosition();
           });
         }
       }
     },
    
    removeHighlights() {
      document.querySelectorAll('.guide-highlight').forEach(el => {
        el.classList.remove('guide-highlight');
      });
    },
    
         checkIfFirstTime() {
       const guideCompleted = localStorage.getItem('servermint-guide-completed');
       if (!guideCompleted) {
         // Show guide after a short delay to ensure DOM is ready
         setTimeout(() => {
           this.startGuide();
         }, 1500);
       }
     },
     
           updateTooltipPosition() {
        // Debounce the position update to prevent ResizeObserver errors
        if (this.resizeTimeout) {
          clearTimeout(this.resizeTimeout);
        }
        this.resizeTimeout = setTimeout(() => {
          // Force a re-computation of the tooltip position
          this.$forceUpdate();
        }, 100);
      }
  },
  
         mounted() {
      this.checkIfFirstTime();
      
      // Add window resize listener to update tooltip position with debouncing
      let resizeTimeout;
      const debouncedResizeHandler = () => {
        clearTimeout(resizeTimeout);
        resizeTimeout = setTimeout(() => {
          this.updateTooltipPosition();
        }, 150);
      };
      
      window.addEventListener('resize', debouncedResizeHandler);
      
      // Store the handler for cleanup
      this.resizeHandler = debouncedResizeHandler;
    },
   
       beforeUnmount() {
      // Clean up event listener and timeout
      if (this.resizeHandler) {
        window.removeEventListener('resize', this.resizeHandler);
      }
      if (this.resizeTimeout) {
        clearTimeout(this.resizeTimeout);
      }
    }
}
</script>

<style scoped>
.guide-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
  pointer-events: none;
}

.guide-backdrop {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(2px);
  pointer-events: auto;
}

.guide-tooltip {
  position: absolute;
  max-width: 400px;
  min-width: 280px;
  background-color: #1e1e1e;
  border: 2px solid #4ade80;
  border-radius: 12px;
  padding: 0;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  pointer-events: auto;
  z-index: 10000;
  overflow: hidden;
  word-wrap: break-word;
}

/* Responsive adjustments for smaller screens */
@media (max-width: 768px) {
  .guide-tooltip {
    max-width: 320px;
    min-width: 280px;
    margin: 0 20px;
    left: 50% !important;
    transform: translateX(-50%) !important;
  }
  
  .guide-content {
    padding: 0 16px 12px 16px;
  }
  
  .guide-header {
    padding: 12px 16px 0 16px;
  }
  
  .guide-actions {
    padding: 12px 16px 16px 16px;
  }
}

@media (max-width: 480px) {
  .guide-tooltip {
    max-width: 280px;
    min-width: 240px;
    margin: 0 10px;
  }
}

.guide-tooltip::before {
  content: '';
  position: absolute;
  width: 0;
  height: 0;
  border: 8px solid transparent;
}

.guide-tooltip.top::before {
  bottom: -16px;
  left: 50%;
  transform: translateX(-50%);
  border-top-color: #4ade80;
}

.guide-tooltip.bottom::before {
  top: -16px;
  left: 50%;
  transform: translateX(-50%);
  border-bottom-color: #4ade80;
}

.guide-tooltip.left::before {
  right: -16px;
  top: 50%;
  transform: translateY(-50%);
  border-left-color: #4ade80;
}

.guide-tooltip.right::before {
  left: -16px;
  top: 50%;
  transform: translateY(-50%);
  border-right-color: #4ade80;
}

.guide-tooltip.center {
  transform: translate(-50%, -50%) !important;
}

.guide-tooltip.center::before {
  display: none;
}

.guide-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px 0 20px;
  border-bottom: 1px solid #333;
  margin-bottom: 16px;
}

.guide-step-indicator {
  font-size: 12px;
  color: #888;
  font-weight: 500;
}

.guide-close-btn {
  color: #888;
}

.guide-content {
  padding: 0 20px 16px 20px;
}

.guide-title {
  font-size: 18px;
  font-weight: 600;
  color: #fff;
  margin: 0 0 8px 0;
}

.guide-description {
  font-size: 14px;
  color: #ccc;
  line-height: 1.5;
  margin: 0 0 12px 0;
}

.guide-hint {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: #4ade80;
  background-color: rgba(74, 222, 128, 0.1);
  padding: 8px 12px;
  border-radius: 6px;
  border-left: 3px solid #4ade80;
}

.guide-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px 20px 20px;
  border-top: 1px solid #333;
  margin-top: 16px;
}

.guide-skip {
  position: absolute;
  top: 20px;
  right: 20px;
  pointer-events: auto;
}

.guide-skip-btn {
  color: #888;
  font-size: 12px;
}

/* Highlight effect for targeted elements */
:global(.guide-highlight) {
  position: relative;
  z-index: 10001;
  box-shadow: 0 0 0 3px rgba(74, 222, 128, 0.4), 0 0 15px rgba(74, 222, 128, 0.3);
  border-radius: 6px;
  animation: guide-pulse 2s infinite;
  transition: all 0.3s ease;
}

@keyframes guide-pulse {
  0%, 100% {
    box-shadow: 0 0 0 3px rgba(74, 222, 128, 0.4), 0 0 15px rgba(74, 222, 128, 0.3);
  }
  50% {
    box-shadow: 0 0 0 4px rgba(74, 222, 128, 0.5), 0 0 20px rgba(74, 222, 128, 0.4);
  }
}
</style> 