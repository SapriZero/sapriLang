// Componenti UI riutilizzabili

const Components = {
    // Toolbar principale
    renderToolbar: function(currentPage) {
        return `
            <div class="toolbar">
                <div class="toolbar-logo">📝 URCM</div>
                <div class="toolbar-menu">
                    <button onclick="window.location.href='/'" ${currentPage === 'home' ? 'class="active"' : ''}>Home</button>
                    <button onclick="window.location.href='/editor'" ${currentPage === 'editor' ? 'class="active"' : ''}>Editor</button>
                </div>
                <div class="toolbar-right">
                    <span class="status-indicator" id="server-status">
                        <span class="status-dot" id="status-dot"></span>
                        <span id="status-text">Server</span>
                    </span>
                </div>
            </div>
        `;
    },

    // Sidebar sinistra
    renderSidebar: function(files = [], currentFile = null) {
        let fileListHtml = files.map(f => `
            <li class="${currentFile === f.name ? 'active' : ''}"
                onclick="window.loadFile('${f.name}')">
                📄 ${f.name}
            </li>
        `).join('');

        return `
            <h3>📁 Scripts</h3>
            <div class="new-file-input">
                <input type="text" id="new-filename" placeholder="nuovo.sapri">
                <button onclick="window.createFile()">+</button>
            </div>
            <ul class="file-list">
                ${fileListHtml || '<li style="color: #666;">Nessun file</li>'}
            </ul>
        `;
    },

    // Notifica
    showNotification: function(message, type = 'info', duration = 3000) {
        const notif = document.createElement('div');
        notif.className = `notification ${type}`;
        notif.textContent = message;
        document.body.appendChild(notif);

        setTimeout(() => notif.remove(), duration);
    },

    // Aggiorna stato server
    updateServerStatus: function(connected) {
        const dot = document.getElementById('status-dot');
        const text = document.getElementById('status-text');

        if (connected) {
            dot.className = 'status-dot status-connected';
            text.textContent = 'Server connesso';
        } else {
            dot.className = 'status-dot status-disconnected';
            text.textContent = 'Server offline';
        }
    }
};

// Esponi globalmente
window.Components = Components;
