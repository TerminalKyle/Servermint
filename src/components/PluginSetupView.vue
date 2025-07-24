<template>
  <div class="plugin-setup-view">
    <!-- Header -->
    <div class="setup-header mb-6">
      <div class="d-flex align-center justify-space-between">
        <div>
          <h1 class="text-h4 font-weight-bold mb-2">Plugin & Mod Setup</h1>
          <p class="text-body-1 text-medium-emphasis">Create and configure Minecraft plugins and mods</p>
        </div>
        <v-btn
          variant="outlined"
          color="primary"
          prepend-icon="mdi-arrow-left"
          @click="$router.push('/mods')"
          rounded="lg"
        >
          Back to Mods
        </v-btn>
      </div>
    </div>

    <!-- Setup Wizard -->
    <v-card class="setup-wizard" elevation="4">
      <!-- Progress Bar -->
      <div class="progress-section pa-6 pb-4">
        <div class="d-flex align-center justify-space-between mb-4">
          <h2 class="text-h6 font-weight-medium">Setup Progress</h2>
          <span class="text-caption text-medium-emphasis">{{ currentStep }}/{{ totalSteps }}</span>
        </div>
        <v-progress-linear
          :model-value="(currentStep / totalSteps) * 100"
          color="primary"
          height="8"
          rounded="lg"
        ></v-progress-linear>
      </div>

      <!-- Step Content -->
      <div class="step-content pa-6">
        <!-- Step 1: Project Type Selection -->
        <div v-if="currentStep === 1" class="step-1">
          <h3 class="text-h5 font-weight-medium mb-4">Choose Your Project Type</h3>
          <p class="text-body-1 text-medium-emphasis mb-6">
            Select the type of project you want to create. This will determine the template and configuration options.
          </p>
          
          <div class="project-types">
            <v-row>
              <v-col cols="12" md="6">
                <v-card
                  :class="['project-type-card', { 'selected': selectedType === 'java-plugin' }]"
                  @click="selectedType = 'java-plugin'"
                  elevation="2"
                  rounded="lg"
                >
                  <div class="pa-6">
                    <div class="d-flex align-center mb-4">
                      <v-avatar size="48" color="primary" class="mr-4">
                        <v-icon size="24" color="white">mdi-language-java</v-icon>
                      </v-avatar>
                      <div>
                        <h4 class="text-h6 font-weight-medium">Java Plugin</h4>
                        <p class="text-caption text-medium-emphasis">Bukkit/Spigot/Paper</p>
                      </div>
                    </div>
                    <p class="text-body-2 text-medium-emphasis">
                      Create plugins for Java Edition servers using the Bukkit API. Supports Spigot, Paper, and other Bukkit-based servers.
                    </p>
                    <div class="mt-4">
                      <v-chip size="small" color="primary" variant="outlined" class="mr-2">Java</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined" class="mr-2">Bukkit API</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined">Maven/Gradle</v-chip>
                    </div>
                  </div>
                </v-card>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-card
                  :class="['project-type-card', { 'selected': selectedType === 'bedrock-addon' }]"
                  @click="selectedType = 'bedrock-addon'"
                  elevation="2"
                  rounded="lg"
                >
                  <div class="pa-6">
                    <div class="d-flex align-center mb-4">
                      <v-avatar size="48" color="primary" class="mr-4">
                        <v-icon size="24" color="white">mdi-cube-outline</v-icon>
                      </v-avatar>
                      <div>
                        <h4 class="text-h6 font-weight-medium">Bedrock Addon</h4>
                        <p class="text-caption text-medium-emphasis">Behavior & Resource Packs</p>
                      </div>
                    </div>
                    <p class="text-body-2 text-medium-emphasis">
                      Create addons for Bedrock Edition with behavior packs (scripts) and resource packs (textures/models).
                    </p>
                    <div class="mt-4">
                      <v-chip size="small" color="primary" variant="outlined" class="mr-2">JavaScript</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined" class="mr-2">JSON</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined">Manifest</v-chip>
                    </div>
                  </div>
                </v-card>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-card
                  :class="['project-type-card', { 'selected': selectedType === 'forge-mod' }]"
                  @click="selectedType = 'forge-mod'"
                  elevation="2"
                  rounded="lg"
                >
                  <div class="pa-6">
                    <div class="d-flex align-center mb-4">
                      <v-avatar size="48" color="primary" class="mr-4">
                        <v-icon size="24" color="white">mdi-hammer-wrench</v-icon>
                      </v-avatar>
                      <div>
                        <h4 class="text-h6 font-weight-medium">Forge Mod</h4>
                        <p class="text-caption text-medium-emphasis">Client & Server Mods</p>
                      </div>
                    </div>
                    <p class="text-body-2 text-medium-emphasis">
                      Create mods for Minecraft using the Forge modding framework. Supports both client and server-side modifications.
                    </p>
                    <div class="mt-4">
                      <v-chip size="small" color="primary" variant="outlined" class="mr-2">Java</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined" class="mr-2">Forge API</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined">Gradle</v-chip>
                    </div>
                  </div>
                </v-card>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-card
                  :class="['project-type-card', { 'selected': selectedType === 'fabric-mod' }]"
                  @click="selectedType = 'fabric-mod'"
                  elevation="2"
                  rounded="lg"
                >
                  <div class="pa-6">
                    <div class="d-flex align-center mb-4">
                      <v-avatar size="48" color="primary" class="mr-4">
                        <v-icon size="24" color="white">mdi-puzzle</v-icon>
                      </v-avatar>
                      <div>
                        <h4 class="text-h6 font-weight-medium">Fabric Mod</h4>
                        <p class="text-caption text-medium-emphasis">Lightweight Modding</p>
                      </div>
                    </div>
                    <p class="text-body-2 text-medium-emphasis">
                      Create lightweight mods using the Fabric modding framework. Fast, modern, and community-driven.
                    </p>
                    <div class="mt-4">
                      <v-chip size="small" color="primary" variant="outlined" class="mr-2">Java</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined" class="mr-2">Fabric API</v-chip>
                      <v-chip size="small" color="secondary" variant="outlined">Gradle</v-chip>
                    </div>
                  </div>
                </v-card>
              </v-col>
            </v-row>
          </div>
        </div>

        <!-- Step 2: Project Configuration -->
        <div v-if="currentStep === 2" class="step-2">
          <h3 class="text-h5 font-weight-medium mb-4">Project Configuration</h3>
          <p class="text-body-1 text-medium-emphasis mb-6">
            Configure your project details and settings.
          </p>
          
          <v-form ref="configForm" v-model="configValid">
            <v-row>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="projectConfig.name"
                  label="Project Name"
                  variant="outlined"
                  :rules="[rules.required, rules.projectName]"
                  placeholder="MyAwesomePlugin"
                  hint="Use PascalCase for Java projects, lowercase for Bedrock"
                  persistent-hint
                  rounded="lg"
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="projectConfig.version"
                  label="Version"
                  variant="outlined"
                  :rules="[rules.required, rules.version]"
                  placeholder="1.0.0"
                  hint="Follow semantic versioning (e.g., 1.0.0)"
                  persistent-hint
                  rounded="lg"
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="projectConfig.author"
                  label="Author"
                  variant="outlined"
                  :rules="[rules.required]"
                  placeholder="Your Name"
                  rounded="lg"
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="projectConfig.description"
                  label="Description"
                  variant="outlined"
                  :rules="[rules.required]"
                  placeholder="A brief description of your project"
                  rounded="lg"
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="projectConfig.website"
                  label="Website (Optional)"
                  variant="outlined"
                  placeholder="https://github.com/username/project"
                  rounded="lg"
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="projectConfig.mcVersion"
                  label="Minecraft Version"
                  variant="outlined"
                  :rules="[rules.required]"
                  :placeholder="getMcVersionPlaceholder()"
                  :hint="getMcVersionHint()"
                  persistent-hint
                  rounded="lg"
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" v-if="selectedType === 'java-plugin'">
                <v-select
                  v-model="projectConfig.serverType"
                  label="Server Type"
                  variant="outlined"
                  :items="serverTypes"
                  :rules="[rules.required]"
                  rounded="lg"
                ></v-select>
              </v-col>
              
              <v-col cols="12" v-if="selectedType === 'bedrock-addon'">
                <v-select
                  v-model="projectConfig.addonType"
                  label="Addon Type"
                  variant="outlined"
                  :items="addonTypes"
                  :rules="[rules.required]"
                  multiple
                  rounded="lg"
                ></v-select>
              </v-col>
            </v-row>
          </v-form>
        </div>

        <!-- Step 3: Project Location -->
        <div v-if="currentStep === 3" class="step-3">
          <h3 class="text-h5 font-weight-medium mb-4">Project Location</h3>
          <p class="text-body-1 text-medium-emphasis mb-6">
            Choose where to create your project files.
          </p>
          
          <v-row>
            <v-col cols="12">
              <v-text-field
                v-model="projectConfig.location"
                label="Project Directory"
                variant="outlined"
                :rules="[rules.required]"
                placeholder="C:\Projects\MyPlugin"
                prepend-inner-icon="mdi-folder"
                rounded="lg"
              >
                <template v-slot:append>
                  <v-btn
                    variant="text"
                    color="primary"
                    @click="selectProjectLocation"
                    rounded="lg"
                  >
                    Browse
                  </v-btn>
                </template>
              </v-text-field>
            </v-col>
            
            <v-col cols="12">
              <v-alert
                type="info"
                variant="tonal"
                rounded="lg"
                class="mb-4"
              >
                <template v-slot:prepend>
                  <v-icon>mdi-information</v-icon>
                </template>
                <div>
                  <strong>Recommended structure:</strong><br>
                  <code class="text-caption">{{ getRecommendedStructure() }}</code>
                </div>
              </v-alert>
            </v-col>
          </v-row>
        </div>

        <!-- Step 4: Dependencies & Features -->
        <div v-if="currentStep === 4" class="step-4">
          <h3 class="text-h5 font-weight-medium mb-4">Dependencies & Features</h3>
          <p class="text-body-1 text-medium-emphasis mb-6">
            Select additional dependencies and features for your project.
          </p>
          
          <v-row>
            <v-col cols="12" v-if="selectedType === 'java-plugin'">
              <v-card variant="outlined" rounded="lg" class="mb-4">
                <v-card-title class="text-h6">Common Dependencies</v-card-title>
                <v-card-text>
                  <v-checkbox
                    v-model="projectConfig.dependencies.vault"
                    label="Vault (Economy API)"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.dependencies.worldedit"
                    label="WorldEdit"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.dependencies.worldguard"
                    label="WorldGuard"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.dependencies.placeholderapi"
                    label="PlaceholderAPI"
                    color="primary"
                  ></v-checkbox>
                </v-card-text>
              </v-card>
            </v-col>
            
            <v-col cols="12">
              <v-card variant="outlined" rounded="lg">
                <v-card-title class="text-h6">Project Features</v-card-title>
                <v-card-text>
                  <v-checkbox
                    v-model="projectConfig.features.commands"
                    label="Commands System"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.features.events"
                    label="Event Handling"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.features.config"
                    label="Configuration Files"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.features.database"
                    label="Database Integration"
                    color="primary"
                  ></v-checkbox>
                  <v-checkbox
                    v-model="projectConfig.features.api"
                    label="API for other plugins"
                    color="primary"
                  ></v-checkbox>
                </v-card-text>
              </v-card>
            </v-col>
          </v-row>
        </div>

        <!-- Step 5: Review & Create -->
        <div v-if="currentStep === 5" class="step-5">
          <h3 class="text-h5 font-weight-medium mb-4">Review & Create</h3>
          <p class="text-body-1 text-medium-emphasis mb-6">
            Review your project configuration before creating.
          </p>
          
          <v-card variant="outlined" rounded="lg" class="mb-6">
            <v-card-title class="text-h6">Project Summary</v-card-title>
            <v-card-text>
              <v-row>
                <v-col cols="6">
                  <strong>Project Type:</strong><br>
                  <span class="text-medium-emphasis">{{ getProjectTypeName() }}</span>
                </v-col>
                <v-col cols="6">
                  <strong>Name:</strong><br>
                  <span class="text-medium-emphasis">{{ projectConfig.name }}</span>
                </v-col>
                <v-col cols="6">
                  <strong>Version:</strong><br>
                  <span class="text-medium-emphasis">{{ projectConfig.version }}</span>
                </v-col>
                <v-col cols="6">
                  <strong>Author:</strong><br>
                  <span class="text-medium-emphasis">{{ projectConfig.author }}</span>
                </v-col>
                <v-col cols="12">
                  <strong>Description:</strong><br>
                  <span class="text-medium-emphasis">{{ projectConfig.description }}</span>
                </v-col>
                <v-col cols="12">
                  <strong>Location:</strong><br>
                  <span class="text-medium-emphasis">{{ projectConfig.location }}</span>
                </v-col>
              </v-row>
            </v-card-text>
          </v-card>
          
          <v-alert
            type="success"
            variant="tonal"
            rounded="lg"
            class="mb-4"
          >
            <template v-slot:prepend>
              <v-icon>mdi-check-circle</v-icon>
            </template>
            <div>
              <strong>Ready to create!</strong><br>
              Your project will be generated with all the necessary files and structure.
            </div>
          </v-alert>
        </div>
      </div>

      <!-- Navigation Buttons -->
      <div class="step-navigation pa-6 pt-0">
        <div class="d-flex justify-space-between">
          <v-btn
            v-if="currentStep > 1"
            variant="outlined"
            color="secondary"
            @click="previousStep"
            rounded="lg"
          >
            <v-icon class="mr-2">mdi-arrow-left</v-icon>
            Previous
          </v-btn>
          <div v-else></div>
          
          <v-btn
            v-if="currentStep < totalSteps"
            variant="contained"
            color="primary"
            @click="nextStep"
            :disabled="!canProceed"
            rounded="lg"
          >
            Next
            <v-icon class="ml-2">mdi-arrow-right</v-icon>
          </v-btn>
          
          <v-btn
            v-if="currentStep === totalSteps"
            variant="contained"
            color="success"
            @click="createProject"
            :loading="creating"
            :disabled="!configValid"
            rounded="lg"
          >
            <v-icon class="mr-2">mdi-plus</v-icon>
            Create Project
          </v-btn>
        </div>
      </div>
    </v-card>
  </div>
