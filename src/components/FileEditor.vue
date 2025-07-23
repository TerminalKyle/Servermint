<template>
  <v-dialog v-model="show" max-width="1200" max-height="800" persistent>
    <v-card class="file-editor">
      <!-- Header -->
      <v-card-title class="d-flex align-center pa-4">
        <v-icon class="mr-3" color="primary">mdi-file-edit</v-icon>
        <div class="flex-grow-1">
          <div class="text-h6 font-weight-bold">{{ fileName }}</div>
          <div class="text-caption text-medium-emphasis">{{ filePath }}</div>
        </div>
        
        <v-chip
          v-if="hasUnsavedChanges"
          color="warning"
          size="small"
          class="mr-3"
        >
          <v-icon size="small" class="mr-1">mdi-pencil</v-icon>
          Unsaved Changes
        </v-chip>
        
        <v-btn
          icon
          variant="text"
          @click="closeEditor"
          :disabled="saving"
        >
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-card-title>

      <!-- Toolbar -->
      <v-card-text class="pa-0">
        <div class="editor-toolbar d-flex align-center pa-3">
          <v-btn
            size="small"
            variant="outlined"
            prepend-icon="mdi-content-save"
            @click="saveFile"
            :loading="saving"
            :disabled="!hasUnsavedChanges"
            class="mr-2"
          >
            Save
          </v-btn>
          
          <v-btn
            size="small"
            variant="outlined"
            prepend-icon="mdi-refresh"
            @click="reloadFile"
            :loading="loading"
            class="mr-2"
          >
            Reload
          </v-btn>
          
          <v-btn
            size="small"
            variant="outlined"
            prepend-icon="mdi-format-indent-increase"
            @click="formatCode"
            :disabled="!canFormat"
            class="mr-2"
          >
            Format
          </v-btn>
          
          <v-spacer></v-spacer>
          
          <v-chip
            size="small"
            variant="outlined"
            class="mr-2"
          >
            <v-icon size="small" class="mr-1">mdi-text</v-icon>
            {{ lineCount }} lines
          </v-chip>
          
          <v-chip
            size="small"
            variant="outlined"
          >
            <v-icon size="small" class="mr-1">mdi-character</v-icon>
            {{ characterCount }} chars
          </v-chip>
        </div>
      </v-card-text>

      <!-- Editor -->
      <div class="editor-container">
        <div class="editor-wrapper">
          <!-- Line numbers -->
          <div class="line-numbers" ref="lineNumbers">
            <div
              v-for="lineNum in lineCount"
              :key="lineNum"
              class="line-number"
              :class="{ 'current-line': currentLine === lineNum }"
            >
              {{ lineNum }}
            </div>
          </div>
          
          <!-- Text area -->
          <v-textarea
            ref="editor"
            v-model="content"
            variant="outlined"
            hide-details
            auto-grow
            no-resize
            class="code-editor"
            @input="onContentChange"
            @scroll="onScroll"
            @keydown="onKeyDown"
            placeholder="File content will appear here..."
            :disabled="loading"
            rows="20"
          ></v-textarea>
        </div>
      </div>

      <!-- Footer -->
      <v-card-actions class="pa-4">
        <v-spacer></v-spacer>
        
        <v-btn
          variant="text"
          @click="closeEditor"
          :disabled="saving"
        >
          Cancel
        </v-btn>
        
        <v-btn
          color="primary"
          @click="saveFile"
          :loading="saving"
          :disabled="!hasUnsavedChanges"
        >
          <v-icon class="mr-2">mdi-content-save</v-icon>
          Save Changes
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import { store } from '../store.js'

