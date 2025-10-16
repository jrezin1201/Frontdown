import * as vscode from 'vscode-languageclient/node';

export function createLanguageClient() {
  const serverOptions: vscode.ServerOptions = {
    run: { module: 'server.js', transport: vscode.TransportKind.ipc },
    debug: { module: 'server.js', transport: vscode.TransportKind.ipc }
  };

  const clientOptions: vscode.LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'raven' }]
  };

  return new vscode.LanguageClient('ravenLanguageServer', 'RavensOne Language Server', serverOptions, clientOptions);
}