</template>

<script>
import { store } from '../store.js'

export default {
  name: 'PluginSetupView',
  data() {
    return {
      store: store,
      currentStep: 1,
      totalSteps: 5,
      selectedType: '',
      configValid: false,
      creating: false,
      
      projectConfig: {
        name: '',
        version: '1.0.0',
        author: '',
        description: '',
        website: '',
        mcVersion: '',
        location: '',
        serverType: '',
        addonType: [],
        dependencies: {
          vault: false,
          worldedit: false,
          worldguard: false,
          placeholderapi: false
        },
        features: {
          commands: true,
          events: true,
          config: true,
          database: false,
          api: false
        }
      },
      
      rules: {
        required: v => !!v || 'This field is required',
        projectName: v => {
          if (this.selectedType === 'bedrock-addon') {
            return /^[a-z0-9_-]+$/.test(v) || 'Use only lowercase letters, numbers, hyphens, and underscores';
          }
          return /^[A-Z][a-zA-Z0-9]*$/.test(v) || 'Use PascalCase (e.g., MyPlugin)';
        },
        version: v => /^\d+\.\d+\.\d+$/.test(v) || 'Use semantic versioning (e.g., 1.0.0)'
      },
      
      serverTypes: ['Spigot', 'Paper', 'Bukkit', 'Folia'],
      addonTypes: ['Behavior Pack', 'Resource Pack', 'Both']
    }
  },
  
  computed: {
    canProceed() {
      switch (this.currentStep) {
        case 1:
          return !!this.selectedType;
        case 2:
          return this.configValid;
        case 3:
          return !!this.projectConfig.location;
        case 4:
          return true; // Dependencies are optional
        default:
          return true;
      }
    }
  },
  
  methods: {
    nextStep() {
      if (this.canProceed && this.currentStep < this.totalSteps) {
        this.currentStep++;
      }
    },
    
    previousStep() {
      if (this.currentStep > 1) {
        this.currentStep--;
      }
    },
    
    getProjectTypeName() {
      const names = {
        'java-plugin': 'Java Plugin (Bukkit/Spigot/Paper)',
        'bedrock-addon': 'Bedrock Addon (Behavior/Resource Packs)',
        'forge-mod': 'Forge Mod',
        'fabric-mod': 'Fabric Mod'
      };
      return names[this.selectedType] || '';
    },
    
    getMcVersionPlaceholder() {
      const versions = {
        'java-plugin': '1.20.4',
        'bedrock-addon': '1.20.50',
        'forge-mod': '1.20.4',
        'fabric-mod': '1.20.4'
      };
      return versions[this.selectedType] || '1.20.4';
    },
    
    getMcVersionHint() {
      const hints = {
        'java-plugin': 'Latest stable version recommended',
        'bedrock-addon': 'Use the version you want to target',
        'forge-mod': 'Check Forge compatibility',
        'fabric-mod': 'Check Fabric compatibility'
      };
      return hints[this.selectedType] || '';
    },
    
    getRecommendedStructure() {
      const structures = {
        'java-plugin': 'MyPlugin/\n├── src/main/java/\n├── src/main/resources/\n└── pom.xml',
        'bedrock-addon': 'MyAddon/\n├── behavior_pack/\n├── resource_pack/\n└── manifest.json',
        'forge-mod': 'MyMod/\n├── src/main/java/\n├── src/main/resources/\n└── build.gradle',
        'fabric-mod': 'MyMod/\n├── src/main/java/\n├── src/main/resources/\n└── build.gradle'
      };
      return structures[this.selectedType] || '';
    },
    
    selectProjectLocation() {
      // Set default location to C:\servermint\projects
      this.projectConfig.location = `C:\\servermint\\projects\\${this.projectConfig.name || 'MyProject'}`;
    },
    
    async createProject() {
      this.creating = true;
      
      try {
        // Create the project directory
        await this.store.tauriAPI.createDir(this.projectConfig.location);
        
        // Generate project files based on type
        await this.generateProjectFiles();
        
        // Save project to store
        const projectData = {
          id: Date.now().toString(),
          name: this.projectConfig.name,
          type: this.selectedType,
          version: this.projectConfig.version,
          author: this.projectConfig.author,
          description: this.projectConfig.description,
          location: this.projectConfig.location,
          mcVersion: this.projectConfig.mcVersion,
          serverType: this.projectConfig.serverType,
          addonType: this.projectConfig.addonType,
          dependencies: this.projectConfig.dependencies,
          features: this.projectConfig.features,
          createdAt: new Date().toISOString(),
          lastModified: new Date().toISOString()
        };
        
        this.store.addProject(projectData);
        
        // Show success message
        this.store.showToast('Project created successfully!', 'success');
        
        // Navigate to plugin setup view (which will show the project)
        this.$router.push('/plugin-setup');
      } catch (error) {
        console.error('Error creating project:', error);
        this.store.showToast('Failed to create project: ' + error.message, 'error');
      } finally {
        this.creating = false;
      }
    },
    
    async generateProjectFiles() {
      const projectPath = this.projectConfig.location;
      const projectName = this.projectConfig.name;
      
      switch (this.selectedType) {
        case 'java-plugin':
          await this.generateJavaPluginFiles(projectPath, projectName);
          break;
        case 'bedrock-addon':
          await this.generateBedrockAddonFiles(projectPath, projectName);
          break;
        case 'forge-mod':
          await this.generateForgeModFiles(projectPath, projectName);
          break;
        case 'fabric-mod':
          await this.generateFabricModFiles(projectPath, projectName);
          break;
      }
    },
    
    async generateJavaPluginFiles(projectPath, projectName) {
      // Create Maven structure
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/java`);
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/resources`);
      await this.store.tauriAPI.createDir(`${projectPath}/src/test/java`);
      
      // Generate pom.xml
      const pomXml = this.generatePomXml(projectName);
      await this.store.tauriAPI.writeFile(`${projectPath}/pom.xml`, pomXml);
      
      // Generate plugin.yml
      const pluginYml = this.generatePluginYml(projectName);
      await this.store.tauriAPI.writeFile(`${projectPath}/src/main/resources/plugin.yml`, pluginYml);
      
      // Generate main plugin class
      const mainClass = this.generateMainPluginClass(projectName);
      const packagePath = projectName.toLowerCase();
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/java/${packagePath}`);
      await this.store.tauriAPI.writeFile(`${projectPath}/src/main/java/${packagePath}/${projectName}.java`, mainClass);
      
      // Generate config.yml if enabled
      if (this.projectConfig.features.config) {
        const configYml = this.generateConfigYml();
        await this.store.tauriAPI.writeFile(`${projectPath}/src/main/resources/config.yml`, configYml);
      }
    },
    
    async generateBedrockAddonFiles(projectPath, projectName) {
      const addonName = projectName.toLowerCase();
      
      if (this.projectConfig.addonType.includes('Behavior Pack') || this.projectConfig.addonType.includes('Both')) {
        await this.store.tauriAPI.createDir(`${projectPath}/behavior_pack`);
        await this.store.tauriAPI.createDir(`${projectPath}/behavior_pack/scripts`);
        
        // Generate behavior pack manifest
        const behaviorManifest = this.generateBehaviorManifest(addonName);
        await this.store.tauriAPI.writeFile(`${projectPath}/behavior_pack/manifest.json`, behaviorManifest);
        
        // Generate main script
        const mainScript = this.generateMainScript(addonName);
        await this.store.tauriAPI.writeFile(`${projectPath}/behavior_pack/scripts/main.js`, mainScript);
      }
      
      if (this.projectConfig.addonType.includes('Resource Pack') || this.projectConfig.addonType.includes('Both')) {
        await this.store.tauriAPI.createDir(`${projectPath}/resource_pack`);
        await this.store.tauriAPI.createDir(`${projectPath}/resource_pack/textures`);
        
        // Generate resource pack manifest
        const resourceManifest = this.generateResourceManifest(addonName);
        await this.store.tauriAPI.writeFile(`${projectPath}/resource_pack/manifest.json`, resourceManifest);
      }
    },
    
    async generateForgeModFiles(projectPath, projectName) {
      // Create Gradle structure
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/java`);
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/resources`);
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/resources/META-INF`);
      
      // Generate build.gradle
      const buildGradle = this.generateForgeBuildGradle(projectName);
      await this.store.tauriAPI.writeFile(`${projectPath}/build.gradle`, buildGradle);
      
      // Generate mods.toml
      const modsToml = this.generateModsToml(projectName);
      await this.store.tauriAPI.writeFile(`${projectPath}/src/main/resources/META-INF/mods.toml`, modsToml);
      
      // Generate main mod class
      const mainClass = this.generateForgeMainClass(projectName);
      const packagePath = projectName.toLowerCase();
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/java/${packagePath}`);
      await this.store.tauriAPI.writeFile(`${projectPath}/src/main/java/${packagePath}/${projectName}.java`, mainClass);
    },
    
    async generateFabricModFiles(projectPath, projectName) {
      // Create Gradle structure
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/java`);
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/resources`);
      
      // Generate build.gradle
      const buildGradle = this.generateFabricBuildGradle(projectName);
      await this.store.tauriAPI.writeFile(`${projectPath}/build.gradle`, buildGradle);
      
      // Generate fabric.mod.json
      const fabricModJson = this.generateFabricModJson(projectName);
      await this.store.tauriAPI.writeFile(`${projectPath}/src/main/resources/fabric.mod.json`, fabricModJson);
      
      // Generate main mod class
      const mainClass = this.generateFabricMainClass(projectName);
      const packagePath = projectName.toLowerCase();
      await this.store.tauriAPI.createDir(`${projectPath}/src/main/java/${packagePath}`);
      await this.store.tauriAPI.writeFile(`${projectPath}/src/main/java/${packagePath}/${projectName}.java`, mainClass);
    },
    
    generatePomXml(projectName) {
      return `<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>com.example</groupId>
    <artifactId>${projectName.toLowerCase()}</artifactId>
    <version>${this.projectConfig.version}</version>
    <packaging>jar</packaging>

    <name>${projectName}</name>
    <description>${this.projectConfig.description}</description>

    <properties>
        <maven.compiler.source>17</maven.compiler.source>
        <maven.compiler.target>17</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>

    <repositories>
        <repository>
            <id>spigot-repo</id>
            <url>https://hub.spigotmc.org/nexus/content/repositories/snapshots/</url>
        </repository>
    </repositories>

    <dependencies>
        <dependency>
            <groupId>org.spigotmc</groupId>
            <artifactId>spigot-api</artifactId>
            <version>${this.projectConfig.mcVersion}-R0.1-SNAPSHOT</version>
            <scope>provided</scope>
        </dependency>
    </dependencies>
