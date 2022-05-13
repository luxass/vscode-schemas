import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {

	console.log('Congratulations, your extension "schema-extractor" is now active!');

	let disposable = vscode.commands.registerCommand('schema-extractor.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World from Schema Extractor!');
	});

	context.subscriptions.push(disposable);
}
