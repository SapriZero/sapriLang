// Main per la pagina home

document.addEventListener('DOMContentLoaded', async function() {
    console.log('Main.js loaded');

    // Render toolbar
    const toolbarContainer = document.getElementById('toolbar-container');
    if (toolbarContainer) {
        toolbarContainer.innerHTML = Components.renderToolbar('home');
    }

    // Render sidebar vuota
    const sidebarContainer = document.getElementById('sidebar-container');
    if (sidebarContainer) {
        sidebarContainer.innerHTML = Components.renderSidebar([], null);
    }

    // Verifica connessione server
    checkServerStatus();
    setInterval(checkServerStatus, 30000);
    
        console.log('Main.js loaded');
        
    console.log('toolbar-container:', document.getElementById('toolbar-container'));
    console.log('sidebar-container:', document.getElementById('sidebar-container'));
    console.log('content-container:', document.getElementById('content-container'));
});

async function checkServerStatus() {
    try {
        const response = await fetch('/api/files');
        Components.updateServerStatus(response.ok);
    } catch {
        Components.updateServerStatus(false);
    }
}

