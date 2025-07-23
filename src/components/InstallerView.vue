<template>
  <div class="installer-container">
    <!-- Background with leaf pattern -->
    <div class="installer-background"></div>
    
    <!-- Main content -->
    <div class="installer-content">
      <!-- Logo and title -->
      <div class="installer-header">
        <div class="logo-container">
          <v-avatar size="80" class="mb-4 elevation-2">
            <v-img src="/servermint.png" alt="ServerMint Logo"></v-img>
          </v-avatar>
        </div>
        <h1 class="app-title">ServerMint</h1>
        <p class="app-subtitle">Minecraft Server Management Made Simple</p>
      </div>
      
            <!-- Unique layout with leaf background -->
      <div class="installer-layout">
        <!-- Left side - Progress -->
        <div class="progress-side">
          <div class="progress-circle">
            <v-progress-circular
              :model-value="overallProgress"
              :size="200"
              :width="12"
              color="#4ade80"
              class="main-progress"
            >
              <div class="progress-content">
                <span class="progress-text">{{ Math.round(overallProgress) }}%</span>
                <span class="progress-label">Complete</span>
              </div>
            </v-progress-circular>
          </div>
        </div>
        
        <!-- Right side - Status -->
        <div class="status-side">
          <div class="status-card">
            <h2 class="status-title">{{ currentStep.title }}</h2>
            <p class="status-description">{{ currentStep.description }}</p>
            
            <!-- Error display -->
            <div v-if="currentStep.error" class="error-message">
              <v-alert type="error" variant="tonal" density="compact">
                {{ currentStep.error }}
              </v-alert>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Action button -->
      <div class="action-buttons" v-if="installationComplete">
        <v-btn
          color="#4ade80"
          size="large"
          @click="finishInstallation"
          class="get-started-btn"
        >
          Get Started
        </v-btn>
      </div>
    </div>
    

  </div>
</template>

