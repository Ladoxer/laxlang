import * as vscode from 'vscode';
import { spawn } from 'child_process';
export function activate(ctx: vscode.ExtensionContext) {
    ctx.subscriptions.push(
        vscode.commands.registerCommand('laxlang.compile', () => {
            const f = vscode.window.activeTextEditor?.document.uri.fsPath;
            if (f) spawn('laxlang', [f], { stdio: 'inherit' });
        })
    );
}
export function deactivate() {}