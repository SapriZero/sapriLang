// Editor CodeMirror e funzioni principali

let editor;
let currentFile = null;
let ws = null;

document.addEventListener('DOMContentLoaded', async function() {
    // Render toolbar
    document.getElementById('toolbar-container').innerHTML = Components.renderToolbar('editor');

    // Inizializza CodeMirror
    editor = CodeMirror.fromTextArea(document.getElementById('editor'), {
        lineNumbers: true,
        mode: 'javascript',
        theme: 'dracula',
        autofocus: true,
        lineWrapping: true,
        indentUnit: 4,
        tabSize: 4
    });

    // Carica lista file e render sidebar
    await loadFiles();

    // WebSocket per console
    connectWebSocket();

    // Verifica server
    checkServerStatus();
    setInterval(checkServerStatus, 30000);
});

async function loadFiles() {
    try {
        const response = await fetch('/api/files');
        const files = await response.json();

        document.getElementById('sidebar-container').innerHTML = Components.renderSidebar(files, currentFile);
    } catch (error) {
        Components.showNotification('Errore caricamento file', 'error');
    }
}

window.loadFile = async function(name) {
    try {
        const response = await fetch(`/api/files/${encodeURIComponent(name)}`);
        if (response.ok) {
            const content = await response.text();
            editor.setValue(content);
            currentFile = name;
            document.getElementById('current-file').textContent = name;

            // Ricarica sidebar per aggiornare active
            await loadFiles();
        }
    } catch (error) {
        Components.showNotification('Errore caricamento file', 'error');
    }
};

window.saveFile = async function() {
    if (!currentFile) {
        Components.showNotification('Nessun file selezionato', 'error');
        return;
    }

    try {
        const content = editor.getValue();
        const response = await fetch(`/api/files/${encodeURIComponent(currentFile)}`, {
            method: 'POST',
            headers: { 'Content-Type': 'text/plain' },
            body: content
        });

        if (response.ok) {
            Components.showNotification('✅ File salvato', 'success');
        }
    } catch (error) {
        Components.showNotification('Errore salvataggio', 'error');
    }
};

window.createFile = async function() {
    const input = document.getElementById('new-filename');
    let name = input.value.trim();

    if (!name) return;
    if (!name.endsWith('.sapri')) name += '.sapri';

    try {
        const response = await fetch(`/api/files/${encodeURIComponent(name)}`, {
            method: 'POST',
            headers: { 'Content-Type': 'text/plain' },
            body: '# Nuovo script URCM\n'
        });

        if (response.ok) {
            input.value = '';
            await loadFiles();
            await window.loadFile(name);
            Components.showNotification('✅ File creato', 'success');
        }
    } catch (error) {
        Components.showNotification('Errore creazione file', 'error');
    }
};

window.executeFile = async function() {
    const code = editor.getValue();
    addToConsole('🚀 Esecuzione...');

    try {
        const response = await fetch('/api/execute', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ code })
        });

        const result = await response.json();
        if (result.success) {
            result.output.forEach(line => addToConsole(line));
        } else {
            addToConsole('❌ ' + result.error, true);
        }
    } catch (error) {
        addToConsole('❌ Errore esecuzione', true);
    }
};

function addToConsole(text, isError = false) {
    const consoleDiv = document.getElementById('console');
    const line = document.createElement('div');
    line.className = isError ? 'console-error' : 'console-line';
    line.textContent = '> ' + text;
    consoleDiv.appendChild(line);
    consoleDiv.scrollTop = consoleDiv.scrollHeight;
}

function connectWebSocket() {
    ws = new WebSocket('ws://' + window.location.host + '/ws');

    ws.onmessage = function(event) {
        addToConsole(event.data);
    };

    ws.onclose = function() {
        setTimeout(connectWebSocket, 1000);
    };
}

async function checkServerStatus() {
    try {
        const response = await fetch('/api/files');
        Components.updateServerStatus(response.ok);
    } catch {
        Components.updateServerStatus(false);
    }
}