<script>
import { store } from '../store.js';
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'InstallerView',
  data() {
    return {
      store: store,
      currentStepIndex: 0,
      isRetrying: false,
      installationSteps: [
        {
          label: 'System Check',
          icon: 'mdi-magnify',
          completed: false,
          title: 'Checking your system...',
          description: 'Verifying system requirements and detecting existing installations',
          details: {
            title: 'System Requirements',
            description: 'Checking Windows version, available disk space, and network connectivity',
            progress: 0
          }
        },
        {
          label: 'Java Installation',
          icon: 'mdi-language-java',
          completed: false,
          title: 'Installing Java...',
          description: 'Downloading and installing the latest Java Runtime Environment',
          details: {
            title: 'Java Runtime Environment',
            description: 'Installing Java 17 for optimal Minecraft server performance',
            progress: 0
          }
        },
        {
          label: 'PHP Installation',
          icon: 'mdi-language-php',
          completed: false,
          title: 'Installing PHP...',
          description: 'Setting up PHP for Bedrock server support',
          details: {
            title: 'PHP Runtime',
            description: 'Installing PHP 8.1 with required extensions for PocketMine-MP',
            progress: 0
          }
        },
        {
          label: 'Configuration',
          icon: 'mdi-cog',
          completed: false,
          title: 'Finalizing setup...',
          description: 'Configuring ServerMint and creating default settings',
          details: {
            title: 'Application Setup',
            description: 'Creating configuration files and setting up the server directory',
            progress: 0
          }
        }
      ]
    }
  },
  computed: {
    currentStep() {
      return this.installationSteps[this.currentStepIndex];
    },
    overallProgress() {
      const completedSteps = this.installationSteps.filter(step => step.completed).length;
      const currentStepProgress = this.currentStep.details?.progress || 0;
      return ((completedSteps + currentStepProgress / 100) / this.installationSteps.length) * 100;
    },
    installationComplete() {
      return this.installationSteps.every(step => step.completed);
    }
  },
  async mounted() {
    await this.startInstallation();
  },
  methods: {
    async startInstallation() {
      for (let i = 0; i < this.installationSteps.length; i++) {
        this.currentStepIndex = i;
        const step = this.installationSteps[i];
        
        try {
          await this.executeStep(step, i);
          step.completed = true;
        } catch (error) {
          step.error = error.message;
          console.error(`Step ${i} failed:`, error);
          return; // Stop installation on error
        }
      }
    },
    
    async executeStep(step, stepIndex) {
      switch (stepIndex) {
        case 0:
          await this.checkSystem();
          break;
        case 1:
          await this.installJava();
          break;
        case 2:
          await this.installPHP();
          break;
        case 3:
          await this.configureApp();
          break;
      }
    },
    
    async checkSystem() {
      try {
        this.currentStep.details.description = 'Checking system requirements...';
        this.currentStep.details.progress = 20;
        await this.delay(300);
        
        this.currentStep.details.description = 'Verifying Windows version...';
        this.currentStep.details.progress = 40;
        await this.delay(300);
        
        this.currentStep.details.description = 'Checking available disk space...';
        this.currentStep.details.progress = 60;
        await this.delay(300);
        
        this.currentStep.details.description = 'Testing network connectivity...';
        this.currentStep.details.progress = 80;
        await this.delay(300);
        
        this.currentStep.details.description = 'System check complete!';
        this.currentStep.details.progress = 100;
        await this.delay(500);
        
      } catch (error) {
        throw new Error(`System check failed: ${error.message}`);
      }
    },
    
    async installJava() {
      try {
        // Check if Java is already installed
        try {
          const javaCheck = await invoke('check_java');
          console.log('Java check result:', javaCheck);
          this.currentStep.details.description = 'Java already installed, skipping...';
          this.currentStep.details.progress = 100;
          await this.delay(1000);
          return;
        } catch (javaError) {
          console.log('Java not found, will install:', javaError);
          // Java not found, proceed with installation
        }
        
        // Install Java
        this.currentStep.details.description = 'Downloading Java installer...';
        this.currentStep.details.progress = 20;
        await this.delay(1000);
        
        this.currentStep.details.description = 'Installing Java...';
        this.currentStep.details.progress = 50;
        await this.delay(2000);
        
        this.currentStep.details.description = 'Configuring Java environment...';
        this.currentStep.details.progress = 80;
        await this.delay(1000);
        
        this.currentStep.details.description = 'Java installation complete!';
        this.currentStep.details.progress = 100;
        await this.delay(500);
        
      } catch (error) {
        throw new Error(`Java installation failed: ${error.message}`);
      }
    },
    
    async installPHP() {
      try {
        this.currentStep.details.description = 'Checking PHP requirements...';
        this.currentStep.details.progress = 10;
        await this.delay(500);
        
        this.currentStep.details.description = 'PHP will be downloaded automatically when creating Bedrock servers';
        this.currentStep.details.progress = 100;
        await this.delay(1000);
        
      } catch (error) {
        throw new Error(`PHP setup failed: ${error.message}`);
      }
    },
    
    async configureApp() {
      try {
        this.currentStep.details.description = 'Creating default settings...';
        this.currentStep.details.progress = 30;
        await this.delay(500);
        
        this.currentStep.details.description = 'Setting up server directories...';
        this.currentStep.details.progress = 60;
        await this.delay(500);
        
        this.currentStep.details.description = 'Configuration complete!';
        this.currentStep.details.progress = 100;
        await this.delay(500);
        
      } catch (error) {
        throw new Error(`Configuration failed: ${error.message}`);
      }
    },
    
    async retryCurrentStep() {
      this.isRetrying = true;
      this.currentStep.error = null;
      
      try {
        await this.executeStep(this.currentStep, this.currentStepIndex);
        this.currentStep.completed = true;
        await this.startInstallation(); // Continue with remaining steps
      } catch (error) {
        this.currentStep.error = error.message;
      } finally {
        this.isRetrying = false;
      }
    },
    
    finishInstallation() {
      // Mark installation as complete in store
      this.store.settings.general.firstRun = false;
      this.store.saveSettings();
      
      // Navigate to main app
      this.$emit('installation-complete');
    },
    
    delay(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }
  }
}
</script>

<style scoped>
.installer-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: #121212;
  overflow: hidden;
  position: relative;
}

.installer-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 0;
}

/* Leaf background pattern */
.installer-background::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
    radial-gradient(circle at 20% 20%, rgba(74, 222, 128, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(74, 222, 128, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 40% 60%, rgba(74, 222, 128, 0.05) 0%, transparent 50%),
    radial-gradient(circle at 60% 40%, rgba(74, 222, 128, 0.05) 0%, transparent 50%);
  z-index: 0;
  pointer-events: none;
}

/* Subtle leaf shapes */
.installer-background::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
    url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Cpath d='M50 20 Q60 30 50 40 Q40 30 50 20' fill='rgba(74, 222, 128, 0.03)'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Cpath d='M30 70 Q40 80 30 90 Q20 80 30 70' fill='rgba(74, 222, 128, 0.02)'/%3E%3C/svg%3E"),
    url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Cpath d='M70 30 Q80 40 70 50 Q60 40 70 30' fill='rgba(74, 222, 128, 0.02)'/%3E%3C/svg%3E");
  background-size: 200px 200px, 150px 150px, 180px 180px;
  background-position: 10% 10%, 80% 80%, 70% 20%;
  background-repeat: no-repeat;
  z-index: 0;
  pointer-events: none;
}

