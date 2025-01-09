import { ExecutorContext } from '@nx/devkit';
import { execSync } from 'child_process';

export interface RustExecutorOptions {
  command: 'test' | 'build' | 'run';
}

export default async function rustExecutor(
  options: RustExecutorOptions,
  context: ExecutorContext
) {
  if (!context.projectName) {
    throw new Error('Project name is required');
  }
  const projectRoot = context.projectsConfigurations.projects[context.projectName].root;
  const rustPath = `${projectRoot}/rust`;
  
  try {
    execSync(`cd ${rustPath} && cargo ${options.command}`, { stdio: 'inherit' });
    return { success: true };
  } catch (e) {
    return { success: false };
  }
} 