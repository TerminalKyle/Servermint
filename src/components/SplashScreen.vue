<template>
  <div v-if="show" class="splash-screen" :class="{ 'fade-out': isFadingOut }">
    <!-- Animated background -->
    <div class="splash-background">
      <div class="particle particle-1"></div>
      <div class="particle particle-2"></div>
      <div class="particle particle-3"></div>
      <div class="particle particle-4"></div>
      <div class="particle particle-5"></div>
    </div>
    
    <!-- Logo container -->
    <div class="logo-container">
      <!-- Animated ServerMint logo -->
      <div class="logo-wrapper" :class="{ 'animate-in': animationStep >= 1 }">
        <img 
          v-if="!imageError"
          src="/servermint.png" 
          alt="ServerMint Logo" 
          class="logo-image"
          :class="{ 'logo-visible': animationStep >= 2 }"
          @load="onImageLoad"
          @error="onImageError"
        />
        <div 
          v-else 
          class="fallback-logo"
          :class="{ 'logo-visible': animationStep >= 2 }"
        >
          <div class="fallback-icon">🌿</div>
        </div>
        <div class="logo-glow" :class="{ 'glow-visible': animationStep >= 2 }"></div>
      </div>
      
      <!-- App name -->
      <div class="app-name" :class="{ 'name-visible': animationStep >= 3 }">
        <span class="name-text">Server</span>
        <span class="name-text accent">Mint</span>
      </div>
      
      <!-- Loading dots -->
      <div class="loading-dots" :class="{ 'dots-visible': animationStep >= 4 }">
        <div class="dot"></div>
        <div class="dot"></div>
        <div class="dot"></div>
      </div>
    </div>
    
    <!-- Audio element for sound effect -->
    <audio ref="splashAudio" preload="auto">
      <source src="/splash-sound.mp3" type="audio/mpeg">
      <source src="/splash-sound.wav" type="audio/wav">
    </audio>
  </div>
</template>

<script>
export default {
  name: 'SplashScreen',
  props: {
    show: {
      type: Boolean,
      default: true
    }
  },
  data() {
    return {
      animationStep: 0,
      audioPlayed: false,
      isFadingOut: false,
      imageLoaded: false,
      imageError: false
    }
  },
  mounted() {
    this.startAnimation();
    
    // Force animation to continue even if image doesn't load
    setTimeout(() => {
      if (this.animationStep < 2) {
        console.log('Forcing animation to continue');
        this.animationStep = 2;
      }
    }, 2000);
  },
  methods: {
    async startAnimation() {
      console.log('Starting splash animation...');
      
      // Play sound effect
      await this.playSound();
      
      // Step 1: Logo wrapper appears
      setTimeout(() => {
        console.log('Animation step 1: Logo wrapper appears');
        this.animationStep = 1;
      }, 200);
      
      // Step 2: Logo image becomes visible
      setTimeout(() => {
        console.log('Animation step 2: Logo image visible');
        this.animationStep = 2;
      }, 800);
      
      // Step 3: App name appears
      setTimeout(() => {
        console.log('Animation step 3: App name appears');
        this.animationStep = 3;
      }, 1200);
      
      // Step 4: Loading dots appear
      setTimeout(() => {
        console.log('Animation step 4: Loading dots appear');
        this.animationStep = 4;
      }, 1600);
      
      // Complete animation and emit event
      setTimeout(() => {
        console.log('Animation complete, starting fade out');
        this.isFadingOut = true;
        setTimeout(() => {
          console.log('Fade out complete, emitting event');
          this.$emit('animation-complete');
        }, 500);
      }, 3000);
    },
    
    async playSound() {
      if (this.audioPlayed) return;
      
      try {
        // Try to play the audio file first
        const audio = this.$refs.splashAudio;
        if (audio) {
          audio.volume = 0.3; // Set volume to 30%
          await audio.play();
          this.audioPlayed = true;
          return;
        }
      } catch (error) {
        console.log('Could not play splash sound file, trying Web Audio API...');
      }
      
      // Fallback: Generate a simple sound using Web Audio API
      try {
        const audioContext = new (window.AudioContext || window.webkitAudioContext)();
        const oscillator = audioContext.createOscillator();
        const gainNode = audioContext.createGain();
        
        oscillator.connect(gainNode);
        gainNode.connect(audioContext.destination);
        
        oscillator.frequency.setValueAtTime(800, audioContext.currentTime);
        oscillator.frequency.exponentialRampToValueAtTime(400, audioContext.currentTime + 0.3);
        
        gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
        gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.3);
        
        oscillator.start(audioContext.currentTime);
        oscillator.stop(audioContext.currentTime + 0.3);
        
        this.audioPlayed = true;
      } catch (error) {
        console.log('Could not play any splash sound:', error);
      }
    },
    
    onImageLoad() {
      console.log('ServerMint logo loaded successfully');
      this.imageLoaded = true;
    },
    
    onImageError() {
      console.error('Failed to load ServerMint logo');
      this.imageError = true;
      // Continue with animation even if image fails
      this.animationStep = 2;
    }
  }
}
</script>

