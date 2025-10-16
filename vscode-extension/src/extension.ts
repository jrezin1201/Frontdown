import * as vscode from 'vscode';
import { createLanguageClient } from './languageClient';

export async function activate(context: vscode.ExtensionContext) {
  const client = createLanguageClient();
  context.subscriptions.push(client.start());
}

export function deactivate(): Thenable<void> | undefined {
  return undefined;
}
