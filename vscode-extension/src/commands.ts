import * as vscode from 'vscode';

export function registerCommands(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand('ravensone.sayHello', () => {
    vscode.window.showInformationMessage('Hello from RavensOne!');
  });

  context.subscriptions.push(disposable);
}