</project>`;
    },
    
    generatePluginYml(projectName) {
      return `name: ${projectName}
version: ${this.projectConfig.version}
main: ${projectName.toLowerCase()}.${projectName}
author: ${this.projectConfig.author}
description: ${this.projectConfig.description}
api-version: 1.20
website: ${this.projectConfig.website || ''}`;
    },
    
    generateMainPluginClass(projectName) {
      const packageName = projectName.toLowerCase();
      return `package ${packageName};

import org.bukkit.plugin.java.JavaPlugin;

public class ${projectName} extends JavaPlugin {
    
    @Override
    public void onEnable() {
        getLogger().info("${projectName} has been enabled!");
    }
    
    @Override
    public void onDisable() {
        getLogger().info("${projectName} has been disabled!");
    }
}`;
    },
    
    generateConfigYml() {
      return `# ${this.projectConfig.name} Configuration

# General settings
general:
  debug: false
  prefix: "&8[&a${this.projectConfig.name}&8] &r"`;
    },
    
    generateBehaviorManifest(addonName) {
      return `{
  "format_version": 2,
  "header": {
    "name": "${addonName}",
    "description": "${this.projectConfig.description}",
    "uuid": "${this.generateUUID()}",
    "version": [1, 0, 0],
    "min_engine_version": [1, 20, 50]
  },
  "modules": [
    {
      "type": "script",
      "language": "javascript",
      "uuid": "${this.generateUUID()}",
      "version": [1, 0, 0],
      "entry": "scripts/main.js"
    }
  ]
}`;
    },
    
    generateResourceManifest(addonName) {
      return `{
  "format_version": 2,
  "header": {
    "name": "${addonName}",
    "description": "${this.projectConfig.description}",
    "uuid": "${this.generateUUID()}",
    "version": [1, 0, 0],
    "min_engine_version": [1, 20, 50]
  },
  "modules": [
    {
      "type": "resources",
      "uuid": "${this.generateUUID()}",
      "version": [1, 0, 0]
    }
  ]
}`;
    },
    
    generateMainScript(addonName) {
      return `import { world, system } from "@minecraft/server";