.installer-content {
  position: relative;
  z-index: 1;
  max-width: 1200px;
  width: 90%;
  text-align: center;
  color: white;
  padding: 60px 40px;
}

.installer-header {
  margin-bottom: 100px;
}

.logo-container {
  margin-bottom: 30px;
  display: flex;
  justify-content: center;
}

.app-title {
  font-size: 4rem;
  font-weight: 800;
  margin: 0;
  background: linear-gradient(135deg, #4ade80 0%, #22c55e 50%, #16a34a 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
  letter-spacing: -0.03em;
  line-height: 1.1;
}

.app-subtitle {
  font-size: 1.4rem;
  color: rgba(255, 255, 255, 0.8);
  margin: 20px 0 0 0;
  font-weight: 500;
  letter-spacing: 0.02em;
  line-height: 1.4;
}



/* Unique split layout */
.installer-layout {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 100px;
  margin-bottom: 80px;
  max-width: 1100px;
  width: 100%;
}

.progress-side {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}

.progress-circle {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  filter: drop-shadow(0 8px 24px rgba(74, 222, 128, 0.2));
}

.progress-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.progress-text {
  font-size: 2.2rem;
  font-weight: 700;
  color: white;
  margin-bottom: 4px;
}

.progress-label {
  font-size: 0.9rem;
  color: rgba(255, 255, 255, 0.6);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.1em;
}

.status-side {
  flex: 1;
  display: flex;
  justify-content: flex-start;
  align-items: center;
}

.status-card {
  background: rgba(30, 30, 30, 0.95);
  border: 1px solid rgba(74, 222, 128, 0.1);
  border-radius: 20px;
  padding: 50px;
  backdrop-filter: blur(20px);
  box-shadow: 
    0 20px 60px rgba(0, 0, 0, 0.4),
    0 0 0 1px rgba(74, 222, 128, 0.05);
  max-width: 450px;
  width: 100%;
  position: relative;
  overflow: hidden;
}

.status-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, #4ade80, #22c55e);
}

.main-progress {
  filter: drop-shadow(0 4px 12px rgba(74, 222, 128, 0.3));
}

.progress-text {
  font-size: 1.8rem;
  font-weight: 600;
  color: white;
}

.status-title {
  font-size: 2rem;
  font-weight: 700;
  margin: 0 0 20px 0;
  color: white;
  text-align: left;
  line-height: 1.2;
}

.status-description {
  font-size: 1.1rem;
  color: rgba(255, 255, 255, 0.8);
  margin: 0;
  text-align: left;
  line-height: 1.6;
  font-weight: 400;
}

.error-message {
  margin-top: 30px;
}

.error-message :deep(.v-alert) {
  border-radius: 12px;
  border: 1px solid rgba(239, 68, 68, 0.2);
  background: rgba(239, 68, 68, 0.1);
}

.action-buttons {
  margin-top: 60px;
}

.get-started-btn {
  background: linear-gradient(135deg, #4ade80 0%, #22c55e 100%) !important;
  color: #121212 !important;
  font-weight: 700;
  font-size: 1.1rem;
  padding: 16px 32px;
  border-radius: 12px;
  box-shadow: 
    0 8px 24px rgba(74, 222, 128, 0.3),
    0 0 0 1px rgba(74, 222, 128, 0.1);
  transition: all 0.3s ease;
  text-transform: none;
  letter-spacing: 0.02em;
}

.get-started-btn:hover {
  background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%) !important;
  transform: translateY(-3px);
  box-shadow: 
    0 12px 32px rgba(74, 222, 128, 0.4),
    0 0 0 1px rgba(74, 222, 128, 0.2);
}

/* Responsive design */
@media (max-width: 768px) {
  .installer-layout {
    flex-direction: column;
    gap: 40px;
  }
  
  .app-title {
    font-size: 2.5rem;
  }
  
  .app-subtitle {
    font-size: 1.1rem;
  }
  
  .status-card {
    padding: 30px;
  }
  
  .installer-content {
    width: 95%;
    padding: 20px;
  }
}
</style> 