<style scoped>
.splash-screen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 50%, #0a0a0a 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  overflow: hidden;
  opacity: 1;
  transition: opacity 0.5s ease;
}

.splash-screen.fade-out {
  opacity: 0;
}

.splash-background {
  position: absolute;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.particle {
  position: absolute;
  background: radial-gradient(circle, rgba(74, 222, 128, 0.3) 0%, rgba(74, 222, 128, 0) 70%);
  border-radius: 50%;
  animation: float 6s ease-in-out infinite;
}

.particle-1 {
  width: 200px;
  height: 200px;
  top: 10%;
  left: 10%;
  animation-delay: 0s;
}

.particle-2 {
  width: 150px;
  height: 150px;
  top: 70%;
  right: 15%;
  animation-delay: 1s;
}

.particle-3 {
  width: 100px;
  height: 100px;
  top: 30%;
  right: 30%;
  animation-delay: 2s;
}

.particle-4 {
  width: 120px;
  height: 120px;
  bottom: 20%;
  left: 20%;
  animation-delay: 3s;
}

.particle-5 {
  width: 80px;
  height: 80px;
  top: 60%;
  left: 60%;
  animation-delay: 4s;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px) scale(1);
    opacity: 0.3;
  }
  50% {
    transform: translateY(-20px) scale(1.1);
    opacity: 0.6;
  }
}

.logo-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  z-index: 1;
}

.logo-wrapper {
  position: relative;
  width: 120px;
  height: 120px;
  opacity: 0;
  transform: scale(0.5) rotateY(180deg);
  animation: logoAppear 0.8s ease-out forwards;
}

.logo-wrapper.animate-in {
  animation: logoFloat 4s ease-in-out infinite;
}

@keyframes logoAppear {
  0% {
    opacity: 0;
    transform: scale(0.5) rotateY(180deg);
  }
  100% {
    opacity: 1;
    transform: scale(1) rotateY(0deg);
  }
}

@keyframes logoFloat {
  0%, 100% {
    transform: scale(1) translateY(0px) rotateY(0deg);
  }
  25% {
    transform: scale(1.05) translateY(-8px) rotateY(10deg);
  }
  50% {
    transform: scale(1.1) translateY(-15px) rotateY(0deg);
  }
  75% {
    transform: scale(1.05) translateY(-8px) rotateY(-10deg);
  }
}

.logo-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
  opacity: 0;
  transition: opacity 0.5s ease;
  filter: drop-shadow(0 0 20px rgba(74, 222, 128, 0.3));
}

.logo-image.logo-visible {
  opacity: 1;
}

.logo-glow {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 140px;
  height: 140px;
  background: radial-gradient(circle, rgba(74, 222, 128, 0.2) 0%, rgba(74, 222, 128, 0) 70%);
  border-radius: 50%;
  opacity: 0;
  transition: opacity 0.5s ease;
  z-index: -1;
}

.logo-glow.glow-visible {
  opacity: 1;
  animation: glowPulse 2s ease-in-out infinite;
}

@keyframes glowPulse {
  0%, 100% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.2;
  }
  50% {
    transform: translate(-50%, -50%) scale(1.1);
    opacity: 0.4;
  }
}

.fallback-logo {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.5s ease;
}

.fallback-logo.logo-visible {
  opacity: 1;
}

.fallback-icon {
  font-size: 4rem;
  color: #4ade80;
  filter: drop-shadow(0 0 20px rgba(74, 222, 128, 0.5));
}

.app-name {
  display: flex;
  gap: 0.5rem;
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.8s ease;
}

.app-name.name-visible {
  opacity: 1;
  transform: translateY(0);
}

.name-text {
  font-size: 2.5rem;
  font-weight: 700;
  color: #ffffff;
  text-shadow: 0 0 20px rgba(74, 222, 128, 0.5);
}

.name-text.accent {
  color: #4ade80;
  animation: glow 2s ease-in-out infinite alternate;
}

@keyframes glow {
  0% {
    text-shadow: 0 0 20px rgba(74, 222, 128, 0.5);
  }
  100% {
    text-shadow: 0 0 30px rgba(74, 222, 128, 0.8), 0 0 40px rgba(74, 222, 128, 0.3);
  }
}

.loading-dots {
  display: flex;
  gap: 0.5rem;
  opacity: 0;
  transform: translateY(10px);
  transition: all 0.5s ease;
}

.loading-dots.dots-visible {
  opacity: 1;
  transform: translateY(0);
}

.dot {
  width: 8px;
  height: 8px;
  background: #4ade80;
  border-radius: 50%;
  animation: dotPulse 1.5s ease-in-out infinite;
}

.dot:nth-child(2) {
  animation-delay: 0.2s;
}

.dot:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes dotPulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.5;
  }
  50% {
    transform: scale(1.2);
    opacity: 1;
  }
}
</style> 