// ${addonName} - ${this.projectConfig.description}
// Created by ${this.projectConfig.author}

world.beforeEvents.chatSend.subscribe((eventData) => {
  const player = eventData.sender;
  const message = eventData.message;
  
  // Your addon logic here
  console.warn(\`[\${addonName}] \${player.name} said: \${message}\`);
});`;
    },
    
    generateForgeBuildGradle(projectName) {
      return `plugins {
    id 'net.minecraftforge.gradle' version '5.1.+'
}

apply plugin: 'eclipse'
apply plugin: 'maven-publish'

version = '${this.projectConfig.version}'
group = 'com.example.${projectName.toLowerCase()}'
archivesBaseName = '${projectName.toLowerCase()}'

java.toolchain.languageVersion = JavaLanguageVersion.of(17)

minecraft {
    mappings channel: 'official', version: '${this.projectConfig.mcVersion}'
    
    runs {
        client {
            workingDirectory project.file('run')
            property 'forge.logging.markers', 'REGISTRIES'
            property 'forge.logging.console.level', 'debug'
            mods {
                ${projectName.toLowerCase()} {
                    source sourceSets.main
                }
            }
        }
        
        server {
            workingDirectory project.file('run')
            property 'forge.logging.markers', 'REGISTRIES'
            property 'forge.logging.console.level', 'debug'
            mods {
                ${projectName.toLowerCase()} {
                    source sourceSets.main
                }
            }
        }
    }
}

dependencies {
    minecraft 'net.minecraftforge:forge:${this.projectConfig.mcVersion}-47.1.0'
}

jar {
    manifest {
        attributes([
            "Specification-Title"     : "${projectName.toLowerCase()}",
            "Specification-Vendor"    : "${projectName.toLowerCase()}sareus",
            "Specification-Version"   : "1",
            "Implementation-Title"    : project.name,
            "Implementation-Version"  : project.jar.archiveVersion,
            "Implementation-Vendor"   : "${projectName.toLowerCase()}sareus",
            "Implementation-Timestamp": new Date().format("yyyy-MM-dd'T'HH:mm:ssZ")
        ])
    }
}`;
    },
    
    generateModsToml(projectName) {
      return `modLoader="javafml"
loaderVersion="[47,)"
license="All rights reserved"

[[mods]]
modId="${projectName.toLowerCase()}"
version="${this.projectConfig.version}"
displayName="${projectName}"
description='''${this.projectConfig.description}'''
authors="${this.projectConfig.author}"`;
    },
    
    generateForgeMainClass(projectName) {
      const packageName = projectName.toLowerCase();
      return `package ${packageName};

import net.minecraftforge.common.MinecraftForge;
import net.minecraftforge.fml.common.Mod;

@Mod(${packageName}.MOD_ID)
public class ${projectName} {
    public static final String MOD_ID = "${projectName.toLowerCase()}";

    public ${projectName}() {
        MinecraftForge.EVENT_BUS.register(this);
    }
}`;
    },
    
    generateFabricBuildGradle(projectName) {
      return `plugins {
    id 'fabric-loom' version '1.2-SNAPSHOT'
    id 'maven-publish'
}

version = '${this.projectConfig.version}'
group = 'com.example.${projectName.toLowerCase()}'
archivesBaseName = '${projectName.toLowerCase()}'

repositories {
}

dependencies {
    minecraft "com.mojang:minecraft:${this.projectConfig.mcVersion}"
    mappings "net.fabricmc:yarn:${this.projectConfig.mcVersion}+build.1:v2"
    modImplementation "net.fabricmc:fabric-loader:0.14.21"
    modImplementation "net.fabricmc.fabric-api:fabric-api:0.83.0+1.20.4"
}

processResources {
    inputs.property "version", project.version
    filteringCharset "UTF-8"

    filesMatching("fabric.mod.json") {
        expand "version": project.version
    }
}

tasks.withType(JavaCompile).configureEach {
    it.options.release = 17
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
    withSourcesJar()
}

jar {
    from("LICENSE") {
        rename { "\${it}_\${project.archivesBaseName}"}
    }
}`;
    },
    
    generateFabricModJson(projectName) {
      return `{
  "schemaVersion": 1,
  "id": "${projectName.toLowerCase()}",
  "version": "${this.projectConfig.version}",
  "name": "${projectName}",
  "description": "${this.projectConfig.description}",
  "authors": [
    "${this.projectConfig.author}"
  ],
  "contact": {
    "homepage": "${this.projectConfig.website || ''}",
    "sources": "${this.projectConfig.website || ''}"
  },
  "license": "MIT",
  "icon": "assets/${projectName.toLowerCase()}/icon.png",
  "environment": "*",
  "entrypoints": {
    "main": [
      "${projectName.toLowerCase()}.${projectName}"
    ]
  },
  "depends": {
    "fabricloader": ">=0.14.21",
    "minecraft": "~${this.projectConfig.mcVersion}",
    "java": ">=17",
    "fabric-api": "*"
  }
}`;
    },
    
    generateFabricMainClass(projectName) {
      const packageName = projectName.toLowerCase();
      return `package ${packageName};

import net.fabricmc.api.ModInitializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ${projectName} implements ModInitializer {
    public static final String MOD_ID = "${projectName.toLowerCase()}";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

    @Override
    public void onInitialize() {
        LOGGER.info("Initializing ${projectName}!");
    }
}`;
    },
    
    generateUUID() {
      return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        const r = Math.random() * 16 | 0;
        const v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
      });
    }
  }
}
</script>

<style scoped>
.plugin-setup-view {
  max-width: 1200px;
  margin: 0 auto;
}

.setup-wizard {
  background-color: rgba(30, 30, 30, 0.95) !important;
  border: 1px solid rgba(74, 222, 128, 0.05);
}

.project-type-card {
  cursor: pointer;
  transition: all 0.2s ease;
  border: 2px solid transparent;
}

.project-type-card:hover {
  transform: translateY(-2px);
  border-color: rgba(74, 222, 128, 0.2);
}

.project-type-card.selected {
  border-color: var(--v-theme-primary);
  background-color: rgba(74, 222, 128, 0.05);
}

.step-content {
  min-height: 400px;
}

.step-navigation {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

/* Custom scrollbar for the view */
.plugin-setup-view ::-webkit-scrollbar {
  width: 8px;
}

.plugin-setup-view ::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}

.plugin-setup-view ::-webkit-scrollbar-thumb {
  background: rgba(74, 222, 128, 0.3);
  border-radius: 4px;
}

.plugin-setup-view ::-webkit-scrollbar-thumb:hover {
  background: rgba(74, 222, 128, 0.5);
}
</style> 