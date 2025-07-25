import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    let disposable = vscode.commands.registerCommand('laxlang.helloWorld', () => {
        vscode.window.showInformationMessage('Hello from LaxLang!');
    });
    context.subscriptions.push(disposable);
}

export function deactivate() {}
