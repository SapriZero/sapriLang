// ==UserScript==
// @name         URCM Chat Saver
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  Salva le risposte della chat nel server URCM
// @author       You
// @match        https://chat.openai.com/*
// @match        https://claude.ai/*
// @match        https://gemini.google.com/*
// @match        https://chat.deepseek.com/*
// @grant        GM_xmlhttpRequest
// @grant        GM_addStyle
// ==/UserScript==

(function() {
    'use strict';

    // Configurazione
    const SERVER_URL = 'http://localhost:3000';
    const SAVE_ENDPOINT = `${SERVER_URL}/api/ai/save`;

    // Aggiungi stili
    GM_addStyle(`
        .urcm-toolbar {
            position: fixed;
            top: 10px;
            right: 20px;
            z-index: 9999;
            background: #2d2d30;
            border: 1px solid #3e3e42;
            border-radius: 6px;
            padding: 6px 12px;
            display: flex;
            gap: 8px;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            box-shadow: 0 2px 8px rgba(0,0,0,0.3);
            opacity: 0.3;
            transition: opacity 0.2s;
        }

        .urcm-toolbar:hover {
            opacity: 1;
        }

        .urcm-toolbar button {
            background: #0e639c;
            color: white;
            border: none;
            padding: 4px 10px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 13px;
            font-weight: 500;
            transition: background 0.2s;
        }

        .urcm-toolbar button:hover {
            background: #1177bb;
        }

        .urcm-toolbar select {
            background: #3c3c3c;
            color: white;
            border: 1px solid #3e3e42;
            border-radius: 4px;
            padding: 4px 8px;
            font-size: 13px;
        }

        .urcm-toolbar .status {
            color: #6a9955;
            font-size: 12px;
            padding: 4px 8px;
            border-left: 1px solid #3e3e42;
            margin-left: 4px;
        }

        .urcm-notification {
            position: fixed;
            bottom: 20px;
            right: 20px;
            background: #252526;
            color: #d4d4d4;
            border-left: 4px solid #0e639c;
            padding: 10px 16px;
            border-radius: 4px;
            font-family: sans-serif;
            font-size: 14px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.3);
            z-index: 10000;
            animation: slideIn 0.3s;
        }

        .urcm-notification.success {
            border-left-color: #6a9955;
        }

        .urcm-notification.error {
            border-left-color: #f48771;
        }

        @keyframes slideIn {
            from { transform: translateX(100%); opacity: 0; }
            to { transform: translateX(0); opacity: 1; }
        }
    `);

    // Crea la barra
    function createToolbar() {
        const toolbar = document.createElement('div');
        toolbar.className = 'urcm-toolbar';

        // Pulsante salva risposta
        const saveBtn = document.createElement('button');
        saveBtn.textContent = '💾 Salva risposta';
        saveBtn.onclick = saveLastResponse;

        // Pulsante salva conversazione
        const saveConvBtn = document.createElement('button');
        saveConvBtn.textContent = '💬 Salva chat';
        saveConvBtn.onclick = saveConversation;

        // Selettore formato
        const formatSelect = document.createElement('select');
        formatSelect.innerHTML = `
            <option value="sapri">SAPRI</option>
            <option value="txt">Testo</option>
            <option value="json">JSON</option>
        `;

        // Status indicator
        const status = document.createElement('span');
        status.className = 'status';
        status.textContent = '●';
        status.title = 'Connesso al server URCM';

        toolbar.appendChild(saveBtn);
        toolbar.appendChild(saveConvBtn);
        toolbar.appendChild(formatSelect);
        toolbar.appendChild(status);

        return toolbar;
    }

    // Mostra notifica
    function showNotification(message, type = 'info') {
        const notif = document.createElement('div');
        notif.className = `urcm-notification ${type}`;
        notif.textContent = message;
        document.body.appendChild(notif);

        setTimeout(() => {
            notif.remove();
        }, 3000);
    }
    
	function getLastResponse() {
	    // Per ChatGPT
	    const chatGptMessages = document.querySelectorAll('[data-message-author-role="assistant"]');
	    if (chatGptMessages.length > 0) {
	        const last = chatGptMessages[chatGptMessages.length - 1];
	        return last.textContent;
	    }
	
	    // Per Claude
	    const claudeMessages = document.querySelectorAll('.font-claude-message');
	    if (claudeMessages.length > 0) {
	        const last = claudeMessages[claudeMessages.length - 1];
	        return last.textContent;
	    }
	
	    // Per Gemini
	    const geminiMessages = document.querySelectorAll('.message-content');
	    if (geminiMessages.length > 0) {
	        const last = geminiMessages[geminiMessages.length - 1];
	        return last.textContent;
	    }
	
	    // Per Deepseek
	    const dsMessages = document.querySelectorAll('.ds-markdown');
	    if (dsMessages.length > 0) {
	        // Prendi l'ultimo messaggio (risposta più recente)
	        const last = dsMessages[dsMessages.length - 1];
	        return last.textContent;
	    }
	    
	    // Alternativa per Deepseek
	    const messageElements = document.querySelectorAll('[class*="message"]');
	    if (messageElements.length > 0) {
	        const last = messageElements[messageElements.length - 1];
	        return last.textContent;
	    }
	
	    return null;
	}

    // Estrai tutta la conversazione
    function getConversation() {
        let conversation = [];

        // Per ChatGPT
        const messages = document.querySelectorAll('[data-message-author-role]');
        messages.forEach(msg => {
            const role = msg.getAttribute('data-message-author-role');
            const text = msg.textContent;
            conversation.push({ role, text });
        });
        
        if (window.location.host.includes('deepseek.com')) {
		    const messages = document.querySelectorAll('.fbb737a4, .ds-markdown');
		    messages.forEach(msg => {
		        const isUser = msg.closest('[class*="user"]') || msg.previousElementSibling?.textContent.includes('You');
		        conversation.push({
		            role: isUser ? 'user' : 'assistant',
		            text: msg.textContent
		        });
		    });
		    return conversation;
		}
	    
	    return conversation;
    }

    // Salva ultima risposta
    function saveLastResponse() {
        const response = getLastResponse();
        if (!response) {
            showNotification('Nessuna risposta trovata', 'error');
            return;
        }

        const format = document.querySelector('.urcm-toolbar select').value;
        const filename = `chat_${new Date().toISOString().slice(0,19).replace(/:/g, '-')}.${format}`;


		// Nelle funzioni saveLastResponse e saveConversation, cambia:
		GM_xmlhttpRequest({
		    method: 'POST',
		    url: `${SERVER_URL}/api/ai/save`,  // nuovo endpoint,
		    headers: {
		        'Content-Type': 'application/json'
		    },
		    data: JSON.stringify({
		        content: response,
		        filename: filename
		    }),
		
            onload: function(response) {
                if (response.status === 200) {
                    showNotification(`✅ Salvato come ${filename}`, 'success');
                } else {
                    showNotification(`❌ Errore: ${response.status}`, 'error');
                }
            },
            onerror: function() {
                showNotification('❌ Server non raggiungibile', 'error');
            }
        });
    }

    // Salva conversazione
    function saveConversation() {
        const conversation = getConversation();
        if (!conversation.length) {
            showNotification('Nessuna conversazione trovata', 'error');
            return;
        }

        const format = document.querySelector('.urcm-toolbar select').value;
        let content = '';

        if (format === 'json') {
            content = JSON.stringify(conversation, null, 2);
        } else {
            conversation.forEach(msg => {
                content += `[${msg.role}]\n${msg.text}\n\n`;
            });
        }

        const filename = `chat_full_${new Date().toISOString().slice(0,19).replace(/:/g, '-')}.${format}`;

        GM_xmlhttpRequest({
            method: 'POST',
            url: `${SERVER_URL}/api/files/${filename}`,
            headers: {
                'Content-Type': 'text/plain'
            },
            data: content,
            onload: function(response) {
                if (response.status === 200) {
                    showNotification(`✅ Conversazione salvata`, 'success');
                } else {
                    showNotification(`❌ Errore: ${response.status}`, 'error');
                }
            },
            onerror: function() {
                showNotification('❌ Server non raggiungibile', 'error');
            }
        });
    }

    // Inizializza quando la pagina è pronta
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            document.body.appendChild(createToolbar());
        });
    } else {
        document.body.appendChild(createToolbar());
    }

    // Verifica server ogni 30 secondi
    setInterval(() => {
        GM_xmlhttpRequest({
            method: 'GET',
            url: `${SERVER_URL}/api/files`,
            onload: function(response) {
                const status = document.querySelector('.urcm-toolbar .status');
                if (response.status === 200) {
                    status.style.color = '#6a9955';
                    status.title = 'Connesso al server URCM';
                } else {
                    status.style.color = '#f48771';
                    status.title = 'Server non raggiungibile';
                }
            },
            onerror: function() {
                const status = document.querySelector('.urcm-toolbar .status');
                status.style.color = '#f48771';
                status.title = 'Server non raggiungibile';
            }
        });
    }, 30000);
})();
