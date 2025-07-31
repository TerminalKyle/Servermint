const { defineConfig } = require('@vue/cli-service')

// Get the host from environment variable for mobile development
const host = process.env.TAURI_DEV_HOST

module.exports = defineConfig({
  transpileDependencies: true,
  configureWebpack: {
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          loader: 'ts-loader',
          options: {
            transpileOnly: true
          }
        }
      ]
    }
  },
  devServer: {
    host: host || 'localhost',
    port: 8080,
    allowedHosts: 'all',
    client: {
      webSocketURL: host ? `ws://${host}:8081/ws` : 'auto://0.0.0.0:0/ws'
    }
  }
})