export default {
  name: 'FileEditor',
  props: {
    modelValue: {
      type: Boolean,
      default: false
    },
    serverId: {
      type: String,
      required: true
    },
    filePath: {
      type: String,
      required: true
    }
  },
  emits: ['update:modelValue', 'saved', 'error'],
  data() {
    return {
      content: '',
      originalContent: '',
      loading: false,
      saving: false,
      currentLine: 1,
      store: store
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
    },
    fileName() {
      return this.filePath.split('/').pop() || this.filePath
    },
    hasUnsavedChanges() {
      return this.content !== this.originalContent
    },
    lineCount() {
      const content = String(this.content || '')
      return content.split('\n').length
    },
    characterCount() {
      const content = String(this.content || '')
      return content.length
    },
    canFormat() {
      const ext = this.fileName.split('.').pop()?.toLowerCase()
      return ['json', 'xml', 'yml', 'yaml', 'properties'].includes(ext)
    }
  },
  watch: {
    show(newValue) {
      if (newValue) {
        this.loadFile()
      } else {
        this.resetEditor()
      }
    }
  },
  methods: {
    async loadFile() {
      this.loading = true
      try {
        console.log(`Loading file: ${this.filePath} for server: ${this.serverId}`)
        const result = await this.store.readServerFile(this.serverId, this.filePath)
        console.log('File read result:', result)
        
        if (result.success) {
          console.log('File content length:', result.content?.length || 0)
          console.log('File content preview:', result.content?.substring(0, 100))
          this.content = result.content || ''
          this.originalContent = this.content
          this.$nextTick(() => {
            this.updateLineNumbers()
          })
        } else {
          throw new Error(result.error || 'Failed to load file')
        }
      } catch (error) {
        console.error('Error loading file:', error)
        this.$emit('error', `Failed to load file: ${error.message}`)
        this.show = false
      } finally {
        this.loading = false
      }
    },
    
    async saveFile() {
      if (!this.hasUnsavedChanges) return
      
      this.saving = true
      try {
        const result = await this.store.saveServerFile(this.serverId, this.filePath, this.content)
        if (result.success) {
          this.originalContent = this.content
          this.$emit('saved', this.filePath)
          if (window.showSuccess) {
            window.showSuccess('File Saved', `${this.fileName} has been saved successfully.`)
          }
        } else {
          throw new Error(result.error || 'Failed to save file')
        }
      } catch (error) {
        console.error('Error saving file:', error)
        this.$emit('error', `Failed to save file: ${error.message}`)
        if (window.showError) {
          window.showError('Save Failed', `Failed to save ${this.fileName}: ${error.message}`)
        }
      } finally {
        this.saving = false
      }
    },
    
    async reloadFile() {
      await this.loadFile()
    },
    
    formatCode() {
      try {
        const ext = this.fileName.split('.').pop()?.toLowerCase()
        
        if (ext === 'json') {
          const parsed = JSON.parse(this.content)
          this.content = JSON.stringify(parsed, null, 2)
        } else if (ext === 'xml') {
          // Simple XML formatting (basic indentation)
          this.content = this.formatXML(this.content)
        } else if (ext === 'yml' || ext === 'yaml') {
          // YAML formatting (basic)
          this.content = this.formatYAML(this.content)
        } else if (ext === 'properties') {
          // Properties file formatting (sort keys)
          this.content = this.formatProperties(this.content)
        }
        
        this.updateLineNumbers()
      } catch (error) {
        console.error('Error formatting code:', error)
        if (window.showError) {
          window.showError('Format Failed', `Failed to format ${this.fileName}: ${error.message}`)
        }
      }
    },
    
    formatXML(xml) {
      // Basic XML formatting with indentation
      let formatted = ''
      let indent = ''
      const tab = '  '
      
      xml.split(/>\s*</).forEach(node => {
        if (node.match(/^\/\w/)) {
          indent = indent.substring(tab.length)
        }
        formatted += indent + '<' + node + '>\r\n'
        if (node.match(/^<?\w[^>]*[^/]$/)) {
          indent += tab
        }
      })
      
      return formatted.substring(1, formatted.length - 3)
    },
    
    formatYAML(yaml) {
      // Basic YAML formatting (just ensure consistent indentation)
      return yaml.split('\n').map(line => {
        const trimmed = line.trim()
        if (trimmed.startsWith('-') || trimmed.includes(':')) {
          return '  ' + trimmed
        }
        return trimmed
      }).join('\n')
    },
    
    formatProperties(properties) {
      // Sort properties by key
      const lines = properties.split('\n')
      const sortedLines = lines
        .filter(line => line.trim() && !line.trim().startsWith('#'))
        .sort()
      
      const comments = lines.filter(line => line.trim().startsWith('#'))
      
      return [...comments, ...sortedLines].join('\n')
    },
    
    onContentChange() {
      this.updateLineNumbers()
    },
    
    onScroll(event) {
      console.log('Scroll event triggered:', event.target.scrollTop);
      // Sync line numbers scroll with editor scroll
      if (this.$refs.lineNumbers) {
        this.$refs.lineNumbers.scrollTop = event.target.scrollTop
      }
    },
    
    onKeyDown(event) {
      // Handle Ctrl+S for save
      if ((event.ctrlKey || event.metaKey) && event.key === 's') {
        event.preventDefault()
        this.saveFile()
      }
      
      // Handle Ctrl+Z for undo (let browser handle it)
      // Handle Ctrl+Y for redo (let browser handle it)
    },
    
    updateLineNumbers() {
      this.$nextTick(() => {
        if (this.$refs.lineNumbers) {
          const content = String(this.content || '')
          const lines = content.split('\n')
          const lineNumbers = this.$refs.lineNumbers.children
          
          // Update line numbers
          for (let i = 0; i < lineNumbers.length; i++) {
            if (i < lines.length) {
              lineNumbers[i].style.display = 'block'
            } else {
              lineNumbers[i].style.display = 'none'
            }
          }
        }
      })
    },
    
    resetEditor() {
      this.content = ''
      this.originalContent = ''
      this.currentLine = 1
      this.loading = false
      this.saving = false
    },
    
    closeEditor() {
      if (this.hasUnsavedChanges) {
        if (confirm('You have unsaved changes. Are you sure you want to close without saving?')) {
          this.show = false
        }
      } else {
        this.show = false
      }
    }
  }
}
</script>

