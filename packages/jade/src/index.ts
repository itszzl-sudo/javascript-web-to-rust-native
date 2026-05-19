import { spawn } from 'child_process';
import path from 'path';
import fs from 'fs';

export interface CompileOptions {
  input: string;
  output?: string;
  name?: string;
  embed?: boolean;
}

export interface CompileResult {
  success: boolean;
  output?: string;
  error?: string;
}

/**
 * Compile JavaScript to native executable
 */
export async function compile(options: CompileOptions): Promise<CompileResult> {
  const binary = getBinary();
  
  if (!fs.existsSync(binary)) {
    return {
      success: false,
      error: `Binary not found: ${binary}. Please run "npm install" to download the binary.`
    };
  }
  
  const args = [
    '--input', options.input,
    '--output', options.output || 'dist',
    '--name', options.name || 'app',
  ];
  
  if (options.embed) {
    args.push('--embed');
  }
  
  return new Promise((resolve) => {
    const proc = spawn(binary, args, {
      stdio: ['ignore', 'pipe', 'pipe']
    });
    
    let stdout = '';
    let stderr = '';
    
    proc.stdout?.on('data', (data) => {
      stdout += data.toString();
    });
    
    proc.stderr?.on('data', (data) => {
      stderr += data.toString();
    });
    
    proc.on('close', (code) => {
      if (code === 0) {
        resolve({
          success: true,
          output: stdout
        });
      } else {
        resolve({
          success: false,
          error: stderr || stdout
        });
      }
    });
    
    proc.on('error', (error) => {
      resolve({
        success: false,
        error: error.message
      });
    });
  });
}

/**
 * Get platform binary path
 */
function getBinary(): string {
  const platform = process.platform;
  const arch = process.arch;
  
  const binaryName: Record<string, Record<string, string>> = {
    win32: { x64: 'director.exe', ia32: 'director.exe' },
    darwin: { x64: 'director', arm64: 'director' },
    linux: { x64: 'director', arm64: 'director' }
  };
  
  const name = binaryName[platform]?.[arch];
  
  if (!name) {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
  
  return path.join(__dirname, '..', 'binaries', platform, arch, name);
}

// Export version
export const VERSION: string = '0.1.0';
