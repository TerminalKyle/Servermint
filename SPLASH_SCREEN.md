# Splash Screen Feature

## Overview
The ServerMint app now includes a beautiful animated splash screen that displays when the app starts. It features:

- **3D Animated Cube Logo**: A rotating cube with your logo inside
- **Particle Effects**: Floating green particles in the background
- **Sound Effects**: Audio that plays during the animation
- **Smooth Transitions**: Elegant fade-in effects for each element

## Features

### Animation Sequence
1. **Cube Appears** (0.2s): The 3D cube logo fades in and starts rotating
2. **Letter Reveals** (0.8s): The letter inside the cube becomes visible
3. **App Name** (1.2s): "ServerMint" text appears with glowing effect
4. **Loading Dots** (1.6s): Animated dots appear at the bottom
5. **Complete** (3.0s): Splash screen fades out and main app appears

### Sound Effects
The splash screen includes audio that plays during the animation:

- **Primary**: Tries to play `/public/splash-sound.mp3` or `/public/splash-sound.wav`
- **Fallback**: Generates a simple tone using Web Audio API if no audio file is found
- **Volume**: Set to 30% to avoid being too loud

## Customization

### Adding Custom Sound
1. Replace `/public/splash-sound.mp3` with your own audio file
2. Recommended specifications:
   - **Duration**: 2-3 seconds
   - **Format**: MP3 or WAV
   - **File Size**: Under 500KB
   - **Quality**: 44.1kHz, 128kbps or higher

### Disabling Splash Screen
Users can disable the splash screen in the app settings:
- Go to Settings → General
- Toggle "Show Splash Screen" off

### Modifying Animation
The animation timing and effects can be customized in `src/components/SplashScreen.vue`:

- **Timing**: Adjust the `setTimeout` delays in `startAnimation()`
- **Colors**: Modify the CSS variables for green accent colors
- **Effects**: Add or remove particle effects and animations

## Technical Details

### Components
- **SplashScreen.vue**: Main splash screen component
- **App.vue**: Integration and state management
- **Audio Files**: Located in `/public/` directory

### Dependencies
- **Vue.js**: Component framework
- **Web Audio API**: Fallback sound generation
- **CSS3**: Animations and 3D transforms

### Browser Support
- **Modern Browsers**: Full support with 3D transforms and Web Audio API
- **Fallbacks**: Graceful degradation for older browsers
- **Mobile**: Responsive design for mobile devices

## Troubleshooting

### Sound Not Playing
1. Check browser autoplay policies
2. Ensure audio file exists in `/public/` directory
3. Check browser console for errors
4. Verify file format is supported (MP3/WAV)

### Animation Issues
1. Check browser support for CSS3 transforms
2. Verify JavaScript is enabled
3. Check for console errors

### Performance
1. Keep audio files small (<500KB)
2. Optimize particle count for slower devices
3. Consider reducing animation complexity on mobile

## Future Enhancements
- Custom logo upload functionality
- Multiple animation themes
- User-customizable colors
- Integration with system theme
- Accessibility improvements 