<style scoped>
.file-editor {
  height: 80vh;
  display: flex;
  flex-direction: column;
}

.editor-toolbar {
  background-color: rgba(30, 30, 30, 0.95);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.editor-container {
  flex: 1;
  overflow: hidden;
  position: relative;
  min-height: 0;
}

.editor-wrapper {
  display: flex;
  height: 100%;
  overflow: hidden;
  min-height: 0;
}

.line-numbers {
  width: 60px;
  background-color: rgba(20, 20, 20, 0.95);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  overflow-y: auto;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 0;
  user-select: none;
}

.line-number {
  padding: 8px 8px 8px 0;
  color: rgba(255, 255, 255, 0.5);
  text-align: right;
  min-height: 1.5em;
}

.line-number.current-line {
  color: #4ade80;
  background-color: rgba(74, 222, 128, 0.1);
}

.code-editor {
  flex: 1;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  background-color: rgba(18, 18, 18, 0.95);
  border: none;
  outline: none;
  resize: none;
  padding: 8px 12px;
  min-height: 0;
  overflow: auto;
}

.code-editor :deep(.v-field__field) {
  background-color: transparent;
  border: none;
  box-shadow: none;
}

.code-editor :deep(.v-field__input) {
  background-color: transparent;
  color: #ffffff;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 8px 12px;
  min-height: 100%;
  overflow: auto;
}

.code-editor :deep(.v-field__input:focus) {
  box-shadow: none;
}

/* Remove shadows from top and bottom of editor */
.code-editor :deep(.v-field) {
  box-shadow: none !important;
}

.code-editor :deep(.v-field__field) {
  box-shadow: none !important;
}

.code-editor :deep(.v-field__outline) {
  box-shadow: none !important;
}

/* Remove any inner shadows or gradients */
.code-editor :deep(.v-field__input) {
  box-shadow: none !important;
  background: rgba(18, 18, 18, 0.95) !important;
}

/* Ensure no shadows on the textarea itself */
.code-editor :deep(textarea) {
  box-shadow: none !important;
  background: rgba(18, 18, 18, 0.95) !important;
  overflow: auto !important;
  resize: none !important;
}

/* Show scrollbars and ensure scrolling works */
.line-numbers::-webkit-scrollbar,
.code-editor :deep(.v-field__input)::-webkit-scrollbar,
.code-editor :deep(textarea)::-webkit-scrollbar {
  width: 12px;
  height: 12px;
  display: block !important;
}

.line-numbers::-webkit-scrollbar-track,
.code-editor :deep(.v-field__input)::-webkit-scrollbar-track,
.code-editor :deep(textarea)::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
}

.line-numbers::-webkit-scrollbar-thumb,
.code-editor :deep(.v-field__input)::-webkit-scrollbar-thumb,
.code-editor :deep(textarea)::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.3);
  border-radius: 6px;
}

.line-numbers::-webkit-scrollbar-thumb:hover,
.code-editor :deep(.v-field__input)::-webkit-scrollbar-thumb:hover,
.code-editor :deep(textarea)::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.5);
}

/* Ensure scrollbars are visible in Firefox */
.line-numbers,
.code-editor :deep(.v-field__input),
.code-editor :deep(textarea) {
  scrollbar-width: auto;
  scrollbar-color: rgba(255, 255, 255, 0.3) rgba(255, 255, 255, 0.05);
}
</style> 